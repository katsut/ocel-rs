//! OCEL 2.0 `SQLite` reader and writer.
//!
//! Follows the OCEL 2.0 relational schema: `event_map_type` / `object_map_type`
//! map readable type names to sanitized per-type table suffixes; base tables
//! `event` / `object` / `event_object` / `object_object`; and one `event_<Type>`
//! / `object_<Type>` table per type. Declared attribute types are encoded as the
//! column type of the per-type tables (`INTEGER` / `REAL` / `BOOLEAN` /
//! `TIMESTAMP` / `TEXT`) and values are stored natively, so typed logs round-trip.
//! Files written by tools that use plain `TEXT` columns (e.g. `PM4Py`) read
//! leniently as strings.

use std::collections::BTreeMap;
use std::path::Path;

use chrono::{DateTime, Utc};
use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection};

use crate::io::coerce::{apply_declared_types, parse_time_lenient};
use crate::io::IoError;
use crate::model::{
    AttrType, AttrValue, AttributeDefinition, Event, EventAttribute, EventType, Object,
    ObjectAttribute, ObjectType, Ocel, Relationship,
};

const CHANGED_FIELD: &str = "ocel_changed_field";

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

fn sanitize_suffix(name: &str) -> String {
    name.chars().filter(char::is_ascii_alphanumeric).collect()
}

fn parse_time(s: &str) -> Result<DateTime<Utc>, IoError> {
    parse_time_lenient(s).ok_or_else(|| IoError::Format(format!("invalid timestamp: {s}")))
}

fn format_time(t: DateTime<Utc>) -> String {
    t.naive_utc().format("%Y-%m-%d %H:%M:%S%.f").to_string()
}

/// The `SQLite` column type used to encode a declared attribute type.
fn column_type(ty: AttrType) -> &'static str {
    match ty {
        AttrType::String => "TEXT",
        AttrType::Integer => "INTEGER",
        AttrType::Float => "REAL",
        AttrType::Boolean => "BOOLEAN",
        AttrType::Time => "TIMESTAMP",
    }
}

/// Recover a declared attribute type from a column declaration.
fn attr_type_of_column(decl: &str) -> AttrType {
    let d = decl.to_ascii_uppercase();
    if d.contains("INT") {
        AttrType::Integer
    } else if d.contains("REAL") || d.contains("FLOA") || d.contains("DOUB") {
        AttrType::Float
    } else if d.contains("BOOL") {
        AttrType::Boolean
    } else if d.contains("TIME") || d.contains("DATE") {
        AttrType::Time
    } else {
        AttrType::String
    }
}

fn attr_defs(cols: &[(String, String)]) -> Vec<AttributeDefinition> {
    cols.iter()
        .map(|(name, decl)| AttributeDefinition {
            name: name.clone(),
            value_type: attr_type_of_column(decl),
        })
        .collect()
}

fn attr_columns_ddl(attributes: &[AttributeDefinition]) -> String {
    let mut ddl = String::new();
    for a in attributes {
        ddl.push_str(", ");
        ddl.push_str(&quote_ident(&a.name));
        ddl.push(' ');
        ddl.push_str(column_type(a.value_type));
    }
    ddl
}

/// Convert a raw `SQLite` value to an attribute value (`None` for NULL/blob).
/// Declaration-driven coercion refines it afterwards.
fn attr_from_sql(value: Value) -> Option<AttrValue> {
    match value {
        Value::Null | Value::Blob(_) => None,
        Value::Integer(i) => Some(AttrValue::Integer(i)),
        Value::Real(f) => Some(AttrValue::Float(f)),
        Value::Text(s) => Some(AttrValue::String(s)),
    }
}

/// Convert an attribute value to its native `SQLite` storage value.
fn sql_value(value: &AttrValue) -> Value {
    match value {
        AttrValue::String(s) => Value::Text(s.clone()),
        AttrValue::Integer(i) => Value::Integer(*i),
        AttrValue::Float(f) => Value::Real(*f),
        AttrValue::Boolean(b) => Value::Integer(i64::from(*b)),
        AttrValue::Time(t) => Value::Text(format_time(*t)),
    }
}

fn epoch() -> DateTime<Utc> {
    DateTime::from_timestamp(0, 0).expect("unix epoch is valid")
}

// ---------------------------------------------------------------------------
// read
// ---------------------------------------------------------------------------

/// Read an [`Ocel`] from an OCEL 2.0 `SQLite` file.
pub fn read_path<P: AsRef<Path>>(path: P) -> Result<Ocel, IoError> {
    let conn = Connection::open(path)?;
    read_connection(&conn)
}

