# json-schema-rs (git)

[json-schema-rs](https://github.com/goddtriffin/json-schema-rs) built from
unreleased main, pinned in `Cargo.toml` to
`rev = "8032443b0cd62c8f99ee1b4776e715fe553f8111"` (2026-11-16). The workspace
version is still 0.0.6, so `build.rs` reads both the version and the sha out of
`Cargo.lock`, `version` prints `json-schema-rs 0.0.6+8032443`, and results
land in `results/json-schema-rs-0.0.6-8032443/`. Bumping the `rev` starts a
new results directory; the earlier pins keep theirs.

## What changed since 0.0.6

0.0.6 is the last published version (crates.io, 2026-03). This directory first
held a 0.0.6 integration; its results stay under `results/json-schema-rs-0.0.6/`,
hidden from the tables, and that glue is gone. Required groups passed, by pin:

| draft | 0.0.6 | 892e088 (2026-10-30) | 1d1774d (2026-11-03) | 8032443 (2026-11-16) |
|---|---|---|---|---|
| draft4 | 2/160 (1.2%) | 77/160 (48.1%) | 95/161 (59.0%) | 95/161 (59.0%) |
| draft6 | 2/233 (0.9%) | 115/233 (49.4%) | 138/234 (59.0%) | 136/234 (58.1%) |
| draft7 | 2/258 (0.8%) | 123/258 (47.7%) | 146/259 (56.4%) | 144/259 (55.6%) |
| draft2019-09 | 5/373 (1.3%) | 151/373 (40.5%) | 187/374 (50.0%) | 185/374 (49.5%) |
| draft2020-12 | 5/384 (1.3%) | 153/384 (39.8%) | 186/385 (48.3%) | 184/385 (47.8%) |

The two newest columns are on the suite at 245a45a, which added a required
`multipleOf` group to every draft; the earlier pins were not rerun.

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
- **1d1774d → 8032443** is two weeks of codegen work: types "accept every
  schema-valid instance" (a key only `then`/`else` names,
  `additionalProperties` beside `patternProperties`, and `items` beside
  `prefixItems` no longer narrow the Rust type), integers read `1.0` from
  draft 6 on, a root map or array is a self-validating newtype instead of an
  alias, typed formats (`std::net` for `ipv4`/`ipv6`; `uuid` and `chrono`
  behind features that are off), lighter unions, `Nullable<T>`, string-enum
  impls, and the output naming its root type. In `cargo run -- diff`: 11
  groups improved, 49 regressed, none of them to a generation failure.
  Required passes drop by two in every draft but 4. The regressions, largest
  first:
  - `patternProperties` now types the keys the schema doesn't name. So
    `{"additionalProperties": false, "patternProperties": ...}` is a struct
    with a flattened `additional` map, and a bare `patternProperties` root is
    a newtype over `BTreeMap<String, Value>`; both deserialize shape-only,
    and only `TryFrom` checks the patterns, where before the constraint
    reached `Deserialize` through a newtype. That is the five
    `ecmascript-regex.json` `patternProperties` groups per draft (optional,
    25) and, per draft, `patternProperties validates properties matching a
    regex`, `additionalProperties being false does not allow other
    properties` and `non-ASCII pattern with additionalProperties` (15).
  - `oneOf with boolean schemas, all true` and `more than one true` (draft 6
    on, 8): a `oneOf` whose branches are all `true` (or `{}`) now gets
    `deserialize_untagged!`, first variant wins, instead of the validated
    `deserialize_union!`, so every value is accepted where every value should
    be rejected. `record_variant_targets` (`type_graph/passes.rs`) drops a
    union when no branch has a validation target, and `BufferValidators::new`
    then skips it. A tool bug, and the one regression that isn't a design
    choice.
  - 2020-12 `prefixItems validation adjusts the starting index for items`:
    `items` beside `prefixItems` no longer types the elements.

  The improvements: `properties, patternProperties, additionalProperties
  interaction` (every draft) from mixed to false accept, 2020-12's two
  `format-assertion` groups to pass, and the optional `ipv4`/`ipv6` groups in
  2019-09 and 2020-12 from mixed to false reject: formats are typed by default
  (`FormatTypes::Typed`), so an invalid address is rejected where the dialect
  says `format` is only an annotation. The crate documents that as a
  deliberate divergence; `FollowDialect` is the spec-exact setting and isn't
  the default.

