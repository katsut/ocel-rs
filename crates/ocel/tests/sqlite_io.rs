use std::path::PathBuf;

use chrono::{DateTime, Utc};
use ocel::io::sqlite;
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

fn string_attr_def(name: &str) -> AttributeDefinition {
    AttributeDefinition {
        name: name.to_owned(),
        value_type: AttrType::String,
    }
}

/// A model shaped like what the `SQLite` reader produces (String values, String
/// attribute types), so it round-trips exactly.
fn sample() -> Ocel {
    Ocel {
        event_types: vec![EventType {
            name: "place order".into(),
            attributes: vec![string_attr_def("channel")],
        }],
        object_types: vec![
            ObjectType {
                name: "customer".into(),
                attributes: vec![],
            },
            ObjectType {
                name: "order".into(),
                attributes: vec![string_attr_def("total")],
            },
        ],
        events: vec![Event {
            id: "e1".into(),
            event_type: "place order".into(),
            time: ts(1_000_000),
            attributes: vec![EventAttribute {
                name: "channel".into(),
                value: AttrValue::String("web".into()),
            }],
            relationships: vec![
                Relationship {
                    object_id: "c1".into(),
                    qualifier: "placed by".into(),
                },
                Relationship {
                    object_id: "o1".into(),
                    qualifier: "order".into(),
                },
            ],
        }],
        objects: vec![
            Object {
                id: "c1".into(),
                object_type: "customer".into(),
                attributes: vec![],
                relationships: vec![],
            },
            Object {
                id: "o1".into(),
                object_type: "order".into(),
                attributes: vec![
                    ObjectAttribute {
                        name: "total".into(),
                        value: AttrValue::String("100".into()),
                        time: ts(0),
                    },
                    ObjectAttribute {
                        name: "total".into(),
                        value: AttrValue::String("120".into()),
                        time: ts(2_000_000),
                    },
                ],
                relationships: vec![Relationship {
                    object_id: "c1".into(),
                    qualifier: "placed by".into(),
                }],
            },
        ],
    }
}

#[test]
fn write_read_content_and_stability() {
    let path = tmp("ocel-rs-sqlite-content.db");
    sqlite::write_path(&sample(), &path).unwrap();
    let m1 = sqlite::read_path(&path).unwrap();

    assert_eq!(m1.events.len(), 1);
    assert_eq!(m1.objects.len(), 2);

    let order = m1.objects.iter().find(|o| o.id == "o1").unwrap();
    assert_eq!(
        order.attribute_at("total", ts(1_000)),
        Some(&AttrValue::String("100".into()))
    );
    assert_eq!(
        order.attribute_at("total", ts(3_000_000)),
        Some(&AttrValue::String("120".into()))
    );

    assert_eq!(m1.e2o().filter(|r| r.event_id == "e1").count(), 2);
    assert!(m1.o2o().any(|r| r.source_id == "o1" && r.target_id == "c1"));

    // writing what we read reproduces the same model (stability).
    let path2 = tmp("ocel-rs-sqlite-content-2.db");
    sqlite::write_path(&m1, &path2).unwrap();
    let m2 = sqlite::read_path(&path2).unwrap();
    assert_eq!(m1, m2);

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&path2);
}

fn official_path() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures/official/ocel20_example.sqlite");
    p
}

/// Round-trip the official `PM4Py` `SQLite` example. Skips when absent; run
/// `sh scripts/fetch-official-fixtures.sh` to enable it.
#[test]
fn official_sqlite_round_trips_if_present() {
    let path = official_path();
    if !path.exists() {
        eprintln!(
            "skipping: official fixture not present ({})",
            path.display()
        );
        return;
    }
    let a = sqlite::read_path(&path).unwrap();
    assert_eq!(a.events.len(), 13);
    assert_eq!(a.objects.len(), 9);
    assert_eq!(a.e2o().count(), 20);
    assert_eq!(a.o2o().count(), 7);

    // known dynamic change: PO1 po_quantity 500 -> 600
    let po1 = a.objects.iter().find(|o| o.id == "PO1").unwrap();
    assert_eq!(
        po1.attribute_at(
            "po_quantity",
            DateTime::parse_from_rfc3339("2022-06-01T00:00:00Z")
                .unwrap()
                .to_utc()
        ),
        Some(&AttrValue::String("600".into()))
    );

    let path2 = tmp("ocel-rs-sqlite-official-roundtrip.db");
    sqlite::write_path(&a, &path2).unwrap();
    let b = sqlite::read_path(&path2).unwrap();
    assert_eq!(a, b);
    let _ = std::fs::remove_file(&path2);
}
