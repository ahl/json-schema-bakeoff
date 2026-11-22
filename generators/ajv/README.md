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
runs through the same generate -> build -> run pipeline, the same runner
protocol, the same `cases.json`, so wherever the two validator rows agree
and the code generators don't, the difference is the generators; and
wherever this row disagrees with the Rust one, the cause is either ajv or
something about the Node half of the harness.

It is not a 100% row. It passes **498 of 540** Standard groups (92.2%),
and every group it misses is ajv's, not the harness's -- most of them are
groups ajv's *own* CI skips. See [Known deviations](#known-deviations); each
one was reproduced against bare ajv, outside this harness, before being
written down.

| tier | groups | ✅ | 🙈 | 🚧 | 🤪 | 💀 | ❌ | 🤯 | pass rate |
|---|---|---|---|---|---|---|---|---|---|
| Standard | 540 | 498 | 6 | 20 | 5 | 4 | 4 | 3 | 92.2% |
| Optional | 98 | 47 | 37 | 10 | 0 | 0 | 4 | 0 | 48.0% |
| Compound | 5 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 100% |
| Upcoming | 25 | 0 | 0 | 0 | 0 | 0 | 25 | 0 | 0% |

Against suite 1.041, which ran every group once per draft, this row passed
1353 of 1413 non-optional groups
([archive](../../archive/suite-1.041/results/ajv-8.20.0/README.md)).
Every Standard miss here is a group that missed there too; the lower rate is
the denominator losing its easy draft 4/6/7 duplicates while the hard
2020-12 groups stay.

The 25 Upcoming groups are ❌ because they run under dialect `v1`, and ajv
has no class for it.

## Shape

