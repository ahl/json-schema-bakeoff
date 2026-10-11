# ogen v1.24.0

An OpenAPI v3 generator for Go, so its design center is an API document
rather than a schema: it emits a client, a server, and a Go type per schema
whose JSON codec and validator are written out as code against
`github.com/go-faster/jx` instead of `encoding/json` and reflection. Each
group here is therefore wrapped as an OpenAPI 3.1 document whose
`components.schemas.Root` is the group's schema — 3.1 schema objects are
JSON Schema 2020-12, so the schema goes in untranslated — with one synthetic
operation that takes it as a request body, because ogen only generates types
something references. ogen splits the work across two methods, and the check
calls both: `Decode` enforces the type and the required-field set, `Validate`
enforces lengths, bounds, patterns and `uniqueItems`.

_generated 2026-10-11T19:51:21Z_

**279 of 1408 non-optional groups pass (19.8%)**; 301 optional groups reported separately

| | bucket | meaning |
|---|---|---|
| ✅ | `pass` | every case matched |
| 🙈 | `false_accept` | deserialize succeeded but `valid: false` (too loose) |
| 🚧 | `false_reject` | deserialize failed but `valid: true` (too strict) |
| 🤪 | `mixed` | both false_accept and false_reject cases in one group |
| 💀 | `runtime_panic` | a test case panicked during deserialize |
| 🦀 | `compile_fail` | generated code doesn't compile |
| ❌ | `generation_error` | the generator returned an error |
| 🤯 | `generation_panic` | the generator crashed or hung |

## Core

### Groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 35 | 80 | 1 | 0 | 0 | 9 | 35 | 0 | 160 | 21.9% |
| draft6 | 42 | 109 | 2 | 0 | 0 | 9 | 71 | 0 | 233 | 18.0% |
| draft7 | 52 | 118 | 2 | 0 | 0 | 11 | 75 | 0 | 258 | 20.2% |
| draft2019-09 | 77 | 169 | 2 | 0 | 0 | 16 | 109 | 0 | 373 | 20.6% |
| draft2020-12 | 73 | 169 | 2 | 0 | 0 | 18 | 122 | 0 | 384 | 19.0% |
| **Total** | **279** | **645** | **9** | **0** | **0** | **63** | **412** | **0** | **1408** | **19.8%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 22 | 11 | 1 | 0 | 0 | 4 | 1 | 0 | 39 | 56.4% |
| draft6 | 15 | 12 | 1 | 0 | 0 | 4 | 12 | 0 | 44 | 34.1% |
| draft7 | 18 | 28 | 1 | 0 | 0 | 6 | 11 | 0 | 64 | 28.1% |
| draft2019-09 | 19 | 34 | 1 | 0 | 0 | 9 | 13 | 0 | 76 | 25.0% |
| draft2020-12 | 19 | 34 | 1 | 0 | 0 | 11 | 13 | 0 | 78 | 24.4% |
| **Total** | **93** | **119** | **5** | **0** | **0** | **34** | **50** | **0** | **301** | **30.9%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 320 | 446 | 71.7% | 618 | 51.8% |
| draft6 | 398 | 583 | 68.3% | 841 | 47.3% |
| draft7 | 455 | 651 | 69.9% | 929 | 49.0% |
| draft2019-09 | 586 | 864 | 67.8% | 1261 | 46.5% |
| draft2020-12 | 578 | 861 | 67.1% | 1301 | 44.4% |
| **Total** | **2337** | **3405** | **68.6%** | **4950** | **47.2%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 126 | 206 | 61.2% | 391 | 32.2% |
| draft6 | 179 | 304 | 58.9% | 520 | 34.4% |
| draft7 | 318 | 571 | 55.7% | 922 | 34.5% |
| draft2019-09 | 338 | 604 | 56.0% | 1043 | 32.4% |
| draft2020-12 | 338 | 604 | 56.0% | 1047 | 32.3% |
| **Total** | **1299** | **2289** | **56.7%** | **3923** | **33.1%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 9 | 🙈✅🙈✅✅❌🙈🙈✅ |
| additionalProperties.json | 2 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈✅🙈✅ |
| allOf.json | 5 | 3 | 0 | 0 | 0 | 1 | 0 | 0 | 9 | 🙈🙈🙈✅✅✅✅✅🦀 |
| anyOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 5 | ❌❌❌❌✅ |
| default.json | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🚧 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| enum.json | 0 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 16 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| format.json | 2 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 6 | ✅🦀🦀✅🦀🦀 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maximum.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minimum.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| multipleOf.json | 0 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 5 | 🙈🙈🙈❌❌ |
| not.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| oneOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 7 | ❌❌❌❌❌❌✅ |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈✅ |
| properties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅🙈 |
| ref.json | 0 | 8 | 0 | 0 | 0 | 4 | 7 | 0 | 19 | 🙈🙈🙈❌🦀🙈❌❌🙈🙈❌🙈❌❌🙈❌🦀🦀🦀 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| required.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| type.json | 8 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 11 | ✅✅✅✅✅✅✅❌✅❌❌ |
| uniqueItems.json | 2 | 1 | 0 | 0 | 0 | 0 | 3 | 0 | 6 | ❌❌❌✅✅🙈 |
| **Total** | **35** | **80** | **1** | **0** | **0** | **9** | **35** | **0** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/ecmascript-regex.json | 15 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/zeroTerminatedFloats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **22** | **11** | **1** | **0** | **0** | **4** | **1** | **0** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 5 | 0 | 0 | 0 | 0 | 1 | 0 | 10 | 🙈🙈✅🙈✅✅❌🙈🙈✅ |
| additionalProperties.json | 2 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈✅🙈✅ |
| allOf.json | 5 | 3 | 0 | 0 | 0 | 1 | 3 | 0 | 12 | 🙈🙈🙈❌❌❌✅✅✅✅✅🦀 |
| anyOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 8 | ❌❌❌❌❌❌❌✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| default.json | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🚧 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 0 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 5 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 9 | ✅🦀🦀✅🦀✅🦀✅✅ |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 3 | 3 | 0 | 0 | 0 | 0 | 3 | 0 | 9 | 🙈🙈❌❌❌🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 0 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 5 | 🙈🙈🙈❌❌ |
| not.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈✅✅ |
| oneOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌✅ |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈❌✅ |
| properties.json | 1 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈❌🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 0 | 12 | 0 | 0 | 0 | 4 | 15 | 0 | 31 | 🙈🙈🙈❌🦀🙈❌❌🙈🙈❌❌❌🙈❌❌❌🙈❌❌❌🙈🙈🙈🙈❌❌❌🦀🦀🦀 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 7 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 11 | 🚧✅✅✅✅✅✅❌✅❌❌ |
| uniqueItems.json | 2 | 1 | 0 | 0 | 0 | 0 | 3 | 0 | 6 | ❌❌❌✅✅🙈 |
| **Total** | **42** | **109** | **2** | **0** | **0** | **9** | **71** | **0** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 0 | 1 | 0 | 0 | 0 | 2 | 0 | 7 | 🚧✅✅✅❌✅❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈❌❌❌❌❌ |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **15** | **12** | **1** | **0** | **0** | **4** | **12** | **0** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 5 | 0 | 0 | 0 | 0 | 1 | 0 | 10 | 🙈🙈✅🙈✅✅❌🙈🙈✅ |
| additionalProperties.json | 2 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈✅🙈✅ |
| allOf.json | 5 | 3 | 0 | 0 | 0 | 1 | 3 | 0 | 12 | 🙈🙈🙈❌❌❌✅✅✅✅✅🦀 |
| anyOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 8 | ❌❌❌❌❌❌❌✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| default.json | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🚧 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 0 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 11 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 17 | ✅✅✅🦀🦀✅✅🦀🦀🦀✅✅✅✅🦀✅✅ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 3 | 3 | 0 | 0 | 0 | 0 | 3 | 0 | 9 | 🙈🙈❌❌❌🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 0 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 5 | 🙈🙈🙈❌❌ |
| not.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈✅✅ |
| oneOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌✅ |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈❌✅ |
| properties.json | 1 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈❌🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 0 | 12 | 0 | 0 | 0 | 4 | 19 | 0 | 35 | 🙈🙈🙈❌🦀🙈❌❌🙈🙈❌❌❌🙈❌❌❌🙈❌❌❌❌🙈🙈🙈🙈❌❌❌❌❌❌🦀🦀🦀 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 7 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 11 | 🚧✅✅✅✅✅✅❌✅❌❌ |
| uniqueItems.json | 2 | 1 | 0 | 0 | 0 | 0 | 3 | 0 | 6 | ❌❌❌✅✅🙈 |
| **Total** | **52** | **118** | **2** | **0** | **0** | **11** | **75** | **0** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 0 | 1 | 0 | 0 | 0 | 2 | 0 | 7 | 🚧✅✅✅❌✅❌ |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈❌❌❌❌❌ |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | ❌🙈🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **18** | **28** | **1** | **0** | **0** | **6** | **11** | **0** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 5 | 0 | 0 | 0 | 0 | 1 | 0 | 10 | 🙈🙈✅🙈✅✅❌🙈🙈✅ |
| additionalProperties.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 5 | 3 | 0 | 0 | 0 | 1 | 3 | 0 | 12 | 🙈🙈🙈❌❌❌✅✅✅✅✅🦀 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 8 | ❌❌❌❌❌❌❌✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🚧 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| enum.json | 0 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 16 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 11 | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 19 | ✅✅✅🦀🦀✅✅🦀🦀🦀✅✅✅✅🦀✅✅🦀🦀 |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 3 | 3 | 0 | 0 | 0 | 0 | 3 | 0 | 9 | 🙈🙈❌❌❌🙈✅✅✅ |
| maxContains.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈🙈🙈🙈 |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minContains.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅🙈🙈🙈🙈🙈✅🙈 |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 0 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 5 | 🙈🙈🙈❌❌ |
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌✅ |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈❌✅ |
| properties.json | 1 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈❌🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 1 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈✅🙈🙈🙈🙈🙈🙈 |
| ref.json | 0 | 12 | 0 | 0 | 0 | 5 | 20 | 0 | 37 | 🙈🙈🙈❌🦀🙈❌🙈🙈❌❌❌🙈🦀🙈❌❌❌❌❌❌❌🙈🙈🙈🙈❌❌❌❌❌❌❌🦀🦀🦀❌ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 7 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 11 | 🚧✅✅✅✅✅✅❌✅❌❌ |
| unevaluatedItems.json | 7 | 8 | 0 | 0 | 0 | 0 | 11 | 0 | 26 | ✅🙈🙈✅🙈✅🙈🙈❌❌✅✅❌❌🙈🙈❌❌❌❌❌❌✅✅🙈❌ |
| unevaluatedProperties.json | 9 | 20 | 0 | 0 | 0 | 2 | 13 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈❌✅✅❌❌🙈🙈🙈🙈🙈❌🦀🦀❌❌❌✅✅🙈🙈🙈🙈❌❌❌❌❌❌✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 1 | 0 | 0 | 0 | 0 | 3 | 0 | 6 | ❌❌❌✅✅🙈 |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **77** | **169** | **2** | **0** | **0** | **16** | **109** | **0** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 4 | 0 | 1 | 0 | 0 | 0 | 2 | 0 | 7 | 🚧✅✅✅❌✅❌ |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈❌❌❌❌❌ |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/duration.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uuid.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/no-schema.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 0 | 4 | 0 | 0 | 0 | 1 | 0 | 0 | 5 | 🙈🙈🙈🦀🙈 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **19** | **34** | **1** | **0** | **0** | **9** | **13** | **0** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 5 | 3 | 0 | 0 | 0 | 1 | 3 | 0 | 12 | 🙈🙈🙈❌❌❌✅✅✅✅✅🦀 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 8 | ❌❌❌❌❌❌❌✅ |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🚧 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| dynamicRef.json | 0 | 5 | 0 | 0 | 0 | 0 | 16 | 0 | 21 | 🙈🙈❌❌❌❌❌❌❌❌❌🙈🙈❌❌❌❌❌🙈❌❌ |
| enum.json | 0 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 16 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 11 | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 19 | ✅✅✅🦀🦀✅✅🦀🦀🦀✅✅✅✅🦀✅✅🦀🦀 |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 2 | 3 | 0 | 0 | 0 | 0 | 5 | 0 | 10 | 🙈❌❌❌✅❌🙈🙈❌✅ |
| maxContains.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈🙈🙈🙈 |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minContains.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅🙈🙈🙈🙈🙈✅🙈 |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 0 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 5 | 🙈🙈🙈❌❌ |
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 1 | 0 | 0 | 0 | 0 | 0 | 10 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌✅ |
| pattern.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈✅✅ |
| patternProperties.json | 2 | 3 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈🙈❌✅✅ |
| prefixItems.json | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈✅✅ |
| properties.json | 1 | 4 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈❌🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 0 | 12 | 0 | 0 | 0 | 5 | 19 | 0 | 36 | 🙈🙈🙈❌🦀🙈❌🙈🙈❌❌❌🙈🦀🙈❌❌❌❌❌❌❌🙈🙈🙈🙈❌❌❌❌❌❌❌🦀🦀🦀 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 7 | 0 | 1 | 0 | 0 | 0 | 3 | 0 | 11 | 🚧✅✅✅✅✅✅❌✅❌❌ |
| unevaluatedItems.json | 5 | 14 | 0 | 0 | 0 | 2 | 8 | 0 | 29 | ✅🙈🙈✅🙈❌🙈🙈❌❌✅❌❌🙈🙈❌🦀🦀❌🙈❌🙈🙈🙈🙈✅✅🙈🙈 |
| unevaluatedProperties.json | 9 | 21 | 0 | 0 | 0 | 2 | 12 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈❌✅✅❌❌🙈🙈🙈🙈🙈❌🦀🦀❌❌❌✅✅🙈🙈🙈🙈❌❌❌🙈❌❌✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 6 | ❌❌❌✅✅❌ |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **73** | **169** | **2** | **0** | **0** | **18** | **122** | **0** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 4 | 0 | 1 | 0 | 0 | 0 | 2 | 0 | 7 | 🚧✅✅✅❌✅❌ |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 10 | 5 | 0 | 0 | 0 | 0 | 5 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈❌❌❌❌❌ |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/duration.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uuid.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/no-schema.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 0 | 4 | 0 | 0 | 0 | 1 | 0 | 0 | 5 | 🙈🙈🙈🦀🙈 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| **Total** | **19** | **34** | **1** | **0** | **0** | **11** | **13** | **0** | **78** |  |

