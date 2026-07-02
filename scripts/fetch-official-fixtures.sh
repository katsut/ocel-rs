#!/usr/bin/env sh
# Download official OCEL 2.0 fixtures for extended round-trip tests.
# These files are NOT committed (to keep their licenses out of this MIT crate).
# Output: crates/ocel-core/tests/fixtures/official/
#
# Sources:
# - PM4Py ocel20_example (small, all-feature)
# - Zenodo "Order Management" (DOI 10.5281/zenodo.8337463, ~21K events) — pass
#   `--large` to include it (~64 MB across three formats).
set -eu

dir="$(dirname "$0")/../crates/ocel-core/tests/fixtures/official"
base="https://raw.githubusercontent.com/process-intelligence-solutions/pm4py/release/tests/input_data/ocel"
zenodo="https://zenodo.org/api/records/18373906/files"

mkdir -p "$dir"
curl -fsSL "$base/ocel20_example.jsonocel" -o "$dir/ocel20_example.jsonocel"
curl -fsSL "$base/ocel20_example.xmlocel" -o "$dir/ocel20_example.xmlocel"
curl -fsSL "$base/ocel20_example.sqlite" -o "$dir/ocel20_example.sqlite"
echo "Fetched PM4Py example fixtures into $dir"

if [ "${1:-}" = "--large" ]; then
    curl -fsSL "$zenodo/order-management.sqlite/content" -o "$dir/order-management.sqlite"
    curl -fsSL "$zenodo/order-management.json/content" -o "$dir/order-management.json"
    curl -fsSL "$zenodo/order-management.xml/content" -o "$dir/order-management.xml"
    echo "Fetched Zenodo Order Management (21K events) into $dir"
fi
