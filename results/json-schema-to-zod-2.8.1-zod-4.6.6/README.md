# json-schema-to-zod 2.8.1 + zod 4.6.6

Turns a JSON Schema into a zod schema expression, so you get a static
TypeScript type and a runtime parser from one artifact. Its dispatcher keys
on `type`: a typed schema picks up bounds, lengths, patterns, enums and
`const`, while a schema without `type` becomes `z.any()` whatever else it
says. It has no `$ref` resolver. Unmaintained since March 2026; 2.8.1 is the
last release, and it runs on zod 4 here.

_generated 2026-10-11T18:53:39Z_

**538 of 1408 non-optional groups pass (38.2%)**; 301 optional groups reported separately

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
| draft4 | 62 | 86 | 10 | 2 | 0 | 0 | 0 | 0 | 160 | 38.8% |
| draft6 | 95 | 120 | 16 | 2 | 0 | 0 | 0 | 0 | 233 | 40.8% |
| draft7 | 108 | 131 | 17 | 2 | 0 | 0 | 0 | 0 | 258 | 41.9% |
| draft2019-09 | 136 | 214 | 21 | 2 | 0 | 0 | 0 | 0 | 373 | 36.5% |
| draft2020-12 | 137 | 223 | 23 | 1 | 0 | 0 | 0 | 0 | 384 | 35.7% |
| **Total** | **538** | **774** | **87** | **9** | **0** | **0** | **0** | **0** | **1408** | **38.2%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 18 | 17 | 4 | 0 | 0 | 0 | 0 | 0 | 39 | 46.2% |
| draft6 | 18 | 22 | 4 | 0 | 0 | 0 | 0 | 0 | 44 | 40.9% |
| draft7 | 21 | 39 | 4 | 0 | 0 | 0 | 0 | 0 | 64 | 32.8% |
| draft2019-09 | 23 | 49 | 4 | 0 | 0 | 0 | 0 | 0 | 76 | 30.3% |
| draft2020-12 | 23 | 51 | 4 | 0 | 0 | 0 | 0 | 0 | 78 | 29.5% |
| **Total** | **103** | **178** | **20** | **0** | **0** | **0** | **0** | **0** | **301** | **34.2%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 457 | 618 | 73.9% | 618 | 73.9% |
| draft6 | 627 | 841 | 74.6% | 841 | 74.6% |
| draft7 | 702 | 929 | 75.6% | 929 | 75.6% |
| draft2019-09 | 905 | 1261 | 71.8% | 1261 | 71.8% |
| draft2020-12 | 928 | 1301 | 71.3% | 1301 | 71.3% |
| **Total** | **3619** | **4950** | **73.1%** | **4950** | **73.1%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 193 | 391 | 49.4% | 391 | 49.4% |
| draft6 | 268 | 520 | 51.5% | 520 | 51.5% |
| draft7 | 449 | 922 | 48.7% | 922 | 48.7% |
| draft2019-09 | 515 | 1043 | 49.4% | 1043 | 49.4% |
| draft2020-12 | 517 | 1047 | 49.4% | 1047 | 49.4% |
| **Total** | **1942** | **3923** | **49.5%** | **3923** | **49.5%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 2 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈✅🙈✅ |
| allOf.json | 5 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈✅✅✅✅✅🙈 |
| anyOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| dependencies.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| enum.json | 11 | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 16 | ✅🚧✅✅✅✅🚧✅🚧✅🚧✅🚧✅✅✅ |
| format.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 3 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🤪✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maximum.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minimum.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 5 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅🙈✅✅ |
| oneOf.json | 2 | 0 | 4 | 1 | 0 | 0 | 0 | 0 | 7 | 🤪🚧🚧✅🚧🚧✅ |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈✅ |
| properties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅🙈 |
| ref.json | 0 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 19 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🚧🙈🙈🙈🙈 |
| refRemote.json | 0 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈🙈🙈 |
| required.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **62** | **86** | **10** | **2** | **0** | **0** | **0** | **0** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/ecmascript-regex.json | 13 | 5 | 2 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈🚧✅✅✅🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/zeroTerminatedFloats.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **18** | **17** | **4** | **0** | **0** | **0** | **0** | **0** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 2 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈✅🙈✅ |
| allOf.json | 8 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 🙈🙈🙈✅✅✅✅✅✅✅✅🙈 |
| anyOf.json | 5 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅🙈✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 11 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 17 | ✅🚧🚧✅✅✅🚧🚧🚧🚧✅✅✅✅✅✅✅ |
| contains.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 10 | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 15 | ✅🚧✅✅✅✅🚧✅🚧✅🚧✅🚧✅✅ |
| exclusiveMaximum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| exclusiveMinimum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 4 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 9 | 🙈🙈✅🙈🙈🤪✅✅✅ |
| maxItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 7 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅🙈✅✅✅✅ |
| oneOf.json | 6 | 0 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🤪🚧✅✅✅✅🚧✅🚧🚧✅ |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 1 | 29 | 1 | 0 | 0 | 0 | 0 | 0 | 31 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅🙈🙈🙈🙈🙈🙈🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| refRemote.json | 0 | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **95** | **120** | **16** | **2** | **0** | **0** | **0** | **0** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/ecmascript-regex.json | 13 | 5 | 2 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈🚧✅✅✅🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **18** | **22** | **4** | **0** | **0** | **0** | **0** | **0** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 2 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈✅🙈✅ |
| allOf.json | 8 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 🙈🙈🙈✅✅✅✅✅✅✅✅🙈 |
| anyOf.json | 5 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅🙈✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 11 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 17 | ✅🚧🚧✅✅✅🚧🚧🚧🚧✅✅✅✅✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 10 | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 15 | ✅🚧✅✅✅✅🚧✅🚧✅🚧✅🚧✅✅ |
| exclusiveMaximum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| exclusiveMinimum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 5 | 6 | 1 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅✅🙈🚧🙈🙈 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 4 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 9 | 🙈🙈✅🙈🙈🤪✅✅✅ |
| maxItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 7 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅🙈✅✅✅✅ |
| oneOf.json | 6 | 0 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🤪🚧✅✅✅✅🚧✅🚧🚧✅ |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 1 | 33 | 1 | 0 | 0 | 0 | 0 | 0 | 35 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅🙈🙈🙈🙈🙈🙈🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| refRemote.json | 0 | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **108** | **131** | **17** | **2** | **0** | **0** | **0** | **0** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/ecmascript-regex.json | 13 | 5 | 2 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈🚧✅✅✅🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **21** | **39** | **4** | **0** | **0** | **0** | **0** | **0** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 8 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 🙈🙈🙈✅✅✅✅✅✅✅✅🙈 |
| anchor.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| anyOf.json | 5 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅🙈✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 11 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 17 | ✅🚧🚧✅✅✅🚧🚧🚧🚧✅✅✅✅✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| defs.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| enum.json | 11 | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 16 | ✅🚧✅✅✅✅🚧✅🚧✅🚧✅🚧✅✅✅ |
| exclusiveMaximum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| exclusiveMinimum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 5 | 6 | 1 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅✅🙈🚧🙈🙈 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 4 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 9 | 🙈🙈✅🙈🙈🤪✅✅✅ |
| maxContains.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈🙈🙈🙈 |
| maxItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minContains.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅🙈🙈🙈🙈🙈✅🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 7 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅🙈✅✅✅✅🚧 |
| oneOf.json | 6 | 0 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🤪🚧✅✅✅✅🚧✅🚧🚧✅ |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 1 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈✅🙈🙈🙈🙈🙈🙈 |
| ref.json | 1 | 35 | 1 | 0 | 0 | 0 | 0 | 0 | 37 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈✅🙈🙈🙈🙈🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| refRemote.json | 0 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 7 | 18 | 1 | 0 | 0 | 0 | 0 | 0 | 26 | ✅🙈🙈✅🙈✅🙈🙈🙈🙈✅✅🙈🚧🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈 |
| unevaluatedProperties.json | 9 | 33 | 2 | 0 | 0 | 0 | 0 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🚧🚧✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| vocabulary.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| **Total** | **136** | **214** | **21** | **2** | **0** | **0** | **0** | **0** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/cross-draft.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/ecmascript-regex.json | 13 | 5 | 2 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈🚧✅✅✅🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/duration.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uuid.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/no-schema.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **23** | **49** | **4** | **0** | **0** | **0** | **0** | **0** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 8 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | 🙈🙈🙈✅✅✅✅✅✅✅✅🙈 |
| anchor.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| anyOf.json | 5 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈✅✅✅🙈✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 11 | 0 | 6 | 0 | 0 | 0 | 0 | 0 | 17 | ✅🚧🚧✅✅✅🚧🚧🚧🚧✅✅✅✅✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| defs.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| dynamicRef.json | 4 | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | 🙈🙈🙈🙈🙈🙈✅✅✅🙈✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| enum.json | 11 | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 16 | ✅🚧✅✅✅✅🚧✅🚧✅🚧✅🚧✅✅✅ |
| exclusiveMaximum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| exclusiveMinimum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 5 | 6 | 1 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅✅🙈🚧🙈🙈 |
| infinite-loop-detection.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| items.json | 3 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈✅🙈🙈✅🙈🙈🙈🙈✅ |
| maxContains.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈🙈🙈🙈 |
| maxItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minContains.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅🙈🙈🙈🙈🙈✅🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 7 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅🙈✅✅✅✅🚧 |
| oneOf.json | 6 | 0 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🤪🚧✅✅✅✅🚧✅🚧🚧✅ |
| pattern.json | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈✅🚧 |
| patternProperties.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅✅ |
| prefixItems.json | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈✅✅ |
| properties.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅🙈 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 1 | 34 | 1 | 0 | 0 | 0 | 0 | 0 | 36 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈✅🙈🙈🙈🙈🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| refRemote.json | 0 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 7 | 21 | 1 | 0 | 0 | 0 | 0 | 0 | 29 | ✅🙈🙈✅🙈✅🙈🙈🙈✅✅🙈🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈 |
| unevaluatedProperties.json | 9 | 32 | 3 | 0 | 0 | 0 | 0 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🚧🚧✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| vocabulary.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| **Total** | **137** | **223** | **23** | **1** | **0** | **0** | **0** | **0** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/cross-draft.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/ecmascript-regex.json | 13 | 5 | 2 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅🙈🙈🙈🙈🙈🚧✅✅✅🚧 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format-assertion.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/duration.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uuid.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/no-schema.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **23** | **51** | **4** | **0** | **0** | **0** | **0** | **0** | **78** |  |

