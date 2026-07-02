use std::path::PathBuf;

use chrono::DateTime;
use ocel::io::{json, sqlite};
use ocel::Ocel;

fn fixture(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures");
    p.push(name);
    p
}

fn small() -> Ocel {
    json::read_path(fixture("order_management_small.json")).unwrap()
}

#[test]
fn filter_event_types_keeps_related_objects() {
    let ocel = small();
    let filtered = ocel.filter_event_types(&["place order"]);
    assert_eq!(filtered.validate(), Ok(()));
    // e1 only; it references o1, c1, i1, i2 — c2 (isolated) is dropped.
    // Objects keep their original order.
    assert_eq!(filtered.events.len(), 1);
    let ids: Vec<_> = filtered.objects.iter().map(|o| o.id.as_str()).collect();
    assert_eq!(ids, vec!["c1", "o1", "i1", "i2"]);
}

#[test]
fn filter_events_strips_o2o_to_dropped_objects() {
    let ocel = small();
    // keep only e2 ("confirm order"): references o1 alone.
    let filtered = ocel.filter_events(|e| e.id == "e2");
    assert_eq!(filtered.validate(), Ok(()));
    assert_eq!(filtered.objects.len(), 1);
    let o1 = &filtered.objects[0];
    assert_eq!(o1.id, "o1");
    // o1's O2O to c1/i1/i2 must be stripped (those objects are dropped).
    assert!(o1.relationships.is_empty());
}

#[test]
fn filter_time_range_is_inclusive() {
    let ocel = small();
    let from = DateTime::parse_from_rfc3339("2023-01-02T10:00:00Z")
        .unwrap()
        .to_utc();
    let to = DateTime::parse_from_rfc3339("2023-01-03T14:30:00Z")
        .unwrap()
        .to_utc();
    let filtered = ocel.filter_time_range(from, to);
    assert_eq!(filtered.validate(), Ok(()));
    let ids: Vec<_> = filtered.events.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(ids, vec!["e2", "e3"]);
}

#[test]
fn filter_object_types_drops_unrelated_events() {
    let ocel = small();
    // keep only items: e1 references i1/i2 -> kept (with stripped E2O);
    // e2/e3 reference no item -> dropped.
    let filtered = ocel.filter_object_types(&["item"]);
    assert_eq!(filtered.validate(), Ok(()));
    assert_eq!(filtered.events.len(), 1);
    assert_eq!(filtered.events[0].id, "e1");
    let rel_ids: Vec<_> = filtered.events[0]
        .relationships
        .iter()
        .map(|r| r.object_id.as_str())
        .collect();
    assert_eq!(rel_ids, vec!["i1", "i2"]);
    assert_eq!(filtered.objects.len(), 2);
}

#[test]
fn filter_everything_yields_empty_valid_log() {
    let ocel = small();
    let filtered = ocel.filter_events(|_| false);
    assert_eq!(filtered.validate(), Ok(()));
    assert!(filtered.events.is_empty());
    assert!(filtered.objects.is_empty());
    // type declarations are preserved
    assert_eq!(filtered.event_types, ocel.event_types);
}

/// Every filter operation yields a log that validates, on real data.
#[test]
fn filters_produce_valid_sublogs_on_official_data_if_present() {
    let path = fixture("official/ocel20_example.sqlite");
    if !path.exists() {
        eprintln!("skipping: official fixture not present");
        return;
    }
    let ocel = sqlite::read_path(&path).unwrap();

    let by_type = ocel.filter_event_types(&["Create Purchase Order"]);
    assert_eq!(by_type.validate(), Ok(()));
    assert!(!by_type.events.is_empty());

    let by_object = ocel.filter_object_types(&["Invoice", "Payment"]);
    assert_eq!(by_object.validate(), Ok(()));
    assert!(!by_object.events.is_empty());

    let mid = ocel.events[ocel.events.len() / 2].time;
    let by_time = ocel.filter_time_range(ocel.events[0].time, mid);
    assert_eq!(by_time.validate(), Ok(()));
}

#[test]
fn filters_scale_to_order_management_if_present() {
    let path = fixture("official/order-management.sqlite");
    if !path.exists() {
        eprintln!("skipping: large fixture not present");
        return;
    }
    let ocel = sqlite::read_path(&path).unwrap();
    let filtered = ocel.filter_object_types(&["orders", "items"]);
    assert_eq!(filtered.validate(), Ok(()));
    assert!(!filtered.events.is_empty());
    assert!(filtered.events.len() <= ocel.events.len());
    // only orders/items objects survive
    assert!(filtered.objects.len() < ocel.objects.len());
    assert!(filtered
        .objects
        .iter()
        .all(|o| o.object_type == "orders" || o.object_type == "items"));
}
