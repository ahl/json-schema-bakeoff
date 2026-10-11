# schemafy

[schemafy](https://github.com/Marwes/schemafy) is the original Rust JSON
Schema → serde generator. It's normally used as the `schemafy!` proc macro,
which drops types for a schema file into a module at compile time;
`schemafy_lib` is the library behind that macro, and this generator calls it
directly.

- **Version:** `schemafy_lib` is pinned in [`Cargo.toml`](Cargo.toml)
  (currently `=0.6.0`, the latest release). `build.rs` reads the resolved
  version out of `Cargo.lock` so the identity string — and therefore
  `results/<id>/` — always names the version that produced the numbers.
- **API:** `Expander::new(Some("Root"), "::schemafy_core::", &schema)`
  followed by `expander.expand(&schema)`, which returns a
  `proc_macro2::TokenStream`. The alternative,
  `Generator::builder().with_root_name_str("Root").with_input_file(path)`,
  wraps the same `Expander` but reads the schema from a file and `panic!`s
  on a read or parse failure, which would turn every unparseable schema into
  a panic rather than an error. `generate.rs` deserializes the schema itself
  and drives the `Expander`.

## Root naming

`Root` is schemafy's `root_name`, and schemafy uses it verbatim (it is
already PascalCase). What it names depends on the schema:

- a `pub struct Root` when the root has `properties`, or
  `additionalProperties: false`;
- a `pub enum Root` (or `pub type Root = Option<Root_>` when `null` is one
  of the values) when the root has a non-empty `enum`;
- otherwise `pub type Root = <something>` — `i64`, `String`, `bool`, `f64`,
  `Vec<T>`, `::std::collections::BTreeMap<String, T>`, `serde_json::Value`
  for an unconstrained or unrepresentable root, or a named type for a
  `$ref`.

There is one case where `Root` doesn't exist at all: schemafy skips a
self-referential alias, so a root of `{"$ref": "#"}` — whose `$ref`
resolves to the root's own name — emits nothing. The module then carries a
marker const explaining that and still references `Root`, so rustc's
"cannot find type `Root`" puts the group in `compile_fail`, which is the
honest outcome.

## What the glue adds and strips

- **Adds `use ::serde::{Deserialize, Serialize};`.** schemafy's output
  names the derives unqualified, exactly as schemafy's own crate does with
  `#[macro_use] extern crate serde_derive`; the caller is expected to have
  imported them, and schemafy's doc example does. The import is added to
  every module.
- **Adds `use ::serde_repr::{Deserialize_repr, Serialize_repr};`** only to
  a module that actually derives them (an integer `enum` with `enumNames`).
- **Strips inner attributes (`#![...]`) and `extern crate` items**, which
  are legal at a crate root but not in the non-root module the batch crate
  compiles each group as. schemafy 0.6 emits neither; the strip is
  belt-and-braces against a future version that does.
- Output is run through `prettyplease` when it parses as a `syn::File`, and
  handed to rustc as raw tokens when it doesn't, so unparseable output lands
  in `compile_fail` with a real diagnostic instead of being hidden.

Nothing else is changed: the structure, the field names, the `#[serde]`
attributes, and the missing validation are all schemafy's.

## Batch deps

`generate` writes [`batch-deps.toml`](../../rust-batch/src/main.rs) into the
output directory with two pins beyond the batch crate's standing
serde/serde_json/regress/chrono/uuid:

```toml
schemafy_core = "=0.6.0"
serde_repr = "=0.1.21"
```

`schemafy_core` holds `one_or_many`, which schemafy's output names in
`#[serde(with = "::schemafy_core::one_or_many")]` for an `anyOf` of `T` and
`array of T`; `serde_repr` backs the `_repr` derives. As it happens the
suite reaches neither — `enumNames` is not a standard keyword, and the
`one_or_many` shortcut needs a very specific `anyOf` shape — but both are
part of schemafy's output contract, so the batch crate carries them.

## Draft

**`draft` is passed and ignored.** schemafy has exactly one dialect and no
setting to change it: its input type is `schemafy_lib::Schema`, a struct
generated from the draft-4 meta-schema, and `$schema` is just another field
it reads and never acts on. Its results are therefore identical in every
suite directory, and the per-draft differences on the scoreboard come
entirely from the suite's own contents — including the structural fact that
`exclusiveMaximum`/`exclusiveMinimum` are booleans in draft 4 and numbers
from draft 6 on, which is why draft 4 has no `generation_error` groups and
the later drafts have many.

## Quirks

- **`error` means one thing only.** `Expander::expand` returns a
  `TokenStream`, not a `Result`; schemafy reports every problem by
  panicking. So `generation_error` is exclusively "the schema does not
  deserialize as `schemafy_lib::Schema`" — a boolean schema (`true`/`false`,
  draft 6 and up), a numeric `exclusiveMaximum`/`exclusiveMinimum`, an
  integer `maxLength`-style keyword that doesn't fit, a `type` that isn't
  one of the seven draft-4 simple types. Everything else that goes wrong is
  `generation_panic`.
