# go-jsonschema

[go-jsonschema](https://github.com/omissis/go-jsonschema) — formerly
`atombender/go-jsonschema`, which is still its Go module path — generates Go
types plus hand-written `UnmarshalJSON` methods that re-check the schema's
validation keywords while decoding.

Pinned in `go.mod`:

```
require github.com/atombender/go-jsonschema v0.24.1
```

That require line is the only place the version appears. `version` reports
it from this binary's own build info (`debug.ReadBuildInfo`, falling back to
parsing `go.mod`), so bumping the pin moves the results directory
(`results/go-jsonschema-v0.24.1/`) by itself.

```sh
cargo run -- run --generator go-jsonschema --draft draft7
cargo run -- run --generator go-jsonschema --filter 'required.json::required validation'
```

## Files

| | |
|---|---|
| `generator.toml` | the three commands the harness runs |
| `go.mod`, `go.sum` | the pin, and the hashes the batch module borrows |
| `main.go` | command dispatch, the version lookup, `writeAll` |
| `generate.go` | `generate` (parent + worker) and the check entry point |
| `build.go` | `build`: the batch module, the converge loop, the runner source |
| `batchdeps.go` | build-tagged blank imports: the modules *generated* code can need |

The glue is a Go program run as `go run .`; its module declares `go 1.25.0`
because the tool's own module does. Nothing is vendored: the first command
populates the module cache (that is the only step that touches the network),
and everything after that runs with `GOPROXY=off`.

## The tool as a library, not a CLI

`generate` links `pkg/generator` rather than shelling out to the
`go-jsonschema` binary. The CLI accepts many schema files in one invocation,
but it cannot be used to measure them: its flag mappings
(`--schema-package`, `--schema-output`, `--schema-root-type`) are keyed by
each schema's **`$id`**, and almost nothing in the suite declares one — so
every schema in a batch collapses onto the same output file and the same
package — and its file loop calls `abortWithErr` on the first schema that
fails, which would throw away the rest of the batch.

The library gives what batching was for: one long-lived process, a fresh
`generator.Generator` per schema, and an error or a recovered panic
attributed to the schema that produced it. There is no per-group process
spawn in the normal path.

**The fallback.** A generation can still take the process down for real —
a stack overflow in Go is fatal and unrecoverable. So `generate` is a parent
that writes a plan file and runs itself as a `worker` over it. The worker
prints one status line per schema as it goes; when it dies, or stalls past
the 60s per-schema timeout, the parent records that one schema as a `panic`
and restarts the worker at the next index. In the limit that degenerates to
one invocation per schema, which is the per-schema fallback — but only for
the schemas that actually crash. (On the full suite, none do.)

## Root naming

The tool names the root type after the schema's **file name**, unless a
`RootType` mapping matches the schema's `$id`. Both paths are used: each
schema is written to `<scratch>/go-jsonschema-schemas/<key>/Root` with no
extension, and a mapping with an empty schema ID (which is what a schema
with no `$id` has) sets `RootType: "Root"`. So the root type is `Root`
wherever the tool emits one at all.

Two cases where it isn't:

- **No root `type` keyword, no root type.** `generateRootType` returns early
  when `len(schema.Type) == 0`, so `{"properties": {...}, "required": [...]}`
  — the shape most of the suite uses — produces a file with nothing in it
  but the package clause. That is `compile_fail` (`undefined: Root`), and it
  is the single largest bucket on every draft. It is the tool's behavior,
  not a glue artifact; faking a root type would measure the harness instead.
- **Root `$ref`.** `{"$ref": "#/definitions/x"}` emits `type X`, named after
  the definition, and no `Root`. Also `compile_fail`.

## Options

Everything is the CLI's default except the three things the contract fixes:
`DefaultPackageName` (the module key, so the file declares `package <key>`),
`DefaultOutputName` (`<out_dir>/<key>.go`) and the root type name. In
particular `ExtraImports` is off, `OnlyModels` is off (so the validating
`UnmarshalJSON` methods are emitted — that is the whole point of the row),
`MinSizedInts`, `MinimalNames`, `StructNameFromTitle` and the `Disable*`
flags are off, and `Tags` is `json,yaml,mapstructure`.

`draft` is ignored, and the schema bytes are passed through untouched,
`$schema` and all. go-jsonschema has no dialect setting and never reads
`$schema` — its schema model (`pkg/schemas/model.go`) has no field for it. It
takes `$defs` where present and falls back to `definitions`, for every draft
alike, so a draft-aware run would produce identical results.

## The check entry point

