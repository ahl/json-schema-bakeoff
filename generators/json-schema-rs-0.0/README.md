# json-schema-rs 0.0

[json-schema-rs](https://github.com/goddtriffin/json-schema-rs), pinned to
`=0.0.6` in `Cargo.toml`. The crate version comes out of `Cargo.lock` via
`build.rs`, so `version` prints `json-schema-rs 0.0.6` and that names the
results directory.

## Draft

The contract hands every group the suite directory it came from — `draft4`,
`draft6`, `draft7`, `draft2019-09`, `draft2020-12` — as the `draft` field of
each generate item (and as `BAKEOFF_DRAFT`; the per-item field is the one
this generator reads, since it is the one that travels with the schema).
The crate's one dialect knob is `JsonSchemaSettings::spec_version`, and all
five directories map onto a variant:

| suite directory | `SpecVersion` |
|---|---|
| `draft4` | `Draft04` |
| `draft6` | `Draft06` |
| `draft7` | `Draft07` |
| `draft2019-09` | `Draft201909` |
| `draft2020-12` | `Draft202012` |

(The enum also has `Draft00` through `Draft03` and `Draft05`, which the suite
has no directories for.) A draft name outside that table — or a missing one —
is a `generation_error` for the group naming the draft, rather than a
generation against a guessed dialect.

**The suite directory wins over the schema's own `$schema`,** the same choice
`generators/jsonschema-0.58` makes. Setting `spec_version` is what makes that
true: the crate consults `$schema` only when the setting is `None`, and falls
back to 2020-12 when `$schema` is absent, which most suite schemas are. The
directory is the suite's own claim about which dialect a group tests and is
never missing, so it is the one to follow. Schemas themselves are passed
through byte-for-byte, `$schema` included.

**It changes nothing in 0.0.6.** `spec_version` is inert: the only thing that
reads it is the public `resolved_spec_version`, which the crate calls from
nowhere but its own tests — not from parsing, codegen or validation. So the
draft wiring buys no result today; it is here because the contract says a tool
that takes a dialect should be given one, and because a later version that
makes the setting live would then be measured correctly. A test
(`the_dialect_is_inert_in_this_version`) pins the inertness on a schema built
out of dialect-sensitive keywords, so if that ever changes it shows up as a
test failure rather than as a silent shift in the scoreboard.

## Settings

`JsonSchemaSettings::default()` for ingestion apart from `spec_version` above,
so `disallow_unknown_fields` stays `false` and keywords the crate doesn't
model are ignored rather than rejected.

`CodeGenSettings` with `dedupe_mode(DedupeMode::Disabled)` and the default
`ModelNameSource::TitleFirst`. `Disabled` is not a tuning choice: it is the
only mode whose output is usable as a module.

- `Disabled` returns one buffer and no shared buffer, so a module is the
  buffer verbatim.
- `Full` and `Functional` move shared items into the shared buffer and have
  the per-schema buffer reach them with `pub use crate::A;`, which cannot
  resolve inside a non-root module. (`Full` also merges structurally identical
  structs, so `{"a": {...}, "b": {...}}` with two identical object properties
  comes out as `b: Option<A>` — a different type graph for the same schema.)

Those are the real reasons. Two claims an earlier draft of this file made are
wrong, and are recorded here so nobody re-derives them: `CodeGenError::
RootNotObject` is reported in `Full` as well as in `Disabled`, and `Full`
emits the root `anyOf`/`oneOf` enum too. The only mode-dependent behavior that
matters here is the shared buffer.

Within a single schema there is little for dedupe to do: `collect_structs`
keeps a `seen` set and drops a second struct that would take a name it has
already used. Enums get no such treatment in any mode — see the duplicate
`RootAnyOf` under Notes.

## The root type's name

There is no "call the root `Root`" setting. The root item is named from the
root schema's `title`; failing that, from the `$defs`/`definitions` key when
the root is a `$ref`; failing that, `Root`. A root `anyOf` or `oneOf` becomes
an enum with `AnyOf`/`OneOf` appended (`RootAnyOf`, `MyThingOneOf`).

`generate.rs` predicts that name from the parsed schema and confirms the item
was actually declared. When the prediction misses it falls back to the last
`pub struct`, then to the last `pub enum`.

The fallback is not theoretical: across all five drafts, 109 groups generate
code and 21 of them take it, every one of them a root `allOf`. That is the
case the prediction cannot cover — the crate merges a root `allOf` with a
private function and names the merged schema, which no longer carries the
root's own `title` — and the merge result is always a struct. Structs are
emitted after every enum and in post-order, so the last one is the root, and
all 21 choose `Root`, which is right.

The ordering assumption only holds for a struct root: were the prediction ever
to miss on an *enum* root, the fallback would pick a struct emitted later in
the buffer and measure the wrong type. No suite group is in that position (of
the 71 groups whose root is an enum, none emits a struct at all), but a module
where a root-`anyOf` schema also produced structs would be worth re-checking
by hand.

## What is stripped from the generated code

Nothing. The buffer opens with a `//!` doc comment and `use` items, both
legal at the top of a module file, and never emits `extern crate` or a
`crate::`-rooted path (that last part is what `DedupeMode::Disabled` buys).
The module is the buffer followed by `__conformance_check`, which is just
`serde_json::from_value::<Root>(v)`: the `#[json_schema(...)]` attributes the
tool emits alongside its serde attributes are inert to serde, and that is the
point — the measurement is what plain serde deserialization enforces.

## What the batch crate needed

`generate` writes `<out_dir>/batch-deps.toml`, which rust-batch appends to
the batch crate's manifest:

```toml
json-schema-rs = "=0.0.6"
json-schema-rs-macro = "=0.0.6"

[build-dependencies]
syn = { version = "2", features = ["full"] }
```

Both versions are `concat!`ed from the same `JSR_CRATE_VERSION` that names the
results directory, so bumping the pin in `Cargo.toml` can't leave the batch
crate compiling the generated code against a different json-schema-rs. The two
crates are released together; if some future version ships without a matching
macro release, the batch crate fails to resolve, loudly, which is the failure
we want.

Every generated struct derives `json_schema_rs_macro::ToJsonSchema` and
carries `#[json_schema(...)]` helper attributes, so the macro crate has to be
there for the attributes to be inert; the derive expands to
`impl ::json_schema_rs::ToJsonSchema for ...`, so the library is needed too
(which drags `clap` into the batch crate — unavoidable, since the library
ships its CLI in the same crate).

The `syn` line is a workaround rather than a dependency of the generated
code: `json-schema-rs-macro` 0.0.6 refers to `syn::File` but depends on `syn`
without its `full` feature, so it does not compile unless something else in
the host (proc-macro/build) dependency graph enables that feature. Cargo's v2
feature resolver keeps normal and host dependencies separate, so a normal
`syn` dependency has no effect; a build-dependency does, and does so without
the batch crate needing a build script.

One consequence worth knowing: `ToJsonSchema` is implemented for the scalar
types, `Option`, `Vec`, `HashSet`, `Box` and `BTreeMap<String, V>` but not
for `serde_json::Value`, which is what the emitter falls back to for a
property it can't type. Generated code with such a field fails to compile on
the derive even though its serde types are fine. No group in the suite reached
that state — the `Value` fallback shows up in `anyOf`/`oneOf` enum variants,
and those enums carry no `ToJsonSchema` derive — but it is a plausible
`compile_fail` cause elsewhere.

## Notes

- A self-recursive schema hangs the tool: `{"type": "object", "properties":
  {"x": {"$ref": "#"}}}` walks `$ref`s with no visited set and never returns.
  That's the only `generation_panic` in the suite, recorded by the 60s
  timeout. Because of it the panic hook is installed once at the top rather
  than around each generation — the per-generation form needs a
  process-global lock, and the abandoned thread would hold it and turn every
  later group in the same `generate` process into a bogus timeout.
  (`generators/typify-0.10/src/generate.rs` has the per-generation form; it
  has no visible effect there only because typify doesn't hang.) The hook
  only intercepts panics while a generation is in flight (`IN_GENERATION`);
  a panic in this program's own code still prints, because an installed-
  forever hook that eats everything would reach the harness as "generate
  printed something other than a status array" and nothing else.
- A boolean schema (`true`/`false`) doesn't deserialize into the crate's
  model at all, so it is a `generation_error` from the parse step.
- So is any schema whose root is not `type: "object"` with non-empty
  `properties` — including a bare `{"properties": {...}}` with no `type`.
  That is most of the suite.
- The crate models one keyword per field, so `type: ["string", "null"]`,
  tuple `items`, and friends fail to parse rather than being ignored.
- Three groups per draft are `compile_fail`, and all three are the tool's
  doing rather than this glue's:
  - `anyOf.json::nested anyOf` and `oneOf.json::nested oneOf` emit the same
    `pub enum RootAnyOf` / `RootOneOf` twice, once for the inner applicator
    and once for the root. Every `DedupeMode` does this, `Full` included.
  - `default.json` emits `fn default_root_alpha() -> Option<f64> { Some(5) }`
    for `"default": 5` on a `"type": "number"` property — an integer literal
    where an `f64` is wanted.
- A root `allOf` merge drops the root schema's own sibling keywords:
  `allOf.json::allOf with base schema` has `properties: {bar}` next to its
  `allOf`, and the generated `Root` has only `foo` — hence 🙈 rather than ✅.
- Constraints are recorded as `#[json_schema(min_length = 3, pattern = "^x")]`
  next to the serde attributes, which serde ignores — that is the thing this
  bakeoff measures. The one constraint that does reach serde is a numeric
  range: `{"type": "integer", "minimum": 1, "maximum": 5}` is emitted as
  `u8`, so serde rejects `-1` and `300` while accepting `0` and `6`. Partial
  enforcement, by way of type choice rather than intent.
- A `"type": "null"` property is dropped from the struct entirely, `allOf` or
  not, and dropped even when it is `required`; so is anything else the
  emitter has no type for. The property simply isn't in the struct, which
  with serde means "accepted and ignored": another 🙈.
