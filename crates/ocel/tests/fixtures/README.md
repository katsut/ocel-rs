# Test fixtures

Hand-authored OCEL 2.0 fixtures used by the integration tests. All files here are
original, MIT-licensed test data (generic names only: `Alice`, `Bob`, `example.com`).

| File | Purpose |
|------|---------|
| `minimal.json` | Empty but valid OCEL log (edge case). |
| `order_management_small.json` | Full-feature log: typed attributes, dynamic object attributes (`total` changes over time), `E2O` with multiple qualifiers, `O2O` relationships. |
| `edge_cases.json` | Empty attribute lists and a duplicated `E2O` relationship (same object + qualifier), which the spec permits. |

## Official PM4Py example (not committed)

The official PM4Py `ocel20_example` files (`.jsonocel` / `.xmlocel` / `.sqlite`) are
**not** vendored here to avoid mixing their license into this MIT crate. Fetch them
on demand into `official/` (git-ignored) for extended round-trip tests:

```sh
sh scripts/fetch-official-fixtures.sh
```

Tests that depend on `official/` skip themselves when the files are absent.
