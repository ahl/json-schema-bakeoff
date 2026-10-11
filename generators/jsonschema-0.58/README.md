# jsonschema-0.58

[jsonschema](https://github.com/Stranger6667/jsonschema) 0.58 line, pinned in
`Cargo.toml` and read back out of `Cargo.lock` by `build.rs`, so the results
directory names the exact version that produced them.

## What this row means

`jsonschema` is a validator, not a code generator. It is in the bakeoff as a
**calibration row**, not as a competitor: it passes essentially the whole
official suite on its own terms, so wherever this row is not ✅, the cause is
something about *the harness* — a policy it applies, a limitation of the
contract, or a bug — and not the tool's conformance.

As of the draft pass-through it is ✅ on **every** required group in all five
drafts (1408 of 1408), so it is a clean baseline: read the other rows against
it. A group that is 🚧 or 🙈 for every generator *including* this one is
probably the harness; a group that is ✅ here and not elsewhere is a real
difference in what the generator models.

It fits the generator contract unchanged. The "generated module" for a group
embeds that group's schema verbatim and its `__conformance_check` compiles
the schema with `jsonschema` (once per process, in a `LazyLock`) and
validates the instance:

- accept = `validator.validate(&instance)` returned `Ok`
- reject = the message of the first validation error
- a schema `jsonschema` refuses to compile is kept as the compile error and
  reported for every case in the group, as `schema failed to compile: …`.
  That shows up as 🚧 when any case in the group was `valid: true`, and as ✅
  when every case was `valid: false` — a group can look like a pass for the
  wrong reason, so check the per-case error text before celebrating one.

The schema is embedded as raw JSON text (`serde_json::value::RawValue`) with
a raw-string delimiter sized to the content, so the bytes the harness sends
are the bytes the validator parses. Nothing round-trips through `f64` on the
way in, which `multipleOf: 0.0001` and the bignum bounds care about; the
batch crate parses it back with `float_roundtrip` on.

## Draft

The contract hands every group the suite directory it came from — `draft4`,
`draft6`, `draft7`, `draft2019-09`, `draft2020-12` — as the `draft` field of
each generate item (and as `BAKEOFF_DRAFT` in the environment; the per-item
field is what this generator reads, since that is the one that travels with
the schema). It maps one-to-one onto a `jsonschema::Draft`:

| suite directory | `jsonschema::Draft` |
|---|---|
| `draft4` | `Draft4` |
| `draft6` | `Draft6` |
| `draft7` | `Draft7` |
| `draft2019-09` | `Draft201909` |
| `draft2020-12` | `Draft202012` |

and each module calls `.with_draft(…)` with it. Any other name is a
`generation_error` naming the draft, rather than a module that won't compile.

**The suite directory wins over the schema's own `$schema`.** In this crate
`with_draft` is an override, not a default — a schema that declares
`$schema: draft-07` still compiles as 2020-12 under
`with_draft(Draft202012)`, and vice versa (there is a test pinning both
directions). That is the behavior we want: the directory a group lives in is
the suite's own claim about which dialect the group is testing, and it is
never absent, whereas `$schema` is absent from most suite schemas. A root
`$schema` that disagrees with the directory is the suite saying something
about `$schema` handling, not asking for a different dialect — and
`$schema` inside *referenced resources* is untouched by this, so the
cross-draft and `vocabulary.json` groups still get their own dialects and
still pass.

This is the whole ballgame for this row. Before the draft was passed through,
everything defaulted to 2020-12 and draft 4/6/7 sat at 81–91% on required
tests — the `items` array form, draft 4's boolean `exclusiveMaximum`, `id`
versus `$id`, `$ref` siblings, and `$ref`s to a draft's own meta-schema all
failed against the wrong meta-schema. With the draft wired up, all five
drafts are at **100.0% on required groups**.

## Remote references

The harness's standing policy is that the suite's `http://localhost:1234/…`
references are not resolved and a generator records whatever it does with
them (typify panics, quicktype reports a fetch error). This row is the
exception: it installs a `jsonschema::Retrieve` implementation that maps
`http://localhost:1234/<path>` onto `<BAKEOFF_ROOT>/suite/remotes/<path>` and
reads it from disk. The absolute remotes directory is baked into each module
at generate time, since the batch crate runs from a scratch directory of its
own.

Any other URI — any other host, scheme, or anything but a plain relative
path under `remotes/` (no root, no `..`, no empty path) — is an error naming
the URI, never a fetch and never a read outside that directory. The
reference layer normalizes `..` (and percent-decodes `%2e` first) before the
retriever ever sees a URI, so the guard is there for the cases normalization
leaves alone, `http://localhost:1234//etc/hosts` among them. The retriever
lives in `src/retrieve.inc.rs`, embedded verbatim in every module and
`include!`d by this crate's tests, so those refusals are tested rather than
asserted. Nothing here can reach the network:
`jsonschema` is depended on with `default-features = false`, which drops its
`resolve-http` feature and the whole reqwest/rustls stack, so there is no
HTTP client compiled in to be reached by accident.

Resolving remotes here is deliberate: it is the only way to tell how many
groups the other rows are *forfeiting* to the policy rather than failing on
their merits. For draft 2020-12, `refRemote.json` is 15 groups of the 384
required ones (11 in draft 7, 8 in draft 4); `ref.json` adds a handful more
that reach a remote. Those
groups are ✅ here and can be read as a floor on what a generator gives up
by not resolving them.

## Formats

Left at the crate's default, which is the spec's: `format` is an assertion in
draft 4/6/7 and an annotation from 2019-09 on. That split shows in the
results: `optional/format` is 100% for the old drafts and 🙈 on 24 groups in
each of the new ones, where an invalid string passes because `format` is only
an annotation there.

`should_validate_formats(true)` was tried. It clears those 48 optional groups
but fails 19 *required* 2020-12 groups: `format.json` there has a case per
format, "invalid X string is only an annotation by default", marked valid.
Required conformance wins, so the option is off.

Unknown formats are ignored, also the default. One wrinkle worth knowing:
`idn-hostname` and `idn-email` are backed by the crate's `idna` feature, so
turning default features off made them *unknown* formats — silently ignored,
which looked exactly like the validator being too loose in draft 7. `idna`
is therefore re-enabled by hand in `Cargo.toml` and in the `batch-deps.toml`
the generate step writes. If you ever change the feature set here, check
`optional/format` for formats that quietly stopped being checked.

## Layout

| file | what |
|---|---|
| `generator.toml` | the three contract commands; `build` delegates to `rust-batch` |
| `Cargo.toml` | the pin (`=0.58.6`, `default-features = false` plus `idna`) |
| `build.rs` | lifts the pinned version out of `Cargo.lock` into `JSONSCHEMA_CRATE_VERSION` |
| `src/main.rs` | `version` and `generate`; also writes `batch-deps.toml` so the batch crate depends on the same pinned `jsonschema` |
| `src/generate.rs` | the draft mapping, the module template, the raw-string sizing, and tests that check the pinned crate still behaves the way the template assumes |
| `src/retrieve.inc.rs` | the `jsonschema::Retrieve` impl the modules embed, compiled into the tests too so what it serves and refuses is checked |

Generation can only fail on an unrecognized draft name — embedding a schema
in a string always works — but it still runs on a 256 MiB-stack thread under `catch_unwind`
with a timeout and the shared SIGABRT crash guard, the same shape as the real
code generators, so a surprise is recorded as `generation_panic` instead of
taking the run down.
