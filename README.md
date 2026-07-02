# ocel-rs

An [OCEL 2.0](https://www.ocel-standard.org/) toolkit for Rust and Python —
read, write, convert, validate, filter, and sample object-centric event logs.

> 🚧 **Unreleased.** The v0.1 core and Python bindings are feature-complete and
> verified against the official PM4Py example and the 21K-event Zenodo
> Order Management log; the API may still change before the first release.

## Features

- **OCEL 2.0-native data model** — events, objects, qualified E2O/O2O
  relationships, dynamic (timestamped) object attributes, typed attribute values
- **Three formats, one model** — JSON / SQLite / XML read+write with
  declaration-driven typing: the same log reads identically from any format,
  and round-trips losslessly across them
- **Validation** — spec-conformance checks tuned against the official datasets
- **Object interaction graph** — neighbors and connected components
- **OCEL-aware filtering & sampling** — filters and connected-components
  sampling that always produce valid sub-logs (events, objects, and relations
  stay consistent)
- **`ocel` CLI** — convert and validate from the command line
- **Python bindings** — the `ocel` module with columnar exports that feed
  straight into Polars/pandas
- **MIT licensed** — note that PM4Py is AGPL-3.0; ocel-rs is a permissive
  alternative for the OCEL 2.0 I/O + preprocessing layer

## Performance

Same data (Zenodo [Order Management](https://doi.org/10.5281/zenodo.8337463),
21,008 events / 10,840 objects), same operations, median of 3 runs — Apple M4
Max, Python 3.13, pm4py 2.7.23:

| operation | ocel (Rust) | pm4py | speedup |
|---|---:|---:|---:|
| read SQLite (21K events) | 60 ms | 425 ms | 7.1x |
| read JSON | 51 ms | 586 ms | 11.6x |
| read XML | 71 ms | 390 ms | 5.5x |
| filter by 3 event types | 10 ms | 16 ms | 1.7x |
| write SQLite | 256 ms | 413 ms | 1.6x |

Reproduce with [`scripts/bench-pm4py-compare.py`](scripts/bench-pm4py-compare.py)
(after `sh scripts/fetch-official-fixtures.sh --large`).

## Quickstart (Python)

```sh
cd crates/ocel-py && maturin develop  # local build; PyPI release pending
```

```python
import ocel
import polars as pl

log = ocel.read("order-management.sqlite")     # .json/.jsonocel, .sqlite/.db, .xml/.xmlocel
assert log.validate() == []                    # spec-conformance (empty = valid)

events = pl.DataFrame(log.events())            # id / type / time
rels = pl.DataFrame(log.relations())           # E2O: event_id / object_id / qualifier
attrs = pl.DataFrame(log.object_attributes(), strict=False)  # typed values -> mixed column

sub = log.filter_event_types(["place order"])  # consistent sub-log
sample = log.sample_components(10)             # connected-components sampling
sample.write_json("sample.json")
```

## Quickstart (Rust)

```rust
use ocel_core::io::{json, sqlite};

// Read an OCEL 2.0 JSON log and write it out as SQLite.
let ocel = json::read_path("log.jsonocel")?;
sqlite::write_path(&ocel, "log.sqlite")?;

// Validate, filter, sample.
ocel.validate().map_err(|v| format!("{v:?}"))?;
let sub = ocel.filter_event_types(&["place order"]);
let sample = ocel.sample_components(10);
```

See [`crates/ocel-core/examples/roundtrip.rs`](crates/ocel-core/examples/roundtrip.rs)
for a runnable example (`cargo run -p ocel-core --example roundtrip`).

## Quickstart (CLI)

```sh
ocel convert log.jsonocel log.sqlite   # format by file extension
ocel validate log.sqlite               # non-zero exit on violations
```

## Workspace

```
crates/
├── ocel-core/    # data model + I/O + validation + graph/filter/sampling
├── ocel-cli/     # `ocel` command-line tool
└── ocel-py/      # Python bindings (module name: ocel)
```

## License

MIT
