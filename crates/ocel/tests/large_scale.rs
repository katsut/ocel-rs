//! Large-scale verification against the Zenodo "Order Management" log
//! (DOI 10.5281/zenodo.8337463, ~21K events).
//!
//! All tests skip when the fixtures are absent; run
//! `sh scripts/fetch-official-fixtures.sh --large` to enable them.

use std::path::PathBuf;

use ocel::io::{json, sqlite, xml};
use ocel::Ocel;

fn official(name: &str) -> Option<PathBuf> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures/official");
    p.push(name);
    if p.exists() {
        Some(p)
    } else {
        eprintln!("skipping: large fixture not present ({})", p.display());
        None
    }
}

fn assert_shape(ocel: &Ocel) {
    assert_eq!(ocel.events.len(), 21_008);
    assert_eq!(ocel.objects.len(), 10_840);
    assert_eq!(ocel.event_types.len(), 11);
    assert_eq!(ocel.object_types.len(), 6);
    assert_eq!(ocel.validate(), Ok(()));
}

#[test]
fn order_management_sqlite_round_trips() {
    let Some(path) = official("order-management.sqlite") else {
        return;
    };
    let a = sqlite::read_path(&path).unwrap();
    assert_shape(&a);

    let out = std::env::temp_dir().join("ocel-rs-om-roundtrip.sqlite");
    sqlite::write_path(&a, &out).unwrap();
    let b = sqlite::read_path(&out).unwrap();
    assert_eq!(a, b);
    let _ = std::fs::remove_file(&out);
}

#[test]
fn order_management_json_round_trips() {
    let Some(path) = official("order-management.json") else {
        return;
    };
    let a = json::read_path(&path).unwrap();
    assert_shape(&a);
    let b = json::read_str(&json::write_string(&a).unwrap()).unwrap();
    assert_eq!(a, b);
}

#[test]
fn order_management_xml_round_trips() {
    let Some(path) = official("order-management.xml") else {
        return;
    };
    let a = xml::read_path(&path).unwrap();
    assert_shape(&a);
    let b = xml::read_str(&xml::write_string(&a).unwrap()).unwrap();
    assert_eq!(a, b);
}