## Failures

### ❌ `generation_error` (462)

130 distinct first-line errors; most common:

- **63×** `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- **49×** `callback: json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- **15×** `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: oneOf: collect variants: Root: sum types with same…`
- **15×** `unexpected type bool`
- **14×** `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: empty uniqueItems not implemented`

Per group:

- `draft4/additionalItems.json::additionalItems does not look in applicators, invalid case` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: flatten allOf: items length is different: 2 and 1`
- `draft4/anyOf.json::anyOf` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: complex anyOf not implemented`
- `draft4/anyOf.json::anyOf with base schema` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: collect variants: Root: sum types with same…`
- `draft4/anyOf.json::anyOf complex types` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: collect variants: Root: sum types with same…`
- `draft4/anyOf.json::anyOf with one empty schema` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: complex anyOf not implemented`
- `draft4/definitions.json::validate definition against metaschema` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://json-schema.org/draft-04/schema#": get "http:/…`
- `draft4/multipleOf.json::float division = inf` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: int validator: set multipleOf: float as int: non-zero fr…`
- `draft4/multipleOf.json::small multiple of large integer` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: int validator: set multipleOf: unexpected floating point…`
- `draft4/oneOf.json::oneOf` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: oneOf: oneOf Root: variant Int: no unique fields, …`
- `draft4/oneOf.json::oneOf with base schema` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: oneOf: collect variants: Root: sum types with same…`
- `draft4/oneOf.json::oneOf complex types` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: oneOf: collect variants: Root: sum types with same…`
- `draft4/oneOf.json::oneOf with empty schema` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: oneOf: oneOf Root: variant Float64: no unique fiel…`
- `draft4/oneOf.json::oneOf with required` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: RootSum: oneOf: collect variants: RootSum: sum types wit…`
- `draft4/oneOf.json::oneOf with missing optional property` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: oneOf: collect variants: Root: sum types with same…`
- `draft4/ref.json::escaped pointer ref` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "percent": $ref: resolve "#/definitions/percent%25field": r…`
- `draft4/ref.json::$ref prevents a sibling id from changing the base uri` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "foo.json": get "jsonschema://dummy/foo.json": …`
- `draft4/ref.json::remote ref, containing refs itself` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://json-schema.org/draft-04/schema#": get "http:/…`
- `draft4/ref.json::Recursive references between schemas` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "nodes": items: $ref: resolve "node": get "jsonschema://dum…`
- `draft4/ref.json::Location-independent identifier` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "#foo": resolve: invalid pointer "foo": pointer…`
- `draft4/ref.json::Location-independent identifier with base URI change in subschema` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "http://localhost:1234/nested.json#foo": get "h…`
- `draft4/ref.json::id must be resolved against nearest parent, not just immediate parent` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": allOf: $ref: resolve "http://example.com/b/d.json": get "http://exam…`
- `draft4/refRemote.json::remote ref` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/integer.json": get "http://loca…`
- `draft4/refRemote.json::fragment within remote ref` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/draft4/subSchemas.json#/definit…`
- `draft4/refRemote.json::ref within remote ref` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/draft4/subSchemas.json#/definit…`
- `draft4/refRemote.json::base URI change` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": items: items: $ref: resolve "folderInteger.json": get "jsonschema://…`
- `draft4/refRemote.json::base URI change - change folder` — `parse spec: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "list": $ref: resolve "#/definitions/baz": items: at 1:587: $ref: r…`
- `draft4/refRemote.json::base URI change - change folder in subschema` — `parse spec: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "list": $ref: resolve "#/definitions/baz/definitions/bar": items: a…`
- `draft4/refRemote.json::root ref in remote ref` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": property "name": $ref: resolve "draft4/name.json#/definitions/orNull…`
- `draft4/refRemote.json::Location-independent identifier in remote ref` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://localhost:1234/draft4/locationIndependentIdent…`
- `draft4/type.json::multiple types can be specified in an array` — `multiple types are not supported: [integer, string]`
- `draft4/type.json::type: array or object` — `multiple types are not supported: [array, object]`
- `draft4/type.json::type: array, object or null` — `multiple types are not supported: [array, object, null]`
- `draft4/uniqueItems.json::uniqueItems validation` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: empty uniqueItems not implemented`
- `draft4/uniqueItems.json::uniqueItems with an array of items` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: empty uniqueItems not implemented`
- `draft4/uniqueItems.json::uniqueItems with an array of items and additionalItems=false` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: empty uniqueItems not implemented`
- `draft6/additionalItems.json::additionalItems does not look in applicators, invalid case` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: flatten allOf: items length is different: 2 and 1`
- `draft6/allOf.json::allOf with boolean schemas, all true` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `draft6/allOf.json::allOf with boolean schemas, some false` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `draft6/allOf.json::allOf with boolean schemas, all false` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `draft6/anyOf.json::anyOf` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: complex anyOf not implemented`
- `draft6/anyOf.json::anyOf with base schema` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: collect variants: Root: sum types with same…`
- `draft6/anyOf.json::anyOf with boolean schemas, all true` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `draft6/anyOf.json::anyOf with boolean schemas, some true` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `draft6/anyOf.json::anyOf with boolean schemas, all false` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `draft6/anyOf.json::anyOf complex types` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: collect variants: Root: sum types with same…`
- `draft6/anyOf.json::anyOf with one empty schema` — `make ir: operations: path "/": post: requestBody: contents: media: "application/json": generate schema: Root: anyOf: complex anyOf not implemented`
- `draft6/boolean_schema.json::boolean schema 'true'` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `draft6/boolean_schema.json::boolean schema 'false'` — `json: cannot unmarshal bool into Go value of type struct { ogen.plain; Type jsonschema.RawType "json:\"type\"" }`
- `draft6/definitions.json::validate definition against metaschema` — `parse spec: at 1:1: parse components: schemas: "Root": resolve "#/components/schemas/Root": $ref: resolve "http://json-schema.org/draft-06/schema#": get "http:/…`
- `draft6/exclusiveMaximum.json::exclusiveMaximum validation` — `json: cannot unmarshal number into Go struct field .exclusiveMaximum of type bool`
- … and 412 more