- **Unknown keywords are dropped silently.** `Schema` has no
  `deny_unknown_fields`, so `const`, `$defs`, `if`/`then`/`else`,
  `unevaluatedProperties`, `contains`, `propertyNames` and friends are read
  past and contribute nothing to the type. That is the single biggest source
  of `false_accept`.
- **Panics are load-bearing.** A `$ref` that `allOf` merging has to resolve
  and can't (`Expected definition: ...`), a non-string `enum` value
  (`Expected string for enum got 1`), `enumNames` of a different length
  than `enum`, and an `additionalProperties` schema that doesn't
  deserialize all `panic!`.
- **Recursion has no cycle check, so some schemas overflow the stack**, and
  a stack overflow aborts the process regardless of stack size or
  `catch_unwind`. `generate` is therefore a supervisor over a
  `generate-worker` subprocess that streams one status line per group; a
  worker that dies costs that one group (recorded as a panic with the
  runtime's message) rather than the whole draft. This is the same
  restart-past-the-cursor shape the harness uses for runner processes, and
  it is the one structural difference from the typify generators. Five
  groups in the suite reach it, all in draft2019-09's
  `unevaluatedProperties.json`: a root with `type: "object"`, `properties`,
  and an `allOf` whose merged subschema contributes no properties of its own
  sends `Expander`'s `allOf` merging into unbounded recursion. (A plainly
  self-referential `$ref`, by contrast, does *not* overflow — schemafy turns
  it into a `Box<T>` field.)

  Three details earn their keep:

  - The worker prints `{"ready": {"start": n}}` as soon as it has the group
    list, the way the harness's runner protocol does. Without it, a worker
    that fails *before* generating anything (an unreadable group file, a
    broken executable) looks exactly like a worker that crashed on the
    group at the cursor, and the supervisor would blame the whole draft on
    schemas one group at a time. No handshake is a hard failure instead.
  - A generation that exceeds the 60 s timeout is abandoned, and the
    abandoned thread still holds the panic-hook mutex and may yet overflow
    the stack. So the worker reports the group (`{"timeout": ...}`) and
    exits; the supervisor resumes in a fresh process at the next group and
    blames nobody for the exit. Otherwise one timeout would wedge every
    group after it on the hook mutex, or abort the process later under
    whatever group happened to be current.
  - The supervisor gives a worker 90 s to say *anything* before concluding
    it is stuck somewhere its own per-group timeout doesn't cover, killing
    it, and blaming the group at the cursor. The per-group timeout is the
    first line of defense; this is the backstop, and it means no single
    group can stall a draft indefinitely.
- **A `$ref` in a field position is never resolved — it is turned into a
  type name.** `{"$ref": "#/properties/foo"}` becomes the type `Foo`, from
  the last path component, whether or not anything defines `Foo`. Only
  `definitions` entries get types emitted, so most of `ref.json` and all of
  `refRemote.json` either name a type that doesn't exist (`compile_fail`) or
  panic. Remote `http://localhost:1234/...` references are left to fail,
  per the harness policy; schemafy takes the fragment and ignores the host,
  so they come out as dangling names or panics, never fetches.
- **Field and variant names collide.** Names are snake_cased and
  PascalCased with `Inflector` and sanitized by replacing every
  non-alphanumeric character with `_`, with no uniquifying pass, so
  `properties: {"foo\tbar": ..., "foo\nbar": ...}` yields two fields called
  `foo_bar` and `compile_fail`.
- **`oneOf` becomes an untagged enum, and an `id` on a subschema breaks
  it.** Variants are named from each subschema's `id` when it has one and
  `Variant0`, `Variant1`, ... when it doesn't; the variant *type* is the
  `$ref`'s definition name, else a generated `Root<variant name>`. The
  inline cases work — `oneOf: [{"type": "integer"}, {"type": "string"}]`
  really does accept an integer or a string — but `id` is used raw: an `id`
  of `"#alpha"` panics in `quote` (`"#alpha" is not a valid Ident`), and an
  `id` of `"beta_thing"` emits the variant `beta_thing(Rootbeta_thing)`
  against a type schemafy declared as `RootbetaThing`, which doesn't
  compile.
- **`anyOf` is `serde_json::Value`** unless it is exactly the
  `one_or_many` shape, so it accepts everything.
- **Required-ness is the only constraint it models.** A property not in
  `required` becomes `Option<T>` with `skip_serializing_if`; everything else
  — `minimum`, `maxLength`, `pattern`, `uniqueItems`, `format`, `multipleOf`
  — is absent from the type. `format.json` is 100% precisely because the
  non-optional format tests all assert that a mismatched format is *valid*.
