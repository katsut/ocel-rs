//! Cross-format type fidelity: the same logical log read from JSON, `SQLite`,
//! or `XML` must produce the identical typed [`Ocel`].

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use ocel::io::{json, sqlite, xml};
use ocel::{
    AttrType, AttrValue, AttributeDefinition, Event, EventAttribute, EventType, Object,
    ObjectAttribute, ObjectType, Ocel, Relationship,
};

fn ts(secs: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(secs, 0).unwrap()
}

fn tmp(name: &str) -> PathBuf {
    std::env::temp_dir().join(name)
}

fn def(name: &str, ty: AttrType) -> AttributeDefinition {
    AttributeDefinition {
        name: name.to_owned(),
        value_type: ty,
    }
}

/// A typed model covering all five attribute types.
///
/// Ordering is chosen so every format reads back in the same order the model
/// declares (types sorted by name; events/objects grouped by type, ids ascending;
/// relationships sorted by (`object_id`, qualifier); attributes in declaration
/// order, changes chronological) — `SQLite` reads in exactly that order.
fn sample() -> Ocel {
    Ocel {
        event_types: vec![EventType {
            name: "pay".into(),
            attributes: vec![
                def("amount", AttrType::Float),
                def("count", AttrType::Integer),
                def("express", AttrType::Boolean),
                def("note", AttrType::String),
                def("due", AttrType::Time),
            ],
        }],
        object_types: vec![ObjectType {
            name: "order".into(),
            attributes: vec![
                def("total", AttrType::Float),
                def("open", AttrType::Boolean),
            ],
        }],
        events: vec![Event {
            id: "e1".into(),
            event_type: "pay".into(),
            time: ts(1_000_000),
            attributes: vec![
                EventAttribute {
                    name: "amount".into(),
                    value: AttrValue::Float(120.5),
                },
                EventAttribute {
                    name: "count".into(),
                    value: AttrValue::Integer(3),
                },
                EventAttribute {
                    name: "express".into(),
                    value: AttrValue::Boolean(true),
                },
                EventAttribute {
                    name: "note".into(),
                    value: AttrValue::String("rush".into()),
                },
                EventAttribute {
                    name: "due".into(),
                    value: AttrValue::Time(ts(2_000_000)),
                },
            ],
            relationships: vec![Relationship {
                object_id: "o1".into(),
                qualifier: "order".into(),
            }],
        }],
        objects: vec![
            Object {
                id: "o1".into(),
                object_type: "order".into(),
                attributes: vec![
                    ObjectAttribute {
                        name: "total".into(),
                        value: AttrValue::Float(100.0),
                        time: ts(0),
                    },
                    ObjectAttribute {
                        name: "open".into(),
                        value: AttrValue::Boolean(true),
                        time: ts(0),
                    },
                    ObjectAttribute {
                        name: "total".into(),
                        value: AttrValue::Float(120.5),
                        time: ts(1_500_000),
                    },
                ],
                relationships: vec![Relationship {
                    object_id: "o2".into(),
                    qualifier: "related".into(),
                }],
            },
            Object {
                id: "o2".into(),
                object_type: "order".into(),
                attributes: vec![],
                relationships: vec![],
            },
        ],
    }
}

#[test]
fn json_round_trip_preserves_types() {
    let model = sample();
    let back = json::read_str(&json::write_string(&model).unwrap()).unwrap();
    assert_eq!(model, back);
}

#[test]
fn xml_round_trip_preserves_types() {
    let model = sample();
    let back = xml::read_str(&xml::write_string(&model).unwrap()).unwrap();
    assert_eq!(model, back);
}

#[test]
fn sqlite_round_trip_preserves_types() {
    let model = sample();
    let path = tmp("ocel-rs-typed-io.sqlite");
    sqlite::write_path(&model, &path).unwrap();
    let back = sqlite::read_path(&path).unwrap();
    assert_eq!(model, back);
    let _ = std::fs::remove_file(&path);
}

/// The full chain: JSON -> `SQLite` -> XML and back, staying identical throughout.
#[test]
fn cross_format_chain_is_lossless() {
    let model = sample();

    let from_json = json::read_str(&json::write_string(&model).unwrap()).unwrap();

    let sqlite_path = tmp("ocel-rs-typed-chain.sqlite");
    sqlite::write_path(&from_json, &sqlite_path).unwrap();
    let from_sqlite = sqlite::read_path(&sqlite_path).unwrap();

    let from_xml = xml::read_str(&xml::write_string(&from_sqlite).unwrap()).unwrap();

    assert_eq!(model, from_json);
    assert_eq!(model, from_sqlite);
    assert_eq!(model, from_xml);
    let _ = std::fs::remove_file(&sqlite_path);
}

/// Official-style JSON stores values as strings; declared types coerce them.
#[test]
fn string_values_coerce_to_declared_types() {
    let text = r#"{
      "eventTypes": [
        { "name": "pay", "attributes": [
          { "name": "amount", "type": "float" },
          { "name": "count", "type": "integer" },
          { "name": "express", "type": "boolean" }
        ]}
      ],
      "objectTypes": [],
      "events": [
        { "id": "e1", "type": "pay", "time": "2023-01-01T00:00:00Z",
          "attributes": [
            { "name": "amount", "value": "120.5" },
            { "name": "count", "value": "3" },
            { "name": "express", "value": "true" }
          ],
          "relationships": [] }
      ],
      "objects": []
    }"#;
    let ocel = json::read_str(text).unwrap();
    let attrs = &ocel.events[0].attributes;
    assert_eq!(attrs[0].value, AttrValue::Float(120.5));
    assert_eq!(attrs[1].value, AttrValue::Integer(3));
    assert_eq!(attrs[2].value, AttrValue::Boolean(true));
}
