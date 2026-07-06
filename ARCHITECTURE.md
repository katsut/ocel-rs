# Architecture

How ocel-rs holds and moves OCEL 2.0 data.

## One in-memory model, three formats

`Ocel` is a plain struct-of-vecs (event types, object types, events, objects)
that mirrors the OCEL 2.0 exchange model. JSON, SQLite, and XML are readers
and writers over that one model, and **cross-format equality is the
contract**: reading what any writer wrote yields the same log, and event
order is the master-table order in every format — same-second events keep a
deterministic, format-independent order that downstream trace semantics rely
on.

## Declaration-driven typing

Attribute values are typed (`AttrValue`: string / integer / float / boolean /
time), and `io::coerce` applies the log's own type declarations while
reading, so a value that arrives as text in one format and as a native type
in another lands identically. Where declarations and reality disagree, the
official datasets win over the spec.

## Validation as a gate, not a suggestion

`validate()` returns the full violation list (dangling E2O/O2O references,
undeclared types, id collisions). Upstream, ocel-etl's `StagingLog` refuses
to emit an `Ocel` that does not pass — so everything downstream may assume a
well-formed log.

## OCEL-aware subsetting

`filter` and `sample` uphold the valid-sublog invariants: removing objects
drops the events that referenced only them, relationship targets always
exist, and connected-component sampling is deterministic. The object
interaction `graph` is std-only — no graph-library dependency.

## Bindings

`ocel-cli` is a thin `convert`/`validate` wrapper (deliberately minimal).
`ocel-py` exposes the same model to Python (`import ocel`) with columnar and
Arrow zero-copy outputs sized for Polars.
