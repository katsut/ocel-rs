use std::path::PathBuf;

use ocel_core::io::{json, sqlite};
use ocel_core::Ocel;

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
fn sample_first_component_keeps_its_events() {
    let ocel = small();
    // components: [c1, i1, i2, o1], [c2]
    let sampled = ocel.sample_components(1);
    assert_eq!(sampled.validate(), Ok(()));
    // every event references the first component; c2 is dropped.
    assert_eq!(sampled.events.len(), 3);
    assert_eq!(sampled.objects.len(), 4);
    assert!(!sampled.objects.iter().any(|o| o.id == "c2"));
}

#[test]
fn sample_zero_components_is_empty() {
    let ocel = small();
    let sampled = ocel.sample_components(0);
    assert_eq!(sampled.validate(), Ok(()));
    assert!(sampled.events.is_empty());
    assert!(sampled.objects.is_empty());
}

#[test]
fn sample_more_components_than_exist_keeps_everything() {
    let ocel = small();
    let sampled = ocel.sample_components(10);
    assert_eq!(sampled.validate(), Ok(()));
    assert_eq!(sampled.events.len(), ocel.events.len());
    assert_eq!(sampled.objects.len(), ocel.objects.len());
}

#[test]
fn filter_components_by_membership() {
    let ocel = small();
    // keep only the isolated component {c2}: no event references it.
    let sampled = ocel.filter_components(|c| c.contains(&"c2"));
    assert_eq!(sampled.validate(), Ok(()));
    assert!(sampled.events.is_empty());
    let ids: Vec<_> = sampled.objects.iter().map(|o| o.id.as_str()).collect();
    assert_eq!(ids, vec!["c2"]);
}

#[test]
fn sampling_is_deterministic() {
    let ocel = small();
    assert_eq!(ocel.sample_components(1), ocel.sample_components(1));
}

#[test]
fn sampling_scales_to_order_management_if_present() {
    let path = fixture("official/order-management.sqlite");
    if !path.exists() {
        eprintln!("skipping: large fixture not present");
        return;
    }
    let ocel = sqlite::read_path(&path).unwrap();
    let total = ocel.object_graph().connected_components().len();
    let sampled = ocel.sample_components(total.min(3));
    assert_eq!(sampled.validate(), Ok(()));
    assert!(!sampled.objects.is_empty());
    if total > 3 {
        assert!(sampled.objects.len() < ocel.objects.len());
        assert!(sampled.events.len() < ocel.events.len());
    }
}
