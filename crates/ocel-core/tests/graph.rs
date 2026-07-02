use std::collections::BTreeSet;
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
fn neighbors_from_events_and_o2o() {
    let ocel = small();
    let graph = ocel.object_graph();
    // o1 interacts with c1/i1/i2 (co-occurrence in e1 + O2O), not with c2.
    let neighbors: Vec<_> = graph.neighbors("o1").collect();
    assert_eq!(neighbors, vec!["c1", "i1", "i2"]);
    // c2 never appears in any event or O2O.
    assert_eq!(graph.neighbors("c2").count(), 0);
    assert!(graph.contains("c2"));
    assert_eq!(graph.neighbors("unknown").count(), 0);
    assert!(!graph.contains("unknown"));
}

#[test]
fn components_partition_the_objects() {
    let ocel = small();
    let graph = ocel.object_graph();
    let components = graph.connected_components();
    // {c1, i1, i2, o1} connected via e1/O2O; c2 isolated.
    assert_eq!(components, vec![vec!["c1", "i1", "i2", "o1"], vec!["c2"]]);
}

#[test]
fn empty_log_has_empty_graph() {
    let ocel = json::read_path(fixture("minimal.json")).unwrap();
    let graph = ocel.object_graph();
    assert!(graph.is_empty());
    assert_eq!(graph.connected_components().len(), 0);
}

/// Invariants on real data: components partition the objects, co-occurring
/// objects share a component, and the decomposition is deterministic.
fn assert_graph_invariants(ocel: &Ocel) {
    let graph = ocel.object_graph();
    let components = graph.connected_components();

    // partition: every object exactly once
    let mut seen = BTreeSet::new();
    for component in &components {
        for id in component {
            assert!(seen.insert(*id), "object {id} appears in two components");
        }
    }
    assert_eq!(seen.len(), ocel.objects.len());

    // co-occurring objects share a component
    let component_of: std::collections::BTreeMap<&str, usize> = components
        .iter()
        .enumerate()
        .flat_map(|(i, c)| c.iter().map(move |id| (*id, i)))
        .collect();
    for event in &ocel.events {
        let mut ids = event.relationships.iter().map(|r| r.object_id.as_str());
        if let Some(first) = ids.next() {
            for other in ids {
                assert_eq!(
                    component_of[first], component_of[other],
                    "event {} spans components",
                    event.id
                );
            }
        }
    }

    // deterministic
    assert_eq!(components, ocel.object_graph().connected_components());
}

#[test]
fn official_example_graph_invariants_if_present() {
    let path = fixture("official/ocel20_example.sqlite");
    if !path.exists() {
        eprintln!("skipping: official fixture not present");
        return;
    }
    assert_graph_invariants(&sqlite::read_path(&path).unwrap());
}

#[test]
fn order_management_graph_invariants_if_present() {
    let path = fixture("official/order-management.sqlite");
    if !path.exists() {
        eprintln!("skipping: large fixture not present");
        return;
    }
    let ocel = sqlite::read_path(&path).unwrap();
    assert_graph_invariants(&ocel);
    // 21K-event log decomposes into a non-trivial number of components.
    let n = ocel.object_graph().connected_components().len();
    assert!(n >= 1, "expected at least one component, got {n}");
}
