"""Fair, reproducible comparison of ocel (Rust bindings) vs pm4py.

Same data, same operations, median of 7 runs. Prints a Markdown table.

Fairness: pm4py's readers return pandas DataFrames, so the read rows include
materializing all six of ocel's columnar exports into Polars DataFrames.
Reading into the Rust model alone is reported separately.

Setup:
    sh scripts/fetch-official-fixtures.sh --large
    pip install pm4py polars  # plus the ocel wheel (see crates/ocel-py/README.md)
    python scripts/bench-pm4py-compare.py
"""

import statistics
import time
import warnings

warnings.filterwarnings("ignore")

import ocel  # noqa: E402
import pm4py  # noqa: E402
import polars as pl  # noqa: E402

FIXTURES = "crates/ocel/tests/fixtures/official"
SQLITE = f"{FIXTURES}/order-management.sqlite"
TYPES = ["place order", "confirm order", "pay order"]
RUNS = 7


def bench(fn):
    times = []
    for _ in range(RUNS):
        t0 = time.perf_counter()
        fn()
        times.append((time.perf_counter() - t0) * 1000)
    return statistics.median(times)


def to_dataframes(log):
    """Materialize every columnar export, matching pm4py's DataFrame-based read."""
    pl.DataFrame(log.events())
    pl.DataFrame(log.objects())
    pl.DataFrame(log.relations())
    pl.DataFrame(log.o2o())
    pl.DataFrame(log.event_attributes(), strict=False)
    pl.DataFrame(log.object_attributes(), strict=False)


def main():
    results = []
    model_only = []
    for fmt, path, pm_read in [
        ("SQLite", SQLITE, pm4py.read_ocel2_sqlite),
        ("JSON", f"{FIXTURES}/order-management.json", pm4py.read_ocel2_json),
        ("XML", f"{FIXTURES}/order-management.xml", pm4py.read_ocel2_xml),
    ]:
        fair = bench(lambda: to_dataframes(ocel.read(path)))
        pm = bench(lambda: pm_read(path))
        results.append((f"read {fmt} → DataFrames", fair, pm))
        model_only.append((fmt, bench(lambda: ocel.read(path))))

    rlog = ocel.read_sqlite(SQLITE)
    plog = pm4py.read_ocel2_sqlite(SQLITE)
    results.append(
        (
            "filter by 3 event types",
            bench(lambda: rlog.filter_event_types(TYPES)),
            bench(
                lambda: pm4py.filter_ocel_event_attribute(
                    plog, "ocel:activity", TYPES, positive=True
                )
            ),
        )
    )
    results.append(
        (
            "write SQLite",
            bench(lambda: rlog.write_sqlite("/tmp/bench-ocel.sqlite")),
            bench(lambda: pm4py.write_ocel2_sqlite(plog, "/tmp/bench-pm4py.sqlite")),
        )
    )

    print(f"pm4py {pm4py.__version__}, median of {RUNS} runs\n")
    print("| operation | ocel (Rust) | pm4py | speedup |")
    print("|---|---:|---:|---:|")
    for name, r, p in results:
        print(f"| {name} | {r:.0f} ms | {p:.0f} ms | {p / r:.1f}x |")
    parts = ", ".join(f"{fmt} {ms:.0f} ms" for fmt, ms in model_only)
    print(f"\nRust model only (no DataFrame materialization): {parts}")


if __name__ == "__main__":
    main()