fn read_connection(conn: &Connection) -> Result<Ocel, IoError> {
    let mut event_types = Vec::new();
    let mut events = Vec::new();
    for (name, suffix) in read_type_map(conn, "event_map_type")? {
        let table = format!("event_{suffix}");
        let attr_cols = attr_columns(conn, &table, false)?;
        event_types.push(EventType {
            name: name.clone(),
            attributes: attr_defs(&attr_cols),
        });
        let names: Vec<String> = attr_cols.into_iter().map(|(name, _)| name).collect();
        read_events_of_type(conn, &table, &name, &names, &mut events)?;
    }

    let mut object_types = Vec::new();
    let mut objects = Vec::new();
    for (name, suffix) in read_type_map(conn, "object_map_type")? {
        let table = format!("object_{suffix}");
        let has_changed = columns_of(conn, &table)?
            .iter()
            .any(|(name, _)| name == CHANGED_FIELD);
        let attr_cols = attr_columns(conn, &table, has_changed)?;
        object_types.push(ObjectType {
            name: name.clone(),
            attributes: attr_defs(&attr_cols),
        });
        let names: Vec<String> = attr_cols.into_iter().map(|(name, _)| name).collect();
        read_objects_of_type(conn, &table, &name, &names, has_changed, &mut objects)?;
    }

    attach_e2o(conn, &mut events)?;
    attach_o2o(conn, &mut objects)?;

    let mut ocel = Ocel {
        event_types,
        object_types,
        events,
        objects,
    };
    apply_declared_types(&mut ocel);
    Ok(ocel)
}

/// Column (name, declared type) pairs of `table`.
fn columns_of(conn: &Connection, table: &str) -> Result<Vec<(String, String)>, IoError> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", quote_ident(table)))?;
    let cols = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>("name")?, row.get::<_, String>("type")?))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(cols)
}

fn attr_columns(
    conn: &Connection,
    table: &str,
    has_changed: bool,
) -> Result<Vec<(String, String)>, IoError> {
    let cols = columns_of(conn, table)?;
    Ok(cols
        .into_iter()
        .filter(|(name, _)| {
            name != "ocel_id" && name != "ocel_time" && !(has_changed && name == CHANGED_FIELD)
        })
        .collect())
}

fn read_type_map(conn: &Connection, table: &str) -> Result<Vec<(String, String)>, IoError> {
    let mut stmt = conn.prepare(&format!(
        "SELECT ocel_type, ocel_type_map FROM {} ORDER BY ocel_type",
        quote_ident(table)
    ))?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

fn read_events_of_type(
    conn: &Connection,
    table: &str,
    type_name: &str,
    attr_cols: &[String],
    events: &mut Vec<Event>,
) -> Result<(), IoError> {
    let mut stmt = conn.prepare(&format!(
        "SELECT * FROM {} ORDER BY ocel_id",
        quote_ident(table)
    ))?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let id: String = row.get("ocel_id")?;
        let time = parse_time(&row.get::<_, String>("ocel_time")?)?;
        let mut attributes = Vec::new();
        for col in attr_cols {
            if let Some(value) = attr_from_sql(row.get::<_, Value>(col.as_str())?) {
                attributes.push(EventAttribute {
                    name: col.clone(),
                    value,
                });
            }
        }
        events.push(Event {
            id,
            event_type: type_name.to_owned(),
            time,
            attributes,
            relationships: Vec::new(),
        });
    }
    Ok(())
}

fn read_objects_of_type(
    conn: &Connection,
    table: &str,
    type_name: &str,
    attr_cols: &[String],
    has_changed: bool,
    objects: &mut Vec<Object>,
) -> Result<(), IoError> {
    let mut stmt = conn.prepare(&format!(
        "SELECT * FROM {} ORDER BY ocel_id, ocel_time",
        quote_ident(table)
    ))?;
    let mut rows = stmt.query([])?;
    let mut grouped: BTreeMap<String, Vec<ObjectAttribute>> = BTreeMap::new();
    while let Some(row) = rows.next()? {
        let id: String = row.get("ocel_id")?;
        let time = parse_time(&row.get::<_, String>("ocel_time")?)?;
        let changed: Option<String> = if has_changed {
            row.get(CHANGED_FIELD)?
        } else {
            None
        };
        let entry = grouped.entry(id).or_default();
        match changed.as_deref() {
            Some(field) if !field.is_empty() => {
                if let Some(value) = attr_from_sql(row.get::<_, Value>(field)?) {
                    entry.push(ObjectAttribute {
                        name: field.to_owned(),
                        value,
                        time,
                    });
                }
            }
            _ => {
                for col in attr_cols {
                    if let Some(value) = attr_from_sql(row.get::<_, Value>(col.as_str())?) {
                        entry.push(ObjectAttribute {
                            name: col.clone(),
                            value,
                            time,
                        });
                    }
                }
            }
        }
    }
    for (id, attributes) in grouped {
        objects.push(Object {
            id,
            object_type: type_name.to_owned(),
            attributes,
            relationships: Vec::new(),
        });
    }
    Ok(())
}

