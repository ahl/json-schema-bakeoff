# quicktype 26.0.0 rust

Types in a dozen languages from JSON samples or schemas. Its design center is
the shape of data: ergonomic, compilable types everywhere. Validation beyond
shape is out of scope, so bounds, lengths, patterns, `const`, and `oneOf`
exclusivity sail through. TypeScript adds a runtime type check; Go leans on
encoding/json's leniency.

_generated 2026-10-25T14:55:58Z_

**544 of 1408 non-optional groups pass (38.6%)**; 301 optional groups reported separately

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
| draft4 | 66 | 63 | 5 | 4 | 0 | 0 | 22 | 0 | 160 | 41.2% |
| draft6 | 96 | 84 | 6 | 5 | 0 | 0 | 39 | 3 | 233 | 41.2% |
| draft7 | 112 | 93 | 6 | 5 | 0 | 0 | 39 | 3 | 258 | 43.4% |
| draft2019-09 | 135 | 169 | 11 | 4 | 0 | 0 | 51 | 3 | 373 | 36.2% |
| draft2020-12 | 135 | 177 | 10 | 3 | 0 | 0 | 56 | 3 | 384 | 35.2% |
| **Total** | **544** | **586** | **38** | **21** | **0** | **0** | **207** | **12** | **1408** | **38.6%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 7 | 30 | 1 | 0 | 0 | 0 | 1 | 0 | 39 | 17.9% |
| draft6 | 5 | 34 | 2 | 0 | 0 | 0 | 3 | 0 | 44 | 11.4% |
| draft7 | 8 | 52 | 2 | 0 | 0 | 0 | 2 | 0 | 64 | 12.5% |
| draft2019-09 | 12 | 56 | 2 | 0 | 0 | 0 | 6 | 0 | 76 | 15.8% |
| draft2020-12 | 12 | 58 | 2 | 0 | 0 | 0 | 6 | 0 | 78 | 15.4% |
| **Total** | **44** | **230** | **9** | **0** | **0** | **0** | **18** | **0** | **301** | **14.6%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 424 | 565 | 75.0% | 618 | 68.6% |
| draft6 | 553 | 738 | 74.9% | 841 | 65.8% |
| draft7 | 630 | 826 | 76.3% | 929 | 67.8% |
| draft2019-09 | 808 | 1129 | 71.6% | 1261 | 64.1% |
| draft2020-12 | 828 | 1157 | 71.6% | 1302 | 63.6% |
| **Total** | **3243** | **4415** | **73.5%** | **4951** | **65.5%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 169 | 391 | 43.2% | 394 | 42.9% |
| draft6 | 240 | 516 | 46.5% | 523 | 45.9% |
| draft7 | 427 | 928 | 46.0% | 933 | 45.8% |
| draft2019-09 | 491 | 1040 | 47.2% | 1054 | 46.6% |
| draft2020-12 | 493 | 1044 | 47.2% | 1058 | 46.6% |
| **Total** | **1820** | **3919** | **46.4%** | **3962** | **45.9%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 3 | 4 | 1 | 1 | 0 | 0 | 0 | 0 | 9 | 🙈✅🙈✅🚧🙈🤪🙈✅ |
| additionalProperties.json | 4 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈✅✅✅✅ |
| allOf.json | 5 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅🙈🙈✅✅✅✅🙈🙈 |
| anyOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈✅✅🙈 |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 0 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| enum.json | 9 | 1 | 2 | 0 | 0 | 0 | 4 | 0 | 16 | 🙈🚧🚧✅✅✅❌✅❌✅❌✅❌✅✅✅ |
| format.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 6 | ✅🤪🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maximum.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minimum.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| oneOf.json | 0 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈🙈 |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈✅ |
| properties.json | 4 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈✅✅✅ |
| ref.json | 9 | 1 | 0 | 0 | 0 | 0 | 9 | 0 | 19 | 🙈✅✅❌✅✅❌❌✅✅❌❌❌❌❌❌✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 8 | ❌❌❌❌❌❌❌❌ |
| required.json | 1 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 🤪✅🙈🙈 |
| type.json | 9 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅🙈✅✅✅🚧 |
| uniqueItems.json | 2 | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 6 | 🙈🤪🙈✅🚧✅ |
| **Total** | **66** | **63** | **5** | **4** | **0** | **0** | **22** | **0** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/ecmascript-regex.json | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/zeroTerminatedFloats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **7** | **30** | **1** | **0** | **0** | **0** | **1** | **0** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 4 | 1 | 1 | 0 | 0 | 0 | 0 | 10 | 🙈✅✅🙈✅🚧🙈🤪🙈✅ |
| additionalProperties.json | 4 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈✅✅✅✅ |
| allOf.json | 6 | 4 | 0 | 0 | 0 | 0 | 2 | 0 | 12 | ✅🙈🙈✅❌❌✅✅✅✅🙈🙈 |
| anyOf.json | 3 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 8 | 🙈🙈✅❌❌✅✅🙈 |
| boolean_schema.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ✅❌ |
| const.json | 7 | 3 | 0 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈✅✅✅ |
| contains.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 8 | 1 | 2 | 0 | 0 | 0 | 4 | 0 | 15 | 🙈🚧🚧✅✅✅❌✅❌✅❌✅❌✅✅ |
| exclusiveMaximum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| exclusiveMinimum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 5 | 1 | 0 | 1 | 0 | 0 | 2 | 0 | 9 | ✅🤪✅❌❌🙈✅✅✅ |
| maxItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈✅✅ |
| oneOf.json | 0 | 8 | 0 | 0 | 0 | 0 | 3 | 0 | 11 | 🙈🙈🙈❌❌❌🙈🙈🙈🙈🙈 |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 4 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 21 | 1 | 0 | 1 | 0 | 0 | 5 | 3 | 31 | 🙈✅✅❌✅✅🤪❌✅✅✅❌✅❌✅✅✅❌✅✅🤯✅✅✅✅🤯🤯✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 5 | 🤪✅✅🙈🙈 |
| type.json | 8 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅🙈✅✅✅🚧 |
| uniqueItems.json | 2 | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 6 | 🙈🤪🙈✅🚧✅ |
| **Total** | **96** | **84** | **6** | **5** | **0** | **0** | **39** | **3** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/ecmascript-regex.json | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
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
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **5** | **34** | **2** | **0** | **0** | **0** | **3** | **0** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 4 | 1 | 1 | 0 | 0 | 0 | 0 | 10 | 🙈✅✅🙈✅🚧🙈🤪🙈✅ |
| additionalProperties.json | 4 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈✅✅✅✅ |
| allOf.json | 6 | 4 | 0 | 0 | 0 | 0 | 2 | 0 | 12 | ✅🙈🙈✅❌❌✅✅✅✅🙈🙈 |
| anyOf.json | 3 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 8 | 🙈🙈✅❌❌✅✅🙈 |
| boolean_schema.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ✅❌ |
| const.json | 7 | 3 | 0 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 8 | 1 | 2 | 0 | 0 | 0 | 4 | 0 | 15 | 🙈🚧🚧✅✅✅❌✅❌✅❌✅❌✅✅ |
| exclusiveMaximum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| exclusiveMinimum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 5 | 1 | 0 | 1 | 0 | 0 | 2 | 0 | 9 | ✅🤪✅❌❌🙈✅✅✅ |
| maxItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maxProperties.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 🙈🙈🙈🙈🙈🙈✅✅ |
| oneOf.json | 0 | 8 | 0 | 0 | 0 | 0 | 3 | 0 | 11 | 🙈🙈🙈❌❌❌🙈🙈🙈🙈🙈 |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 4 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 25 | 1 | 0 | 1 | 0 | 0 | 5 | 3 | 35 | 🙈✅✅❌✅✅🤪❌✅✅✅❌✅❌✅✅✅❌✅✅✅🤯✅✅✅✅🤯🤯✅✅✅✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 5 | 🤪✅✅🙈🙈 |
| type.json | 8 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅🙈✅✅✅🚧 |
| uniqueItems.json | 2 | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 6 | 🙈🤪🙈✅🚧✅ |
| **Total** | **112** | **93** | **6** | **5** | **0** | **0** | **39** | **3** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
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
| optional/id.json | 0 | 2 | 0 | 0 | 0 | 0 | 1 | 0 | 3 | ❌🙈🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **8** | **52** | **2** | **0** | **0** | **0** | **2** | **0** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 4 | 1 | 1 | 0 | 0 | 0 | 0 | 10 | 🙈✅✅🙈✅🚧🙈🤪🙈✅ |
| additionalProperties.json | 5 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈✅✅✅✅✅🙈 |
| allOf.json | 6 | 4 | 0 | 0 | 0 | 0 | 2 | 0 | 12 | ✅🙈🙈✅❌❌✅✅✅✅🙈🙈 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 3 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 8 | 🙈🙈✅❌❌✅✅🙈 |
| boolean_schema.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ✅❌ |
| const.json | 7 | 3 | 0 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| enum.json | 8 | 1 | 2 | 0 | 0 | 0 | 5 | 0 | 16 | 🙈🚧🚧✅✅✅❌✅❌✅❌✅❌✅❌✅ |
| exclusiveMaximum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| exclusiveMinimum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 5 | 1 | 0 | 1 | 0 | 0 | 2 | 0 | 9 | ✅🤪✅❌❌🙈✅✅✅ |
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
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 0 | 8 | 0 | 0 | 0 | 0 | 3 | 0 | 11 | 🙈🙈🙈❌❌❌🙈🙈🙈🙈🙈 |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈✅ |
| properties.json | 4 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 1 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈✅🙈🙈🙈🙈🙈🙈 |
| ref.json | 22 | 4 | 1 | 0 | 0 | 0 | 7 | 3 | 37 | 🙈✅✅❌✅🙈❌✅✅✅❌✅❌🙈❌✅✅✅🙈❌✅🤯✅✅✅✅🤯🤯❌✅✅✅✅✅✅✅🚧 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 5 | 🤪✅✅🙈🙈 |
| type.json | 8 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅🙈✅✅✅🚧 |
| unevaluatedItems.json | 5 | 17 | 4 | 0 | 0 | 0 | 0 | 0 | 26 | ✅🙈🙈✅🙈🚧🙈🙈🙈🙈🚧🚧🙈🙈✅🚧🙈🙈🙈🙈🙈🙈✅✅🙈🙈 |
| unevaluatedProperties.json | 9 | 35 | 0 | 0 | 0 | 0 | 0 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 6 | 🙈🤪🙈✅🚧✅ |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **135** | **169** | **11** | **4** | **0** | **0** | **51** | **3** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/ecmascript-regex.json | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
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
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/no-schema.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 3 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 5 | ✅❌✅✅❌ |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **12** | **56** | **2** | **0** | **0** | **0** | **6** | **0** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 5 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈✅✅✅✅✅🙈 |
| allOf.json | 6 | 4 | 0 | 0 | 0 | 0 | 2 | 0 | 12 | ✅🙈🙈✅❌❌✅✅✅✅🙈🙈 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 3 | 3 | 0 | 0 | 0 | 0 | 2 | 0 | 8 | 🙈🙈✅❌❌✅✅🙈 |
| boolean_schema.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ✅❌ |
| const.json | 7 | 3 | 0 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈✅✅✅ |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| dynamicRef.json | 4 | 12 | 0 | 0 | 0 | 0 | 5 | 0 | 21 | 🙈🙈🙈🙈🙈🙈✅✅✅🙈✅🙈🙈❌❌❌❌❌🙈🙈🙈 |
| enum.json | 8 | 1 | 2 | 0 | 0 | 0 | 5 | 0 | 16 | 🙈🚧🚧✅✅✅❌✅❌✅❌✅❌✅❌✅ |
| exclusiveMaximum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| exclusiveMinimum.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 5 | 0 | 0 | 0 | 0 | 1 | 0 | 10 | ✅✅❌🙈✅🙈🙈🙈🙈✅ |
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
| not.json | 2 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈🙈🙈🙈✅✅🙈 |
| oneOf.json | 0 | 8 | 0 | 0 | 0 | 0 | 3 | 0 | 11 | 🙈🙈🙈❌❌❌🙈🙈🙈🙈🙈 |
| pattern.json | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈✅🙈 |
| patternProperties.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈✅🙈 |
| prefixItems.json | 1 | 0 | 1 | 1 | 0 | 0 | 1 | 0 | 4 | 🤪❌🚧✅ |
| properties.json | 4 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 22 | 4 | 0 | 0 | 0 | 0 | 7 | 3 | 36 | 🙈✅✅❌✅🙈❌✅✅✅❌✅❌🙈❌✅✅✅🙈❌✅🤯✅✅✅✅🤯🤯❌✅✅✅✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 5 | 🤪✅✅🙈🙈 |
| type.json | 8 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅🙈✅✅✅🚧 |
| unevaluatedItems.json | 6 | 19 | 4 | 0 | 0 | 0 | 0 | 0 | 29 | ✅🙈🙈✅🙈🚧✅🙈🙈🚧🚧🙈🙈✅🚧🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈 |
| unevaluatedProperties.json | 9 | 35 | 0 | 0 | 0 | 0 | 0 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 6 | 🙈🤪🙈✅🚧✅ |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **135** | **177** | **10** | **3** | **0** | **0** | **56** | **3** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 4 | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 0 | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
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
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/no-schema.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 3 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 5 | ✅❌✅✅❌ |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **12** | **58** | **2** | **0** | **0** | **0** | **6** | **0** | **78** |  |

