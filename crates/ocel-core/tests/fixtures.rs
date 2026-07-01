use std::path::PathBuf;

use ocel_core::{AttrValue, Ocel};

fn fixture(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn parse(name: &str) -> Ocel {
    serde_json::from_str(&fixture(name)).unwrap_or_else(|e| panic!("parse {name}: {e}"))
}

/// Rebuild a parsed log through the validation gate, asserting it is a valid OCEL log.
fn validate(ocel: &Ocel) -> Ocel {
    let mut b = Ocel::builder();
    for t in ocel.event_types.iter().cloned() {
        b.add_event_type(t);
    }
    for t in ocel.object_types.iter().cloned() {
        b.add_object_type(t);
    }
    for o in ocel.objects.iter().cloned() {
        b.add_object(o);
    }
    for e in ocel.events.iter().cloned() {
        b.add_event(e);
    }
    b.build().expect("fixture must be a valid OCEL log")
}

#[test]
fn minimal_parses() {
    let ocel = parse("minimal.json");
    assert!(ocel.event_types.is_empty());
    assert!(ocel.events.is_empty());
    assert!(ocel.objects.is_empty());
    assert_eq!(validate(&ocel), ocel);
}

#[test]
fn order_management_parses_and_validates() {
    let ocel = parse("order_management_small.json");
    assert_eq!(ocel.events.len(), 3);
    assert_eq!(ocel.objects.len(), 5);
    assert_eq!(validate(&ocel), ocel);
}

#[test]
fn order_management_dynamic_attribute() {
    let ocel = parse("order_management_small.json");
    let order = ocel.objects.iter().find(|o| o.id == "o1").unwrap();
    let before = chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
        .unwrap()
        .to_utc();
    let after = chrono::DateTime::parse_from_rfc3339("2023-06-01T00:00:00Z")
        .unwrap()
        .to_utc();
    assert_eq!(
        order.attribute_at("total", before),
        Some(&AttrValue::Float(100.0))
    );
    assert_eq!(
        order.attribute_at("total", after),
        Some(&AttrValue::Float(120.0))
    );
}

#[test]
fn order_management_e2o_flattened() {
    let ocel = parse("order_management_small.json");
    let count = ocel.e2o().filter(|r| r.event_id == "e1").count();
    assert_eq!(count, 4);
    assert!(ocel
        .o2o()
        .any(|r| r.source_id == "o1" && r.target_id == "c1"));
}

#[test]
fn edge_cases_duplicate_relationship_preserved() {
    let ocel = parse("edge_cases.json");
    // duplicated E2O (same object + qualifier) is permitted and preserved.
    assert_eq!(ocel.events[0].relationships.len(), 2);
    assert_eq!(validate(&ocel), ocel);
}