The glue differs from the 0.0.6 glue: the dialect setting moved, the root
comes from the output, and the batch crate's dependencies are git pins.

## Provenance

The repo is run by Claude Code. At the pinned commit it has a `CLAUDE.md` of
"hard requirements" addressed to the agent and five skills under
`.claude/skills/` (`contribution-guide`, `optimize-library`, `release-notes`,
`competitor-json-schema-codegen-analysis`,
`synthetic-schema-codegen-test`). At 1d1774d, 62 of the 79 non-merge commits
since July 2026 carried a `Claude-Session:` trailer or a Claude
`Co-Authored-By`. The history has been rewritten since: 1d1774d is no longer
an ancestor of main (a commit with its subject and date is there under
another sha), and of the 187 non-merge commits since July 2026 at 8032443,
none carries either. The commits are written in the same register as before
and say nothing about who wrote them. Everything this row measures is from
late October 2026 on: the type-graph codegen (PRs #68 and #71), the
self-validating work (to 11-03), and two more weeks of codegen, validator and
benchmark PRs; GitHub's compare view lists 222 commits between the pins.

## Toolchain

The crate declares `rust-version = "1.100"`, and cargo refuses to build it on
anything older, both here and in the rust-batch crate that compiles its
output. So every command in `generator.toml` runs `cargo +1.100.0`. A
`rust-toolchain.toml` doesn't work: the harness itself runs under cargo, and
the `RUSTUP_TOOLCHAIN` rustup sets for it wins over a toolchain file. rustup
sets `RUSTUP_TOOLCHAIN=1.100.0` for what `cargo +1.100.0` runs, so the cargo
rust-batch spawns for the batch crate uses 1.100.0 too. To run the unit tests:
`CARGO_TARGET_DIR=../../target cargo +1.100.0 test`.

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

At 8032443:

- `GenerateRustOutput` has `root_types`, the name of each input's root type,
  in scope in that schema's buffer. The glue uses it; the root-detection
  heuristic of the earlier pins is gone (see below).
- `CodeGenInput` carries `JsonSchemaSettings` (`.settings(...)`), which the
  backend bakes into the embedded validators' `DialectSettings`. The glue
  passes the settings it parsed with, as the CLI does; without them the
  embedded validator of a draft 4 to 7 root, which has no `$schema`, would
  run under the default dialect.
- The CLI is its own crate, `jsonschemars`; the library no longer depends on
  clap.
- `CodeGenSettings` gained `format_types` (default `Typed`), `root_name`,
  `visibility`, `crate_path`, `macro_crate_path`, `builders`,
  `string_enum_impls` (default on), `schema_defaults`, `extra_derives`,
  `patches`, `conversions`, `replacements` and `crates`. All at their
  defaults here.
- Generated structs derive `json_schema_rs_macro::Deserialize` rather than
  serde's (serde's semantics, linear compile time for wide structs); unions
  are `deserialize_union!` or `deserialize_untagged!` calls; an optional
  property that allows `null` is `json_schema_rs::Nullable<T>` (no suite
  group produces one).

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
keywords the crate doesn't model are kept in `extensions`, not rejected. The
same settings go into `CodeGenInput`, so the embedded validators use the
suite's draft too. `CodeGenSettings::default()`: `DedupeMode::Full`,
`ModelNameSource::TitleFirst`, no `unknown_properties` override,
`self_validating` and `constrained_newtypes` on, `FormatTypes::Typed`. That
is what `jsonschemars generate` emits with no flags, apart from the draft.

Earlier pins ran with `DedupeMode::Disabled` to protect the root detection;
with the root reported there's no reason to. With one schema per call no
mode produces a shared buffer (only items used by two or more schemas are
shared); the glue errors if one shows up, and none did across the suite. A
full run with `Disabled` at this pin gave the same bucket and case outcomes
in every group.

## The root type's name

`GenerateRustOutput::root_types[0]` names the root: the alias for a root
that isn't a struct, enum or union, else the item, which can be the
definition dedupe merged the root into. Every module records it in a
`// root type: X (root_types)` comment. Earlier pins had no such report and
the glue inferred the root from item order; that heuristic, its rules
(`alias`, `position`, `position-past-def`) and the cross-check against the
`TYPE_VALIDATORS` entries are gone. The unit tests pin that the reported
name is declared in the buffer for a titled root, a `$ref` root, a root
identical to a definition, a union root, scalar and newtype roots, a
self-reference and boolean documents.