## Failures

### 🤯 `generation_panic` (12)

1 distinct first-line errors; most common:

- **12×** `Error: URNs do not have any generally defined hierarchical components`

Per group:

- `draft6/ref.json::simple URN base URI with $ref via the URN` — `Error: URNs do not have any generally defined hierarchical components`
- `draft6/ref.json::URN base URI with URN and JSON pointer ref` — `Error: URNs do not have any generally defined hierarchical components`
- `draft6/ref.json::URN base URI with URN and anchor ref` — `Error: URNs do not have any generally defined hierarchical components`
- `draft7/ref.json::simple URN base URI with $ref via the URN` — `Error: URNs do not have any generally defined hierarchical components`
- `draft7/ref.json::URN base URI with URN and JSON pointer ref` — `Error: URNs do not have any generally defined hierarchical components`
- `draft7/ref.json::URN base URI with URN and anchor ref` — `Error: URNs do not have any generally defined hierarchical components`
- `draft2019-09/ref.json::simple URN base URI with $ref via the URN` — `Error: URNs do not have any generally defined hierarchical components`
- `draft2019-09/ref.json::URN base URI with URN and JSON pointer ref` — `Error: URNs do not have any generally defined hierarchical components`
- `draft2019-09/ref.json::URN base URI with URN and anchor ref` — `Error: URNs do not have any generally defined hierarchical components`
- `draft2020-12/ref.json::simple URN base URI with $ref via the URN` — `Error: URNs do not have any generally defined hierarchical components`
- `draft2020-12/ref.json::URN base URI with URN and JSON pointer ref` — `Error: URNs do not have any generally defined hierarchical components`
- `draft2020-12/ref.json::URN base URI with URN and anchor ref` — `Error: URNs do not have any generally defined hierarchical components`