## Failures

### 🤪 `mixed` (9)

2 distinct first-line errors; most common:

- **5×** `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
- **4×** `accepted 'too many sub-items' though valid: false`

Per group:

- `draft4/items.json::items and subitems` — `accepted 'too many sub-items' though valid: false`
  - cases: too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted), fewer items is valid (rejected)
- `draft4/oneOf.json::oneOf` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), neither oneOf valid (accepted)
- `draft6/items.json::items and subitems` — `accepted 'too many sub-items' though valid: false`
  - cases: too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted), fewer items is valid (rejected)
- `draft6/oneOf.json::oneOf` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), neither oneOf valid (accepted)
- `draft7/items.json::items and subitems` — `accepted 'too many sub-items' though valid: false`
  - cases: too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted), fewer items is valid (rejected)
- `draft7/oneOf.json::oneOf` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), neither oneOf valid (accepted)
- `draft2019-09/items.json::items and subitems` — `accepted 'too many sub-items' though valid: false`
  - cases: too many sub-items (accepted), wrong item (accepted), wrong sub-item (accepted), fewer items is valid (rejected)
- `draft2019-09/oneOf.json::oneOf` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), neither oneOf valid (accepted)
- `draft2020-12/oneOf.json::oneOf` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), neither oneOf valid (accepted)

### 🚧 `false_reject` (107)

26 distinct first-line errors; most common:

- **9×** `[false] is valid: ZodError: <root>: Invalid input: expected false`
- **9×** `[true] is valid: ZodError: <root>: Invalid input: expected true`
- **5×** `[0] is valid: ZodError: <root>: Invalid input: expected 0`
- **5×** `[1] is valid: ZodError: <root>: Invalid input: expected 1`
- **5×** `a bignum is an integer: ZodError: <root>: Too big: expected int to be <=9007199254740991`

Per group:

- `draft4/enum.json::heterogeneous enum validation` — `one of the enum is valid: ZodError: <root>: Invalid input`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft4/enum.json::enum with [false] does not match [0]` — `[false] is valid: ZodError: <root>: Invalid input: expected false`
  - cases: [false] is valid (rejected)
