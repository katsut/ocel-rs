"""Fair, reproducible comparison of ocel (Rust bindings) vs pm4py.

Same data, same operations, median of 3 runs. Prints a Markdown table.

Setup:
    sh scripts/fetch-official-fixtures.sh --large
    pip install pm4py  # plus the ocel wheel (see crates/ocel-py/README.md)
    python scripts/bench-pm4py-compare.py
"""

import statistics
import time
import warnings

warnings.filterwarnings("ignore")

import ocel  # noqa: E402
import pm4py  # noqa: E402

FIXTURES = "crates/ocel-core/tests/fixtures/official"
SQLITE = f"{FIXTURES}/order-management.sqlite"
JSON = f"{FIXTURES}/order-management.json"
XML = f"{FIXTURES}/order-management.xml"
TYPES = ["place order", "confirm order", "pay order"]
RUNS = 3


def bench(fn):
    times = []
    for _ in range(RUNS):
        t0 = time.perf_counter()
        fn()
        times.append((time.perf_counter() - t0) * 1000)
    return statistics.median(times)


def main():
    results = [
        (
            "read SQLite (21K events)",
            bench(lambda: ocel.read_sqlite(SQLITE)),
            bench(lambda: pm4py.read_ocel2_sqlite(SQLITE)),
        ),
        (
            "read JSON",
            bench(lambda: ocel.read_json(JSON)),
            bench(lambda: pm4py.read_ocel2_json(JSON)),
        ),
        (
            "read XML",
            bench(lambda: ocel.read_xml(XML)),
            bench(lambda: pm4py.read_ocel2_xml(XML)),
        ),
    ]

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


if __name__ == "__main__":
    main()
