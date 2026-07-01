use std::path::PathBuf;

use chrono::{DateTime, Utc};
use ocel_core::io::{json, sqlite, xml};
use ocel_core::{
    AttrType, AttributeDefinition, Event, EventType, ObjectType, Ocel, Relationship, Violation,
};

fn ts(secs: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(secs, 0).unwrap()
}

fn fixture(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures");
    p.push(name);
    p
}

#[test]
fn valid_fixture_passes() {
    let ocel = json::read_path(fixture("order_management_small.json")).unwrap();
    assert_eq!(ocel.validate(), Ok(()));
}

#[test]
fn minimal_passes() {
    let ocel = json::read_path(fixture("minimal.json")).unwrap();
    assert_eq!(ocel.validate(), Ok(()));
}

#[test]
fn detects_dangling_e2o_and_undeclared_type() {
    let ocel = Ocel {
        event_types: vec![],
        object_types: vec![],
        events: vec![Event {
            id: "e1".into(),
            event_type: "ghost".into(),
            time: ts(0),
            attributes: vec![],
            relationships: vec![Relationship {
                object_id: "missing".into(),
                qualifier: "q".into(),
            }],
        }],
        objects: vec![],
    };
    let violations = ocel.validate().unwrap_err();
    assert!(violations.contains(&Violation::UndeclaredEventType {
        event: "e1".into(),
        event_type: "ghost".into(),
    }));
    assert!(violations.contains(&Violation::DanglingE2O {
        event: "e1".into(),
        object: "missing".into(),
    }));
}

#[test]
fn detects_undeclared_attribute() {
    let ocel = Ocel {
        event_types: vec![EventType {
            name: "t".into(),
            attributes: vec![],
        }],
        object_types: vec![],
        events: vec![Event {
            id: "e1".into(),
            event_type: "t".into(),
            time: ts(0),
            attributes: vec![ocel_core::EventAttribute {
                name: "surprise".into(),
                value: ocel_core::AttrValue::String("x".into()),
            }],
            relationships: vec![],
        }],
        objects: vec![],
    };
    assert!(ocel
        .validate()
        .unwrap_err()
        .contains(&Violation::UndeclaredEventAttribute {
            event: "e1".into(),
            attribute: "surprise".into(),
        }));
}

#[test]
fn detects_attribute_name_collision() {
    let dup = || AttributeDefinition {
        name: "shared".into(),
        value_type: AttrType::String,
    };
    let ocel = Ocel {
        event_types: vec![],
        object_types: vec![
            ObjectType {
                name: "a".into(),
                attributes: vec![dup()],
            },
            ObjectType {
                name: "b".into(),
                attributes: vec![dup()],
            },
        ],
        events: vec![],
        objects: vec![],
    };
    assert!(ocel
        .validate()
        .unwrap_err()
        .contains(&Violation::AttributeNameCollision {
            attribute: "shared".into(),
        }));
}

/// The official `PM4Py` examples validate cleanly across all three formats.
/// Skips when absent; run `sh scripts/fetch-official-fixtures.sh` to enable it.
#[test]
fn official_examples_validate_if_present() {
    let json_path = fixture("official/ocel20_example.jsonocel");
    if !json_path.exists() {
        eprintln!("skipping: official fixtures not present");
        return;
    }
    assert_eq!(json::read_path(&json_path).unwrap().validate(), Ok(()));
    assert_eq!(
        sqlite::read_path(fixture("official/ocel20_example.sqlite"))
            .unwrap()
            .validate(),
        Ok(())
    );
    assert_eq!(
        xml::read_path(fixture("official/ocel20_example.xmlocel"))
            .unwrap()
            .validate(),
        Ok(())
    );
}