Appended to the tool's output, so a module whose root type came out unusable
— or never came out — fails to compile on its own and the diagnostic is
attributed to its own key:

```go
import bakeoffjson "encoding/json"

func ConformanceCheck(data []byte) error {
	var v Root
	return bakeoffjson.Unmarshal(data, &v)
}
```

`var v Root` works whether `Root` is a struct or an alias of a scalar,
slice or map, and unmarshalling into its address is what triggers the
generated `UnmarshalJSON` where there is one. The import is spliced in right
after the package clause (Go wants imports before declarations) and is
aliased because the generated file usually imports `encoding/json` itself.
Nothing else about the output is modified or stripped.

## Build and run

`build` writes a module at `<batch_dir>` (`module bakeoff`, `go 1.21`) with
one package directory per key and a `main.go` runner that imports the live
ones, then `go build -o runner-bin .` with `GOWORK=off GOPROXY=off
GOTOOLCHAIN=local`. Diagnostics (`# bakeoff/g_abc` headers and
`g_abc/g_abc.go:12:3: ...` spans) are attributed by package directory, the
blamed modules are dropped from `main.go`, and the loop runs again — capped
at ten passes. An error that can't be attributed to any module is a hard
failure, not N `compile_fail`s.

The batch module is dependency-free unless the generated code imports
something: `build` parses each module's imports, maps any non-stdlib path to
the longest matching require in this directory's `go.mod`, and writes those
requires into the batch's `go.mod` along with a copy of `go.sum`, so
`GOPROXY=off` still resolves from the module cache. Two constructs reach
that path — a `format` that renders to the tool's own `pkg/types`, and a
typed `additionalProperties`, which decodes through
`github.com/go-viper/mapstructure/v2` (a module the tool imports into its
output without depending on it itself). `batchdeps.go` is where both are
pinned: a build-tagged file of blank imports whose only job is to keep them
in `go.mod` and `go.sum`, the way `rust-batch` hands serde and regress to
generated Rust. A batch that needs a dependency also adopts this module's
`go` directive instead of `go 1.21`, because the tool emits
range-over-int (Go 1.22) in exactly that code.

`encoding/json` is left exactly as it comes: no `DisallowUnknownFields`.
Unknown properties are ignored and missing ones zero-valued, which is Go's
default and part of what is being measured. A returned error is a `reject`
and only a recovered panic is a `panic`; a stack overflow is fatal in Go,
and the harness notices the dead process by itself. Each case's `data`
stays a `json.RawMessage` so `json.Unmarshal` sees the digits `cases.json`
holds.

## No network

The tool's schema loader fetches `http(s)://` references with
`&http.Client{}`, i.e. through `http.DefaultTransport`. The worker replaces
that transport with one that fails every request, so a remote reference
fails in-process and instantly:

```
could not follow $ref "http://localhost:1234/integer.json" to file
"http://localhost:1234/integer.json": cannot load schema
failed to load schema ...: the bakeoff harness runs offline:
remote references are not fetched
```

So nothing is ever fetched. In practice the transport is reached by only two
groups in the whole suite (`optional/cross-draft.json`), because the tool
has to get as far as *resolving* a reference to try it: a schema whose root
is `{"$ref": "http://localhost:1234/..."}` with no sibling `type` returns
before any reference is followed, so it comes back `ok` with an empty module
and lands in `compile_fail`. Most of `refRemote.json` fails earlier still —
on a *relative* ref (`folderInteger.json`, `./bar.json`) that the file
loader looks for next to the schema and doesn't find.

## Quirks worth knowing

**A root with no `type` produces no type.** Covered above; it is the
dominant outcome — 1084 of the 1408 required groups are
`compile_fail: undefined: Root`. Keyword files whose groups never declare a
root type (`allOf.json`, `anyOf.json`, `oneOf.json`, `not.json`,
`if-then-else.json`, `const.json`, `maxLength.json`, `minimum.json`,
`pattern.json`, `required.json`, `properties.json`, `ref.json`, ...) are
`compile_fail` nearly end-to-end.

**Validation that does happen.** Where there *is* a root type, the generated
`UnmarshalJSON` checks `required`, `enum` (by `reflect.DeepEqual` against the
literal values), `minLength`/`maxLength` (rune counts),
`pattern`, `minimum`/`maximum`/`exclusiveMinimum`/`exclusiveMaximum`,
`multipleOf` (with a `1e-10` tolerance on floats), `minItems`/`maxItems`,
and `const` on string and boolean fields. That is unusually thorough for a
code generator and is where this row's passes come from —
`enum.json`, `items.json`, `pattern.json`, `patternProperties.json`,
`type.json`, `unevaluatedProperties.json`.

