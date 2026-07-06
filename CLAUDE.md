# CLAUDE.md — ocel-rs

The OCEL 2.0 core: data model, type-faithful JSON/SQLite/XML I/O, validation,
object graph, OCEL-aware filtering and sampling. Three crates in one
workspace; everything downstream (etl, mine, studio) builds on `ocel`.
Concepts in [ARCHITECTURE.md](ARCHITECTURE.md).

## Build, test, verify

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check
sh scripts/fetch-official-fixtures.sh --large   # Zenodo fixtures (21K log)
# python wheel:
uvx maturin build --manifest-path crates/ocel-py/Cargo.toml --release
```

## Map

- `crates/ocel/src/model/` — `Ocel`, `Event`, `Object`, `AttrValue`
  (`attr.rs`: typed values + `to_text`/`attr_type`), `builder.rs`
- `crates/ocel/src/io/` — `json.rs` / `sqlite.rs` / `xml.rs` +
  `coerce.rs` (declaration-driven typing) + `mod.rs`
  (`read_path`/`write_path` by extension)
- `crates/ocel/src/validate.rs` — well-formedness (`Violation` list)
- `crates/ocel/src/graph.rs` — object interaction graph (std only)
- `crates/ocel/src/filter.rs`, `sample.rs` — valid-sublog filtering,
  deterministic connected-component sampling
- `crates/ocel-cli` — bin `ocel` (`convert` / `validate`); kept minimal by
  policy (no feature investment)
- `crates/ocel-py` — PyO3 module `ocel` (abi3-py311), Arrow zero-copy
  columns for Polars

## Invariants and traps

- **Cross-format equality is the contract**: read(write(log)) must round-trip
  exactly across JSON/SQLite/XML; every reader restores the master
  `event`/`object` table order (v0.1.4 fixed type-grouped sqlite reads —
  same-second ordering is format-independent now).
- SQLite per-type table suffixes must survive non-ASCII names (Unicode
  alphanumerics + numeric disambiguators; Japanese event types broke once).
- Filters uphold the valid-sublog invariants — never hand-roll event/object
  removal outside `filter.rs` semantics.
- `ocel convert` opens sqlite inputs read-only (a CREATE-flag bug once
  materialized missing input files).
- Publish train: `ocel` → dependents; owner GO required; tag `vX.Y.Z`.
  crates.io API calls need a User-Agent header.

## Conventions

Issue → branch → PR → CI green → squash-merge. rustfmt via pre-commit hook
(`git config core.hooksPath .githooks`). Design docs live in the private
ocel-workspace, not here.