fn attach_e2o(conn: &Connection, events: &mut [Event]) -> Result<(), IoError> {
    let index: BTreeMap<&str, usize> = events
        .iter()
        .enumerate()
        .map(|(i, e)| (e.id.as_str(), i))
        .collect();
    let mut positions = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT ocel_event_id, ocel_object_id, ocel_qualifier FROM event_object \
         ORDER BY ocel_event_id, ocel_object_id, ocel_qualifier",
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let event_id: String = row.get(0)?;
        let object_id: String = row.get(1)?;
        let qualifier: String = row.get(2)?;
        if let Some(&i) = index.get(event_id.as_str()) {
            positions.push((
                i,
                Relationship {
                    object_id,
                    qualifier,
                },
            ));
        }
    }
    for (i, rel) in positions {
        events[i].relationships.push(rel);
    }
    Ok(())
}

fn attach_o2o(conn: &Connection, objects: &mut [Object]) -> Result<(), IoError> {
    let index: BTreeMap<&str, usize> = objects
        .iter()
        .enumerate()
        .map(|(i, o)| (o.id.as_str(), i))
        .collect();
    let mut positions = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT ocel_source_id, ocel_target_id, ocel_qualifier FROM object_object \
         ORDER BY ocel_source_id, ocel_target_id, ocel_qualifier",
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let source_id: String = row.get(0)?;
        let object_id: String = row.get(1)?;
        let qualifier: String = row.get(2)?;
        if let Some(&i) = index.get(source_id.as_str()) {
            positions.push((
                i,
                Relationship {
                    object_id,
                    qualifier,
                },
            ));
        }
    }
    for (i, rel) in positions {
        objects[i].relationships.push(rel);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// write
// ---------------------------------------------------------------------------

/// Write an [`Ocel`] to an OCEL 2.0 `SQLite` file, overwriting it if it exists.
pub fn write_path<P: AsRef<Path>>(ocel: &Ocel, path: P) -> Result<(), IoError> {
    let path = path.as_ref();
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    let mut conn = Connection::open(path)?;
    write_connection(&mut conn, ocel)?;
    Ok(())
}

fn write_connection(conn: &mut Connection, ocel: &Ocel) -> Result<(), IoError> {
    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;
         CREATE TABLE \"event_map_type\" (\"ocel_type\" TEXT, \"ocel_type_map\" TEXT, PRIMARY KEY(\"ocel_type\"));
         CREATE TABLE \"object_map_type\" (\"ocel_type\" TEXT, \"ocel_type_map\" TEXT, PRIMARY KEY(\"ocel_type\"));
         CREATE TABLE \"event\" (\"ocel_id\" TEXT, \"ocel_type\" TEXT, PRIMARY KEY(\"ocel_id\"));
         CREATE TABLE \"object\" (\"ocel_id\" TEXT, \"ocel_type\" TEXT, PRIMARY KEY(\"ocel_id\"));
         CREATE TABLE \"event_object\" (\"ocel_event_id\" TEXT, \"ocel_object_id\" TEXT, \"ocel_qualifier\" TEXT, PRIMARY KEY(\"ocel_event_id\",\"ocel_object_id\",\"ocel_qualifier\"));
         CREATE TABLE \"object_object\" (\"ocel_source_id\" TEXT, \"ocel_target_id\" TEXT, \"ocel_qualifier\" TEXT, PRIMARY KEY(\"ocel_source_id\",\"ocel_target_id\",\"ocel_qualifier\"));",
    )?;
    let tx = conn.transaction()?;
    let event_suffix = write_event_types(&tx, ocel)?;
    let object_suffix = write_object_types(&tx, ocel)?;
    write_base_rows(&tx, ocel)?;
    write_relations(&tx, ocel)?;
    write_events(&tx, ocel, &event_suffix)?;
    write_objects(&tx, ocel, &object_suffix)?;
    tx.commit()?;
    Ok(())
}

