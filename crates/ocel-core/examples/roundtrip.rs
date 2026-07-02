//! Build a tiny OCEL 2.0 log, then round-trip it through JSON and `SQLite`.
//!
//! Run with: `cargo run -p ocel-core --example roundtrip`

use chrono::Utc;
use ocel_core::io::{json, sqlite};
use ocel_core::{Event, EventType, Object, ObjectType, Ocel, Relationship};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a small log through the validating builder.
    let mut builder = Ocel::builder();
    builder.add_event_type(EventType {
        name: "place order".into(),
        attributes: vec![],
    });
    builder.add_object_type(ObjectType {
        name: "order".into(),
        attributes: vec![],
    });
    builder.add_object(Object {
        id: "o1".into(),
        object_type: "order".into(),
        attributes: vec![],
        relationships: vec![],
    });
    builder.add_event(Event {
        id: "e1".into(),
        event_type: "place order".into(),
        time: Utc::now(),
        attributes: vec![],
        relationships: vec![Relationship {
            object_id: "o1".into(),
            qualifier: "order".into(),
        }],
    });
    let ocel = builder.build().map_err(|v| format!("{v:?}"))?;

    // Serialize to JSON.
    println!("{}", json::write_string(&ocel)?);

    // Write SQLite, read it back, and validate.
    let path = std::env::temp_dir().join("ocel-roundtrip-example.sqlite");
    sqlite::write_path(&ocel, &path)?;
    let restored = sqlite::read_path(&path)?;
    restored.validate().map_err(|v| format!("{v:?}"))?;
    println!(
        "restored {} event(s), {} object(s)",
        restored.events.len(),
        restored.objects.len()
    );
    std::fs::remove_file(&path)?;
    Ok(())
}