- `draft4/enum.json::enum with [true] does not match [1]` — `[true] is valid: ZodError: <root>: Invalid input: expected true`
  - cases: [true] is valid (rejected)
- `draft4/enum.json::enum with [0] does not match [false]` — `[0] is valid: ZodError: <root>: Invalid input: expected 0`
  - cases: [0] is valid (rejected), [0.0] is valid (rejected)
- `draft4/enum.json::enum with [1] does not match [true]` — `[1] is valid: ZodError: <root>: Invalid input: expected 1`
  - cases: [1] is valid (rejected), [1.0] is valid (rejected)
- `draft4/oneOf.json::oneOf with base schema` — `one oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: one oneOf valid (rejected)
- `draft4/oneOf.json::oneOf complex types` — `first oneOf valid (complex): ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (complex) (rejected), second oneOf valid (complex) (rejected)
- `draft4/oneOf.json::oneOf with required` — `first valid - valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first valid - valid (rejected), second valid - valid (rejected)
- `draft4/oneOf.json::oneOf with missing optional property` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `draft4/ref.json::naive replacement of $ref with its destination is not correct` — `match the enum exactly: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: match the enum exactly (rejected)
- `draft6/const.json::const with object` — `same object is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: same object is valid (rejected), same object with different property order is valid (rejected)
- `draft6/const.json::const with array` — `same array is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: same array is valid (rejected)
- `draft6/const.json::const with [false] does not match [0]` — `[false] is valid: ZodError: <root>: Invalid input: expected false`
  - cases: [false] is valid (rejected)
