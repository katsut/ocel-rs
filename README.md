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

All numbers: Zenodo [Order Management](https://doi.org/10.5281/zenodo.8337463)
(21,008 events / 10,840 objects), median of 7 runs, Apple M4 Max. Fetch the
dataset first: `sh scripts/fetch-official-fixtures.sh --large`.

### Embedded in Rust

Applications (e.g. process-mining tools) work on the model and graph directly —
no DataFrames involved:

| operation | time |
|---|---:|
| read SQLite | 57 ms |
| read JSON | 49 ms |
| read XML | 72 ms |
| validate | 3 ms |
| object graph + connected components | 26 ms |
| filter by 3 event types | 10 ms |
| write SQLite | 252 ms |

Reproduce: `cargo run -p ocel --example bench --release`

### From Python, vs pm4py

Python 3.13, pm4py 2.7.23. Both tools load identical events/objects/E2O/O2O
counts. To keep the comparison fair, the read rows include materializing **all
six** of ocel's columnar exports into Polars DataFrames, since pm4py's readers
return pandas DataFrames:

| operation | ocel (Rust) | pm4py | speedup |
|---|---:|---:|---:|
| read SQLite → DataFrames | 115 ms | 447 ms | 3.9x |
| read JSON → DataFrames | 111 ms | 603 ms | 5.4x |
| read XML → DataFrames | 133 ms | 410 ms | 3.1x |
| filter by 3 event types | 11 ms | 17 ms | 1.6x |
| write SQLite | 257 ms | 395 ms | 1.5x |

Python code that stays on `OcelLog` methods (filter / sample / validate) skips
the DataFrame cost entirely and runs at the Rust-native speeds above.

Reproduce with [`scripts/bench-pm4py-compare.py`](scripts/bench-pm4py-compare.py).

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
use ocel::io::{json, sqlite};

// Read an OCEL 2.0 JSON log and write it out as SQLite.
let ocel = json::read_path("log.jsonocel")?;
sqlite::write_path(&ocel, "log.sqlite")?;

// Validate, filter, sample.
ocel.validate().map_err(|v| format!("{v:?}"))?;
let sub = ocel.filter_event_types(&["place order"]);
let sample = ocel.sample_components(10);
```

See [`crates/ocel/examples/roundtrip.rs`](crates/ocel/examples/roundtrip.rs)
for a runnable example (`cargo run -p ocel --example roundtrip`).

## Quickstart (CLI)

```sh
ocel convert log.jsonocel log.sqlite   # format by file extension
ocel validate log.sqlite               # non-zero exit on violations
```

## Workspace

```
crates/
├── ocel/    # data model + I/O + validation + graph/filter/sampling
├── ocel-cli/     # `ocel` command-line tool
└── ocel-py/      # Python bindings (module name: ocel)
```

## License

MIT
