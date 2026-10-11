# JSON Schema codegen bakeoff

Runs the [JSON Schema Test Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite)
(vendored at `suite/`) through code generators and keeps score: for every
schema group, did the generator produce a type, did it compile, and does
deserializing each test case agree with the suite's `valid` flag?

The scoreboard is [RESULTS.md](RESULTS.md). Each row links to a page with
every failing group and its error.

```sh
git submodule update --init
cargo run -- generators                       # what's under generators/
cargo run -- run                              # every generator, every draft
cargo run -- run --generator typify-0.10 --draft draft7
cargo run -- run --generator quicktype --target go
cargo run -- run --generator typify-0.10 --filter 'ref.json::relative pointer'
cargo run -- render                           # pages from stored data only
cargo run -- diff results/typify-0.10.0-alpha.2/results.json results/typify-0.10.0-beta.7/results.json
cargo run -- key
```

There are two actions. `run` executes the suite and stores data under
`results/<generator version>/results.json`; `render` turns stored data into
pages. A filtered run prints per-case detail with the path to the generated
code and writes under `target/` so it never touches stored results.
Everything scratch lives in `target/`; delete it freely.

[DESIGN.md](DESIGN.md) has the generator contract, the layout, the buckets,
and the caching rules. To add a generator, create `generators/<name>/` with
a `generator.toml` and the three commands it names.

## Generators

- `typify-0.8`: typify 0.8.0, the last stable release before the typespace
  rewrite. Same integration as `typify-0.10` against the 0.8 API.
- `typify-0.10`: [typify](https://github.com/oxidecomputer/typify) 0.10 line,
  pinned in its `Cargo.toml`. The group's `definitions` are registered with
  `add_ref_types`, then the schema is added with the suggested name `Root`;
  `add_root_schema` is avoided because injecting a title makes typify panic
  on merged schemas. A schema that doesn't deserialize as a schemars 0.8
  `RootSchema` (e.g. draft 4's boolean `exclusiveMaximum`) is a
  `generation_error`. typify has no notion of draft; `$schema` is ignored.
- `quicktype`: [quicktype](https://github.com/glideapps/quicktype) through
  `quicktype-core`, pinned in its `package.json`, with targets `rust`, `go`
  and `typescript`. Each group becomes a `JSONSchemaInput` with one source
  named `Root`; references are resolved against an empty store, so the
  suite's remotes come back as `SchemaFetchError`. quicktype models types
  and not constraints, so `false_accept` dominates. See
  [generators/quicktype/README.md](generators/quicktype/README.md) for the
  renderer options and the per-target quirks.
- `json-schema-rs-0.0`: [json-schema-rs](https://github.com/goddtriffin/json-schema-rs)
  0.0 line, pinned in its `Cargo.toml`. Generated one schema at a time with
  `DedupeMode::Disabled`, the only path that reports a non-object root and
  that emits root `anyOf`/`oneOf` enums. The tool requires the root to be
  `type: "object"` with non-empty `properties` — a bare `{"properties": ...}`
  is `CodeGenError::RootNotObject` — so almost the whole suite is a
  `generation_error`. The root type is named from `title`, else the root
  `$ref`'s def key, else `Root`. The suite directory is passed as the
  crate's `spec_version`, which outranks the schema's own `$schema` but is
  inert in 0.0.6, so no result depends on the draft. What does compile is
  mostly 🚧: a root `anyOf`/`oneOf` becomes an externally tagged
  `enum RootAnyOf { Variant0(..), .. }`, which no plain instance
  deserializes into. See
  [generators/json-schema-rs-0.0/README.md](generators/json-schema-rs-0.0/README.md)
  for the settings and what the batch crate needed.
- `jsonschema-0.58`: [jsonschema](https://github.com/Stranger6667/jsonschema)
  0.58 line, pinned in its `Cargo.toml`. A validator rather than a code
  generator, here as a **calibration row**: each module embeds the group's
  schema and validates against it, so anything other than ✅ is the harness's
  doing and not the tool's. It is also the one row that resolves the suite's
  `http://localhost:1234/…` remotes (from `suite/remotes/`, never the
  network), which is how we measure what the other rows forfeit to that
  policy. Each module compiles with the draft of the suite directory it came
  from, which outranks the schema's own `$schema`; with that wired up it is
  ✅ on all 1408 required groups across all five drafts. `format` stays at
  the library default — an assertion in draft 4/6/7, an annotation from
  2019-09 on — so `optional/format` is 100% for the old drafts and 🙈 for the
  new ones. See
  [generators/jsonschema-0.58/README.md](generators/jsonschema-0.58/README.md).
- `schemafy-0.6`: [schemafy](https://github.com/Marwes/schemafy) 0.6 line
  (`schemafy_lib`, the library behind the `schemafy!` macro), pinned in its
  `Cargo.toml`. The oldest generator here, and the only one whose schema
  model *is* a Rust struct: its input type was itself generated from the
  draft-4 meta-schema, so a boolean schema or a numeric `exclusiveMaximum`
  never reaches the generator at all — it's a ❌ at deserialize. There is no
  dialect setting and `$schema` is ignored. `Expander::expand` returns a
  token stream rather than a `Result`, so every other problem — a `$ref`
  `allOf` can't resolve, a non-string `enum`, a schema deep enough to
  overflow the stack — is 🤯; `generate` supervises a worker subprocess so a
  stack overflow costs one group instead of the draft. A `$ref` in a field
  becomes a type *name* from the last path component without being
  resolved, which makes most of `ref.json` and all of `refRemote.json` 🦀 or
  🤯. What does generate models required-ness and nothing else, so 🙈
  dominates. See
  [generators/schemafy-0.6/README.md](generators/schemafy-0.6/README.md).
