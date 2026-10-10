# typify conformance harness

Run the JSON Schema org's official test suite through typify and report, per test, how far the generated code gets. The goal is a repeatable scoreboard for typify 1 and typify 2, not a fix to typify itself. Don't change typify source in this repo.

## Inputs

- **Test suite:** https://github.com/json-schema-org/JSON-Schema-Test-Suite, vendored as a git submodule at `suite/`.
  - `tests/<draft>/*.json`: one file per keyword (e.g. `allOf.json`, `ref.json`). Each file is an array of groups: `{ description, schema, tests: [{ description, data, valid }] }`.
  - Drafts: `draft4`, `draft6`, `draft7`, `draft2019-09`, `draft2020-12`.
  - `tests/<draft>/optional/`: optional behavior (formats, bignum, etc.). Run it but report it separately; don't count it in headline numbers.
  - `remotes/`: schemas that tests reference as `http://localhost:1234/...`. Map that URL prefix to this directory when resolving references; never hit the network.
- **typify 1:** the published `typify` crate (pin the version in `Cargo.toml`).
- **typify 2:** in-progress rewrite. Ask Adam for the repo/branch/path before wiring it up; don't guess.

## Pipeline per schema group

For each `(draft, file, group)`, for each typify version:

1. **Generate.** Feed the group's `schema` to typify as the root, with a fixed root type name (e.g. `Root`). Run generation in a subprocess or under `catch_unwind` so a panic is recorded, not fatal.
2. **Compile.** Emit the generated code into a small crate whose test binary reads the group's test cases and calls `serde_json::from_value::<Root>(data)` on each. (ahl edit: Adding a little bit of complexity here. I think we want to generate a FILE for each, and then maybe we run it through trybuild or something as a pass; anything that fails to build we exclude, but then we can run all of the ones that do build in a single crate; we'll just need to wrap each test in a panic handler--I'd like to discuss this point)
3. **Run.** For each test case, compare deserialize success/failure with the test's `valid` field.

## Outcome buckets

Record exactly one per schema group per version, plus per-case detail when it gets to step 3:

| Bucket | Meaning |
|---|---|
| `generation_panic` | typify panicked |
| `generation_error` | typify returned an error (e.g. unsupported construct) |
| `compile_fail` | generated code doesn't compile |
| `runtime_panic` | a test case panicked during deserialize |
| `false_accept` | deserialize succeeded but `valid: false` |
| `false_reject` | deserialize failed but `valid: true` |
| `pass` | every case matched |

Keep `false_accept` and `false_reject` separate: they mean different things (typify too loose vs. too strict) and get fixed differently. Store the first error message or panic text for every non-pass bucket.

I'd like you to come up with a key perhaps using emojis to fill out a chart.

## Performance

There are a few thousand schema groups. Don't run one `cargo build` per group serially.

- Share one `CARGO_TARGET_DIR` across all generated crates so dependencies (serde, serde_json, regress, chrono, uuid) build once.
- Batch many groups into one crate as separate modules for speed, but when a batch fails to compile, bisect or fall back to per-group crates so one bad schema doesn't sink its neighbors.
- Parallelize across batches.
- Cache by hash of (schema, typify version) so reruns only redo what changed.

## Output

- `results/<version>/results.json`: one record per group with draft, file, group description, bucket, error text, and per-case results.
- `results/<version>/summary.md`: pass rate per draft, and a table of buckets per keyword file. This is what Adam will read.
- A `diff` command comparing two results files (typify 1 vs. 2, or one version before vs. after a change), listing groups that moved between buckets.

## Notes

- Serde deserialize only enforces what typify encodes in types. Validation keywords typify doesn't model (formats, some numeric bounds) will show up as `false_accept`. That's a real finding; record it, don't paper over it.
- The `$schema` / draft matters for semantics. Pass the draft through to typify where it supports that; note where it doesn't.
- Commands should be runnable as `cargo run -- run --version typify1 [--draft draft2020-12] [--filter ref]` and `cargo run -- diff A B`.
- Start with typify 1 on `draft7` end-to-end before broadening.
