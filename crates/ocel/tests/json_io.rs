use std::path::PathBuf;

use ocel::io::json;

fn fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(name);
    path
}

fn round_trip(name: &str) {
    let a = json::read_path(fixture_path(name)).unwrap_or_else(|e| panic!("read {name}: {e}"));
    let out = json::write_string(&a).unwrap();
    let b = json::read_str(&out).unwrap();
    assert_eq!(a, b, "round-trip mismatch for {name}");
}

#[test]
fn round_trip_minimal() {
    round_trip("minimal.json");
}

#[test]
fn round_trip_order_management() {
    round_trip("order_management_small.json");
}

#[test]
fn round_trip_edge_cases() {
    round_trip("edge_cases.json");
}

#[test]
fn write_then_read_path() {
    let a = json::read_path(fixture_path("order_management_small.json")).unwrap();
    let tmp = std::env::temp_dir().join("ocel-rs-json-io-roundtrip.json");
    json::write_path(&a, &tmp).unwrap();
    let b = json::read_path(&tmp).unwrap();
    assert_eq!(a, b);
    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn read_str_rejects_invalid_json() {
    assert!(json::read_str("{ not json").is_err());
}

/// Extended round-trip against the official `PM4Py` example.
/// Skips when the (non-committed) fixture is absent; run
/// `sh scripts/fetch-official-fixtures.sh` to enable it.
#[test]
fn official_example_round_trips_if_present() {
    let path = fixture_path("official/ocel20_example.jsonocel");
    if !path.exists() {
        eprintln!(
            "skipping: official fixture not present ({})",
            path.display()
        );
        return;
    }
    let a = json::read_path(&path).unwrap();
    let out = json::write_string(&a).unwrap();
    let b = json::read_str(&out).unwrap();
    assert_eq!(a, b);
}