### 🦀 `compile_fail` (97)

64 distinct first-line errors; most common:

- **2×** `g_0977f3622f6b2774/g_0977f3622f6b2774.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- **2×** `g_1388923c0d2e8319/g_1388923c0d2e8319.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- **2×** `g_2ab1d1fdb82f12ea/g_2ab1d1fdb82f12ea.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- **2×** `g_3c2350f56a9cd7d5/g_3c2350f56a9cd7d5.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- **2×** `g_45c241d219aafa93/g_45c241d219aafa93.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`

Per group:

- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `g_4f4d7609d5333d3e/g_4f4d7609d5333d3e.go:16:8: undefined: Root`
- `draft4/format.json::ipv4 format` — `g_ff3566f5773cf85b/g_ff3566f5773cf85b.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft4/format.json::ipv6 format` — `g_45c241d219aafa93/g_45c241d219aafa93.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft4/format.json::date-time format` — `g_ca458fad922b8f9a/g_ca458fad922b8f9a.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft4/format.json::uri format` — `g_0977f3622f6b2774/g_0977f3622f6b2774.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- `draft4/ref.json::nested refs` — `g_02d11943a743fc37/g_02d11943a743fc37.go:60:8: undefined: Root`
- `draft4/ref.json::id with file URI still resolves pointers - *nix` — `g_9ebd8832f4b5322f/g_9ebd8832f4b5322f.go:69:8: undefined: Root`
- `draft4/ref.json::id with file URI still resolves pointers - windows` — `g_e370a25ff6a65f28/g_e370a25ff6a65f28.go:69:8: undefined: Root`
- `draft4/ref.json::empty tokens in $ref json-pointer` — `g_1a6410aa264732b6/g_1a6410aa264732b6.go:69:8: undefined: Root`
- `draft6/allOf.json::allOf combined with anyOf, oneOf` — `g_e92785eaca336258/g_e92785eaca336258.go:16:8: undefined: Root`
- `draft6/format.json::ipv4 format` — `g_49023074162538a2/g_49023074162538a2.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft6/format.json::ipv6 format` — `g_1388923c0d2e8319/g_1388923c0d2e8319.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft6/format.json::date-time format` — `g_caa4a16ad6ca1e70/g_caa4a16ad6ca1e70.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft6/format.json::uri format` — `g_b28fcd01cbcd5013/g_b28fcd01cbcd5013.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- `draft6/ref.json::nested refs` — `g_bb244f5c400f7ac4/g_bb244f5c400f7ac4.go:60:8: undefined: Root`
- `draft6/ref.json::$id with file URI still resolves pointers - *nix` — `g_9c6f255003a37c05/g_9c6f255003a37c05.go:69:8: undefined: Root`
- `draft6/ref.json::$id with file URI still resolves pointers - windows` — `g_7eefedd579bf5e34/g_7eefedd579bf5e34.go:69:8: undefined: Root`
- `draft6/ref.json::empty tokens in $ref json-pointer` — `g_54c6c0d3404725f9/g_54c6c0d3404725f9.go:69:8: undefined: Root`
- `draft7/allOf.json::allOf combined with anyOf, oneOf` — `g_e446c1bdb29f7f73/g_e446c1bdb29f7f73.go:16:8: undefined: Root`
- `draft7/format.json::ipv4 format` — `g_76728f030d36f621/g_76728f030d36f621.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft7/format.json::ipv6 format` — `g_ae73a9e62358267e/g_ae73a9e62358267e.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft7/format.json::date format` — `g_4f64ef541430e30c/g_4f64ef541430e30c.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft7/format.json::date-time format` — `g_2ab1d1fdb82f12ea/g_2ab1d1fdb82f12ea.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft7/format.json::time format` — `g_ef337fea3faf1589/g_ef337fea3faf1589.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft7/format.json::uri format` — `g_7b63951497746465/g_7b63951497746465.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- `draft7/ref.json::nested refs` — `g_1f65e8dd946b7222/g_1f65e8dd946b7222.go:60:8: undefined: Root`
- `draft7/ref.json::$id with file URI still resolves pointers - *nix` — `g_d69d275541ebba53/g_d69d275541ebba53.go:69:8: undefined: Root`
- `draft7/ref.json::$id with file URI still resolves pointers - windows` — `g_371e04e5d4e3d0c4/g_371e04e5d4e3d0c4.go:69:8: undefined: Root`
- `draft7/ref.json::empty tokens in $ref json-pointer` — `g_41aedbdba2582d11/g_41aedbdba2582d11.go:69:8: undefined: Root`
- `draft2019-09/allOf.json::allOf combined with anyOf, oneOf` — `g_0fb7689fd656ef14/g_0fb7689fd656ef14.go:16:8: undefined: Root`
- `draft2019-09/format.json::ipv4 format` — `g_e49b67fba2cb4674/g_e49b67fba2cb4674.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft2019-09/format.json::ipv6 format` — `g_6c387534ff8d9eff/g_6c387534ff8d9eff.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft2019-09/format.json::date format` — `g_7dd5388926b89dd2/g_7dd5388926b89dd2.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft2019-09/format.json::date-time format` — `g_7c3f42ee4be16fff/g_7c3f42ee4be16fff.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft2019-09/format.json::time format` — `g_fa4c98101f6a637e/g_fa4c98101f6a637e.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft2019-09/format.json::uri format` — `g_721a083cbf29a5cf/g_721a083cbf29a5cf.go:31:15: cannot use v (variable of struct type url.URL) as jx.Raw value in assignment`
- `draft2019-09/format.json::uuid format` — `g_f7d7395bb3b35f9b/g_f7d7395bb3b35f9b.go:31:15: cannot use v (variable of array type "github.com/google/uuid".UUID) as jx.Raw value in assignment`
- `draft2019-09/format.json::duration format` — `g_5aa6f70ec5408b36/g_5aa6f70ec5408b36.go:31:15: cannot use v (variable of int64 type "time".Duration) as jx.Raw value in assignment`
- `draft2019-09/ref.json::nested refs` — `g_c0deabfba020ce51/g_c0deabfba020ce51.go:60:8: undefined: Root`
- `draft2019-09/ref.json::ref creates new scope when adjacent to keywords` — `g_53b9cb8ba00695f7/g_53b9cb8ba00695f7.go:62:8: undefined: Root`
- `draft2019-09/ref.json::$id with file URI still resolves pointers - *nix` — `g_8034532fef343901/g_8034532fef343901.go:69:8: undefined: Root`
- `draft2019-09/ref.json::$id with file URI still resolves pointers - windows` — `g_f8889c21c6594bb9/g_f8889c21c6594bb9.go:69:8: undefined: Root`
- `draft2019-09/ref.json::empty tokens in $ref json-pointer` — `g_04119bd11c613db6/g_04119bd11c613db6.go:69:8: undefined: Root`
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with $ref` — `g_673af462f9cc4fba/g_673af462f9cc4fba.go:62:8: undefined: Root`
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties before $ref` — `g_673af462f9cc4fba/g_673af462f9cc4fba.go:62:8: undefined: Root`
- `draft2020-12/allOf.json::allOf combined with anyOf, oneOf` — `g_61df76eed5508d9d/g_61df76eed5508d9d.go:16:8: undefined: Root`
- `draft2020-12/format.json::ipv4 format` — `g_ff3d4ce2ba492d26/g_ff3d4ce2ba492d26.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft2020-12/format.json::ipv6 format` — `g_f4df5fcfb3b1574c/g_f4df5fcfb3b1574c.go:31:15: cannot use v (variable of struct type netip.Addr) as jx.Raw value in assignment`
- `draft2020-12/format.json::date format` — `g_3c2350f56a9cd7d5/g_3c2350f56a9cd7d5.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- `draft2020-12/format.json::date-time format` — `g_e397333038542bbb/g_e397333038542bbb.go:31:15: cannot use v (variable of struct type "time".Time) as jx.Raw value in assignment`
- … and 47 more

