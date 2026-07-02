//! Rust-native benchmark over the Zenodo Order Management log (21K events).
//!
//! Run with:
//! ```sh
//! sh scripts/fetch-official-fixtures.sh --large
//! cargo run -p ocel-core --example bench --release
//! ```

use std::path::PathBuf;
use std::time::Instant;

use ocel_core::io::{json, sqlite, xml};
use ocel_core::Ocel;

const RUNS: usize = 7;

fn median_ms(mut f: impl FnMut()) -> f64 {
    let mut times: Vec<f64> = (0..RUNS)
        .map(|_| {
            let t0 = Instant::now();
            f();
            t0.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    times.sort_by(f64::total_cmp);
    times[times.len() / 2]
}

fn main() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/official");
    let sqlite_path = dir.join("order-management.sqlite");
    if !sqlite_path.exists() {
        eprintln!(
            "large fixtures not present — run: sh scripts/fetch-official-fixtures.sh --large"
        );
        std::process::exit(1);
    }
    let json_path = dir.join("order-management.json");
    let xml_path = dir.join("order-management.xml");

    let log: Ocel = sqlite::read_path(&sqlite_path).unwrap();
    println!(
        "Order Management: {} events / {} objects (median of {RUNS} runs)\n",
        log.events.len(),
        log.objects.len(),
    );

    let rows: Vec<(&str, f64)> = vec![
        (
            "read SQLite",
            median_ms(|| drop(sqlite::read_path(&sqlite_path).unwrap())),
        ),
        (
            "read JSON",
            median_ms(|| drop(json::read_path(&json_path).unwrap())),
        ),
        (
            "read XML",
            median_ms(|| drop(xml::read_path(&xml_path).unwrap())),
        ),
        ("validate", median_ms(|| drop(log.validate()))),
        ("object graph + components", {
            median_ms(|| drop(log.object_graph().connected_components()))
        }),
        ("filter by 3 event types", {
            let types = ["place order", "confirm order", "pay order"];
            median_ms(|| drop(log.filter_event_types(&types)))
        }),
        ("write SQLite", {
            let out = std::env::temp_dir().join("ocel-bench-out.sqlite");
            let ms = median_ms(|| sqlite::write_path(&log, &out).unwrap());
            let _ = std::fs::remove_file(&out);
            ms
        }),
    ];

    println!("| operation | time |");
    println!("|---|---:|");
    for (name, ms) in rows {
        println!("| {name} | {ms:.0} ms |");
    }
}