### ❌ `generation_error` (225)

82 distinct first-line errors; most common:

- **64×** `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- **5×** `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/items/items`
- **5×** `SchemaKeyNotInObject: Key foo%22bar not in schema object at Root#foo%22bar`
- **5×** `SchemaKeyNotInObject: Key percent%25field not in schema object at Root#percent%25field`
- **4×** `SchemaArrayItemsMustBeStringOrArray: Array items must be an array or an object, but is false`

Per group:

- `draft4/definitions.json::validate definition against metaschema` — `SchemaFetchError: Could not fetch schema http://json-schema.org/draft-04/schema, referred to from Root#`
- `draft4/enum.json::enum with [false] does not match [0]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft4/enum.json::enum with [true] does not match [1]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft4/enum.json::enum with [0] does not match [false]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft4/enum.json::enum with [1] does not match [true]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft4/ref.json::escaped pointer ref` — `SchemaKeyNotInObject: Key percent%25field not in schema object at Root#percent%25field`
- `draft4/ref.json::$ref prevents a sibling id from changing the base uri` — `SchemaFetchError: Could not fetch schema foo.json, referred to from Root#/allOf/0`
- `draft4/ref.json::remote ref, containing refs itself` — `SchemaFetchError: Could not fetch schema http://json-schema.org/draft-04/schema, referred to from Root#`
- `draft4/ref.json::Recursive references between schemas` — `SchemaFetchError: Could not fetch schema node, referred to from Root#/properties/nodes/items`
- `draft4/ref.json::refs with quote` — `SchemaKeyNotInObject: Key foo%22bar not in schema object at Root#foo%22bar`
- `draft4/ref.json::Location-independent identifier` — `SchemaKeyNotInObject: Key foo not in schema object at Root#foo`
- `draft4/ref.json::Location-independent identifier with base URI change in subschema` — `SchemaFetchError: Could not fetch schema http://localhost:1234/nested.json, referred to from Root#/allOf/0`
- `draft4/ref.json::naive replacement of $ref with its destination is not correct` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft4/ref.json::id must be resolved against nearest parent, not just immediate parent` — `SchemaFetchError: Could not fetch schema http://example.com/b/d.json, referred to from Root#/allOf/0`
- `draft4/refRemote.json::remote ref` — `SchemaFetchError: Could not fetch schema http://localhost:1234/integer.json, referred to from Root#`
- `draft4/refRemote.json::fragment within remote ref` — `SchemaFetchError: Could not fetch schema http://localhost:1234/draft4/subSchemas.json, referred to from Root#`
- `draft4/refRemote.json::ref within remote ref` — `SchemaFetchError: Could not fetch schema http://localhost:1234/draft4/subSchemas.json, referred to from Root#`
- `draft4/refRemote.json::base URI change` — `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/items/items`
- `draft4/refRemote.json::base URI change - change folder` — `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/definitions/baz/items`
- `draft4/refRemote.json::base URI change - change folder in subschema` — `SchemaFetchError: Could not fetch schema folderInteger.json, referred to from Root#/definitions/baz/definitions/bar/items`
- `draft4/refRemote.json::root ref in remote ref` — `SchemaFetchError: Could not fetch schema draft4/name.json, referred to from Root#/properties/name`
- `draft4/refRemote.json::Location-independent identifier in remote ref` — `SchemaFetchError: Could not fetch schema http://localhost:1234/draft4/locationIndependentIdentifier.json, referred to from Root#`
- `draft6/allOf.json::allOf with boolean schemas, some false` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/allOf/1`
- `draft6/allOf.json::allOf with boolean schemas, all false` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/allOf/0`
- `draft6/anyOf.json::anyOf with boolean schemas, some true` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/anyOf/1`
- `draft6/anyOf.json::anyOf with boolean schemas, all false` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/anyOf/0`
- `draft6/boolean_schema.json::boolean schema 'false'` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#`
- `draft6/const.json::const with object` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/const.json::const with array` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/const.json::const with null` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/const.json::const with [false] does not match [0]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/const.json::const with [true] does not match [1]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/const.json::const with {"a": false} does not match {"a": 0}` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/const.json::const with {"a": true} does not match {"a": 1}` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/definitions.json::validate definition against metaschema` — `SchemaFetchError: Could not fetch schema http://json-schema.org/draft-06/schema, referred to from Root#`
- `draft6/enum.json::enum with [false] does not match [0]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/enum.json::enum with [true] does not match [1]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/enum.json::enum with [0] does not match [false]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/enum.json::enum with [1] does not match [true]` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- `draft6/items.json::items with boolean schema (false)` — `SchemaArrayItemsMustBeStringOrArray: Array items must be an array or an object, but is false`
- `draft6/items.json::items with boolean schemas` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/items/1`
- `draft6/oneOf.json::oneOf with boolean schemas, one true` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/oneOf/1`
- `draft6/oneOf.json::oneOf with boolean schemas, more than one true` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/oneOf/2`
- `draft6/oneOf.json::oneOf with boolean schemas, all false` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/oneOf/0`
- `draft6/properties.json::properties with boolean schema` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/properties/bar`
- `draft6/ref.json::escaped pointer ref` — `SchemaKeyNotInObject: Key percent%25field not in schema object at Root#percent%25field`
- `draft6/ref.json::remote ref, containing refs itself` — `SchemaFetchError: Could not fetch schema http://json-schema.org/draft-06/schema, referred to from Root#`
- `draft6/ref.json::$ref to boolean schema false` — `SchemaFalseNotSupported: Schema "false" is not supported at Root#/definitions/bool`
- `draft6/ref.json::refs with quote` — `SchemaKeyNotInObject: Key foo%22bar not in schema object at Root#foo%22bar`
- `draft6/ref.json::naive replacement of $ref with its destination is not correct` — `IRNoEmptyUnions: Trying to make an empty union - do you have an impossible type in your schema?`
- … and 175 more