The contract wants a module per schema group, so the "generated code" for a
group is a small CommonJS module that embeds that group's schema and
validates against it. `generate.mjs` writes `<out_dir>/<key>.js`: four
constants (where `ajv` is installed, the dialect, the schema text, the
case's external schemas as text) followed by `runtime.inc.js` embedded
verbatim.
`runtime.inc.js` is the whole of the interesting behavior -- it is a separate
file rather than a template string so it can be read, reviewed, and hashed
into the generator's source hash.

A module compiles its schema **lazily**, on the first case, and exports:

- `__conformanceCheck(instance)` -- returns normally when the instance is
  valid, throws an `Error` carrying ajv's own `errorsText()` when it isn't.
- `__conformanceCompile()` -- compile and nothing else, used to decide the
  group's status.

The absolute path to `node_modules` is baked into each module because it is
written under `target/` and read back from a batch directory elsewhere;
neither can resolve `ajv` on its own. A generated module is therefore
runnable on its own, which is how two of the findings below were checked.

### Where the status comes from

Unusually for this harness, the group's status is decided in the **generate**
step: that process has ajv in it, so it compiles each schema immediately
after writing the module. A schema ajv refuses (`MissingRefError`,
`"$dynamicRef" only supports hash fragment reference`, `enum must have
non-empty array`) is a `generation_error` ❌ and never reaches the runner. A
`RangeError` -- ajv's reference resolution recursing until the stack goes --
is a `generation_panic` 🤯, because a blown stack is a crash wearing an
error's clothes.

The module keeps its lazy compile anyway, as a guard, and
`__conformanceCompile` rethrows the same error on every call so a schema ajv
rejected reads the same way on the first case and the hundredth.

Two things keep that arrangement from blaming ajv for the harness's
problems. Before judging any schema, generate compiles the schema `true`
under `draft2020-12` through the identical code path as a **preflight**: an
`ajv` that isn't installed would otherwise throw inside every group and be
recorded as a thousand ❌ against the tool. The preflight's dialect is
fixed rather than taken from the first item, because one generate call
mixes dialects and an item in a dialect ajv lacks (`v1`) must be that
item's ❌, not a dead process. And an error carrying a filesystem or
module-resolution code (`ENOENT`, `EACCES`,
`MODULE_NOT_FOUND`, ...) is never a verdict on a schema, so it exits non-zero
instead of becoming a bucket. A `SyntaxError` *does* stay an ❌: ajv builds
its validators with `new Function`, so that one is ajv's codegen failing on
this schema.

The `build` step compiles nothing -- the modules are JavaScript and ajv
compiles the schema itself. It copies the modules into the batch directory,
**loads each one individually** so a module that won't load is attributed to
its own key instead of taking the batch down (the equivalent of attributing
a compiler diagnostic), and writes the runner. Nothing has ever failed to
load; the probe is there so that if something does, it is one `compile_fail`
🦀 and not a mystery.

### The runner

`[process.execPath, "<batch_dir>/runner.js"]` -- `process.execPath`, never
`node` from `PATH`. It requires modules on first use rather than all at
once, so a restart after a dead case doesn't reload the whole batch.

A caught exception is a reject, except a `RangeError`, which is recorded as
a **panic** 💀: that is ajv's generated validator recursing until the stack
goes, and it is the same event that takes the process down (and so is
recorded as a panic) on the Rust rows. A module that fails to *load* is a
panic too, not a reject -- loading isn't validating, so it says nothing about
the instance.

## Dialects

The `draft` field of each generate item, the dialect the harness runs the
case under (the newest one its `compatibility` allows), picks the Ajv
class, which *is* the dialect:

| dialect | class | notes |
|---|---|---|
| `draft4` | `ajv-draft-04` | `id`, boolean `exclusiveMaximum`, no `const`/`contains` |
| `draft6` | `ajv` + draft-06 meta-schema | ajv ships no draft-06 dialect; see below |
| `draft7` | `ajv` | draft-07 is `ajv`'s default dialect |
| `draft2019-09` | `ajv/dist/2019` | |
| `draft2020-12` | `ajv/dist/2020` | |

Any other name is a `generation_error` reading `unsupported dialect <name>`,
rather than a module that misbehaves quietly. That includes `v1`, the
dialect of the Upcoming tier, which ajv has no class for.

**draft6 is validated with the draft-07 dialect.** ajv has no draft-06
keyword set; draft-07 is the nearest thing it ships, and the difference
(`if`/`then`/`else`, `readOnly`, `contentEncoding`, `contentMediaType`) is
keywords a draft-06 schema has no business using. The draft-06
*meta-schema* is added by hand with `addMetaSchema` so that a `$ref` to
`http://json-schema.org/draft-06/schema#` resolves; ajv's own CI does
exactly this. It hardly matters: a case that allows draft 6 almost always
allows draft 7 too, so one Optional group runs as `draft6` and no Standard
group does. Of the 540 Standard groups, 409 run as `draft2020-12`, 50 as
`draft2019-09`, 48 as `draft7` and 33 as `draft4`.

**The harness's dialect outranks the schema's own `$schema`.** That is the
policy, and in the vendored suite it costs nothing: the suite's schemas carry no
root `$schema` except six, and those six name a custom meta-schema that the case
supplies in its `externalSchemas`, which is registered (below). So nothing is
stripped or rewritten: no `$schema` is touched, and every number in the schema
ajv compiles is the harness's digits exactly (the serialization is an equivalent
one, not byte-identical -- see [Numbers](#numbers)). If a future suite revision
adds a case whose root `$schema` names a *different* draft, ajv will report it
as an unresolvable reference and it will show up as a ❌ naming the URI --
visibly, rather than being silently coerced.

## Remote references

The harness's standing policy is that the suite's
`http://localhost:1234/...` references are not resolved and a code generator
records whatever its tool does with them. Validator rows are the exception,
and resolving them here is the only way to measure what that policy costs
the other rows. A case that references other schemas carries them in its
`externalSchemas`, an object from retrieval URI to schema, and the harness
passes that object along with the item. In the suite at e2a68a1 that is 57
cases: 46 Standard, 6 Optional, 5 Upcoming.

The implementation is deliberately boring. ajv does not fetch anything
unless it is handed a `loadSchema` callback, and it is not handed one;
there is no HTTP in this row to be reached by accident. The module embeds
the case's external schemas as text, and `compile` registers each one on
the module's own `Ajv`, under the URI the case gives it, before compiling
the schema:

```js
ajv.addSchema(schema, url, false, false)
```

- Only this case's schemas are registered. Anything else is a missing
  reference, including a suite remote that some other case supplies. There
  is no path to walk out of anywhere, because nothing is read from disk.
- Each module gets its own `Ajv`, because suite schemas routinely declare
  `$id`s of their own and sharing an instance would make them collide.
- **Duplicate identifiers.** `addSchema(schema, key)` registers a schema
  under `key` *and* under its own `$id`, and ajv throws on an identifier
  registered twice. Some external schemas declare an `$id` that differs
  from their key (`different-id-ref-string.json` says
  `real-id-ref-string.json`; `urn-ref-string.json` says a `urn:uuid:`),
  and none collides within a case. A collision would show up as a ❌ on
  the first run, since the generate step compiles every schema.
- **Meta-schemas.** An external schema that declares `$vocabulary` is a
  meta-schema, and the suite points a case's `$schema` at each such one it
  supplies (`vocabulary.json`, `optional/format-assertion.json`). A schema
  used as a `$schema` has to go in through `addMetaSchema`, so
  `$vocabulary` is the rule: declare it and you are registered as a
  meta-schema, otherwise as an ordinary schema.

Both calls pass `_validateSchema = false`. The external schemas are
fixtures, not the subject, and one may be written in another dialect than
the case runs under (`optional/cross-draft.json` does this on purpose), so
it would otherwise be checked against the wrong meta-schema and rejected.

The dialect meta-schemas the suite reaches -- the 2019-09 and 2020-12
`meta/*` vocabularies that `vocabulary.json`'s custom meta-schemas `$ref`,
and the draft-07, 2019-09 and 2020-12 schemas in `optional/cross-draft.json`
-- come from the Ajv class itself, which bundles its own. Nothing special is
done for them.

## Formats

**`format` is annotation-only in this row, in every dialect.** That is ajv's
default, not a setting: the `ajv` package ships **no format implementations
at all**. Every format in the suite is therefore an *unknown* format, and
with `strict: false` an unknown format is ignored. `validateFormats` is left
at its default (`true`) and is moot -- there is nothing installed for it to
validate with. The `ajv-formats` package, which is where ajv's `date-time`,
`email`, `ipv4` and the rest live, is deliberately not a dependency here.

The format cases come in two kinds. The ones tagged `format-annotation`
(39 groups in the root `format-*.json` files) are Standard, and they pass,
because they only check that `format` never *rejects*. The ones tagged
`format-assertion` are Optional, and they are a wall of 🙈: 22 of 23 in the
root files, plus `optional/format-regex.json` (3 of 6) and
`optional/format-time.json` (1 of 1). Every group left standing is one whose
cases are all valid, like `format-hostname.json::validation of host names
without IDNA rules (draft6)`. That is the honest reading of the default
install. `optional/ecmascript-regex.json` tests `pattern`, not `format`, and
passes 20 of 20.

Installing `ajv-formats` would move most of those to ✅ (ajv's own CI
installs it, and then skips `idn-email`, `idn-hostname`, `iri` and
`iri-reference`, which it does not implement). It is left out because the row
reports the library as it comes.

## Options

Set on every instance, in `runtime.inc.js`:

| option | value | why |
|---|---|---|
| `strict` | `false` | The suite is full of keywords, `$id` shapes and unknown formats that ajv's strict mode rejects outright. Strict mode is a linting layer over the spec; this row measures the spec. Required, not cosmetic: with it on, unknown formats *throw* at compile time. |
| `allErrors` | `false` | The default. The harness stores one error per case. |
| `logger` | `false` | stdout is the harness protocol and ajv's default logger is `console`. |

Everything else is ajv's default -- in particular `ownProperties` and
`ignoreKeywordsWithRef`, both of which would move groups to ✅ and both of
which are left alone. See below.

## Known deviations

Every group below was reproduced against bare ajv, with and without the
remotes registered, outside this harness. None of them is a harness
artifact. Most of them appear by name in ajv's own
[`spec/json-schema.spec.ts`](https://github.com/ajv-validator/ajv/blob/master/spec/json-schema.spec.ts)
skip list, which is cited where it applies. The 42 Standard misses fall into
seven families.

### The prototype chain (2 groups)

**`properties.json::properties whose names are Javascript object property
names (2020-12)`** 🚧 and **`required.json::required properties whose names
are Javascript object property names (2020-12)`** 🙈 -- ajv's
`ownProperties` option defaults to `false`, which means it reaches
properties on the prototype chain: `{}` looks like it has a `constructor`
and a `toString`. So `required: ["toString"]` passes on `{}` (too loose) and
`properties: {constructor: {type: "number"}}` fails on `{}` (too strict).
The suite added these groups specifically to catch this, with the comment
"Ensure JS implementations don't universally consider e.g. `__proto__` to
always be present in an object". `ownProperties: true` fixes `required.json`
and half-fixes `properties.json` (it then fails the `__proto__ not valid`
case instead); it is left at the default because it is the default, and
because it isn't a clean fix. These do not appear in ajv's skip list --
ajv's vendored copy of the suite predates them.

### `$ref` siblings in draft 7 and earlier (3 groups)

**`ref-legacy.json::ref overrides any sibling keywords (draft7)`** 🚧,
**`id-legacy.json::$ref prevents a sibling $id from changing the base uri
(draft7)`** 🤪 and **`id-legacy.json::$ref prevents a sibling id from
changing the base uri (draft4)`** 🤪 -- ajv v7+ applies `$ref`'s siblings in
every draft, which is 2019-09 semantics; draft-07 and earlier say siblings
are ignored. ajv's migration guide states it plainly: "`$ref` siblings are
validated by default (consistent with draft 2019-09),
`ignoreKeywordsWithRef` (deprecated) can be used instead to ignore `$ref`
siblings." ajv's own CI sets `ignoreKeywordsWithRef: true` for drafts 6 and
7 *and still skips* the `$id` group, which is exactly what reproduces here:
the deprecated option fixes the first group and not the second.

### Refusals and runaways at compile time (4 groups)

- `enum.json::empty enum (2020-12)` ❌ -- ajv's `enum` keyword throws `enum
  must have non-empty array` at compile time, unconditionally. `{"enum":
  []}` is a schema that accepts nothing; ajv won't compile it.
- `id.json::refs with relative uris and defs (2020-12)` and
  `id.json::relative refs with absolute uris and defs (2020-12)` 🤯 -- in
  ajv's skip list. Here they are worse than a wrong answer: resolution
  recurses until the stack goes, so they are generation panics.
- `id.json::URN ref with nested pointer ref (2020-12)` 🤯 -- the same stack
  overflow, not in ajv's skip list (newer suite group).

### `$recursiveRef` (1 group)

`recursiveRef.json::$recursiveRef with no $recursiveAnchor in the initial
target schema resource (2019-09)` 🤪 -- in ajv's skip list. The other nine
`recursiveRef.json` groups pass.

### `$dynamicRef` (19 groups)

**`dynamicRef.json` misses 19 of 21.** This is the bulk of the gap and it is
where ajv is weakest: 12 🚧, 3 ❌, 2 🙈, 2 💀.

- The 🚧 groups fail on the most basic case ("An array of strings is
  valid", or its equivalent). Eight of them are in ajv's skip list, case for
  case; the rest (`$ref and $dynamicAnchor are independent of order` x2,
  `$dynamicRef skips over intermediate resources - direct reference`,
  `without anchor in fragment behaves identical to $ref`) are newer suite
  groups in the same family.
- The ❌ groups are `"$dynamicRef" only supports hash fragment reference`,
  ajv's own message for a `$dynamicRef` that isn't a bare `#name`.
- The 🙈 groups are `multiple dynamic paths to the $dynamicRef keyword
  (2020-12)` and `$dynamicRef points to a boolean schema (2020-12)`.
- The 💀 groups are `$ref to $dynamicRef finds detached $dynamicAnchor
  (2020-12)` and `$dynamicRef avoids the root of each schema, but scopes are
  still registered (2020-12)`: the validator compiles and then recurses
  until the stack goes, on every case in the group.

The two that pass are `strict-tree schema, guards against misspelled
properties` and `A $ref to a $dynamicAnchor in the same schema resource
behaves like a normal $ref to an $anchor`. `recursiveRef.json` mostly
passes, so this is specifically `$dynamicAnchor` scope resolution.

### `unevaluated*` annotations (11 groups)

- **`unevaluatedItems.json`: 8 of 47**, including the three
  `contains`-interaction groups that are in ajv's skip list verbatim (🙈),
  plus `with minContains = 0` 🚧, `can see annotations from if without then
  and else` 🚧 (twice: once as 2019-09, once as the 2020-12 `prefixItems`
  version), `with nested items` 🤪 and `with $dynamicRef` 💀.
- **`unevaluatedProperties.json`: 3 of 45** -- `with if/then/else, then not
  defined` 🤪 (in ajv's skip list), `can see annotations from if without
  then and else` 🚧, and `with $dynamicRef` 💀.

All of these are 2020-12 except the one 2019-09 group named above. The
three 💀 `with $dynamicRef` groups are the `$dynamicRef` gap showing up in
another file: a runaway recursion in the compiled validator, caught as a
`RangeError` and recorded as a panic rather than a reject (see [The
runner](#the-runner)).

### `$vocabulary` (2 groups)

`vocabulary.json::schema that uses custom metaschema with no validation
vocabulary (2020-12)` and its `(recursive anchor)` sibling (2019-09) 🚧 --
ajv treats `$vocabulary` as a no-op: it registers the custom meta-schema and
then validates with its full keyword set anyway, so `minimum` still bites
where the vocabulary excluded it. (Checked both ways -- `addMetaSchema` and
`addSchema` behave identically, so this is not the registration choice
above.) The other two groups in the file pass.

### Optional tests

Reported separately and never in headline numbers. 47 of 98 pass. The 51
misses, by file:

- **Formats, 26 groups** 🙈 -- see [Formats](#formats).
- `optional/bignum.json` (11 of 14) -- see below.
- `optional/content.json` 🙈 (3 of 3, draft 7) -- ajv does not validate
  `contentEncoding` or `contentMediaType`. In ajv's skip list, and correct
  per the spec, which makes them annotations.
- `optional/cross-draft.json` (4 of 4: 2 🙈, 2 🚧) -- one `Ajv` class is
  one dialect, so a draft-07 instance reading a 2019-09 resource reads it as
  draft-07. A per-resource dialect switch is not something ajv offers.
- `optional/format-assertion.json` ❌ (2 of 2) -- the custom meta-schemas
  `$ref` `https://json-schema.org/draft/2020-12/meta/format-assertion`,
  which the `ajv` package doesn't bundle. In ajv's skip list.
- `optional/non-schemas.json::$id inside an unknown keyword is not a real
  identifier`, as draft7 and as its 2020-12 `($defs)` version ❌ -- ajv
  collects `$id`s from everywhere, including inside keywords it doesn't
  know, and then reports the two identical ones as "resolves to more than
  one schema". The group's name is the test. In ajv's skip list. The other
  11 groups in the file pass.
- `optional/dynamicRef.json` 🚧 (1 of 1) -- the same `$dynamicRef` gap as
  the Standard file.
- `optional/float-overflow.json` 🚧 (1 of 1) -- `multipleOf` on `1e308`
  overflows to `Infinity` in the divide, so ajv rejects a value the suite
  calls valid "if overflow is handled". In ajv's skip list.
- `optional/zero-terminated-floats.json` 🙈 (draft 4) -- draft 4 says `1.0`
  is not an `integer`. JavaScript has one number type and `1.0` *is* `1`, so
  there is nothing for ajv to look at; the group's own title says "some
  languages do not distinguish between different types of numeric value".
  This is a deviation that belongs to the language rather than to ajv, and
  the Rust validator row passes it.

`optional/bignum.json` is where the Node path's one lossy step shows. The
harness carries the suite's digits exactly (serde_json's
`arbitrary_precision`), the runner parses them through `JSON.parse`'s
source-text reviver, and then hands ajv a plain JavaScript value -- which is
all ajv validates. A 50-digit integer is a `float64` by the time any keyword
sees it. Three groups (`integer`, `number`, `string`) survive that. The
other 11 compare values that differ only past `float64` precision:
`maximum` 2^64-1 against 2^64, an `enum` or `const` decimal against one a
hair away, `uniqueItems` over decimals that round to the same double. Where
both sides round to the same number ajv accepts what it shouldn't (5 🙈);
where an exclusive bound or a uniqueness check meets two equal doubles it
rejects what it shouldn't (6 🚧, counting the `multipleOf: 0.5` overflow
group). That is JavaScript's answer, not the harness's.

## Numbers

The harness sends the suite's numbers as their original text. Two places
keep them that way:

- **Schemas** are embedded in the module as *text* and `JSON.parse`d at
  compile time, and so are the case's external schemas. Letting the parser reach
  the numbers and re-serializing would rewrite the schema -- `multipleOf: 1e-8`
  survives, but an integer past 2^53 comes back as a different literal, which is
  a different schema. So `generate.mjs` parses stdin with a reviver that keeps
  every number as its source text and writes it back out unchanged
  (`rawStringify`).

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
  same `float64` the plain parser would -- this is JavaScript's own JSON
  semantics, and `Number(source)` is defined to agree with them -- but it
  puts the lossy step in one visible place.

Both commands check at startup that this Node actually has the reviver's
`context.source` (Node 21+) and refuse to run if it doesn't, rather than
falling back to the plain parser and quietly rewriting every schema.

## No timeout

Every other generator in the bakeoff bounds its tool with a timeout, and
this one can't: ajv compiles synchronously, and synchronous JavaScript is
not interruptible from inside its own process. A schema that made ajv loop
forever would hang the run with no output. Nothing in the suite does --
ajv's runaways, such as the reference resolution in `id.json::refs with
relative uris and defs (2020-12)`, exhaust the stack and surface as a
catchable `RangeError` -- so this is a missing guard rather than a live
problem. Putting the probe-compile in a short-lived worker thread would fix
it, at the cost of shipping every schema across a channel.

## Layout

| file | what |
|---|---|
| `generator.toml` | the three contract commands, the description, `language = "js"`, `kind = "validator"` |
| `package.json`, `package-lock.json` | the exact pins |
| `deps.mjs` | `npm ci` on first use, the stdout muzzle, `writeAll`; copied from `generators/quicktype/deps.mjs` |
| `generate.mjs` | `version`; the module template; number-preserving parse/serialize; compile-for-status |
| `runtime.inc.js` | embedded verbatim in every module: the dialect-to-class map, the external schemas, the options, `__conformanceCheck` |
| `build.mjs` | copy, probe-load each module, write `runner.js`, emit outcomes |

`node_modules/` is not committed; `deps.mjs` runs `npm ci` the first time
either command needs it.

## Reproducing a single group

```sh
cargo run -- run --generator ajv --filter 'dynamicRef.json::$dynamicRef points to a boolean'
```

prints the per-case detail and the path to the generated module under
`target/bakeoff/ajv-<version>/generated/`. That module is a plain CommonJS
file with absolute paths in it, so it can be driven directly:

```sh
node -e 'require("<path>.js").__conformanceCheck({"foo": 42})'
```

which is how the deviations above were separated from the glue.