fn write_event_types(conn: &Connection, ocel: &Ocel) -> Result<BTreeMap<String, String>, IoError> {
    let mut suffixes = BTreeMap::new();
    for et in &ocel.event_types {
        let suffix = sanitize_suffix(&et.name);
        conn.execute(
            "INSERT INTO \"event_map_type\" (\"ocel_type\", \"ocel_type_map\") VALUES (?, ?)",
            (&et.name, &suffix),
        )?;
        let attr_ddl = attr_columns_ddl(&et.attributes);
        conn.execute(
            &format!(
                "CREATE TABLE {} (\"ocel_id\" TEXT, \"ocel_time\" TIMESTAMP{attr_ddl}, \
                 PRIMARY KEY(\"ocel_id\"), FOREIGN KEY(\"ocel_id\") REFERENCES \"event\"(\"ocel_id\"))",
                quote_ident(&format!("event_{suffix}"))
            ),
            [],
        )?;
        suffixes.insert(et.name.clone(), suffix);
    }
    Ok(suffixes)
}

fn write_object_types(conn: &Connection, ocel: &Ocel) -> Result<BTreeMap<String, String>, IoError> {
    let mut suffixes = BTreeMap::new();
    for ot in &ocel.object_types {
        let suffix = sanitize_suffix(&ot.name);
        conn.execute(
            "INSERT INTO \"object_map_type\" (\"ocel_type\", \"ocel_type_map\") VALUES (?, ?)",
            (&ot.name, &suffix),
        )?;
        let attr_ddl = attr_columns_ddl(&ot.attributes);
        let changed_ddl = if object_type_has_changes(ocel, &ot.name) {
            format!(", {} TEXT", quote_ident(CHANGED_FIELD))
        } else {
            String::new()
        };
        conn.execute(
            &format!(
                "CREATE TABLE {} (\"ocel_id\" TEXT{attr_ddl}, \"ocel_time\" TIMESTAMP{changed_ddl}, \
                 FOREIGN KEY(\"ocel_id\") REFERENCES \"object\"(\"ocel_id\"))",
                quote_ident(&format!("object_{suffix}"))
            ),
            [],
        )?;
        suffixes.insert(ot.name.clone(), suffix);
    }
    Ok(suffixes)
}

fn object_type_has_changes(ocel: &Ocel, type_name: &str) -> bool {
    ocel.objects
        .iter()
        .filter(|o| o.object_type == type_name)
        .any(|o| {
            let mut times: Vec<_> = o.attributes.iter().map(|a| a.time).collect();
            times.sort_unstable();
            times.dedup();
            times.len() > 1
        })
}

fn write_base_rows(conn: &Connection, ocel: &Ocel) -> Result<(), IoError> {
    for e in &ocel.events {
        conn.execute(
            "INSERT INTO \"event\" (\"ocel_id\", \"ocel_type\") VALUES (?, ?)",
            (&e.id, &e.event_type),
        )?;
    }
    for o in &ocel.objects {
        conn.execute(
            "INSERT INTO \"object\" (\"ocel_id\", \"ocel_type\") VALUES (?, ?)",
            (&o.id, &o.object_type),
        )?;
    }
    Ok(())
}

fn write_relations(conn: &Connection, ocel: &Ocel) -> Result<(), IoError> {
    for r in ocel.e2o() {
        conn.execute(
            "INSERT OR IGNORE INTO \"event_object\" \
             (\"ocel_event_id\", \"ocel_object_id\", \"ocel_qualifier\") VALUES (?, ?, ?)",
            (r.event_id, r.object_id, r.qualifier),
        )?;
    }
    for r in ocel.o2o() {
        conn.execute(
            "INSERT OR IGNORE INTO \"object_object\" \
             (\"ocel_source_id\", \"ocel_target_id\", \"ocel_qualifier\") VALUES (?, ?, ?)",
            (r.source_id, r.target_id, r.qualifier),
        )?;
    }
    Ok(())
}

fn insert_columns(attr_cols: &[&str], object: bool, has_changed: bool) -> (String, usize) {
    let mut cols: Vec<String> = vec!["ocel_id".to_owned()];
    if object {
        cols.extend(attr_cols.iter().map(|c| (*c).to_owned()));
        cols.push("ocel_time".to_owned());
        if has_changed {
            cols.push(CHANGED_FIELD.to_owned());
        }
    } else {
        cols.push("ocel_time".to_owned());
        cols.extend(attr_cols.iter().map(|c| (*c).to_owned()));
    }
    let count = cols.len();
    let list = cols
        .iter()
        .map(|c| quote_ident(c))
        .collect::<Vec<_>>()
        .join(", ");
    (list, count)
}