One case has no right answer. When the root is nullable and otherwise a
named type, the crate's root is `Option<Root>` internally but it emits only
`Root`; the reported root is `Root`, what a user gets, so `null` is a false
reject. That's two groups in each of the five drafts: `type.json::type:
array, object or null` (the enum `Root` has array and object variants) and
`enum.json::heterogeneous enum-with-null validation` (`{"enum": [6, null]}`
comes out as `enum Root { N6 }`, with `null` split off into the wrapper).

## Self-validating types, and what the harness measures

Since 1d1774d the output embeds the parts of the schema its types need
(`static TYPE_VALIDATORS`, with the schema text and the suite draft as
`default_spec_version`) and checks values through the crate's own validator at
run time. Three cases, and they don't check the same things:

- **Constrained scalars and unions validate in `Deserialize`.** A constrained
  scalar is a newtype whose `Deserialize` validates
  (`json_schema_rs::constrained_newtype!`), so `minLength`, `pattern`,
  `maximum`, `multipleOf` and the rest are enforced at the field. An
  `anyOf`/`oneOf` union's `Deserialize` (`deserialize_union!`, 119 modules)
  buffers the value and asks the validator which branches it satisfies
  (`oneOf`: exactly one), so exclusivity and constrained branches are real.
  A union none of whose branches has anything to check gets
  `deserialize_untagged!` (56 modules), first variant wins, which is the
  `oneOf`-of-`true`s bug above.
- **Structs and enums validate only through `TryFrom<serde_json::Value>` and
  `validate(&self)`**, generated for every type with a validation target. Their
  `Deserialize` checks the shape (the macro crate's derive reads keys into
  fields and an `additional` map and never calls the validator); design.md:
  "it stays as fast as before". Since 8032443 there is also
  `json_schema_rs::Validated<T>`, a wrapper whose `Deserialize` runs the full
  validation; it is opt-in, so not what this row measures.
- **An untyped schema is an alias with no validator.** `{"not": {...}}`,
  `{"contains": {...}}`, `{"if": ..., "then": ...}`, `{"propertyNames": ...}`
  and the like come out as `pub type Root = serde_json::Value;`, and an alias
  gets no `TYPE_VALIDATORS`, no `validate` and no `TryFrom`. 501 of the 1,530
  modules are aliases; 352 of those are `serde_json::Value`. (Fewer than at
  1d1774d: a root map or array is a self-validating newtype now.)

The harness calls `serde_json::from_value::<Root>(v)`, the same entry point as
every other Rust row, because that's what a program that deserializes into the
generated types gets. Of the 410 required false accepts, most have an alias
root, so no entry point would catch them (at 1d1774d it was 315 of 395):
`unevaluatedProperties` (58), `unevaluatedItems` (38), `not` (31),
`if-then-else` (24), `contains` (23), `additionalItems` (20), `uniqueItems`
(20), `propertyNames` (20), `items` (19), `dependencies` (17),
`patternProperties` (15), `maxProperties` (14). A reviewer's one-off run at
1d1774d with `TryFrom<Value>` on every root that implements it (not kept)
turned 79 required groups to pass across all drafts, 44 of them
`unevaluatedProperties`, and moved 8 others between failure buckets. Both
are the crate's choices, which this row reports as made: an untyped schema
is `Value`, and a struct's `Deserialize` is shape-only for speed.

Nothing is stripped from the generated code. The module is the buffer
followed by `__conformance_check`. No module refers to a `shared` module.

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
directory. The library is needed for the `self_validating!`, `constrained_newtype!`
and `deserialize_union!` macros, `json_equal` and the derive's `impl
::json_schema_rs::ToJsonSchema`; the macro crate for
`#[derive(json_schema_rs_macro::Deserialize, json_schema_rs_macro::ToJsonSchema)]`
and their helper attributes.
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
- Against typify 0.10.1 on draft 7 (required): json-schema-rs 144, typify
  80. typify still wins ten groups outright: `not` (forbidden property,
  forbid everything with an empty or `true` schema), `propertyNames` with
  `pattern` or `enum`, `additionalProperties` not looking in applicators and
  its non-ASCII `pattern` group, `items: false`, and the two nullable-root
  groups described above.