- `draft6/const.json::const with [true] does not match [1]` — `[true] is valid: ZodError: <root>: Invalid input: expected true`
  - cases: [true] is valid (rejected)
- `draft6/const.json::const with {"a": false} does not match {"a": 0}` — `{"a": false} is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: {"a": false} is valid (rejected)
- `draft6/const.json::const with {"a": true} does not match {"a": 1}` — `{"a": true} is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: {"a": true} is valid (rejected)
- `draft6/enum.json::heterogeneous enum validation` — `one of the enum is valid: ZodError: <root>: Invalid input`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft6/enum.json::enum with [false] does not match [0]` — `[false] is valid: ZodError: <root>: Invalid input: expected false`
  - cases: [false] is valid (rejected)
- `draft6/enum.json::enum with [true] does not match [1]` — `[true] is valid: ZodError: <root>: Invalid input: expected true`
  - cases: [true] is valid (rejected)
- `draft6/enum.json::enum with [0] does not match [false]` — `[0] is valid: ZodError: <root>: Invalid input: expected 0`
  - cases: [0] is valid (rejected), [0.0] is valid (rejected)
- `draft6/enum.json::enum with [1] does not match [true]` — `[1] is valid: ZodError: <root>: Invalid input: expected 1`
  - cases: [1] is valid (rejected), [1.0] is valid (rejected)
- `draft6/oneOf.json::oneOf with base schema` — `one oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: one oneOf valid (rejected)
- `draft6/oneOf.json::oneOf complex types` — `first oneOf valid (complex): ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (complex) (rejected), second oneOf valid (complex) (rejected)
- `draft6/oneOf.json::oneOf with required` — `first valid - valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first valid - valid (rejected), second valid - valid (rejected)
- `draft6/oneOf.json::oneOf with missing optional property` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `draft6/ref.json::naive replacement of $ref with its destination is not correct` — `match the enum exactly: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: match the enum exactly (rejected)
- `draft7/const.json::const with object` — `same object is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: same object is valid (rejected), same object with different property order is valid (rejected)
- `draft7/const.json::const with array` — `same array is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: same array is valid (rejected)
- `draft7/const.json::const with [false] does not match [0]` — `[false] is valid: ZodError: <root>: Invalid input: expected false`
  - cases: [false] is valid (rejected)
- `draft7/const.json::const with [true] does not match [1]` — `[true] is valid: ZodError: <root>: Invalid input: expected true`
  - cases: [true] is valid (rejected)
- `draft7/const.json::const with {"a": false} does not match {"a": 0}` — `{"a": false} is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: {"a": false} is valid (rejected)
- `draft7/const.json::const with {"a": true} does not match {"a": 1}` — `{"a": true} is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: {"a": true} is valid (rejected)
- `draft7/enum.json::heterogeneous enum validation` — `one of the enum is valid: ZodError: <root>: Invalid input`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft7/enum.json::enum with [false] does not match [0]` — `[false] is valid: ZodError: <root>: Invalid input: expected false`
  - cases: [false] is valid (rejected)
