# json-schema-rs (git)

[json-schema-rs](https://github.com/goddtriffin/json-schema-rs) built from
unreleased main, pinned in `Cargo.toml` to
`rev = "1d1774d1ae3183f2290ac1472323754a335e5dd9"` (2026-11-03). The workspace
version is still 0.0.6, so `build.rs` reads both the version and the sha out of
`Cargo.lock`, `version` prints `json-schema-rs 0.0.6+1d1774d`, and results
land in `results/json-schema-rs-0.0.6-1d1774d/`. Bumping the `rev` starts a
new results directory; the earlier pins keep theirs.

## What changed since 0.0.6

0.0.6 is the last published version (crates.io, 2026-03). This directory first
held a 0.0.6 integration; its results stay under `results/json-schema-rs-0.0.6/`,
hidden from the tables, and that glue is gone. Required groups passed, by pin:

| draft | 0.0.6 | 892e088 (2026-10-30) | 1d1774d (2026-11-03) |
|---|---|---|---|
| draft4 | 2/160 (1.2%) | 77/160 (48.1%) | 94/160 (58.8%) |
| draft6 | 2/233 (0.9%) | 115/233 (49.4%) | 137/233 (58.8%) |
| draft7 | 2/258 (0.8%) | 123/258 (47.7%) | 145/258 (56.2%) |
| draft2019-09 | 5/373 (1.3%) | 151/373 (40.5%) | 186/373 (49.9%) |
| draft2020-12 | 5/384 (1.3%) | 153/384 (39.8%) | 186/384 (48.4%) |

- **0.0.6 → 892e088** is PR #68, the `feat/codegen-type-graph` branch: 0.0.6
  refused any root that wasn't an object with `properties` (1,298 of 1,408
  required groups were a `generation_error`), and the rewrite generates a type
  for any schema. PR #71 (cross-document `$ref` retrieval) changed nothing on
  the suite: the glue keeps the default offline retriever, so the suite's
  remote refs fail as before, which is the policy for every generator. The
  branch head before the merge (`e9c1b42`) was measured too, with the same
  outcome in every group, and that directory was dropped.
