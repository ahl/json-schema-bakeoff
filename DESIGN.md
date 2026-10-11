# Design

A bakeoff of JSON Schema code generators: each generator turns every schema
group in the official JSON Schema Test Suite into a type, the generated
code is compiled, and each test case is deserialized into that type. The
outcome is compared with the suite's `valid` flag.

## Two actions

- `run` executes the suite through generators and stores data under
  `results/<id>/results.json`, then renders.
- `render` regenerates every `results/<id>/README.md` and the top-level
  `RESULTS.md` from stored data. Nothing else writes to `results/`.

Stored results never need regenerating when a generator is added or the
pages change. A generator's results are only recomputed when the generator
version, the schema, the cases, or the harness revision changes.

## Layout

```
harness/            orchestrator: suite loader, buckets, cache, runner protocol, render, diff
rust-batch/         shared build step for generators that emit Rust
generators/<name>/  one directory per generator integration (see below)
suite/              json-schema-org/JSON-Schema-Test-Suite, as a submodule
results/<id>/       committed data (results.json) and its rendered page (README.md)
RESULTS.md          rendered scoreboard: one row per results/<id>
target/             everything scratch: cargo output, generated sources, batch crates
```

## The generator contract

A generator is a directory under `generators/` holding a `generator.toml`:

```toml
name = "quicktype"
targets = ["rust", "go", "typescript"]   # optional; omitted = one implicit target

[commands]
version  = "node generate.mjs version {target}"
generate = "node generate.mjs generate {target} {out_dir}"
build    = "sh build.sh {target} {gen_dir} {batch_dir}"
```

The harness never links a generator. It runs the three commands as
subprocesses through `sh -c`, with cwd set to the generator directory,
`CARGO_TARGET_DIR=<repo>/target`, `BAKEOFF_ROOT`, and `BAKEOFF_TARGET` in
the environment, and `{root}`, `{target}`, `{out_dir}`, `{gen_dir}`,
`{batch_dir}` substituted. stdout is the protocol; stderr is inherited so
build noise reaches the terminal.

1. **`version`** prints an identity string, e.g. `typify 0.10.0-alpha.2` or
   `quicktype 26.0.0 go`. Slugged, it names the results directory and the
   scoreboard row, and it is part of every cache hash.
2. **`generate <out_dir>`** reads a JSON array of `{key, schema}` on stdin
   in one long-lived process (never one process per group), writes
   `<out_dir>/<key>.<ext>` for each schema it can handle, and prints a JSON
   array of `{key, status, message?}` with status `ok`, `error` (the tool
   returned an error) or `panic` (the tool crashed or hung). The root type
   must be named `Root` where the tool lets the caller choose.
3. **`build <gen_dir> <batch_dir>`** reads a JSON array of keys on stdin,
   compiles those modules together, attributes compiler diagnostics back
   to keys, drops the failures, repeats until clean, and prints
   `{"outcomes": {key: "ok" | {"fail": diagnostic}}, "runner": [exe, args]}`.

The runner the build step names speaks the harness's runner protocol:

```
runner <cases.json> <start-index>
```

`cases.json` is a JSON array of `{key, data}`. The runner prints
`{"ready": <start>}` once it has parsed the file, then for each case from
`start` onward one line `{"case": {"i": <index>, "r": "accept" | {"reject": msg} | {"panic": msg}}}`,
flushing after each. Each deserialize must be isolated so one failure
can't take the rest down (`catch_unwind`, `try/catch`, `recover`). If the
process dies anyway, the harness records the case at the cursor as a panic
and restarts the runner from the next index.

Every generator should parse JSON with correctly rounded floats (serde_json
needs the `float_roundtrip` feature); the suite's bignum tests depend on it.

### Generators that emit Rust

`rust-batch build <gen_dir> <batch_dir>` implements the build step for any
generator whose modules are Rust. Each `<gen_dir>/<key>.rs` must define
`pub fn __conformance_check(v: ::serde_json::Value) -> Result<(), String>`.
The batch crate depends on serde, serde_json, regress, chrono and uuid.

### Versioning

A generator directory is per *integration*, named for the API line it was
written against (`typify-0.10`), not the exact version. Bumping the pinned
version edits it in place; the old version's `results/<id>/` stays. Two
directories for the same tool exist only when the glue code differs
(`typify-0.8`, `typify2`).

## Buckets

One per group, worst to best. `cargo run -- key` prints the glyphs.

| | bucket | meaning |
|---|---|---|
| 🤯 | generation_panic | the generator crashed or hung |
| ❌ | generation_error | the generator returned an error |
| 🦀 | compile_fail | generated code doesn't compile |
| 💀 | runtime_panic | a test case crashed during deserialize |
| 🤪 | mixed | both false_accept and false_reject cases in one group |
| 🚧 | false_reject | deserialize failed but `valid: true` (too strict) |
| 🙈 | false_accept | deserialize succeeded but `valid: false` (too loose) |
| ✅ | pass | every case matched |

`false_accept` and `false_reject` stay separate because they have different
causes and fixes. The first error message or panic text is stored for every
non-pass group, plus per-case detail.

## Caching and idempotence

Each stored record carries a hash of the harness revision, the generator
identity, the group id, the schema, and the selected cases. `run` reuses any
record whose hash matches and recomputes the rest; an unchanged rerun is a
byte-for-byte no-op. A draft run replaces only that draft's records, so
drafts accumulate. `--no-cache` recomputes everything selected. Bump
`HARNESS_REV` in `harness/src/pipeline.rs` when the pipeline's semantics
change in a way that should invalidate stored records.

## Optional tests

`tests/<draft>/optional/` runs by default and is reported in its own
section, never in headline numbers. `--no-optional` skips it.

## Remote references

`http://localhost:1234/...` references are not resolved; a generator that
can't handle them records whatever it does (typify panics).
