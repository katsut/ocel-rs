#!/usr/bin/env sh
# Download the official PM4Py OCEL 2.0 example fixtures for extended round-trip tests.
# These files are NOT committed (to keep their license out of this MIT crate).
# Output: crates/ocel-core/tests/fixtures/official/
set -eu

dir="$(dirname "$0")/../crates/ocel-core/tests/fixtures/official"
base="https://raw.githubusercontent.com/process-intelligence-solutions/pm4py/release/tests/input_data/ocel"

mkdir -p "$dir"
curl -fsSL "$base/ocel20_example.jsonocel" -o "$dir/ocel20_example.jsonocel"
curl -fsSL "$base/ocel20_example.xmlocel" -o "$dir/ocel20_example.xmlocel"
curl -fsSL "$base/ocel20_example.sqlite" -o "$dir/ocel20_example.sqlite"

echo "Fetched official fixtures into $dir"
