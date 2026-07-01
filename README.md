# ocel-rs

An OCEL 2.0 data model and I/O library for Rust — read, write, convert, and
validate [OCEL 2.0](https://www.ocel-standard.org/) event logs across the JSON,
SQLite, and XML formats.

> 🚧 **Early development.** The v0.1 core (data model + 3-format I/O + validation
> + CLI) is taking shape; the API may still change before the first release.

## Features

- OCEL 2.0 data model (events, objects, E2O/O2O relationships, dynamic object attributes)
- Read/write **JSON**, **SQLite**, and **XML**, with round-trip fidelity against the official PM4Py examples
- Spec-conformance validation
- `ocel` CLI for conversion and validation

## Quickstart (library)

```rust
use ocel_core::io::{json, sqlite};

// Read an OCEL 2.0 JSON log and write it out as SQLite.
let ocel = json::read_path("log.jsonocel")?;
sqlite::write_path(&ocel, "log.sqlite")?;

// Check it conforms to the OCEL 2.0 spec.
ocel.validate().map_err(|violations| format!("{violations:?}"))?;
```

See [`crates/ocel-core/examples/roundtrip.rs`](crates/ocel-core/examples/roundtrip.rs)
for a runnable example (`cargo run -p ocel-core --example roundtrip`).

## Quickstart (CLI)

```sh
# Convert between formats (chosen by file extension).
ocel convert log.jsonocel log.sqlite

# Validate a log against the OCEL 2.0 specification.
ocel validate log.sqlite
```

## Workspace

```
crates/
├── ocel-core/    # data model + I/O + validation
└── ocel-cli/     # `ocel` command-line tool
```

## License

MIT
