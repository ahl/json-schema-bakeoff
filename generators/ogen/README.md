# ogen

[ogen](https://ogen.dev) ([github.com/ogen-go/ogen](https://github.com/ogen-go/ogen))
is an OpenAPI v3 generator for Go. It writes a whole package — client, server,
and the part this row measures: a Go type per schema whose JSON encoder,
decoder and validator are all emitted as code, against
[`go-faster/jx`](https://github.com/go-faster/jx) rather than `encoding/json`
and reflection. Pinned by `require github.com/ogen-go/ogen v1.24.0` in
`go.mod`, which is the only place the version appears — `version` reports it
from this binary's build info, so bumping the pin moves
`results/ogen-v1.24.0/` by itself.

```sh
cargo run -- run --generator ogen
cargo run -- run --generator ogen --filter 'required.json::required validation'
```

| | |
|---|---|
| `generator.toml` | the three commands the harness runs |
| `go.mod`, `go.sum` | the pin, and the hashes the batch module borrows |
| `main.go` | command dispatch, the version lookup, `writeAll` |
| `generate.go` | the OpenAPI wrapper, `generate` (parent + worker), the file merge, the check |
| `build.go` | `build`: the batch module, the converge loop, the runner source |
| `batchdeps.go` | build-tagged blank imports: the modules *generated* code needs |

The glue is a Go program run as `go run .`; its module declares `go 1.25.0`
because ogen's own module does. Nothing is vendored: the first command
populates the module cache, and that is the only step that touches the
network. Everything after it runs offline — `GOPROXY=off` for the batch
build, and `GOTOOLCHAIN=local` on all three commands so a machine whose Go is
older than 1.25 fails loudly instead of downloading a toolchain mid-run.

## ogen takes a document, not a schema

This is what makes the row different from every other generator here. ogen has
no "here is a schema, give me a type" entry point, so each group's schema is
wrapped:

```json
{
  "openapi": "3.1.0",
  "info": {"title": "bakeoff", "version": "0"},
  "paths": {"/": {"post": {
    "operationId": "check",
    "requestBody": {"required": true, "content": {"application/json":
      {"schema": {"$ref": "#/components/schemas/Root"}}}},
    "responses": {"200": {"description": "ok"}}
  }}},
  "components": {"schemas": {"Root": <the group's schema, verbatim>}},
  <the group's own top-level members, verbatim>
}
```

OpenAPI **3.1** is the version to wrap in: a 3.1 schema object *is* a JSON
Schema 2020-12 schema, so the group's schema needs no translation. The
document is assembled as text with the harness's schema bytes spliced in
untouched — never re-serialized — so the suite's digits reach ogen as written.

**The synthetic operation.** With `"paths": {}` ogen writes an *empty*
`oas_schemas_gen.go`: it walks operations to decide which types are worth
generating, and a component nothing references is not one. So the document
carries one operation whose request body is the component — also exactly the
position a real ogen user's body occupies. It costs one inert extra type
(`CheckOK`), and it shows up in error text, because ogen reports failures
against the operation (`make ir: operations: path "/": post: requestBody:
contents: media: "application/json": …`). That prefix is left in place; it is
where ogen genuinely decided.

**The hoisted members.** A JSON Schema's `$ref` is a pointer into its own
document: `{"$ref": "#/definitions/x"}` means the `definitions` beside it, and
`{"$ref": "#"}` means the schema itself. ogen resolves local references with a
`RootResolver` over the whole OpenAPI document, so under the naive wrapping
those all aim at members of the *wrapper*, and the suite's
`definitions`/`$defs` groups would report on the wrapper rather than on ogen.
So every top-level member of the schema is copied, verbatim, to the document
root as well, and `#/definitions/x`, `#/$defs/x`, `#/properties/foo` and a
bare `#` resolve to what the suite meant. The names *not* hoisted are all
eleven JSON fields of `ogen.Spec`, not just the four the wrapper writes: a
hoisted `tags` or `servers` would be read as part of the API description, and
a `swagger` would make ogen call the document v2. Nothing in the suite uses
any of them.

**What the hoist is worth, measured:** not one pass. Against the old
per-draft suite ([1.041][archive]), draft 7 had 52 passes either way, with
15 groups moving out of `generation_error` into `false_accept` or
`compile_fail`, and draft 2020-12 had 73 either way with 18 moving. It buys
honesty about *where* ogen stops, not score.

## No batching, because it isn't needed

Measured before deciding: with client and server generation off, ogen turns
one of these documents into a package in **6–7 ms**. On the old per-draft
suite ([1.041][archive]), generation for all five drafts was about 10 s of
a 20 s `--no-cache` run that also compiled and ran everything (warm Go
build cache). A naive probe says 23 ms, which is with the default feature
set, i.e. rendering the HTTP plumbing too.

So one invocation per group in one long-lived process is the whole strategy.
Batching into one document would have bought nothing and cost a lot: the
components would land in one Go package, which the harness cannot attribute
per key; one bad schema would take its batch down, needing bisection; and the
component names would stop being `Root`.

`generate` still supervises itself, because a stack overflow in Go is fatal
and unrecoverable: a parent writes a plan file and runs itself as a `worker`
over it, and when the worker dies — or stalls past the 60 s per-group timeout
— the parent records that group as a `panic` and restarts at the next index.
Nothing in the suite crashes or times out, so there are no `generation_panic`
rows.

The worker also **changes directory** first. When goimports rejects a rendered
template, ogen's writer does `os.WriteFile(fileName+".dump", …)` with a bare
base name — relative to the working directory, which the harness sets to *this
generator's directory*, every file of which is hashed into the row's cache
key, so a stray dump would silently invalidate every stored record. The worker
moves to the scratch directory beside its plan file instead.

## Options

Everything is ogen's default except the feature set.

- **`Parser.AllowRemote` off** (its default) for every group without
  `externalSchemas`, and **on** for the 57 that have them, with a client
  that can only see that group's documents; see "Remote references".
- **`Parser.InferSchemaType` off** (its default, and the CLI's —
  `--infer-types` is opt-in). The biggest lever on the row's numbers, so it
  was measured against the old per-draft suite ([1.041][archive]): with
  inference on, draft 7 went from 52 ✅ / 118 🙈 / 2 🚧 to 83 ✅ / 53 🙈 /
  28 🚧 -- 20.2% to 32.2% pass. It stays at the default because the
  contract is to measure the tool as it comes. What it changes is the
  suite's dominant shape, `{"properties": …}` with no `type`: off, that is an
  untyped schema rendered `type Root jx.Raw`, which accepts anything; on, it
  is inferred to be an object. The 🚧 column is why it isn't free.
- **`Generator.Features = {DisableAll: true}`** — the one setting worth
  arguing about. It turns off ogen's six default features, all of which are
  client, server, OpenTelemetry or stub-handler generation, and changes
  nothing ogen decides about the schema: the type graph, the jx codecs and the
  validators are not features and are built in `NewGenerator` either way. It
  buys not rendering ~25 KB of `net/http` plumbing, a router and an otel
  middleware per group, which the batch would compile once per module and
  whose dependencies it would have to pin.

`AllowCrossTypeConstraints` stays at its default (`true`), so a `pattern` on a
number is interpreted rather than rejected. `draft` is passed and **ignored**:
ogen has no dialect setting. A 3.1 document's schema objects are 2020-12 by
definition, that is the only dialect ogen implements, and
`jsonschema.RawSchema` has no `$schema` field at all.

## One file per group

ogen's output for one component spans several files — the structs in
`oas_schemas_gen.go`, the jx codec in `oas_json_gen.go`, `Validate` in
`oas_validators_gen.go`, `setDefaults` in `oas_defaults_gen.go`, and
`oas_cfg_gen.go`, which with client/server off is reduced to the lookup tables
the validators index (`regexMap` of compiled patterns, `ratMap` of
`multipleOf` rationals). `generate` collects them in memory (ogen renders
templates concurrently, so the `FileSystem` is mutex-guarded) and folds **all**
of them into one `<out_dir>/<key>.go` declaring `package <key>`. Nothing inside
a declaration is rewritten: only each file's `package` clause and `import`
block are cut out, because Go allows one of each per file, and the import specs
are merged back at the top. Dropping `oas_cfg_gen.go` as "client config" was
the first thing tried and was wrong — it turned every `pattern` group into
`undefined: regexMap`. Three guards keep a merge problem from masquerading as a
tool problem, none of which goimports-normalized output needs: an alias
restating its own package name is folded into the bare form; two paths binding
one name is a `generation_error` naming both; and a build constraint above a
package clause is an error rather than a silent omission.

## The check entry point

Appended to the merged module. It is ogen's own request-body path, lifted out
of the `decodeCheckRequest` it generates in `oas_request_decoders_gen.go`:

```go
func ConformanceCheck(data []byte) error {
	var v Root
	d := bakeoffjx.DecodeBytes(data)
	if err := v.Decode(d); err != nil {
		return err
	}
	if err := d.Skip(); err != bakeoffio.EOF {
		return bakeoffErrTrailing
	}
	if val, ok := any(&v).(interface{ Validate() error }); ok {
		return val.Validate()
	}
	return nil
}
```

**Both steps count.** A decode error and a validation error are each a
`reject`, because ogen splits the work: `Decode` enforces the type and the
required-field bitset, `Validate` enforces `maxLength`/`minLength`,
`maximum`/`minimum`, `multipleOf`, `pattern`, `maxItems`/`minItems`,
`uniqueItems`, `maxProperties`/`minProperties` and `format`. Calling only the
decoder would measure half the tool and read as "ogen checks types and nothing
else", which is wrong. `Validate` goes through an interface assertion rather
than a direct call because ogen writes `oas_validators_gen.go` only for types
that have something to check — a `{"type": "string"}` root has no `Validate`
method at all. The trailing-data check is ogen's own, the `io` and `jx`
imports are aliased so they can't collide with the generated code's, and raw
case bytes go straight in, so what a 50-digit integer does is `jx`'s
business.

**`Root` and the one place the wrapper's name leaked.** A `$ref` to the
component normally gives `type Root`. But when the component's schema is a sum
type at its root (`{"oneOf": […]}`), ogen's sum-type path loses the
reference's name and names the type after the *operation* — `CheckReq`, from
the `operationId` this wrapper picked. That is the wrapper showing through, so
`assemble` emits `type Root = CheckReq` when ogen declared the latter and not
the former: an alias, so the method set is ogen's own and nothing is renamed
or synthesized. Worth 1 group: `oneOf.json::nested oneOf, to check
validation semantics`. What it deliberately does *not* do is invent a root
type when ogen emitted none -- 34 groups stay `compile_fail` with the
compiler's `undefined: Root`: a root `$ref` (all but one, mostly in
`ref.json`, `id.json`, `id-legacy.json` and `ref-legacy.json`, and 18 of
them remote), where ogen names the type after the *reference target*
(`{"$ref": "#/definitions/a"}` gives `type A`, and a ref to
`integer.json` gives `type Integer`), and `allOf.json`'s "allOf combined
with anyOf, oneOf", where ogen emitted no schema type at all. That is the
convention DESIGN.md sets and the one go-jsonschema's row follows.

## Build

`build` writes a module at `<batch_dir>` (`module bakeoff`, `go 1.25.0` —
*not* the conservative `go 1.21` the other Go rows use, because every module
here imports ogen packages and ogen's own `go.mod` says 1.25) with one package
directory per key and a `main.go` runner over the live ones, then `go build`
with `GOWORK=off GOFLAGS=-mod=mod GOPROXY=off GOTOOLCHAIN=local`. Diagnostics
(`# bakeoff/g_abc` headers, `g_abc/g_abc.go:12:3: …` spans) are attributed by
package directory, the blamed modules are dropped, and the loop runs again —
capped at ten passes. An error that can't be attributed to a module is a hard
failure, not N `compile_fail`s.

**Batch deps.** Unlike the go-jsonschema row, *every* ogen module has
dependencies: `go-faster/jx`, almost always `go-faster/errors` and
`ogen/validate`, and `ogen/ogenregex` (which pulls in `dlclark/regexp2`),
`ogen/json`, `google/uuid`, `net/netip`, `net/url` and `time` as schemas reach
patterns and formats. `build` parses each module's imports, maps any
non-stdlib path to the longest matching `require` in this directory's `go.mod`,
and writes those requires into the batch's `go.mod` with a copy of its
`go.sum`; `-mod=mod` fills in the indirect requires and `GOPROXY=off` resolves
from the module cache. `batchdeps.go` is where they are pinned: a build-tagged
file of blank imports whose only job is to keep them in `go.mod` and `go.sum`,
the way `rust-batch` hands serde to generated Rust. Two deliberate differences
from go-jsonschema's build: an import the pin list can't map is a **hard
failure** rather than a warning (otherwise `GOPROXY=off` reports `no required
module provides package …` against the module that imported it, and this
harness's incomplete pin list gets recorded as that group's `compile_fail`),
and the batch's `go.sum` is copied unconditionally.

## Remote references

ogen fetches remote documents only when `ParseOptions.AllowRemote` is set,
and then through `ParseOptions.Remote` (`gen.RemoteOptions`: an
`HTTPClient`, a `ReadFile` and a `URLToFilePath`), which becomes ogen's
`jsonschema.ExternalResolver`. That is the tool's documented way to hand it
other documents, so for a group with `externalSchemas` the glue flips
`AllowRemote` -- a non-default option, and the one place this row departs
from ogen's defaults on the schema side -- and passes a client whose
`RoundTripper` is that group's table. A GET for a URL in the table gets 200
and the harness's bytes (a `json.RawMessage`, so digits survive); anything
else is an error out of `RoundTrip` rather than a 404, so the refusal is
unambiguous. `ReadFile` always errors. The client never leaves the process.
Groups without `externalSchemas` take exactly the old path: `AllowRemote`
off, which fails every remote reference in-process with

```
parse spec: at 1:1: parse components: schemas: "Root": resolve
"#/components/schemas/Root": $ref: resolve "http://localhost:1234/...":
get "http://localhost:1234/...": external references are disabled
```

The resolver keys on the absolute, fragment-free URL, which is how the table is
keyed too, so a `$ref` to `http://localhost:1234/integer.json` just works. What
it can't do follows from the model. `$id` isn't in it, so an `$id` inside a
retrieved document never registers that document under a second name
(`the-nested-id.json` in the three "nested absolute ref" groups comes back
refused), and a relative `$ref` resolves against ogen's own root base,
`jsonschema://dummy`, not against any `$id`: that's `get
"jsonschema://dummy/folderInteger.json": unsupported scheme "jsonschema"`, 17
groups. Fragments are JSON pointers only, so a remote `#foo` anchor is `invalid
pointer "foo": pointer must start with '/'`, 9 more.

The 57 groups with `externalSchemas` are 4 ✅, 20 🦀 and 33 ❌. Two ✅ are "root ref
in remote ref", in `ref.json` and `ref-legacy.json`. 18 of the 🦀 resolve the
remote document and generate code, but name the type after the remote document
(`type Integer`) because the root is a `$ref`, the same `undefined: Root` every
local root `$ref` gets (see above). Of the groups without `externalSchemas`, 67
report "external references are disabled": 37 are a relative or non-pointer
reference against `jsonschema://dummy` (`{"$ref": "list"}`, `./baseSchema`), and
the rest are absolute URIs a sibling `$id` declares (`urn:uuid:...`, `tag:...`,
`http://localhost:1234/nested.json`). Belt as well as braces: the worker also
replaces `http.DefaultTransport` and `http.DefaultClient` with one that errors
on every request.

## Quirks

- **No `type` keyword means no type.** ogen doesn't infer, so a schema with no
  `type` and no combinator is "any JSON": `type Root jx.Raw`, which accepts
  everything and has no validator. That is where most of the 220 Standard 🙈
  come from, and it is most of the suite -- `const.json` 0/17, `enum.json`
  0/17, and most of `required.json`, `minimum.json`, `maxProperties.json`,
  `propertyNames.json`, `contains.json`, `dependentRequired.json` and
  `dependentSchemas.json`.
- **The schema model is the whole story.** `jsonschema.RawSchema` is a flat
  struct, and the keywords it has no field for are invisible rather than
  merely unsupported: `not`, `if`/`then`/`else`, `contains`/`maxContains`/
  `minContains`, `propertyNames`, the `dependent*` family, `prefixItems`,
  `additionalItems`, the `unevaluated*` family, `$defs`/`definitions`, `$id`,
  `$anchor`, `$dynamicRef`, `$schema`, `$comment`, `contentSchema`. (`$defs`
  and `definitions` still work as `$ref` targets, but only via the hoist.)
  `$ref` is honoured, `allOf` is *merged* into a type rather than checked, and
  `oneOf`/`anyOf` become sum types when ogen can tell the variants apart.
- **`exclusiveMaximum` is a `bool`** — OpenAPI 3.0 and draft-4 semantics. So
  draft 4's `{"maximum": 3, "exclusiveMaximum": true}` is the form ogen
  understands and **drafts 6 through 2020-12's numeric form is a type error**
  (`cannot unmarshal number into Go struct field .exclusiveMaximum of type
  bool`): the `exclusiveMaximum` and `exclusiveMinimum` groups in
  `maximum.json` and `minimum.json`, and two in `optional/bignum.json`.
  Every other generator here fails the other way round.
- **Boolean schemas never reach the generator.** `true`/`false` as a schema is
  `cannot unmarshal bool into Go value of type struct { ogen.plain; … }`: 22
  Standard groups directly, 11 more prefixed `callback:` (the same failure
  through `jx`'s `Arr`/`Obj` decoders, which wrap a failing per-element
  callback that way), 9 as `unexpected type bool` inside `items`, and 6
  where a `$ref` lands on one (`decode: yaml: ... cannot unmarshal !!bool`).
- **`type` as a list only works for nullability.** `["string", "null"]`
  collapses to `string` + `Nullable`; anything else is `multiple types are not
  supported`.
- **The `not implemented` family is large and explicit,** which makes the 169
  Standard ❌ unusually readable: `sum types with same names not implemented`
  (10), `empty uniqueItems not implemented` (4), `complex anyOf not
  implemented` (4), `discriminator inference not implemented` (2), and `set
  multipleOf: float as int: non-zero fractional part` / `unexpected floating
  point character` (one each in `multipleOf.json`) -- a fractional or
  exponent `multipleOf` is refused at generation rather than mis-compiled.
- **ogen emits uncompilable Go for a `format` on an untyped schema.** A real
  bug, worth filing: `{"format": "ipv4"}` with no `type` gives `type Root
  jx.Raw` *and* a decoder calling `json.DecodeIPv4(d)`, so `cannot use v
  (variable of struct type netip.Addr) as jx.Raw value in assignment`. Eight
  formats hit it, and they are all 29 of the non-`undefined: Root`
  `compile_fail` groups (16 Standard) -- `time.Time` (12), `netip.Addr` (8),
  `url.URL` (3), `uuid.UUID` (3), `time.Duration` (3).
- **`pattern` goes to ECMA-262, not RE2, and it shows.** `ogenregex` uses Go's
  `regexp` where it can and `dlclark/regexp2` otherwise, so
  `optional/ecmascript-regex.json` is the row's best suite file at 15/20:
  all 10 groups spelled `{"type": "string", "pattern": …}` pass -- `\cC`,
  `\t`, `$`-vs-trailing-newline, the ASCII-only `\d`/`\w`/`\s` classes --
  and so do the 5 `patternProperties` groups; the 5 untyped `{"pattern": …}`
  groups are 🙈.
- **The Standard format groups pass for the right reason by accident.** The
  39 Standard groups in the `format-*.json` files assert that `format` is an
  annotation or ignores non-strings (every case `valid: true`), and an
  untyped `jx.Raw` root accepts everything, so ogen passes 23 of them; the
  other 16 are the compile bug above. `Validate` does enforce `format`, but
  only for a *typed* string, which those groups never declare.
- **`$ref` is where the row falls apart:** 1 of 23 `ref.json` groups (8 🙈,
  3 ❌, 11 compile failures). Local pointers resolve, but a root `$ref`
  leaves no `Root`, `$id` is not in the model so a sibling `$id` never changes
  the base URI, `$anchor`/`$dynamicRef` are unknown, and a non-pointer
  reference is treated as remote.
- **`jx` is strict where `encoding/json` is lax, worth 8/12 on `type.json`.**
  `type Root int` rejects `null` (go-jsonschema's row, through
  `encoding/json`, accepts it), and `true`, `"1"`, `{}`, `[]`. The one thing it
  gets wrong is `1.0`, which the spec says is an integer; the other three
  misses are multi-type roots (`multiple types are not supported`).
- **Three 🚧, and they are interesting.** `default.json`'s "the default keyword
  does not do anything if the property is missing" — ogen's `default` support
  is real and lives in the *decoder*, so `{}` against `{"alpha": {"type":
  "number", "maximum": 3, "default": 5}}` is filled in with 5 and then
  rejected for exceeding `maximum`. Plus `type.json`'s `1.0` (in "integer
  type matches integers (6)") and `optional/bignum.json`'s `value out of
  range` (in "integer").

## Stored messages

ogen's messages carry no absolute paths (the document is built in memory and
never has a filename), so nothing needs scrubbing in the ordinary case. A
`panic` message carries a stack trace, which does: `scrubStack` keeps the
function, the file's base name and the line, and rewrites machine words as
`0x?` and goroutine ids as `goroutine ?`, so the same crash stores the same
text on any machine and across runs. Nothing panics today, so a version bump
that does will produce a diff worth reading.

[archive]: ../../archive/suite-1.041/results/ogen-v1.24.0/README.md
