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

It is ✅ on **every** Standard group (540 of 540) and every Compound group (5
of 5), so it is a clean baseline: read the other rows against it. A group
that is 🚧 or 🙈 for every generator *including* this one is probably the
harness; a group that is ✅ here and not elsewhere is a real difference in
what the generator models. Optional is 64 of 98 and Upcoming is 0 of 25, for
reasons that are about the harness's policies and the crate's dialect list
(see Formats, Bignums and Draft below), not about conformance.

It fits the generator contract unchanged. The "generated module" for a group
embeds that group's schema and external schemas verbatim, and its
`__conformance_check` compiles the schema with `jsonschema` (once per
process, in a `LazyLock`) and validates the instance:

- accept = `validator.validate(&instance)` returned `Ok`
- reject = the message of the first validation error
- a schema `jsonschema` refuses to compile is kept as the compile error and
  reported for every case in the group, as `schema failed to compile: …`.
  That shows up as 🚧 when any case in the group was `valid: true`, and as ✅
  when every case was `valid: false` — a group can look like a pass for the
  wrong reason, so check the per-case error text before celebrating one.

The schema and the external schemas are each embedded as raw JSON text
(`serde_json::value::RawValue`) with a raw-string delimiter sized to the
content, so the bytes the harness sends are the bytes the validator parses.
Nothing round-trips through `f64` on the way in, which `multipleOf: 0.0001` and
the bignum bounds care about; the batch crate parses it back with
`float_roundtrip` on.

## Draft

The harness runs each case once, under the newest dialect its
`compatibility` allows, and hands that dialect to every generate item as
its `draft` field. It maps one-to-one onto a `jsonschema::Draft`:

| dialect | `jsonschema::Draft` |
|---|---|
| `draft4` | `Draft4` |
| `draft6` | `Draft6` |
| `draft7` | `Draft7` |
| `draft2019-09` | `Draft201909` |
| `draft2020-12` | `Draft202012` |

and each module calls `.with_draft(...)` with it. Any other name is a
`generation_error` naming the dialect, rather than a module that won't
compile. That includes `v1`, the dialect of the Upcoming tier: the pinned
crate has no `Draft` for it, so every Upcoming case is an error reading
`unsupported dialect "v1"`.

**The harness's dialect wins over the schema's own `$schema`.** In this
crate `with_draft` is an override, not a default: a schema that declares
`$schema: draft-07` still compiles as 2020-12 under
`with_draft(Draft202012)`, and vice versa (there is a test pinning both
directions). That is the behavior we want: the dialect is the harness's
claim about which rules the case is run under, and it is never absent,
whereas the suite's schemas carry no `$schema` at all except six that name
a custom meta-schema. Those six still work: the custom meta-schema comes
from the case's external schemas (below) and its `$vocabulary` is honored.
`$schema` inside *referenced resources* is untouched by `with_draft`, so
the cross-draft and `vocabulary.json` cases still get their own dialects
and still pass.

This is the whole ballgame for this row. Most groups run as 2020-12, but 131
of the 540 Standard groups run under an older dialect (48 draft 7, 33 draft
4, 50 draft 2019-09), and compiled as 2020-12 plenty of them fail against
the wrong meta-schema: the `items` array form, draft 4's boolean
`exclusiveMaximum`, `id` versus `$id`, `$ref` siblings, and `$ref`s to a
draft's own meta-schema. With the dialect wired up, Standard is at
**100.0%**.

## Remote references

The harness's standing policy is that the suite's `http://localhost:1234/...`
references are not resolved and a generator records whatever it does with
them (typify panics, quicktype reports a fetch error). This row is the
exception. A case that references other schemas carries them in its
`externalSchemas`, an object from retrieval URI to schema, and the harness
passes that object along with the item. The module embeds it as JSON text,
parses it once when it builds the validator, and installs a
`jsonschema::Retrieve` implementation that serves only those schemas.
`jsonschema` calls the retriever both for a `$ref` target and for a
`$schema` naming a meta-schema it doesn't bundle, so the six custom
meta-schema cases resolve the same way.