### 🤪 `mixed` (21)

5 distinct first-line errors; most common:

- **5×** `accepted '[false, false] from items array is not valid' though valid: false`
- **5×** `accepted 'non-present required property is invalid' though valid: false`
- **5×** `accepted 'wrong types' though valid: false`
- **4×** `valid items: data did not match any variant of untagged enum Root`
- **2×** `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`

Per group:

- `draft4/additionalItems.json::items validation adjusts the starting index for additionalItems` — `valid items: data did not match any variant of untagged enum Root`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `draft4/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft4/required.json::required validation` — `accepted 'non-present required property is invalid' though valid: false`
  - cases: non-present required property is invalid (accepted), ignores null (rejected)
- `draft4/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `draft6/additionalItems.json::items validation adjusts the starting index for additionalItems` — `valid items: data did not match any variant of untagged enum Root`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `draft6/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft6/ref.json::$ref prevents a sibling $id from changing the base uri` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `draft6/required.json::required validation` — `accepted 'non-present required property is invalid' though valid: false`
  - cases: non-present required property is invalid (accepted), ignores null (rejected)
- `draft6/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `draft7/additionalItems.json::items validation adjusts the starting index for additionalItems` — `valid items: data did not match any variant of untagged enum Root`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `draft7/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft7/ref.json::$ref prevents a sibling $id from changing the base uri` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `draft7/required.json::required validation` — `accepted 'non-present required property is invalid' though valid: false`
  - cases: non-present required property is invalid (accepted), ignores null (rejected)