- `draft7/enum.json::enum with [true] does not match [1]` — `[true] is valid: ZodError: <root>: Invalid input: expected true`
  - cases: [true] is valid (rejected)
- `draft7/enum.json::enum with [0] does not match [false]` — `[0] is valid: ZodError: <root>: Invalid input: expected 0`
  - cases: [0] is valid (rejected), [0.0] is valid (rejected)
- `draft7/enum.json::enum with [1] does not match [true]` — `[1] is valid: ZodError: <root>: Invalid input: expected 1`
  - cases: [1] is valid (rejected), [1.0] is valid (rejected)
- `draft7/if-then-else.json::if appears at the end when serialized (keyword processing sequence)` — `other redirects to else and passes: Cannot read properties of undefined (reading 'forEach')`
  - cases: other redirects to else and passes (rejected)
- `draft7/oneOf.json::oneOf with base schema` — `one oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: one oneOf valid (rejected)
- `draft7/oneOf.json::oneOf complex types` — `first oneOf valid (complex): ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (complex) (rejected), second oneOf valid (complex) (rejected)
- `draft7/oneOf.json::oneOf with required` — `first valid - valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first valid - valid (rejected), second valid - valid (rejected)
- `draft7/oneOf.json::oneOf with missing optional property` — `first oneOf valid: ZodError: <root>: Invalid input: Should pass single schema. Passed 2`
  - cases: first oneOf valid (rejected), second oneOf valid (rejected)
- `draft7/ref.json::naive replacement of $ref with its destination is not correct` — `match the enum exactly: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: match the enum exactly (rejected)
- `draft2019-09/const.json::const with object` — `same object is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: same object is valid (rejected), same object with different property order is valid (rejected)
- `draft2019-09/const.json::const with array` — `same array is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: same array is valid (rejected)
- `draft2019-09/const.json::const with [false] does not match [0]` — `[false] is valid: ZodError: <root>: Invalid input: expected false`
  - cases: [false] is valid (rejected)
- `draft2019-09/const.json::const with [true] does not match [1]` — `[true] is valid: ZodError: <root>: Invalid input: expected true`
  - cases: [true] is valid (rejected)