### 🚧 `false_reject` (14)

3 distinct first-line errors; most common:

- **5×** `a bignum is an integer: alias: value out of range`
- **5×** `missing properties are not filled in with the default: invalid: alpha (float: value 5.000000 greater than 3.000000)`
- **4×** `a float with zero fractional part is an integer: alias: unexpected floating point character: unexpected byte 46 '.' at 1`

Per group:

- `draft4/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: invalid: alpha (float: value 5.000000 greater than 3.000000)`
  - cases: missing properties are not filled in with the default (rejected)
- `draft6/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: invalid: alpha (float: value 5.000000 greater than 3.000000)`
  - cases: missing properties are not filled in with the default (rejected)
- `draft6/type.json::integer type matches integers` — `a float with zero fractional part is an integer: alias: unexpected floating point character: unexpected byte 46 '.' at 1`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft7/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: invalid: alpha (float: value 5.000000 greater than 3.000000)`
  - cases: missing properties are not filled in with the default (rejected)
- `draft7/type.json::integer type matches integers` — `a float with zero fractional part is an integer: alias: unexpected floating point character: unexpected byte 46 '.' at 1`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft2019-09/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: invalid: alpha (float: value 5.000000 greater than 3.000000)`
  - cases: missing properties are not filled in with the default (rejected)
