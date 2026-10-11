# JSON Schema codegen bakeoff

Runs the [JSON Schema Test Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite)
(vendored at `suite/`) through code generators and keeps score: for every
schema group, did the generator produce a type, did it compile, and does
deserializing each test case agree with the suite's `valid` flag?

The scoreboard is [RESULTS.md](RESULTS.md). Each row links to a page with
every failing group and its error. RESULTS.md and the per-tool pages are
generated: `cargo run -- run` rewrites them after storing results, and
`cargo run -- render` rebuilds them from stored results alone.

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
- typify 0.3.0 (December 2024) has a results directory but no generator:
  it was measured once with a copy of `typify-0.8` pinned to `=0.3.0`, plus a
  redirect of the tool's stray stdout to stderr, and the glue was then
  removed. Recreate it that way if it ever needs rerunning.
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
- `json-schema-to-zod`: [json-schema-to-zod](https://github.com/StefanTerdell/json-schema-to-zod)
  2.8.1 with zod 4, both pinned in its `package.json`. A source-to-source
  converter rather than a type generator: it returns a *string* holding one
  zod expression, so it emits `.min()`, `.max()`, `.regex()`, `.strict()`
  and `superRefine` closures where the others drop constraints. The catch
  is its dispatcher, which keys on `type` — a schema with no `type` and no
  combinator falls through to `z.any()`, which is 960 of the 1587 modules
  the suite produces and most of the row's 🙈. No dialect setting and no
  `$ref` resolver (a `$ref` root is `z.any()`, local or remote alike). The
  output is erasable TypeScript, so node runs it directly: no `tsc`, and
  the build step is a load probe instead of a compile. 🚧 comes almost
  entirely from `z.literal` on a non-scalar `const`/`enum` and from `oneOf`
  branches that compiled to `z.any()` and so match everything. Deprecated
  upstream since March 2026. See
  [generators/json-schema-to-zod/README.md](generators/json-schema-to-zod/README.md).
- `ajv`: [ajv](https://ajv.js.org/) 8.x plus `ajv-draft-04`, both pinned in
  its `package.json`. A validator rather than a code generator, and the
  **calibration row for the Node path** the way `jsonschema-0.58` is for
  Rust: the same generate → build → run pipeline, the same runner protocol,
  so a group that fails here and on the Rust validator row is the harness's,
  and a group that fails only here is ajv's. The Ajv class comes from the
  suite directory, which outranks a root `$schema`; it is the second row
  that resolves the suite's `http://localhost:1234/…` remotes (all 79 files
  under `suite/remotes/` pre-registered with `addSchema`, never the
  network). It sits at 95.7% of required groups — 97–98% on draft 4/6/7,
  90.6% on 2020-12 — and every miss is ajv's own, most of them groups ajv's
  CI skips by name: `$dynamicRef` scope resolution, `unevaluated*`
  annotations from `contains` and `if`, `$ref` siblings in the old drafts
  (ajv applies them in every draft), and `ownProperties: false`, which lets
  `required: ["toString"]` pass on `{}`. `format` is annotation-only
  everywhere, because the `ajv` package ships no formats at all and
  `ajv-formats` is deliberately not installed. See
  [generators/ajv/README.md](generators/ajv/README.md).
- `go-jsonschema`: [go-jsonschema](https://github.com/omissis/go-jsonschema)
  0.24 line, pinned in its `go.mod` (module path still
  `github.com/atombender/go-jsonschema`). One of the few code generators
  whose output actually *validates*: the tool emits an `UnmarshalJSON` per type that re-checks
  `required`, `enum`, `pattern`, the length, bound and `multipleOf`
  keywords and `const` on strings and booleans. Used as a library, not a
  CLI, because the CLI's per-schema flags are keyed by `$id` (which the
  suite rarely declares) and it aborts the whole batch on the first bad
  schema; the glue runs one generator per schema in one process and
  restarts a worker past anything that crashes. The catch dwarfs the
  validation: a schema whose root has no `type` keyword produces **no root
  type at all**, so `var v Root` doesn't compile and 1084 of the 1408
  required groups are 🦀 — the design center is a config file that starts
  `type: object`, not arbitrary schemas. `$ref` support stops at top-level
  definitions in the same file. See
  [generators/go-jsonschema/README.md](generators/go-jsonschema/README.md).
- `typebox`: [TypeBox](https://github.com/sinclairzx81/typebox) 0.34 driven
  by [schema2typebox](https://github.com/xddq/schema2typebox) 1.7, both
  pinned in its `package.json`. The TypeScript row where the generated
  artifact *is* the validator: `Type.Object({...})` is simultaneously a
  draft-7 schema fragment and a static type, and `TypeCompiler` checks
  instances against it. TypeBox can only check types it built — a raw
  schema throws `Unknown type`, and `Type.Unsafe` just moves the hole — and
  `@sinclair/typebox-codegen` has no JSON Schema front end (it maps
  TypeScript *source* to TypeBox), so the schema goes through
  schema2typebox, the generator TypeBox's own readme points at. That makes
  the row **codegen, not raw**: ❌ is schema2typebox declining, and
  everything from 🦀 on is TypeBox. Its `collect` needs a recognizable
  `type` or combinator at every level, so only 486 of the 1709 groups
  produce a module at all — the same wall `json-schema-to-zod` hits,
  except zod falls through to `z.any()` and accepts (🙈) where this
  errors. Of the 304 core groups that do compile, 227 pass: every
  constraint keyword schema2typebox doesn't consume is passed through to
  the TypeBox options, so lengths, bounds, `pattern`, `uniqueItems`,
  `additionalProperties: false`, `required` and the combinators are all
  really enforced. The hole is `type: "object"` with no `properties`,
  which becomes a bare `Type.Unknown()` and silently discards the rest of
  the schema — 49 of the 469 modules, and a third of the row's 🙈.
  No dialect setting; `format` is left unregistered
  (TypeBox then rejects every value, but no suite group reaches it).
  Compiled with `tsc`, because the output's `@sinclair/typebox` import
  mixes type-only and value names and so isn't erasable. See
  [generators/typebox/README.md](generators/typebox/README.md).
- `ogen`: [ogen](https://ogen.dev) 1.24 line, pinned in its `go.mod`. An
  OpenAPI v3 generator, so the odd row out: it has no bare-schema entry
  point, and each group is wrapped as an OpenAPI **3.1** document whose
  `components.schemas.Root` is the group's schema (3.1 schema objects *are*
  JSON Schema 2020-12, so nothing is translated) with one synthetic `POST /`
  operation taking it as a request body — a component nothing references
  generates no type. The schema's own top-level members are copied to the
  document root too, so `#/definitions/x`, `#/$defs/x` and `#` still point
  where the suite meant; measured, that moves ~15 groups per draft out of ❌
  and changes the pass count by zero. The check is ogen's own request-body
  path: decode with the generated `jx` decoder, insist on EOF, then call the
  generated `Validate()` — ogen splits the work, so skipping the second
  method would measure half the tool. Client, server and otel generation are
  off (they change no decision about the schema and cost 25 KB of `net/http`
  per group); `--infer-types` is left at its default off, which is the single
  biggest lever here — turning it on takes draft 7 from 19.8% to 32.2%,
  because the suite's dominant `{"properties": …}` with no `type` is
  otherwise just `jx.Raw` and accepts everything. That is most of the 645 🙈.
  No dialect setting: `$schema`, `$id`, `$defs`, `not`, `if`/`then`/`else`,
  `contains`, `propertyNames` and the `dependent*`/`unevaluated*` families
  have no field in its schema model at all. `exclusiveMaximum` is a **bool**,
  so draft 4's spelling is the one it takes and drafts 6+ are a ❌ — the
  reverse of every other row. The 🦀 are real ogen bugs: a `format` on an
  untyped schema emits `type Root jx.Raw` plus a decoder returning
  `netip.Addr`/`time.Time`/`url.URL`. See
  [generators/ogen/README.md](generators/ogen/README.md).
