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
fn reading_a_missing_file_creates_nothing() {
    let path = tmp("ocel-rs-sqlite-does-not-exist.db");
    let _ = std::fs::remove_file(&path);
    assert!(sqlite::read_path(&path).is_err());
    // the read-only open must not have created an empty database
    assert!(!path.exists());
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

/// Timestamp ties must resolve identically across formats: the reader must
/// restore the master tables' (write) order, not the per-type-table grouping.
/// Here the log order and the alphabetical type order disagree on purpose
/// (`open issue` written before its same-second `label`, `zeta` before
/// `alpha`) — type-grouped assembly would flip both pairs.
#[test]
fn read_restores_write_order_not_type_grouping() {
    let bare_event = |id: &str, event_type: &str| Event {
        id: id.into(),
        event_type: event_type.into(),
        time: ts(1_000_000),
        attributes: vec![],
        relationships: vec![],
    };
    let bare_object = |id: &str, object_type: &str| Object {
        id: id.into(),
        object_type: object_type.into(),
        attributes: vec![],
        relationships: vec![],
    };
    let model = Ocel {
        event_types: vec![
            EventType {
                name: "open issue".into(),
                attributes: vec![],
            },
            EventType {
                name: "label".into(),
                attributes: vec![],
            },
        ],
        object_types: vec![
            ObjectType {
                name: "zeta".into(),
                attributes: vec![],
            },
            ObjectType {
                name: "alpha".into(),
                attributes: vec![],
            },
        ],
        events: vec![
            bare_event("e-open", "open issue"),
            bare_event("e-label", "label"),
        ],
        objects: vec![
            bare_object("o-zeta", "zeta"),
            bare_object("o-alpha", "alpha"),
        ],
    };

    let path = tmp("ocel-rs-sqlite-order.db");
    sqlite::write_path(&model, &path).unwrap();
    let back = sqlite::read_path(&path).unwrap();

    let event_ids: Vec<&str> = back.events.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(event_ids, vec!["e-open", "e-label"]);
    let object_ids: Vec<&str> = back.objects.iter().map(|o| o.id.as_str()).collect();
    assert_eq!(object_ids, vec!["o-zeta", "o-alpha"]);

    let _ = std::fs::remove_file(&path);
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

/// Non-ASCII type names (and names that sanitize to the same suffix) must
/// keep distinct per-type tables — found importing a Japanese workflow CSV
/// where 申請 / 承認 / 支払 all collapsed to a colliding empty suffix.
#[test]
fn unicode_and_colliding_type_names_round_trip() {
    let bare_event = |id: &str, event_type: &str| Event {
        id: id.into(),
        event_type: event_type.into(),
        time: ts(1_000),
        attributes: vec![],
        relationships: vec![],
    };
    let model = Ocel {
        event_types: vec![
            EventType {
                name: "申請".into(),
                attributes: vec![],
            },
            EventType {
                name: "承認".into(),
                attributes: vec![],
            },
            EventType {
                name: "type!".into(),
                attributes: vec![],
            },
            EventType {
                name: "type?".into(),
                attributes: vec![],
            },
        ],
        object_types: vec![ObjectType {
            name: "申請書".into(),
            attributes: vec![],
        }],
        events: vec![
            bare_event("e1", "申請"),
            bare_event("e2", "承認"),
            bare_event("e3", "type!"),
            bare_event("e4", "type?"),
        ],
        objects: vec![Object {
            id: "k1".into(),
            object_type: "申請書".into(),
            attributes: vec![],
            relationships: vec![],
        }],
    };

    let path = tmp("ocel-rs-sqlite-unicode-types.db");
    sqlite::write_path(&model, &path).unwrap();
    let back = sqlite::read_path(&path).unwrap();
    assert_eq!(back.events.len(), 4);
    let types: Vec<&str> = back.events.iter().map(|e| e.event_type.as_str()).collect();
    assert_eq!(types, vec!["申請", "承認", "type!", "type?"]);
    assert_eq!(back.objects[0].object_type, "申請書");
    let _ = std::fs::remove_file(&path);
}