fn write_events(
    conn: &Connection,
    ocel: &Ocel,
    suffixes: &BTreeMap<String, String>,
) -> Result<(), IoError> {
    for et in &ocel.event_types {
        let suffix = &suffixes[&et.name];
        let attr_cols: Vec<&str> = et.attributes.iter().map(|a| a.name.as_str()).collect();
        let (col_list, count) = insert_columns(&attr_cols, false, false);
        let placeholders = vec!["?"; count].join(", ");
        let sql = format!(
            "INSERT INTO {} ({col_list}) VALUES ({placeholders})",
            quote_ident(&format!("event_{suffix}"))
        );
        let mut stmt = conn.prepare(&sql)?;
        for e in ocel.events.iter().filter(|e| e.event_type == et.name) {
            let values = event_row_values(e, &attr_cols);
            stmt.execute(params_from_iter(values))?;
        }
    }
    Ok(())
}

fn event_row_values(event: &Event, attr_cols: &[&str]) -> Vec<Value> {
    let attrs: BTreeMap<&str, &AttrValue> = event
        .attributes
        .iter()
        .map(|a| (a.name.as_str(), &a.value))
        .collect();
    let mut values = vec![
        Value::Text(event.id.clone()),
        Value::Text(format_time(event.time)),
    ];
    for &col in attr_cols {
        values.push(value_or_null(attrs.get(col).copied()));
    }
    values
}

fn write_objects(
    conn: &Connection,
    ocel: &Ocel,
    suffixes: &BTreeMap<String, String>,
) -> Result<(), IoError> {
    for ot in &ocel.object_types {
        let suffix = &suffixes[&ot.name];
        let attr_cols: Vec<&str> = ot.attributes.iter().map(|a| a.name.as_str()).collect();
        let has_changed = object_type_has_changes(ocel, &ot.name);
        let (col_list, count) = insert_columns(&attr_cols, true, has_changed);
        let placeholders = vec!["?"; count].join(", ");
        let sql = format!(
            "INSERT INTO {} ({col_list}) VALUES ({placeholders})",
            quote_ident(&format!("object_{suffix}"))
        );
        let mut stmt = conn.prepare(&sql)?;
        for o in ocel.objects.iter().filter(|o| o.object_type == ot.name) {
            for values in object_row_values(o, &attr_cols, has_changed) {
                stmt.execute(params_from_iter(values))?;
            }
        }
    }
    Ok(())
}

fn object_row_values(object: &Object, attr_cols: &[&str], has_changed: bool) -> Vec<Vec<Value>> {
    if object.attributes.is_empty() {
        let mut row = vec![Value::Text(object.id.clone())];
        row.extend(attr_cols.iter().map(|_| Value::Null));
        row.push(Value::Text(format_time(epoch())));
        if has_changed {
            row.push(Value::Null);
        }
        return vec![row];
    }

    let earliest = object
        .attributes
        .iter()
        .map(|a| a.time)
        .min()
        .expect("object has attributes");

    let initial: BTreeMap<&str, &AttrValue> = object
        .attributes
        .iter()
        .filter(|a| a.time == earliest)
        .map(|a| (a.name.as_str(), &a.value))
        .collect();
    let mut rows = Vec::new();
    let mut initial_row = vec![Value::Text(object.id.clone())];
    for &col in attr_cols {
        initial_row.push(value_or_null(initial.get(col).copied()));
    }
    initial_row.push(Value::Text(format_time(earliest)));
    if has_changed {
        initial_row.push(Value::Null);
    }
    rows.push(initial_row);

    let mut changes: Vec<&ObjectAttribute> = object
        .attributes
        .iter()
        .filter(|a| a.time > earliest)
        .collect();
    changes.sort_by_key(|a| a.time);
    for change in changes {
        let mut row = vec![Value::Text(object.id.clone())];
        for &col in attr_cols {
            if col == change.name {
                row.push(sql_value(&change.value));
            } else {
                row.push(Value::Null);
            }
        }
        row.push(Value::Text(format_time(change.time)));
        row.push(Value::Text(change.name.clone()));
        rows.push(row);
    }
    rows
}

fn value_or_null(value: Option<&AttrValue>) -> Value {
    value.map_or(Value::Null, sql_value)
}