- `draft7/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `draft2019-09/additionalItems.json::items validation adjusts the starting index for additionalItems` — `valid items: data did not match any variant of untagged enum Root`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `draft2019-09/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft2019-09/required.json::required validation` — `accepted 'non-present required property is invalid' though valid: false`
  - cases: non-present required property is invalid (accepted), ignores null (rejected)
- `draft2019-09/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `draft2020-12/prefixItems.json::a schema given for prefixItems` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft2020-12/required.json::required validation` — `accepted 'non-present required property is invalid' though valid: false`
  - cases: non-present required property is invalid (accepted), ignores null (rejected)
- `draft2020-12/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)

### 🚧 `false_reject` (47)

11 distinct first-line errors; most common:

- **10×** `null is valid: data did not match any variant of untagged enum Root`
- **5×** `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
- **5×** `one of the enum is valid: data did not match any variant of untagged enum Root`
- **5×** `only the first item is validated: data did not match any variant of untagged enum Root`
- **5×** `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`

Per group:

- `draft4/additionalItems.json::additionalItems are allowed by default` — `only the first item is validated: data did not match any variant of untagged enum Root`
  - cases: only the first item is validated (rejected)
- `draft4/enum.json::heterogeneous enum validation` — `one of the enum is valid: data did not match any variant of untagged enum Root`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft4/enum.json::heterogeneous enum-with-null validation` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft4/type.json::type: array, object or null` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft4/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft6/additionalItems.json::additionalItems are allowed by default` — `only the first item is validated: data did not match any variant of untagged enum Root`
  - cases: only the first item is validated (rejected)
