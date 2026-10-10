# typify conformance harness

Runs the [JSON Schema Test Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite)
(vendored at `suite/`) through typify and reports, per schema group, how far
the generated code gets: generation, compilation, and then deserializing each
test case and comparing the outcome with the suite's `valid` flag.

```sh
git submodule update --init
cargo run -- run --version typify1 --draft draft7
cargo run -- run --version typify1 --filter 'ref.json::relative pointer'
cargo run -- diff results/typify1/results.json results/typify1-before/results.json
cargo run -- key
```

`run` writes `results/<version>/results.json` and `summary.md`. A filtered
run prints per-case detail (with the path to the generated code) and writes
to `work/<version>/filtered/` instead, so it never clobbers the headline
results. `--out <dir>` overrides either. `--optional` includes
`tests/<draft>/optional/`, which the summary reports separately.

## Buckets

Run `cargo run -- key` for the glyph key. One bucket per group, worst to best:
`generation_panic`, `generation_error`, `compile_fail`, `runtime_panic`,
`mixed`, `false_accept`, `false_reject`, `pass`. `mixed` means one case was
accepted though invalid and another was rejected though valid, which a single
group can do when typify models only part of a keyword.

## How a group is fed to typify

- The group's `definitions` are registered with `add_ref_types`, then the
  schema itself is added with `add_type_with_name(.., Some("Root"))`. The
  type to deserialize into is whatever typify names the root (`Root`, `i64`,
  `Vec<i64>`, `serde_json::Value`, ...), taken from the typespace view.
- `add_root_schema` is deliberately not used: it only names a root with a
  `title`, and injecting one trips typify into `get_type_name().unwrap()`
  panics on merged schemas, which would mask whether the construct itself is
  supported. The cost is that a `$ref` to `#` (the root) does not resolve.
- A boolean schema (`true` / `false`) is added as `Schema::Bool`.
- A schema that fails to deserialize as a schemars 0.8 `RootSchema` (for
  example `maxItems: 2.0`) is recorded as `generation_error`.
- Remote refs (`http://localhost:1234/...`) are not resolved; typify panics on
  external references and that is recorded.
- typify 0.10 has no notion of draft; `$schema` is parsed into
  `RootSchema::meta_schema` and ignored.

## Pipeline and caching

Generation runs in-process under `catch_unwind` on a big-stack thread with a
timeout. All generated modules go into one batch crate under
`work/<version>/batch/`, which is `cargo check`ed; modules whose diagnostics
point at them are dropped and the check repeats until clean. Survivors are
built into a runner binary that deserializes each case under `catch_unwind`
and prints one JSON line per case; if the runner process dies (stack
overflow), the case at the cursor is recorded as a panic and the runner is
restarted on the next case. `work/target/` is shared so dependencies build
once. Each group's result is cached under `work/<version>/cache/` by a hash
of the schema, the selected cases, the typify version, and a harness
revision constant; `--no-cache` ignores it.