Any other URI is an error naming it: another host, scheme or port, a suite
remote that a *different* case supplies, and anything that looks like a
path. There is no filesystem read and no fetch behind the lookup, only a
map from the case's own keys. The keys are normalized the way the reference
layer normalizes a URI before it calls the retriever, so a lookup matches
however the suite spelled one. The retriever lives in `src/retrieve.inc.rs`,
embedded verbatim in every module and `include!`d by this crate's tests, so
what it serves and refuses is tested against the pinned crate rather than
asserted. Nothing here can reach the network: `jsonschema` is depended on
with `default-features = false`, which drops its `resolve-http` feature and
the whole reqwest/rustls stack, so there is no HTTP client compiled in to
be reached by accident.

Resolving remotes here is deliberate: it is the only way to tell how many
groups the other rows are *forfeiting* to the policy rather than failing on
their merits. In the suite at e2a68a1, 57 cases carry `externalSchemas`:
46 Standard (`id-legacy.json`, `id.json`, `ref.json`, `anchor-legacy.json`
and four more files), 6 Optional and 5 Upcoming. The 52 that run under a
released dialect are all ✅ here and can be read as a floor on what a
generator gives up by not resolving them.

## Formats

Left at the crate's default, which is the spec's: `format` is an assertion in
draft 4/6/7 and an annotation from 2019-09 on. The suite's format-validity
cases (tagged `format-assertion`, so Optional) allow 2020-12, and all but
three run as 2020-12 (one runs as draft 6, two are Upcoming). There an
invalid string passes. That is 26 of the 34 Optional misses, all 🙈: 22
groups across 19 `format-*.json` files (two each in email, hostname and
idn-hostname), three in `optional/format-regex.json`, and the leap-second
group in `optional/format-time.json`.

`should_validate_formats(true)` would assert those, but it fails 19
*Standard* groups: 19 of the `format-*.json` files have a case, "invalid X
string is only an annotation by default", marked valid. Standard
conformance wins, so the option is off.

Unknown formats are ignored, also the default. One wrinkle worth knowing:
`idn-hostname` and `idn-email` are backed by the crate's `idna` feature, so
turning default features off made them *unknown* formats — silently ignored,
which looked exactly like the validator being too loose in draft 7. `idna`
is therefore re-enabled by hand in `Cargo.toml` and in the `batch-deps.toml`
the generate step writes. If you ever change the feature set here, check
by hand for formats that quietly stopped being checked: nearly every
format-validity group runs as 2020-12, where `format` is only an
annotation, so the results won't show it.

## Bignums

The other 8 Optional misses are in `optional/bignum.json`, and they are the
batch crate's doing. It parses the schema and every instance into a
`serde_json::Value` without `arbitrary_precision`, so a number beyond `u64`
or with more digits than an `f64` holds is rounded before the validator
sees it. The exclusive bounds at 9.727837981879871e+26 (four groups, two
under draft 4) and distinct high-precision decimals in `uniqueItems` come
out equal, so they are 🚧; the `minimum`, `enum` and `const` comparisons
with a slightly different value come out equal too, so they are 🙈.

## Layout

| file | what |
|---|---|
| `generator.toml` | the three contract commands; `build` delegates to `rust-batch` |
| `Cargo.toml` | the pin (`=0.60.2`, `default-features = false` plus `idna`) |
| `build.rs` | lifts the pinned version out of `Cargo.lock` into `JSONSCHEMA_CRATE_VERSION` |
| `src/main.rs` | `version` and `generate`; also writes `batch-deps.toml` so the batch crate depends on the same pinned `jsonschema` |
| `src/generate.rs` | the dialect mapping, the module template, the raw-string sizing, and tests that check the pinned crate still behaves the way the template assumes |
| `src/retrieve.inc.rs` | the `jsonschema::Retrieve` impl the modules embed, serving the case's external schemas; compiled into the tests too so what it serves and refuses is checked |

Generation can only fail on an unrecognized dialect — embedding a schema
in a string always works — but it still runs on a 256 MiB-stack thread under `catch_unwind`
with a timeout and the shared SIGABRT crash guard, the same shape as the real
code generators, so a surprise is recorded as `generation_panic` instead of
taking the run down.
