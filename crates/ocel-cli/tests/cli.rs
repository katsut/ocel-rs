use std::path::PathBuf;
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_ocel-cli")
}

fn fixture(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("../ocel-core/tests/fixtures");
    p.push(name);
    p
}

fn tmp(name: &str) -> PathBuf {
    std::env::temp_dir().join(name)
}

#[test]
fn convert_json_to_sqlite_then_validate() {
    let out = tmp("ocel-cli-convert.sqlite");
    let status = Command::new(bin())
        .arg("convert")
        .arg(fixture("order_management_small.json"))
        .arg(&out)
        .status()
        .unwrap();
    assert!(status.success());
    assert!(out.exists());

    let status = Command::new(bin())
        .arg("validate")
        .arg(&out)
        .status()
        .unwrap();
    assert!(status.success());

    let _ = std::fs::remove_file(&out);
}

#[test]
fn convert_json_to_xml() {
    let out = tmp("ocel-cli-convert.xmlocel");
    let status = Command::new(bin())
        .arg("convert")
        .arg(fixture("order_management_small.json"))
        .arg(&out)
        .status()
        .unwrap();
    assert!(status.success());
    assert!(out.exists());
    let _ = std::fs::remove_file(&out);
}

#[test]
fn validate_missing_file_fails() {
    let status = Command::new(bin())
        .arg("validate")
        .arg(tmp("ocel-cli-does-not-exist.jsonocel"))
        .status()
        .unwrap();
    assert!(!status.success());
}

#[test]
fn unknown_extension_fails() {
    let status = Command::new(bin())
        .arg("validate")
        .arg(fixture("order_management_small.txt"))
        .status()
        .unwrap();
    assert!(!status.success());
}
