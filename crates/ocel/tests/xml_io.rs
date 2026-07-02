use std::path::PathBuf;

use chrono::{DateTime, Utc};
use ocel::io::xml;
use ocel::{
    AttrType, AttrValue, AttributeDefinition, Event, EventAttribute, EventType, Object,
    ObjectAttribute, ObjectType, Ocel, Relationship,
};

fn ts(secs: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(secs, 0).unwrap()
}

fn string_attr_def(name: &str) -> AttributeDefinition {
    AttributeDefinition {
        name: name.to_owned(),
        value_type: AttrType::String,
    }
}

/// A model shaped like what the XML reader produces (String values), so it
/// round-trips exactly (XML preserves order).
fn sample() -> Ocel {
    Ocel {
        event_types: vec![EventType {
            name: "place order".into(),
            attributes: vec![string_attr_def("channel")],
        }],
        object_types: vec![ObjectType {
            name: "order".into(),
            attributes: vec![string_attr_def("total")],
        }],
        events: vec![Event {
            id: "e1".into(),
            event_type: "place order".into(),
            time: ts(1_000_000),
            attributes: vec![EventAttribute {
                name: "channel".into(),
                value: AttrValue::String("web".into()),
            }],
            relationships: vec![Relationship {
                object_id: "o1".into(),
                qualifier: "order".into(),
            }],
        }],
        objects: vec![Object {
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
            relationships: vec![],
        }],
    }
}

#[test]
fn round_trip_sample() {
    let model = sample();
    let text = xml::write_string(&model).unwrap();
    let back = xml::read_str(&text).unwrap();
    assert_eq!(model, back);
}

#[test]
fn dynamic_attribute_survives_round_trip() {
    let back = xml::read_str(&xml::write_string(&sample()).unwrap()).unwrap();
    let order = back.objects.iter().find(|o| o.id == "o1").unwrap();
    assert_eq!(
        order.attribute_at("total", ts(3_000_000)),
        Some(&AttrValue::String("120".into()))
    );
}

fn official_path() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures/official/ocel20_example.xmlocel");
    p
}

/// Round-trip the official `PM4Py` `XML` example. Skips when absent; run
/// `sh scripts/fetch-official-fixtures.sh` to enable it.
#[test]
fn official_xml_round_trips_if_present() {
    let path = official_path();
    if !path.exists() {
        eprintln!(
            "skipping: official fixture not present ({})",
            path.display()
        );
        return;
    }
    let a = xml::read_path(&path).unwrap();
    assert_eq!(a.events.len(), 13);
    assert_eq!(a.objects.len(), 9);
    assert_eq!(a.e2o().count(), 20);
    assert_eq!(a.o2o().count(), 7);

    // R3 is_blocked: No -> Yes -> No
    let r3 = a.objects.iter().find(|o| o.id == "R3").unwrap();
    let after = DateTime::parse_from_rfc3339("2022-03-01T00:00:00Z")
        .unwrap()
        .to_utc();
    assert_eq!(
        r3.attribute_at("is_blocked", after),
        Some(&AttrValue::String("No".into()))
    );

    let b = xml::read_str(&xml::write_string(&a).unwrap()).unwrap();
    assert_eq!(a, b);
}