- `draft2019-09/type.json::integer type matches integers` — `a float with zero fractional part is an integer: alias: unexpected floating point character: unexpected byte 46 '.' at 1`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft2020-12/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: invalid: alpha (float: value 5.000000 greater than 3.000000)`
  - cases: missing properties are not filled in with the default (rejected)
- `draft2020-12/type.json::integer type matches integers` — `a float with zero fractional part is an integer: alias: unexpected floating point character: unexpected byte 46 '.' at 1`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft4/optional/bignum.json::integer` — `a bignum is an integer: alias: value out of range`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft6/optional/bignum.json::integer` — `a bignum is an integer: alias: value out of range`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft7/optional/bignum.json::integer` — `a bignum is an integer: alias: value out of range`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft2019-09/optional/bignum.json::integer` — `a bignum is an integer: alias: value out of range`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft2020-12/optional/bignum.json::integer` — `a bignum is an integer: alias: value out of range`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)

### 🙈 `false_accept` (764)

165 distinct first-line errors; most common:

- **29×** `accepted 'mismatch' though valid: false`
- **16×** `accepted 'a non-string is invalid' though valid: false`
- **15×** `accepted 'something else is invalid' though valid: false`
- **15×** `accepted 'too long is invalid' though valid: false`
- **15×** `accepted 'too short is invalid' though valid: false`

