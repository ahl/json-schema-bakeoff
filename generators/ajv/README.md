# ajv

[ajv](https://ajv.js.org/) 8.x, pinned exactly in `package.json` and read
back out of `node_modules/ajv/package.json` by the `version` command, so the
results directory names the version that produced them.

| package | pin | why |
|---|---|---|
| `ajv` | `8.20.0` | draft-06, draft-07, 2019-09 and 2020-12 |
| `ajv-draft-04` | `1.0.0` | the draft-04 dialect, which `ajv` itself dropped |

`ajv-formats` is deliberately **not** installed; see [Formats](#formats).

## What this row means

ajv is a validator, not a code generator. It is in the bakeoff as a
**calibration row for the Node path**, the way
[`jsonschema-0.58`](../jsonschema-0.58/README.md) is for the Rust one: it
runs through the same generate → build → run pipeline, the same runner
protocol, the same `cases.json`, so wherever the two validator rows agree
and the code generators don't, the difference is the generators; and
wherever this row disagrees with the Rust one, the cause is either ajv or
something about the Node half of the harness.

It is not a 100% row. It lands at **95.7%** of required groups across the
five drafts, and every group it misses is ajv's, not the harness's — most
of them are groups ajv's *own* CI skips. See [Known
deviations](#known-deviations); each one was reproduced against bare ajv,
outside this harness, before being written down.

| draft | required groups | ✅ | pass rate |
|---|---|---|---|
| draft4 | 160 | 156 | 97.5% |
| draft6 | 233 | 229 | 98.3% |
| draft7 | 258 | 254 | 98.4% |
| draft2019-09 | 373 | 361 | 96.8% |
| draft2020-12 | 384 | 348 | 90.6% |
| **all** | **1408** | **1348** | **95.7%** |

## Shape

The contract wants a module per schema group, so the "generated code" for a
group is a small CommonJS module that embeds that group's schema and
validates against it. `generate.mjs` writes `<out_dir>/<key>.js`: four
constants (where `ajv` is installed, where the remotes are, the draft, the
schema text) followed by `runtime.inc.js` embedded verbatim.
`runtime.inc.js` is the whole of the interesting behavior — it is a separate
file rather than a template string so it can be read, reviewed, and hashed
into the generator's source hash.

A module compiles its schema **lazily**, on the first case, and exports:

- `__conformanceCheck(instance)` — returns normally when the instance is
  valid, throws an `Error` carrying ajv's own `errorsText()` when it isn't.
- `__conformanceCompile()` — compile and nothing else, used to decide the
  group's status.

Absolute paths are baked into each module because it is written under
`target/` and read back from a batch directory elsewhere; neither can
resolve `ajv` or find `suite/remotes/` on its own. A generated module is
therefore runnable on its own, which is how two of the findings below were
checked.

### Where the status comes from

Unusually for this harness, the group's status is decided in the **generate**
step: that process has ajv in it, so it compiles each schema immediately
after writing the module. A schema ajv refuses (`MissingRefError`,
`"$dynamicRef" only supports hash fragment reference`, `enum must have
non-empty array`) is a `generation_error` ❌ and never reaches the runner. A
`RangeError` — ajv's reference resolution recursing until the stack goes —
is a `generation_panic` 🤯, because a blown stack is a crash wearing an
error's clothes.

The module keeps its lazy compile anyway, as a guard, and
`__conformanceCompile` rethrows the same error on every call so a schema ajv
rejected reads the same way on the first case and the hundredth.

Two things keep that arrangement from blaming ajv for the harness's
problems. Before judging any schema, generate compiles the schema `true`
through the identical code path as a **preflight**: a missing
`suite/remotes/` (an unchecked-out submodule, a wrong `BAKEOFF_ROOT`) or an
`ajv` that isn't installed would otherwise throw inside every group and be
recorded as a thousand ❌ against the tool. And an error carrying a
filesystem or module-resolution code (`ENOENT`, `EACCES`,
`MODULE_NOT_FOUND`, …) is never a verdict on a schema, so it exits non-zero
instead of becoming a bucket. A `SyntaxError` *does* stay an ❌: ajv builds
its validators with `new Function`, so that one is ajv's codegen failing on
this schema.

The `build` step compiles nothing — the modules are JavaScript and ajv
compiles the schema itself. It copies the modules into the batch directory,
**loads each one individually** so a module that won't load is attributed to
its own key instead of taking the batch down (the equivalent of attributing
a compiler diagnostic), and writes the runner. Nothing has ever failed to
load; the probe is there so that if something does, it is one `compile_fail`
🦀 and not a mystery.

### The runner

`[process.execPath, "<batch_dir>/runner.js"]` — `process.execPath`, never
`node` from `PATH`. It requires modules on first use rather than all at
once, so a restart after a dead case doesn't reload the whole batch.

A caught exception is a reject, except a `RangeError`, which is recorded as
a **panic** 💀: that is ajv's generated validator recursing until the stack
goes, and it is the same event that takes the process down (and so is
recorded as a panic) on the Rust rows. A module that fails to *load* is a
panic too, not a reject — loading isn't validating, so it says nothing about
the instance.

## Drafts

The `draft` field of each generate item — the suite directory the group came
from — picks the Ajv class, which *is* the dialect:

| suite directory | class | notes |
|---|---|---|
| `draft4` | `ajv-draft-04` | `id`, boolean `exclusiveMaximum`, no `const`/`contains` |
| `draft6` | `ajv` + draft-06 meta-schema | ajv ships no draft-06 dialect; see below |
| `draft7` | `ajv` | draft-07 is `ajv`'s default dialect |
| `draft2019-09` | `ajv/dist/2019` | |
| `draft2020-12` | `ajv/dist/2020` | |

Any other name is a `generation_error` naming the draft, rather than a
module that misbehaves quietly.

**draft6 is validated with the draft-07 dialect.** ajv has no draft-06
keyword set; draft-07 is the nearest thing it ships, and the difference
(`if`/`then`/`else`, `readOnly`, `contentEncoding`, `contentMediaType`) is
keywords the draft-06 suite never uses. The draft-06 *meta-schema* is added
by hand with `addMetaSchema` so that `definitions.json::validate definition
against metaschema` — a `$ref` to
`http://json-schema.org/draft-06/schema#` — resolves. ajv's own CI does
exactly this. draft6 comes out at 98.3%, failing only the four groups every
old draft fails here.

**The suite directory outranks the schema's own `$schema`.** That is the
policy, and in the vendored suite it costs nothing: no group in `draft4`,
`draft6` or `draft7` sets a root `$schema` at all, and every group in
`draft2019-09` and `draft2020-12` that does names either its own dialect
(441 and 452 groups) or one of the six custom meta-schemas under
`suite/remotes/`, which are registered (below). So nothing is stripped or
rewritten: no `$schema` is touched, and every number in the schema ajv
compiles is the harness's digits exactly (the serialization is an equivalent
one, not byte-identical — see [Numbers](#numbers)). If a future suite
revision adds a group whose root `$schema`
names a *different* draft, ajv will report it as an unresolvable reference
and it will show up as a ❌ naming the URI — visibly, rather than being
silently coerced.

## Remote references

The harness's standing policy is that the suite's
`http://localhost:1234/…` references are not resolved and a code generator
records whatever its tool does with them. Validator rows are the exception,
and resolving them here is the only way to measure what that policy costs
the other rows: `refRemote.json` is 15 of draft 2020-12's 384 required
groups, 11 of draft 7's, 8 of draft 4's, and all of them are ✅ here.

The implementation is deliberately boring. ajv does not fetch anything
unless it is handed a `loadSchema` callback, and it is not handed one — there
is no HTTP in this row to be reached by accident. Instead every file under
`<BAKEOFF_ROOT>/suite/remotes/` (79 of them, walked recursively, read from
disk) is registered up front under the URL the suite references it as:

```js
ajv.addSchema(schema, `http://localhost:1234/${relativePath}`, false, false)
```

- The absolute remotes directory is baked in at generate time, since the
  batch crate runs from a scratch directory of its own.
- The files are read **once per process** (cached on `globalThis`), not once
  per module: a draft is a thousand-odd modules in one node process and they
  all want the same 79 files. Each module still gets its own `Ajv`, because
  suite group schemas routinely declare `$id`s of their own and sharing an
  instance would make them collide.
- Anything that is not one of those 79 files is simply a missing reference.
  There is no path to walk out of the directory because no path arrives from
  outside it: the URL is derived *from* the file, not the other way round.

Two wrinkles, both handled:

- **Duplicate identifiers.** `addSchema(schema, key)` registers a schema
  under `key` *and* under its own `$id`, and ajv throws on an identifier
  registered twice. 46 of the 79 remotes declare no identifier at all; 26
  declare one equal to their own URL; 3 declare a `urn:uuid:` one; and 4
  declare one naming a *different* file than their own
  (`different-id-ref-string.json` says `real-id-ref-string.json` in three
  drafts, `draft7/ignore-dependentRequired.json` says `draft7/integer.json`).
  So it could collide, and doesn't: all 33 identifiers are distinct, and
  none of the 4 odd ones names a file that exists. All 79 register cleanly
  on all five classes, which is checked rather than assumed — the generate
  step compiles every schema, so a registration failure would turn a whole
  draft into errors on the first run. (draft 4's remotes would use `id`
  rather than `$id`, which is the key `ajv-draft-04` reads; none of them
  declares one either way.)
- **Meta-schemas.** Six remotes declare `$vocabulary` —
  `draft2019-09/` and `draft2020-12/`'s `metaschema-no-validation.json` and
  `metaschema-optional-vocabulary.json`, plus
  `draft2020-12/format-assertion-{true,false}.json` — and the suite points
  a group's `$schema` at each of them. A schema used as a `$schema` has to
  go in through `addMetaSchema`, so `$vocabulary` is the rule: declare it
  and you are registered as a meta-schema, otherwise as an ordinary schema.

Both calls pass `_validateSchema = false`. The remotes are fixtures, not the
subject, and every remote is registered on every instance — so without it a
2020-12 remote would be checked against draft 4's meta-schema and rejected
for saying `$defs`. It is also ~79× less meta-schema validation per module.

The draft-specific meta-schemas the suite `$ref`s directly —
`http://json-schema.org/draft-0{4,6,7}/schema#`,
`https://json-schema.org/draft/20{19-09,20-12}/schema`, used by
`definitions.json::validate definition against metaschema` (`defs.json`
from 2019-09 on) and `ref.json::remote ref, containing refs itself` — come
from the Ajv class itself, which bundles its own. Nothing special is done
for them, and all ten of those groups are ✅.

## Formats

**`format` is annotation-only in this row, in every draft.** That is ajv's
default, not a setting: the `ajv` package ships **no format implementations
at all**. Every format in the suite is therefore an *unknown* format, and
with `strict: false` an unknown format is ignored. `validateFormats` is left
at its default (`true`) and is moot — there is nothing installed for it to
validate with. The `ajv-formats` package, which is where ajv's `date-time`,
`email`, `ipv4` and the rest live, is deliberately not a dependency here.

The effect is a wall of 🙈 across `optional/format/`: 6 groups of 7 in
draft 4, 9 of 10 in draft 6, 22 of 26 in draft 7, and 24 of 28 in each of
2019-09 and 2020-12. The only ones left standing are `unknown.json`, where
ignoring the format is the right answer, and the three `pattern` groups in
`ecmascript-regex.json`, which aren't about `format` at all. That is the
honest reading of the default install. It is *not*
the same shape as the Rust row: `jsonschema` has formats compiled in and
treats them as the spec does (an assertion in draft 4/6/7, an annotation
from 2019-09 on), so that row is 100% on `optional/format` for the old
drafts and 🙈 for the new ones. Here it is 🙈 everywhere. The required
`format.json` groups pass in every draft, because the suite only asserts
there that `format` never *rejects*.

Installing `ajv-formats` would move most of those to ✅ (ajv's own CI
installs it, and then skips `idn-email`, `idn-hostname`, `iri` and
`iri-reference`, which it does not implement). It is left out because the row
reports the library as it comes.

Three `optional/format/ecmascript-regex.json` groups are 🙈 for the same
reason — they test `format: "regex"` — and the other three in that file,
which test `pattern`, pass.

## Options

Set on every instance, in `runtime.inc.js`:

| option | value | why |
|---|---|---|
| `strict` | `false` | The suite is full of keywords, `$id` shapes and unknown formats that ajv's strict mode rejects outright. Strict mode is a linting layer over the spec; this row measures the spec. Required, not cosmetic: with it on, unknown formats *throw* at compile time. |
| `allErrors` | `false` | The default. The harness stores one error per case. |
| `logger` | `false` | stdout is the harness protocol and ajv's default logger is `console`. |

Everything else is ajv's default — in particular `ownProperties` and
`ignoreKeywordsWithRef`, both of which would move groups to ✅ and both of
which are left alone. See below.

## Known deviations

Every group below was reproduced against bare ajv, with and without the
remotes registered, outside this harness. None of them is a harness
artifact. Most of them appear by name in ajv's own
[`spec/json-schema.spec.ts`](https://github.com/ajv-validator/ajv/blob/master/spec/json-schema.spec.ts)
skip list, which is cited where it applies.

### In all five drafts (4 groups each in draft 4/6/7)

**`properties.json::properties whose names are Javascript object property
names`** 🚧 and **`required.json::required properties whose names are
Javascript object property names`** 🙈 — ajv's `ownProperties` option
defaults to `false`, which means it reaches properties on the prototype
chain: `{}` looks like it has a `constructor` and a `toString`. So
`required: ["toString"]` passes on `{}` (too loose) and
`properties: {constructor: {type: "number"}}` fails on `{}` (too strict).
The suite added these groups specifically to catch this, with the comment
"Ensure JS implementations don't universally consider e.g. `__proto__` to
always be present in an object". `ownProperties: true` fixes `required.json`
and half-fixes `properties.json` (it then fails the `__proto__ not valid`
case instead); it is left at the default because it is the default, and
because it isn't a clean fix. These do not appear in ajv's skip list — ajv's
vendored copy of the suite predates them.

### draft 4, 6 and 7 only

**`ref.json::ref overrides any sibling keywords`** 🚧 and **`ref.json::$ref
prevents a sibling $id from changing the base uri`** 🤪 — ajv v7+ applies
`$ref`'s siblings in every draft, which is 2019-09 semantics; draft-07 and
earlier say siblings are ignored. ajv's migration guide states it plainly:
"`$ref` siblings are validated by default (consistent with draft 2019-09),
`ignoreKeywordsWithRef` (deprecated) can be used instead to ignore `$ref`
siblings." ajv's own CI sets `ignoreKeywordsWithRef: true` for drafts 6 and
7 *and still skips* the `$id` group, which is exactly what reproduces here:
the deprecated option fixes the first group and not the second.

### draft 2019-09 (12 groups)

- `enum.json::empty enum` ❌ — ajv's `enum` keyword throws `enum must have
  non-empty array` at compile time, unconditionally. `{"enum": []}` is a
  schema that accepts nothing; ajv won't compile it.
- `recursiveRef.json::$recursiveRef with no $recursiveAnchor in the initial
  target schema resource` 🤪 — in ajv's skip list.
- `ref.json::refs with relative uris and defs`,
  `ref.json::relative refs with absolute uris and defs` 🤯 — in ajv's skip
  list. Here they are worse than a wrong answer: resolution recurses until
  the stack goes, so they are generation panics.
- `ref.json::URN ref with nested pointer ref` 🤯 — the same stack overflow,
  not in ajv's skip list (newer suite group).
- `unevaluatedItems.json::unevaluatedItems with nested items` 🤪,
  `unevaluated{Items,Properties}.json::… can see annotations from if without
  then and else` 🚧,
  `unevaluatedProperties.json::unevaluatedProperties with if/then/else, then
  not defined` 🤪 — ajv's annotation tracking for `unevaluated*`. The last
  is in ajv's skip list; the others are newer suite groups in the same
  family.
- `vocabulary.json::schema that uses custom metaschema with with no
  validation vocabulary` 🚧 — ajv treats `$vocabulary` as a no-op: it
  registers the custom meta-schema and then validates with its full keyword
  set anyway, so `minimum` still bites where the vocabulary excluded it.
  (Checked both ways — `addMetaSchema` and `addSchema` behave identically,
  so this is not the registration choice above.) The sibling group, `ignore
  unrecognized optional vocabulary`, passes.
- `properties.json`/`required.json` as above.

### draft 2020-12 (36 groups)

Everything in the 2019-09 list recurs except `recursiveRef.json`, which
2020-12 doesn't have (the keyword became `$dynamicRef`), plus:

- **`dynamicRef.json`: 19 of 21 groups.** This is the bulk of the gap and it
  is where ajv is weakest. Eight groups are 🚧 on the most basic case ("An
  array of strings is valid") — every one of them is in ajv's skip list,
  case for case. Three are ❌ with `"$dynamicRef" only supports hash
  fragment reference`, ajv's own message for a `$dynamicRef` that isn't a
  bare `#name`. The rest (`multiple dynamic paths`, `points to a boolean
  schema`, `skips over intermediate resources`, `avoids the root of each
  schema`, `$ref and $dynamicAnchor are independent of order` ×2, `$ref to
  $dynamicRef finds detached $dynamicAnchor`, `without anchor in fragment`)
  are newer suite groups in the same family; two of them (`$ref to
  $dynamicRef finds detached $dynamicAnchor`, `avoids the root of each
  schema`) are 💀 — the validator compiles and then recurses until the
  stack goes, on every case in the group. `recursiveRef.json`'s 2019-09
  equivalents mostly pass, so this is specifically `$dynamicAnchor` scope
  resolution.
- **`unevaluatedItems.json`: 7 groups**, including the three
  `contains`-interaction groups that are in ajv's skip list verbatim, plus
  `with minContains = 0` 🚧, `can see annotations from if without then and
  else` 🚧, `with nested items` 🤪 and `with $dynamicRef` 💀.
- `unevaluatedProperties.json`: 3 groups — `with if/then/else, then not
  defined` 🤪 (in ajv's skip list), `can see annotations from if without
  then and else` 🚧, and `with $dynamicRef` 💀.

The four 💀 groups are all `$dynamicRef`: a runaway recursion in the
compiled validator, caught as a `RangeError` and recorded as a panic rather
than a reject (see [The runner](#the-runner)).

### Optional tests

Reported separately and never in headline numbers. Besides `optional/format`
(above), the shape is:

- `optional/content.json` 🙈 (3 groups, draft 7 only) — ajv does not validate `contentEncoding` or
  `contentMediaType`. In ajv's skip list, and correct per the spec, which
  makes them annotations.
- `optional/float-overflow.json` 🚧 (all five drafts) — `multipleOf` on
  `1e308` overflows to `Infinity` in the divide, so ajv rejects a value the
  suite calls valid "if overflow is handled". In ajv's skip list.
- `optional/zeroTerminatedFloats.json` 🙈 (draft 4 only) — draft 4 says
  `1.0` is not an `integer`. JavaScript has one number type and `1.0` *is*
  `1`, so there is nothing for ajv to look at; the group's own title says
  "some languages do not distinguish between different types of numeric
  value". This is the one deviation here that belongs to the language rather
  than to ajv, and the Rust validator row passes it.
- `optional/dynamicRef.json` 🚧 (2020-12) — the same `$dynamicRef` gap as
  the required file.
- `optional/unknownKeyword.json::$id inside an unknown keyword is not a real
  identifier` ❌ — ajv collects `$id`s from everywhere, including inside
  keywords it doesn't know, and then reports the two identical ones as
  "resolves to more than one schema". The group's name is the test. In ajv's
  skip list.
- `optional/cross-draft.json` 🙈/🚧 — one `Ajv` class is one dialect, so a
  draft-07 instance reading a 2019-09 resource reads it as draft-07. A
  per-resource dialect switch is not something ajv offers.
- `optional/format-assertion.json` ❌ (2020-12) — the custom meta-schemas
  `$ref` `https://json-schema.org/draft/2020-12/meta/format-assertion`,
  which the `ajv` package doesn't bundle. In ajv's skip list.
- `optional/bignum.json` is **✅, 7 of 7, in every draft.** Worth stating
  plainly, because it is the one place the Node path could have gone wrong:
  the harness carries the suite's digits exactly (serde_json's
  `arbitrary_precision`), the runner parses them through `JSON.parse`'s
  source-text reviver, and then hands ajv a plain JavaScript value — which
  is all ajv validates. A 50-digit integer is a `float64` by the time any
  keyword sees it. The suite's bignum groups survive that: a rounded bignum
  is still `Number.isInteger`, and the comparison groups are built so the
  answer doesn't change when both sides round the same way. So this passes
  not because nothing was lost but because the suite's bignum tests are
  designed not to depend on what JavaScript loses. A suite case that did
  (`maximum: 2^64-1` against `2^64`, say) would be 🙈 here, and that would
  be JavaScript's answer, not the harness's.

## Numbers

The harness sends the suite's numbers as their original text. Two places
keep them that way:

- **Schemas** are embedded in the module as *text* and `JSON.parse`d at
  compile time. Letting the parser reach the numbers and re-serializing
  would rewrite the schema — `multipleOf: 1e-8` survives, but an integer
  past 2^53 comes back as a different literal, which is a different schema.
  So `generate.mjs` parses stdin with a reviver that keeps every number as
  its source text and writes it back out unchanged (`rawStringify`).

  What lands in the module is an *equivalent* serialization, not a
  byte-identical one: `rawStringify` reproduces every number exactly
  (checked for `1e308`, `-0`, `1E+2`, `0.1e-7` and 50-digit integers) but
  object keys come back in `Object.keys` order, which moves integer-like
  keys to the front, and `JSON.stringify` re-escapes strings its own way
  (`😀` becomes the raw emoji, `\/` becomes `/`). None of that
  changes what the schema means, and the harness has already normalized the
  JSON before it sends it.
- **Cases** are parsed in the runner with the same source-text reviver,
  converting each number with `Number(context.source)`. That lands on the
  same `float64` the plain parser would — this is JavaScript's own JSON
  semantics, and `Number(source)` is defined to agree with them — but it
  puts the lossy step in one visible place.

Both commands check at startup that this Node actually has the reviver's
`context.source` (Node 21+) and refuse to run if it doesn't, rather than
falling back to the plain parser and quietly rewriting every schema.

## No timeout

Every other generator in the bakeoff bounds its tool with a timeout, and
this one can't: ajv compiles synchronously, and synchronous JavaScript is
not interruptible from inside its own process. A schema that made ajv loop
forever would hang the draft with no output. Nothing in the suite does —
ajv's one runaway, the reference resolution in `ref.json::refs with relative
uris and defs`, exhausts the stack and surfaces as a catchable `RangeError`
— so this is a missing guard rather than a live problem. Putting the
probe-compile in a short-lived worker thread would fix it, at the cost of
shipping every schema across a channel.

## Layout

| file | what |
|---|---|
| `generator.toml` | the three contract commands, the description, `language = "js"`, `kind = "validator"` |
| `package.json`, `package-lock.json` | the exact pins |
| `deps.mjs` | `npm ci` on first use, the stdout muzzle, `writeAll`; copied from `generators/quicktype/deps.mjs` |
| `generate.mjs` | `version`; the module template; number-preserving parse/serialize; compile-for-status |
| `runtime.inc.js` | embedded verbatim in every module: the draft→class map, the remotes, the options, `__conformanceCheck` |
| `build.mjs` | copy, probe-load each module, write `runner.js`, emit outcomes |

`node_modules/` is not committed; `deps.mjs` runs `npm ci` the first time
either command needs it.

## Reproducing a single group

```sh
cargo run -- run --generator ajv --draft draft2020-12 --filter 'dynamicRef.json::points to a boolean'
```

prints the per-case detail and the path to the generated module under
`target/bakeoff/ajv-<version>/generated/`. That module is a plain CommonJS
file with absolute paths in it, so it can be driven directly:

```sh
node -e 'require("<path>.js").__conformanceCheck({"foo": 42})'
```

which is how the deviations above were separated from the glue.