- `draft6/enum.json::heterogeneous enum validation` — `one of the enum is valid: data did not match any variant of untagged enum Root`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft6/enum.json::heterogeneous enum-with-null validation` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft6/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft6/type.json::type: array, object or null` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft6/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft7/additionalItems.json::additionalItems are allowed by default` — `only the first item is validated: data did not match any variant of untagged enum Root`
  - cases: only the first item is validated (rejected)
- `draft7/enum.json::heterogeneous enum validation` — `one of the enum is valid: data did not match any variant of untagged enum Root`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft7/enum.json::heterogeneous enum-with-null validation` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft7/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft7/type.json::type: array, object or null` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft7/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft2019-09/additionalItems.json::additionalItems are allowed by default` — `only the first item is validated: data did not match any variant of untagged enum Root`
  - cases: only the first item is validated (rejected)
- `draft2019-09/enum.json::heterogeneous enum validation` — `one of the enum is valid: data did not match any variant of untagged enum Root`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft2019-09/enum.json::heterogeneous enum-with-null validation` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft2019-09/ref.json::$ref with $recursiveAnchor` — `extra items allowed for inner arrays: data did not match any variant of untagged enum RootElement`
  - cases: extra items allowed for inner arrays (rejected)
- `draft2019-09/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft2019-09/type.json::type: array, object or null` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with items and additionalItems` — `unevaluatedItems doesn't apply: data did not match any variant of untagged enum Root`
  - cases: unevaluatedItems doesn't apply (rejected)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with nested items and additionalItems` — `with additional items: data did not match any variant of untagged enum Root`
  - cases: with additional items (rejected)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems` — `with additional items: data did not match any variant of untagged enum Root`
  - cases: with additional items (rejected)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with if/then/else` — `when if matches and it has no unevaluated items: data did not match any variant of untagged enum Root`
  - cases: when if matches and it has no unevaluated items (rejected), when if doesn't match and it has no unevaluated items (rejected)