Per group:

- `draft4/additionalItems.json::additionalItems as schema` — `accepted 'additional items do not match schema' though valid: false`
  - cases: additional items do not match schema (accepted)
- `draft4/additionalItems.json::array of items with no additionalItems permitted` — `accepted 'additional items are not permitted' though valid: false`
  - cases: additional items are not permitted (accepted)
- `draft4/additionalItems.json::items validation adjusts the starting index for additionalItems` — `accepted 'wrong type of second item' though valid: false`
  - cases: wrong type of second item (accepted)
- `draft4/additionalItems.json::additionalItems with heterogeneous array` — `accepted 'heterogeneous invalid instance' though valid: false`
  - cases: heterogeneous invalid instance (accepted)
- `draft4/additionalProperties.json::additionalProperties being false does not allow other properties` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted)
- `draft4/additionalProperties.json::non-ASCII pattern with additionalProperties` — `accepted 'not matching the pattern is invalid' though valid: false`
  - cases: not matching the pattern is invalid (accepted)
- `draft4/additionalProperties.json::additionalProperties with schema` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `draft4/additionalProperties.json::additionalProperties can exist by itself` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `draft4/additionalProperties.json::additionalProperties does not look in applicators` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `draft4/allOf.json::allOf` — `accepted 'mismatch second' though valid: false`
  - cases: mismatch second (accepted), mismatch first (accepted), wrong type (accepted)