`pattern` goes to Go's `regexp`, i.e. RE2: no lookahead or backreferences,
and a pattern RE2 can't compile makes *every* value fail, because the
generated code is `if matched, _ := regexp.MatchString(...); !matched` with
the compile error discarded. That is why `optional/ecmascript-regex.json`
has both passes and `false_reject`s.

**Validation that doesn't.** `uniqueItems`, `minProperties`/`maxProperties`,
`dependentRequired`, `additionalItems`, `additionalProperties: false`,
`contains`, `propertyNames`, `not`, `if`/`then`/`else`, `oneOf` and `format`
are not enforced. `anyOf` and `allOf` are *merged into a type* (fields
unioned, a single base type kept) rather than checked, so
`{"type": "string", "anyOf": [{"maxLength": 2}, {"minLength": 4}]}` comes
out as plain `type Root string` and accepts everything a string can be.
Multiple `type`s (`{"type": ["integer", "string"]}`) pick one — the tool's
README calls this "partial support" and the generator warns
`Property has multiple types; will be represented as interface{} with no
validation` — so the other type is a `false_reject` or the whole thing is
wide open.

**Go's `encoding/json` decides the rest.** A scalar root is where the
measurement really becomes Go's: `type Root int` **accepts `null`**
(unmarshalling null into a non-pointer is a documented no-op) and
**rejects `1.0`**, which the spec says is an integer. That single pair is
why every draft's `type.json` has `mixed` groups. No
`DisallowUnknownFields` is set, so unknown properties are ignored and
missing ones zero-valued — Go's default, and part of what is measured.

**`multipleOf` on an integer type can emit uncompilable Go.** A fractional
`multipleOf` is truncated into the integer modulo operator:
`{"type": "integer", "multipleOf": 0.123456789}` renders as `plain % 0`,
and the compiler says `invalid operation: division by zero`. Ten required
groups land there (`multipleOf.json`, and `optional/float-overflow.json`,
whose `math.Mod(plain, 0.5)` on a `float64`-typed `Plain` is a type error).

**`format` is mostly nothing, except where it changes the type.**
`date-time` becomes `*time.Time` (stdlib), and `date`, `time` and
`duration` become `types.SerializableDate`/`SerializableTime`/
`time.Duration` from the tool's own `pkg/types`. Every other format string
is ignored, so `optional/format/*` would be `false_accept` — except that
the suite's format groups don't declare a root `type`, so they are
`compile_fail` instead and the type-changing formats never come up.

**Unparseable schemas are errors, not crashes.** The tool unmarshals a
schema into a struct, so anything that isn't an object where it expects one
is a parse error: boolean schemas (`true`/`false`, all of
`boolean_schema.json`), draft 4's `exclusiveMaximum: true`,
`"items": [...]` as a tuple, `"dependencies"` values. 107 required groups
are that, plus 54 `schema has no root`.

**`$ref` support is top-level definitions in the same file, and little
else.** `#/definitions/a/definitions/b` and a bare `#` inside a definition
are `value must point to definition within file`; a relative file ref
(`folderInteger.json`, `./bar.json`) is tried against the filesystem next to
the schema and fails there, which is how most of `refRemote.json` reports.
A root `$ref` with no sibling `type` emits the referenced type under the
*definition's* name and never a `Root`.

**It never panicked.** Across all five drafts no schema crashed the
generator or hit the 60s timeout, so the worker was never restarted and
there are no `generation_panic` rows.

**The tool's output is not always gofmt-clean**; it falls back to
unformatted source with a warning (`The generated code could not be
formatted automatically`). Warnings go to stderr tagged with the key that
produced them; the file is used as-is either way.

## Stored messages

Error text is scrubbed of absolute paths before it is reported: the tool
names the schema file in most messages, and `results.json` is committed, so
the schema's temp path becomes `<schema>` and a sibling file a relative
`<key>/<name>`. Nothing else about an error message is changed.

A `panic` message carries a stack trace instead, and that needs the same
treatment twice over: its frame lines are absolute paths into the Go
installation, the module cache and this directory, and its function lines
carry argument words as raw addresses that differ on every run. `scrubStack`
keeps the function, the file's base name and the line, and rewrites every
machine word as `0x?`, so the same crash stores the same text anywhere. For
a worker that died for real the excerpt starts at the runtime's banner
(`fatal error: stack overflow`) rather than at the tail of stderr, where
only the idle goroutines are. No schema actually panics today, so none of
this is exercised by the committed results — it is there so that a version
bump that does panic produces a diff worth reading.