- **892e088 → 1d1774d** is "self-validating types" and "constrained newtypes",
  both on by default, plus boolean documents as types (PR #75) and
  `uniqueItems` sets of generated types. In `cargo run -- diff`: 307 groups
  improved, 8 regressed, 7 moved within a bucket. The improvements are the
  constraint keywords that used to be inert attributes (`minimum`,
  `multipleOf`, `pattern`, `minLength`, ...) and every `anyOf`/`oneOf` group
  that needed exclusivity or a constrained branch. The regressions: the
  optional `bignum.json::maximum integer comparison` in every draft
  (the instance `18446744073709551600` is read as an `f64`, which rounds to
  2^64 and exceeds the exact `maximum` of 2^64-1; the crate's
  `arbitrary-precision` feature is off by default), 2019-09's
  `$recursiveRef with nesting`, and the two `vocabulary.json` groups that
  moved from false accept to mixed.

The glue differs from the 0.0.6 glue: the root-type detection is new, the
dialect setting moved, and the batch crate's dependencies are git pins.

## Provenance

The repo is run by Claude Code. At the pinned commit it has a `CLAUDE.md`
(added 2026-07-04) of "hard requirements" addressed to the agent and a
`.claude/skills/contribution-guide` skill it must invoke before any change.
Of the 79 non-merge commits since July 2026, 62 carry a
`Claude-Session:` trailer or a Claude `Co-Authored-By`; the rest are written
in the same register and say nothing about who wrote them. The
January-to-March 2026 commits that built the validator carry no trailer
either. Everything this row measures is from late October 2026 on: PRs #68
and #71 (85 files, about 37,600 insertions, 2026-10-26 to 10-30) and the
self-validating work (81 files, about 24,700 insertions, 10-31 to 11-03).

## Toolchain

The crate declares `rust-version = "1.99"`, and cargo refuses to build it on
anything older, both here and in the rust-batch crate that compiles its
output. So every command in `generator.toml` runs `cargo +1.99.1`. A
`rust-toolchain.toml` doesn't work: the harness itself runs under cargo, and
the `RUSTUP_TOOLCHAIN` rustup sets for it wins over a toolchain file. rustup
sets `RUSTUP_TOOLCHAIN=1.99.1` for what `cargo +1.99.1` runs, so the cargo
rust-batch spawns for the batch crate uses 1.99.1 too. To run the unit tests:
`CARGO_TARGET_DIR=../../target cargo +1.99.1 test`.

## API changes from 0.0.6

- `JsonSchemaSettings::spec_version` is gone. There are two settings now:
  `default_spec_version` (the draft for a document whose root has no
  `$schema`) and `forced_spec_version` (overrides every `$schema` in the
  document, embedded resources included). Settings are built with
  `JsonSchemaSettings::builder()`.
- `CodeGenError::RootNotObject` is gone, along with the restriction it
  reported. The errors left are unresolvable references, registry problems
  (duplicate `$id`, unsupported vocabulary), a `then`/`else` that isn't a
  schema, and I/O. Every other schema gets a type: a root that isn't a
  struct, enum or union is `pub type Root = ...;`.
- The parse entry point is `Schema::new_from_serde_value(&value, &settings)`
  (PR #75 removed `JsonSchema::new_from_*`); a `Schema` is `Bool(bool)` or
  `Object(Box<JsonSchema>)`, so boolean documents parse and generate.
- `generate_rust` takes `impl Into<CodeGenInput>` (PR #71); a `&[JsonSchema]`
  still converts, with no retrieval URI and the default `OfflineRetriever`,
  so a `$ref` to another document is an error naming the URI, as before.
  `HttpRetriever` is behind an `http` feature that is off by default and
  isn't enabled here: the suite's `localhost:1234` refs are left to fail by
  policy, and that is also the tool's default.
- `CodeGenSettings`, `DedupeMode` and `GenerateRustOutput`
  keep their shape; `CodeGenSettings` gained `unknown_properties` (left at
  `None`: follow each schema's `additionalProperties`). Shared items are now
  reached as `super::shared::Name` rather than `crate::Name`.
- The `json_schema::ref_resolver` module the 0.0.6 glue used to predict the
  root's name is gone (references now go through a `Registry`), and the type
  graph that decides names is private.

## Draft

Each item's `draft` maps onto a `SpecVersion` (`draft4` → `Draft04`, `draft6`
→ `Draft06`, `draft7` → `Draft07`, `draft2019-09` → `Draft201909`,
`draft2020-12` → `Draft202012`); anything else is a `generation_error` naming
the draft. It goes in as `default_spec_version`. That's the setting the crate
documents for this use (its own suite and Bowtie harnesses pass the suite
directory's draft there), and it matches the jsonschema row, whose
`with_draft` sets the root's draft and lets embedded resources follow their
own `$schema`. `forced_spec_version` would also override an embedded
resource's `$schema`, which the spec says it must not. For the suite the two
agree at every root: drafts 4 to 7 never carry a root `$schema`, and 2019-09
and 2020-12 carry the directory's own or a `localhost:1234` metaschema the
crate can't load, in which case it falls back to the default.

Unlike 0.0.6, the dialect is live: a unit test (`the_dialect_is_live`) pins
that a draft-4 boolean `exclusiveMaximum` changes the output.

## Settings

`JsonSchemaSettings` defaults apart from the draft: lenient parsing, so
keywords the crate doesn't model are kept in `extensions`, not rejected.
`CodeGenSettings` with `dedupe_mode(DedupeMode::Disabled)`, everything else
default (`ModelNameSource::TitleFirst`, no `unknown_properties` override, and
`self_validating` and `constrained_newtypes` at their default of on).

`Disabled` is not the default (`Full` is), and it doesn't change what serde
accepts. With one schema per call no mode produces a shared buffer (only
items used by two or more schemas are shared). Across the suite, `Full` and
`Disabled` output differed in 9 groups at 892e088 (`definitions.json::validate
definition against metaschema`, `ref.json::remote ref, containing refs
itself`, and the optional `id`/`anchor` "inside an enum" groups), and only
by merging structurally identical items or dropping an unused duplicate
enum. The reason to pick `Disabled` is the root detection below: `Full` can
fold the root into an identical `$defs` entry and give it that entry's name,
which is the one thing the detection counts on not happening. The glue errors
if a shared buffer ever shows up anyway.

## The root type's name

There's no "call the root `Root`" setting, and nothing in the output marks
the root. From the backend's source:

- A root that isn't a struct, enum or union gets `pub type <Name> = ...;`,
  emitted last, and that is the only `pub type` the backend writes. Rule
  `alias`.
- Otherwise the root is a named item. Items are rendered in depth-first
  post-order from the root, then from each `$defs`/`definitions` entry, so
  the root closes the first run: it's an item `i` whose reachable items are
  exactly items `0..=i`. References are read from the item text, skipping
  comments, string literals, and enum variant names (a variant called `Bar`
  isn't a reference to `struct Bar`). Items before the root can qualify too
  (a leaf first item always does); the root is the last item of the first
  run, so the last qualifying item wins. Rule `position`.
- The one way an item after the root qualifies is a definition the root
  doesn't reach that refers back to the root. A definition's node is named
  for its key (that's the best-ranked name proposal), so a qualifying item
  named for a definition key is passed over, unless it's the root's own
  `$ref` target (`#/$defs/<key>`, `#/definitions/<key>`, or an entry whose
  `$id` is the `$ref` as written; a `$ref`-only root and its target are one
  node) or nothing earlier qualifies. Rule `position-past-def`.

Each module records which rule it used in a `// root type: X (rule)`
comment, and the detection runs outside the tool's `catch_unwind`, so a bug
in it kills `generate` instead of being recorded against the tool. Across
all drafts at 1d1774d: 676 `alias`, 834 `position`, 0 `position-past-def`
(1,510 modules). A reviewer cross-checked every `position` pick against an
independent oracle, the type whose `TYPE_VALIDATORS` entry is the root's own
location (`<base>#`), and read the rest by hand; all agree. The roots not
named `Root` or the title: numeric suffixes from a name clash (`Root0`/`Root1`
for a type-array root whose object branch refers back to the root,
`CoreSchemaMetaSchema0` for the metaschema groups), `RootAnyOf`/`RootOneOf`
for a root that is a union (134 modules), a root whose `$ref` is a relative
URI (`BaseSchema`), and `Bigint` for the `$id`/`$ref` order-of-evaluation
groups whose root is a `$ref` to that definition. An earlier version
narrowed the candidates by the root's predicted name; a reviewer found
schemas where that picks a leaf called `Root`, and others where an enum
variant name breaks the reachability. None of them is in the suite (the
results didn't move when the rule changed), and they're unit tests now.

One case has no right answer. When the root is nullable and otherwise a
named type, the crate's root is `Option<Root>` internally but it emits only
`Root`; no item or alias names the nullable root. The glue measures `Root`,
what a user gets, so `null` is a false reject. That's two groups in each of
the five drafts: `type.json::type: array, object or null` (the enum
`Root` has array and object variants) and `enum.json::heterogeneous
enum-with-null validation` (`{"enum": [6, null]}` comes out as
`enum Root { N6 }`, with `null` split off into the wrapper).

## Self-validating types, and what the harness measures

Since 1d1774d the output embeds the parts of the schema its types need
(`static TYPE_VALIDATORS`, with the schema text and the suite draft as
`default_spec_version`) and checks values through the crate's own validator at
run time. Three cases, and they don't check the same things:

- **Constrained scalars and unions validate in `Deserialize`.** A constrained
  scalar is a newtype whose `Deserialize` validates
  (`json_schema_rs::constrained_newtype!`), so `minLength`, `pattern`,
  `maximum`, `multipleOf` and the rest are enforced at the field. An
  `anyOf`/`oneOf` union's `Deserialize` buffers the value and asks the
  validator which branches it satisfies (`oneOf`: exactly one), so exclusivity
  and constrained branches are real.
- **Structs and enums validate only through `TryFrom<serde_json::Value>` and
  `validate(&self)`**, generated for every type with a validation target. Their
  `Deserialize` is serde's and checks the shape (`deserialize_struct!` reads
  keys into fields and an `additional` map and never calls the validator);
  design.md: "it stays as fast as before".
- **An untyped schema is an alias with no validator.** `{"not": {...}}`,
  `{"contains": {...}}`, `{"if": ..., "then": ...}`, `{"propertyNames": ...}`
  and the like come out as `pub type Root = serde_json::Value;` (or
  `Vec<serde_json::Value>` for array keywords without `type`), and an alias
  gets no `TYPE_VALIDATORS`, no `validate` and no `TryFrom`. 676 of the 1,510
  modules are aliases; 382 of those are `serde_json::Value`.

The harness calls `serde_json::from_value::<Root>(v)`, the same entry point as
every other Rust row, because that's what a program that deserializes into the
generated types gets. Of the 395 required false accepts, 315 have an alias
root, so no entry point would catch them: all of `if-then-else` (24),
`contains` (23), `additionalItems` (20), `uniqueItems` (20), `propertyNames`
(20), `patternProperties` (19) and `items` (18), 26 of the 31 in `not`, and 32
of the 38 in `unevaluatedItems`. The struct case is mainly
`unevaluatedProperties` (13 of 58 are aliases). A reviewer's one-off run with
`TryFrom<Value>` on every root that implements it (not kept) turned 79
required groups to pass across all drafts, 44 of them `unevaluatedProperties`,
and moved 8 others between failure buckets. Both are the crate's choices,
which this row reports as made: an untyped schema is `Value`, and a struct's
`Deserialize` is shape-only for speed.

Nothing is stripped from the generated code. The module is the buffer
followed by `__conformance_check`. In `DedupeMode::Disabled` it never refers
to a `shared` module.

### What the validator is worth: a run with it off

The crate has switches for both validating paths, `self_validating(false)`
and `constrained_newtypes(false)`, and upstream says turning them off leaves
the shapes and derives the same. So a one-off run with them off (into
scratch; the row keeps the defaults) says what the types pass on their own,
at 1d1774d:

| draft | groups | pass, defaults | pass, validator off | depend on it |
|---|---|---|---|---|
| draft4 | 160 | 94 | 77 (48.1%) | 17 |
| draft6 | 233 | 137 | 117 (50.2%) | 20 |
| draft7 | 258 | 145 | 125 (48.4%) | 20 |
| draft2019-09 | 373 | 186 | 153 (41.0%) | 34 |
| draft2020-12 | 384 | 186 | 155 (40.4%) | 31 |
| all | 1408 | 748 | 627 (44.5%) | 122 |

Every one of the 122 becomes a false accept with the validator off. By
file: `oneOf` 38 (the variant is picked by validation; off, the first shape
that fits wins), `ref` 13, `unevaluatedProperties` 12, `allOf` 10, `anyOf`
10, `multipleOf` 10, `maximum` 6, `default` 5, `recursiveRef` 5,
`maxLength` 4, `minLength` 4, `unevaluatedItems` 2, `minimum`, `items` and
`pattern` 1 each. One group goes the other way: `$recursiveRef with nesting`
(2019-09) is a false reject with the validator and passes without it.

So the validator is worth about nine points, mostly `oneOf`, and the 395
false accepts are elsewhere: untyped schemas that became a bare `Value`
with no validator at all. typify 0.10.0-alpha.2 on the same drafts: 38.1,
34.3, 31.0, 27.6 and 27.9%.

## What the batch crate needed

`generate` writes `<out_dir>/batch-deps.toml`:

```toml
json-schema-rs = { git = "https://github.com/goddtriffin/json-schema-rs.git", rev = "<sha>" }
json-schema-rs-macro = { git = "https://github.com/goddtriffin/json-schema-rs.git", rev = "<sha>" }
```

The sha comes from the same `Cargo.lock` entry that names the results
directory. The library is needed for the derive's `impl
::json_schema_rs::ToJsonSchema` and for `json_equal`; the macro for
`#[derive(json_schema_rs_macro::ToJsonSchema)]` and its helper attributes.
The 0.0.6 integration's `syn` build-dependency workaround isn't needed: the
macro crate now enables syn's `full` feature itself.

## Notes

- 73 required groups are a `generation_error`: 65 are remote refs
  (`localhost:1234`, left to fail by policy), and 8 are
  `ref.json::refs with relative uris and defs` / `relative refs with absolute
  uris and defs` (each draft from 6), where a `$ref` inside an embedded
  resource with its own `$id` fails to resolve: "is not in this schema's
  registry". Boolean documents generate now: `true` is
  `pub type Root = serde_json::Value;`, `false` an uninhabited enum.
- Recursion works. The self-recursive schema that hung 0.0.6 comes out as
  `Option<Box<Root>>`; no group times out.
- `allOf` is a conjunction, so `allOf with base schema` merges the root's
  own `properties` with the branches' and passes.
- A schema with `format` and no `type` is typed as `String`, so the 70
  `format.json` groups reject non-strings that the spec says pass. Same for
  `{"minimum": 1.1}` (an `f64` newtype that rejects `"x"`) and `{"maxLength":
  2}` (a `String` newtype that rejects `100`): the constraint is now enforced,
  and the untyped case is a false reject instead of the old mixed result.
- A schema with `properties` and no `type` is a struct, so a non-object
  instance is rejected (`ref.json::root pointer ref` rejects `{"foo":
  false}` on the inner `false`). serde also accepts a JSON array for a
  struct, positionally, which is why some `additionalProperties: false`
  groups reject arrays with "expected fewer elements".
- Four `compile_fail`s, all
  `const.json::float and integers are equal up to 64-bit representation
  limits`: the `ToJsonSchema` derive turns
  `#[json_schema(value = 9007199254740992)]` into an `i32` literal, and the
  backend's own `Serialize`/`Deserialize` impls write
  `serde_json::json!(9007199254740992)`, which is inferred as `i32` too.
- Against typify 0.10.0-alpha.2 on draft 7 (required): json-schema-rs 145,
  typify 80. typify still wins ten groups outright: `not` (forbidden property,
  forbid everything with an empty or `true` schema), `propertyNames` with
  `pattern` or `enum`, `additionalProperties` not looking in applicators and
  its non-ASCII `pattern` group, `items: false`, and the two nullable-root
  groups described above.