- `draft4/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch first allOf (accepted), mismatch second allOf (accepted), mismatch both (accepted)
- `draft4/allOf.json::allOf simple types` — `accepted 'mismatch one' though valid: false`
  - cases: mismatch one (accepted)
- `draft4/dependencies.json::dependencies` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted)
- `draft4/dependencies.json::multiple dependencies` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted), missing other dependency (accepted), missing both dependencies (accepted)
- `draft4/dependencies.json::multiple dependencies subschema` — `accepted 'wrong type' though valid: false`
  - cases: wrong type (accepted), wrong type other (accepted), wrong type both (accepted)
- `draft4/dependencies.json::dependencies with escaped characters` — `accepted 'invalid object 1' though valid: false`
  - cases: invalid object 1 (accepted), invalid object 2 (accepted), invalid object 3 (accepted), invalid object 4 (accepted)
- `draft4/dependencies.json::dependent subschema incompatible with root` — `accepted 'matches root' though valid: false`
  - cases: matches root (accepted), matches both (accepted)
- `draft4/enum.json::simple enum validation` — `accepted 'something else is invalid' though valid: false`
  - cases: something else is invalid (accepted)
- `draft4/enum.json::heterogeneous enum validation` — `accepted 'something else is invalid' though valid: false`
  - cases: something else is invalid (accepted), objects are deep compared (accepted), extra properties in object is invalid (accepted)
- `draft4/enum.json::heterogeneous enum-with-null validation` — `accepted 'something else is invalid' though valid: false`
  - cases: something else is invalid (accepted)
- `draft4/enum.json::enums in properties` — `accepted 'wrong foo value' though valid: false`
  - cases: wrong foo value (accepted), wrong bar value (accepted)
- `draft4/enum.json::enum with escaped characters` — `accepted 'another string is invalid' though valid: false`
  - cases: another string is invalid (accepted)
- `draft4/enum.json::enum with false does not match 0` — `accepted 'integer zero is invalid' though valid: false`
  - cases: integer zero is invalid (accepted), float zero is invalid (accepted)
- `draft4/enum.json::enum with [false] does not match [0]` — `accepted '[0] is invalid' though valid: false`
  - cases: [0] is invalid (accepted), [0.0] is invalid (accepted)
- `draft4/enum.json::enum with true does not match 1` — `accepted 'integer one is invalid' though valid: false`
  - cases: integer one is invalid (accepted), float one is invalid (accepted)
- `draft4/enum.json::enum with [true] does not match [1]` — `accepted '[1] is invalid' though valid: false`
  - cases: [1] is invalid (accepted), [1.0] is invalid (accepted)
- `draft4/enum.json::enum with 0 does not match false` — `accepted 'false is invalid' though valid: false`
  - cases: false is invalid (accepted)
- `draft4/enum.json::enum with [0] does not match [false]` — `accepted '[false] is invalid' though valid: false`
  - cases: [false] is invalid (accepted)
- `draft4/enum.json::enum with 1 does not match true` — `accepted 'true is invalid' though valid: false`
  - cases: true is invalid (accepted)
- `draft4/enum.json::enum with [1] does not match [true]` — `accepted '[true] is invalid' though valid: false`
  - cases: [true] is invalid (accepted)
- `draft4/enum.json::nul characters in strings` — `accepted 'do not match string lacking nul' though valid: false`
  - cases: do not match string lacking nul (accepted)
- `draft4/enum.json::characters with the same visual representation but different codepoint` — `accepted 'character looks the same but uses a different codepoint' though valid: false`
  - cases: character looks the same but uses a different codepoint (accepted)
- `draft4/enum.json::characters with the same visual representation, but different number of codepoints` — `accepted 'character looks the same but uses combining marks' though valid: false`
  - cases: character looks the same but uses combining marks (accepted)
- `draft4/infinite-loop-detection.json::evaluating the same schema location against the same data location twice is not a sign of an infinite loop` — `accepted 'failing case' though valid: false`
  - cases: failing case (accepted)
- `draft4/items.json::a schema given for items` — `accepted 'wrong type of items' though valid: false`
  - cases: wrong type of items (accepted)
- `draft4/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted)
- `draft4/items.json::items and subitems` — `accepted 'wrong sub-item' though valid: false`
  - cases: wrong sub-item (accepted)
- `draft4/maxItems.json::maxItems validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted)
- `draft4/maxLength.json::maxLength validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted)
- `draft4/maxProperties.json::maxProperties validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted)
- `draft4/maxProperties.json::maxProperties = 0 means the object is empty` — `accepted 'one property is invalid' though valid: false`
  - cases: one property is invalid (accepted)
- `draft4/maximum.json::maximum validation` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted)
- `draft4/maximum.json::maximum validation with unsigned integer` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted)
- `draft4/maximum.json::maximum validation (explicit false exclusivity)` — `accepted 'above the maximum is invalid' though valid: false`
  - cases: above the maximum is invalid (accepted)
- `draft4/maximum.json::exclusiveMaximum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted)
- `draft4/minItems.json::minItems validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted)
- `draft4/minLength.json::minLength validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), one grapheme is not long enough (accepted)
- `draft4/minProperties.json::minProperties validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted)
- `draft4/minimum.json::minimum validation` — `accepted 'below the minimum is invalid' though valid: false`
  - cases: below the minimum is invalid (accepted)
- `draft4/minimum.json::minimum validation (explicit false exclusivity)` — `accepted 'below the minimum is invalid' though valid: false`
  - cases: below the minimum is invalid (accepted)
- … and 714 more

