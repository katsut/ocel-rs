# ocel (Python)

Python bindings for [ocel-rs](https://github.com/katsut/ocel-rs) — read, write,
validate, filter, and sample [OCEL 2.0](https://www.ocel-standard.org/) event
logs, powered by Rust.

## Install (local development)

```sh
cd crates/ocel-py
maturin develop  # or: maturin build && pip install target/wheels/ocel-*.whl
```

## Quickstart

```python
import ocel
import polars as pl

log = ocel.read("order-management.sqlite")   # .json/.jsonocel, .sqlite/.db, .xml/.xmlocel
print(log)                                   # OcelLog(events=21008, objects=10840, ...)

assert log.validate() == []                  # spec-conformance violations (empty = valid)

events = pl.DataFrame(log.events())          # id / type / time columns
rels = pl.DataFrame(log.relations())         # E2O: event_id / object_id / qualifier

# attribute values are typed (str/int/float/bool/datetime), so the long-format
# value column is mixed-type — pass strict=False to Polars:
attrs = pl.DataFrame(log.object_attributes(), strict=False)

sub = log.filter_event_types(["place order"])  # consistent sub-log
sample = log.sample_components(10)             # connected-components sampling
sample.write_json("sample.json")
```

## API

- `ocel.read(path)` / `read_json` / `read_sqlite` / `read_xml` → `OcelLog`
- `OcelLog.write_json/write_sqlite/write_xml(path)`
- `OcelLog.validate() -> list[str]`
- Columnar exports (feed straight into Polars/pandas):
  `events()`, `event_attributes()`, `objects()`, `object_attributes()`,
  `relations()`, `o2o()`
- OCEL-aware operations: `filter_event_types(names)`,
  `filter_object_types(names)`, `sample_components(n)`,
  `connected_components()`
- Properties: `num_events`, `num_objects`
