# json-schema-rs (git)

[json-schema-rs](https://github.com/goddtriffin/json-schema-rs) built from
unreleased main, pinned in `Cargo.toml` to
`rev = "892e0882d2339236f8e58b460154b838349c5cc4"`. That's the merge of PR #71
("Codegen: cross-document $ref with file and HTTP retrieval") on top of PR
#68, the `feat/codegen-type-graph` branch ("Codegen: TypeGraph IR, recursion,
conjunctions, unions, lossless fields"). The workspace version is still 0.0.6,
so `build.rs` reads both the version and the sha out of `Cargo.lock`,
`version` prints `json-schema-rs 0.0.6+892e088`, and results land in
`results/json-schema-rs-0.0.6-892e088/`. Bumping the `rev` starts a new
results directory.

## What changed since 0.0.6

0.0.6 is the last published version (crates.io, 2026-03). This directory first
held a 0.0.6 integration; its results stay under `results/json-schema-rs-0.0.6/`,
hidden from the tables, and that glue is gone. Required groups passed:

| draft | 0.0.6 | main (892e088) |
|---|---|---|
| draft4 | 2/160 (1.2%) | 77/160 (48.1%) |
| draft6 | 2/233 (0.9%) | 115/233 (49.4%) |
| draft7 | 2/258 (0.8%) | 123/258 (47.7%) |
| draft2019-09 | 5/373 (1.3%) | 151/373 (40.5%) |
| draft2020-12 | 5/384 (1.3%) | 153/384 (39.8%) |

Everything in that jump is PR #68, the `feat/codegen-type-graph` branch: 0.0.6
refused any root that wasn't an object with `properties` (1,298 of 1,408
required groups were a `generation_error`), and the rewrite generates a type
for any schema. The sections below spell out what it does and doesn't do.
PR #71 (cross-document `$ref` retrieval) changes nothing on the suite: the
glue keeps the default offline retriever, so the suite's remote refs fail as
they did before, which is the policy for every generator. The branch head
before the merge (`e9c1b42`) was measured too, with the same outcome in every
one of the 1,709 groups, and that directory was dropped.

The glue differs from the 0.0.6 glue: the root-type detection is new, the
dialect setting moved, and the batch crate's dependencies are git pins.

## Provenance

The repo is run by Claude Code. At the pinned commit it has a `CLAUDE.md`
(added 2026-07-04) of "hard requirements" addressed to the agent and a
`.claude/skills/contribution-guide` skill it must invoke before any change.
Of 218 commits, every one since July 2026 that isn't a merge carries a
`Claude-Session:` trailer (30 trailers, 14 more with a Claude `Co-Authored-By`);
the January-to-March 2026 commits that built the validator carry no trailer,
so nothing here says who wrote those. The type-graph codegen this row
measures is the trailered part: PRs #68 and #71 are 85 files and about
37,600 insertions, committed between 2026-10-26 and 2026-10-30 (71 commits,
33 of them on the 29th).

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
default (`ModelNameSource::TitleFirst`, no `unknown_properties` override).

`Disabled` is not the default (`Full` is), and it doesn't change what serde
accepts. With one schema per call no mode produces a shared buffer (only
items used by two or more schemas are shared). Across the suite, `Full` and
`Disabled` output differs in 9 groups (`definitions.json::validate
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
all drafts: 915 `alias`, 587 `position`, 0 `position-past-def`. I read
the modules where more than one item qualified and the root isn't named
`Root` or the title, and each picked the right type: numeric suffixes from a
name clash (`Root0`/`Root1` for a type-array root whose object branch refers
back to the root, `CoreSchemaMetaSchema0` for the metaschema groups), a root
union named for the `$defs` key of its first `allOf` branch (`One0`), and a
root whose `$ref` is a relative URI (`BaseSchema`). An earlier version
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

## What is stripped from the generated code

Nothing. The buffer opens with a `//!` doc comment and `use` items, and in
`DedupeMode::Disabled` never refers to a `shared` module. The module is the
buffer followed by `__conformance_check`, which is
`serde_json::from_value::<Root>(v)`. The `#[json_schema(...)]` attributes are
inert to serde; the crate's `validate()` is never called. Non-string enums
get hand-written `Deserialize` impls that compare with
`json_schema_rs::json_equal`, which is serde deserialization too, so `enum`
and `const` are enforced for real.

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

- The "root must be an object with `properties`" restriction is gone. In
  0.0.6, 1,298 of 1,408 required groups were a `generation_error`; here 81
  are, and 65 of those are remote refs (`localhost:1234`, left to fail by
  policy). The rest: boolean documents (`true`/`false` as the whole schema, 8
  groups; the crate's CLI refuses them too, with the same error), and 8 groups
  (`ref.json::refs with relative uris and defs`, `relative refs with
  absolute uris and defs`, each draft from 6) where a `$ref` inside an
  embedded resource with its own `$id` fails to resolve: "is not in this
  schema's registry".
- Recursion works. The self-recursive schema that hung 0.0.6 (its only
  `generation_panic`) comes out as `Option<Box<Root>>`; no group times out.
- `allOf` is a conjunction now, so `allOf with base schema` merges the root's
  own `properties` with the branches' and passes.
- `anyOf`/`oneOf` are `#[serde(untagged)]` enums, so `oneOf` exclusivity and
  any constraint a branch carries are lost. All 50 "regressions" in
  `cargo run -- diff` against 0.0.6 went from 🚧 to 🙈, from rejecting
  valid instances to accepting invalid ones; 36 are `anyOf`/`oneOf` groups,
  the rest `$recursiveRef` and `unevaluated*` groups built on them.
- A schema with `format` and no `type` is typed as `String`, so the 70
  `format.json` groups reject non-strings that the spec says pass.
- A schema with `properties` and no `type` is a struct, so a non-object
  instance is rejected (`ref.json::root pointer ref` rejects `{"foo":
  false}` on the inner `false`). serde also accepts a JSON array for a
  struct, positionally, which is why some `additionalProperties: false`
  groups reject arrays with "expected fewer elements".
- Constraints (`minLength`, `pattern`, `maxItems`, `multipleOf`, ...) are
  `#[json_schema(...)]` attributes, which serde ignores; that's most of the
  false accepts. Integer bounds still pick the narrowest integer type, so a
  range is partly enforced by the type.
- Four `compile_fail`s, all
  `const.json::float and integers are equal up to 64-bit representation
  limits`: the `ToJsonSchema` derive turns
  `#[json_schema(value = 9007199254740992)]` into an `i32` literal.