- `draft2019-09/const.json::const with {"a": false} does not match {"a": 0}` — `{"a": false} is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: {"a": false} is valid (rejected)
- `draft2019-09/const.json::const with {"a": true} does not match {"a": 1}` — `{"a": true} is valid: ZodError: <root>: Invalid input: expected [object Object]`
  - cases: {"a": true} is valid (rejected)
- `draft2019-09/enum.json::heterogeneous enum validation` — `one of the enum is valid: ZodError: <root>: Invalid input`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- … and 57 more

### 🙈 `false_accept` (952)

198 distinct first-line errors; most common:

- **35×** `accepted 'mismatch' though valid: false`
- **27×** `accepted 'too long is invalid' though valid: false`
- **27×** `accepted 'too short is invalid' though valid: false`
- **26×** `accepted 'a non-string is invalid' though valid: false`
- **24×** `accepted 'non-number is invalid' though valid: false`

Per group:

- `draft4/additionalItems.json::additionalItems as schema` — `accepted 'additional items do not match schema' though valid: false`
  - cases: additional items do not match schema (accepted)
- `draft4/additionalItems.json::array of items with no additionalItems permitted` — `accepted 'additional items are not permitted' though valid: false`
  - cases: additional items are not permitted (accepted)
- `draft4/additionalItems.json::additionalItems does not look in applicators, invalid case` — `accepted 'items defined in allOf are not examined' though valid: false`
  - cases: items defined in allOf are not examined (accepted)
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
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `accepted 'allOf: false, anyOf: false, oneOf: false' though valid: false`
  - cases: allOf: false, anyOf: false, oneOf: false (accepted), allOf: false, anyOf: false, oneOf: true (accepted), allOf: false, anyOf: true, oneOf: false (accepted), allOf: false, anyOf: true, oneOf: true (accepted), allOf: true, anyOf: false, oneOf: false (accepted), allOf: true, anyOf: false, oneOf: true (accepted), … and 1 more
- `draft4/anyOf.json::anyOf` — `accepted 'neither anyOf valid' though valid: false`
  - cases: neither anyOf valid (accepted)
- `draft4/anyOf.json::anyOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), both anyOf invalid (accepted)
- `draft4/anyOf.json::anyOf complex types` — `accepted 'neither anyOf valid (complex)' though valid: false`
  - cases: neither anyOf valid (complex) (accepted)
- `draft4/definitions.json::validate definition against metaschema` — `accepted 'invalid definition schema' though valid: false`
  - cases: invalid definition schema (accepted)
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
- `draft4/infinite-loop-detection.json::evaluating the same schema location against the same data location twice is not a sign of an infinite loop` — `accepted 'failing case' though valid: false`
  - cases: failing case (accepted)
- `draft4/items.json::a schema given for items` — `accepted 'wrong type of items' though valid: false`
  - cases: wrong type of items (accepted)
- `draft4/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted)
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
- `draft4/minimum.json::exclusiveMinimum validation` — `accepted 'boundary point is invalid' though valid: false`
  - cases: boundary point is invalid (accepted)
- `draft4/minimum.json::minimum validation with signed integer` — `accepted 'float below the minimum is invalid' though valid: false`
  - cases: float below the minimum is invalid (accepted), int below the minimum is invalid (accepted)
- `draft4/multipleOf.json::by int` — `accepted 'int by int fail' though valid: false`
  - cases: int by int fail (accepted)
- `draft4/multipleOf.json::by number` — `accepted '35 is not multiple of 1.5' though valid: false`
  - cases: 35 is not multiple of 1.5 (accepted)
- `draft4/multipleOf.json::by small number` — `accepted '0.00751 is not multiple of 0.0001' though valid: false`
  - cases: 0.00751 is not multiple of 0.0001 (accepted)
- `draft4/not.json::forbidden property` — `accepted 'property present' though valid: false`
  - cases: property present (accepted)
- `draft4/pattern.json::pattern validation` — `accepted 'a non-matching pattern is invalid' though valid: false`
  - cases: a non-matching pattern is invalid (accepted)
- `draft4/patternProperties.json::patternProperties validates properties matching a regex` — `accepted 'a single invalid match is invalid' though valid: false`
  - cases: a single invalid match is invalid (accepted), multiple invalid matches is invalid (accepted)
- `draft4/patternProperties.json::multiple simultaneous patternProperties are validated` — `accepted 'an invalid due to one is invalid' though valid: false`
  - cases: an invalid due to one is invalid (accepted), an invalid due to the other is invalid (accepted), an invalid due to both is invalid (accepted)
- `draft4/patternProperties.json::regexes are not anchored by default and are case sensitive` — `accepted 'recognized members are accounted for' though valid: false`
  - cases: recognized members are accounted for (accepted), regexes are case sensitive, 2 (accepted)
- `draft4/properties.json::object properties validation` — `accepted 'one property invalid is invalid' though valid: false`
  - cases: one property invalid is invalid (accepted), both properties invalid is invalid (accepted)
- … and 902 more