- `draft2019-09/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft2020-12/enum.json::heterogeneous enum validation` — `one of the enum is valid: data did not match any variant of untagged enum Root`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft2020-12/enum.json::heterogeneous enum-with-null validation` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft2020-12/prefixItems.json::additional items are allowed by default` — `only the first item is validated: data did not match any variant of untagged enum Root`
  - cases: only the first item is validated (rejected)
- `draft2020-12/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft2020-12/type.json::type: array, object or null` — `null is valid: data did not match any variant of untagged enum Root`
  - cases: null is valid (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with items and prefixItems` — `unevaluatedItems doesn't apply: data did not match any variant of untagged enum Root`
  - cases: unevaluatedItems doesn't apply (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with nested prefixItems and items` — `with additional items: data did not match any variant of untagged enum Root`
  - cases: with additional items (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems` — `with additional items: data did not match any variant of untagged enum Root`
  - cases: with additional items (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with if/then/else` — `when if matches and it has no unevaluated items: data did not match any variant of untagged enum Root`
  - cases: when if matches and it has no unevaluated items (rejected), when if doesn't match and it has no unevaluated items (rejected)
- `draft2020-12/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: data did not match any variant of untagged enum Root`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft4/optional/bignum.json::integer` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft6/optional/bignum.json::integer` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft6/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: invalid type: floating point '1e+308', expected i64`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft7/optional/bignum.json::integer` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft7/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: invalid type: floating point '1e+308', expected i64`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft2019-09/optional/bignum.json::integer` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft2019-09/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: invalid type: floating point '1e+308', expected i64`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft2020-12/optional/bignum.json::integer` — `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft2020-12/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: invalid type: floating point '1e+308', expected i64`
  - cases: valid if optional overflow handling is implemented (rejected)

### 🙈 `false_accept` (816)

175 distinct first-line errors; most common:

- **27×** `accepted 'too long is invalid' though valid: false`
- **27×** `accepted 'too short is invalid' though valid: false`
- **20×** `accepted 'ascii non-digits' though valid: false`
- **20×** `accepted 'literal unicode character in json string' though valid: false`
- **20×** `accepted 'with unevaluated properties' though valid: false`

Per group:

- `draft4/additionalItems.json::additionalItems as schema` — `accepted 'additional items do not match schema' though valid: false`
  - cases: additional items do not match schema (accepted)
- `draft4/additionalItems.json::array of items with no additionalItems permitted` — `accepted 'additional items are not permitted' though valid: false`
  - cases: additional items are not permitted (accepted)
- `draft4/additionalItems.json::additionalItems does not look in applicators, invalid case` — `accepted 'items defined in allOf are not examined' though valid: false`
  - cases: items defined in allOf are not examined (accepted)
- `draft4/additionalItems.json::additionalItems with heterogeneous array` — `accepted 'heterogeneous invalid instance' though valid: false`
  - cases: heterogeneous invalid instance (accepted)
- `draft4/additionalProperties.json::additionalProperties being false does not allow other properties` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted)
- `draft4/additionalProperties.json::non-ASCII pattern with additionalProperties` — `accepted 'not matching the pattern is invalid' though valid: false`
  - cases: not matching the pattern is invalid (accepted)
- `draft4/additionalProperties.json::additionalProperties with schema` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `draft4/allOf.json::allOf with base schema` — `accepted 'mismatch second allOf' though valid: false`
  - cases: mismatch second allOf (accepted)
- `draft4/allOf.json::allOf simple types` — `accepted 'mismatch one' though valid: false`
  - cases: mismatch one (accepted)
- `draft4/allOf.json::nested allOf, to check validation semantics` — `accepted 'anything non-null is invalid' though valid: false`
  - cases: anything non-null is invalid (accepted)
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `accepted 'allOf: false, anyOf: false, oneOf: false' though valid: false`
  - cases: allOf: false, anyOf: false, oneOf: false (accepted), allOf: false, anyOf: false, oneOf: true (accepted), allOf: false, anyOf: true, oneOf: false (accepted), allOf: false, anyOf: true, oneOf: true (accepted), allOf: true, anyOf: false, oneOf: false (accepted), allOf: true, anyOf: false, oneOf: true (accepted), … and 1 more
- `draft4/anyOf.json::anyOf` — `accepted 'neither anyOf valid' though valid: false`
  - cases: neither anyOf valid (accepted)
- `draft4/anyOf.json::anyOf with base schema` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `draft4/anyOf.json::nested anyOf, to check validation semantics` — `accepted 'anything non-null is invalid' though valid: false`
  - cases: anything non-null is invalid (accepted)
- `draft4/default.json::the default keyword does not do anything if the property is missing` — `accepted 'an explicit property value is checked against maximum (failing)' though valid: false`
  - cases: an explicit property value is checked against maximum (failing) (accepted)
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
- `draft4/items.json::items and subitems` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong sub-item (accepted)
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
- `draft4/not.json::not` — `accepted 'disallowed' though valid: false`
  - cases: disallowed (accepted)
- `draft4/not.json::not multiple types` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted), other mismatch (accepted)
- `draft4/not.json::not more complex schema` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted)
- `draft4/not.json::forbidden property` — `accepted 'property present' though valid: false`
  - cases: property present (accepted)
- `draft4/not.json::forbid everything with empty schema` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted), boolean true is invalid (accepted), boolean false is invalid (accepted), null is invalid (accepted), object is invalid (accepted), … and 3 more
- `draft4/oneOf.json::oneOf` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted), neither oneOf valid (accepted)
- `draft4/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft4/oneOf.json::oneOf complex types` — `accepted 'both oneOf valid (complex)' though valid: false`
  - cases: both oneOf valid (complex) (accepted)
- `draft4/oneOf.json::oneOf with empty schema` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `draft4/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- … and 766 more

