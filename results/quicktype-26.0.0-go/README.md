# quicktype 26.0.0 go

quicktype infers types from JSON samples or schemas and emits them in many
languages. Its design center is the shape of data: produce ergonomic,
compilable types everywhere, with serialization glue for each language.
Validation beyond type and shape is out of scope, so constraint keywords
(bounds, lengths, patterns, `const`, `oneOf` exclusivity) are accepted
silently. The TypeScript target adds a runtime type check; the Go target
leans on encoding/json's leniency.

_generated 2026-10-11T18:13:31Z_

**302 of 1408 non-optional groups pass (21.4%)**; 301 optional groups reported separately

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

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 40 | 37 | 3 | 3 | 55 | 22 | 0 | 160 | 25.0% |
| draft6 | 58 | 42 | 3 | 5 | 83 | 39 | 3 | 233 | 24.9% |
| draft7 | 62 | 42 | 3 | 5 | 104 | 39 | 3 | 258 | 24.0% |
| draft2019-09 | 70 | 95 | 7 | 5 | 142 | 51 | 3 | 373 | 18.8% |
| draft2020-12 | 72 | 94 | 6 | 4 | 149 | 56 | 3 | 384 | 18.8% |
| **Total** | **302** | **310** | **22** | **22** | **533** | **207** | **12** | **1408** | **21.4%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 4 | 15 | 1 | 18 | 1 | 39 | 10.3% |
| draft6 | 2 | 15 | 2 | 22 | 3 | 44 | 4.5% |
| draft7 | 2 | 15 | 2 | 43 | 2 | 64 | 3.1% |
| draft2019-09 | 5 | 15 | 2 | 48 | 6 | 76 | 6.6% |
| draft2020-12 | 5 | 15 | 2 | 50 | 6 | 78 | 6.4% |
| **Total** | **18** | **75** | **9** | **181** | **18** | **301** | **6.0%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 227 | 299 | 75.9% | 618 | 36.7% |
| draft6 | 280 | 366 | 76.5% | 841 | 33.3% |
| draft7 | 288 | 374 | 77.0% | 929 | 31.0% |
| draft2019-09 | 380 | 557 | 68.2% | 1261 | 30.1% |
| draft2020-12 | 366 | 532 | 68.8% | 1301 | 28.1% |
| **Total** | **1541** | **2128** | **72.4%** | **4950** | **31.1%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 33 | 64 | 51.6% | 391 | 8.4% |
| draft6 | 31 | 63 | 49.2% | 520 | 6.0% |
| draft7 | 31 | 63 | 49.2% | 922 | 3.4% |
| draft2019-09 | 37 | 69 | 53.6% | 1043 | 3.5% |
| draft2020-12 | 37 | 69 | 53.6% | 1047 | 3.5% |
| **Total** | **169** | **328** | **51.5%** | **3923** | **4.3%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 1 | 4 | 1 | 1 | 2 | 0 | 9 | 🙈✅🙈🦀🚧🙈🤪🙈🦀 |
| additionalProperties.json | 4 | 3 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈✅✅✅✅ |
| allOf.json | 2 | 2 | 0 | 0 | 5 | 0 | 9 | 🙈🙈🦀🦀🦀✅✅🦀🦀 |
| anyOf.json | 1 | 1 | 0 | 0 | 3 | 0 | 5 | 🦀🙈✅🦀🦀 |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| dependencies.json | 0 | 1 | 0 | 0 | 4 | 0 | 5 | 🦀🦀🦀🦀🙈 |
| enum.json | 5 | 6 | 1 | 0 | 0 | 4 | 16 | 🙈🚧✅🙈🙈✅❌✅❌✅❌✅❌🙈🙈🙈 |
| format.json | 0 | 0 | 0 | 0 | 6 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 1 | 0 | 1 | 0 | 0 | 6 | ✅🤪🙈✅✅✅ |
| maxItems.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| maxLength.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| maxProperties.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 4 | 0 | 4 | 🦀🦀🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| minLength.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| minProperties.json | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| minimum.json | 0 | 0 | 0 | 0 | 4 | 0 | 4 | 🦀🦀🦀🦀 |
| multipleOf.json | 2 | 0 | 0 | 0 | 3 | 0 | 5 | 🦀🦀🦀✅✅ |
| not.json | 0 | 1 | 0 | 0 | 5 | 0 | 6 | 🦀🦀🦀🙈🦀🦀 |
| oneOf.json | 0 | 4 | 0 | 0 | 3 | 0 | 7 | 🦀🙈🙈🦀🙈🙈🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| patternProperties.json | 0 | 0 | 0 | 0 | 4 | 0 | 4 | 🦀🦀🦀🦀 |
| properties.json | 4 | 1 | 0 | 0 | 0 | 0 | 5 | ✅🙈✅✅✅ |
| ref.json | 9 | 1 | 0 | 0 | 0 | 9 | 19 | 🙈✅✅❌✅✅❌❌✅✅❌❌❌❌❌❌✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 8 | 8 | ❌❌❌❌❌❌❌❌ |
| required.json | 1 | 3 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| type.json | 4 | 6 | 0 | 0 | 1 | 0 | 11 | 🙈🙈🙈🙈🙈🙈🦀✅✅✅✅ |
| uniqueItems.json | 0 | 2 | 1 | 1 | 2 | 0 | 6 | 🦀🤪🙈🦀🚧🙈 |
| **Total** | **40** | **37** | **3** | **3** | **55** | **22** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 2 | 0 | 1 | 4 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/zeroTerminatedFloats.json | 1 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **4** | **15** | **1** | **18** | **1** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 2 | 4 | 1 | 1 | 2 | 0 | 0 | 10 | 🙈✅✅🙈🦀🚧🙈🤪🙈🦀 |
| additionalProperties.json | 4 | 3 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈✅✅✅✅ |
| allOf.json | 2 | 2 | 0 | 0 | 6 | 2 | 0 | 12 | 🙈🙈🦀🦀❌❌🦀🦀✅✅🦀🦀 |
| anyOf.json | 1 | 1 | 0 | 0 | 4 | 2 | 0 | 8 | 🦀🙈🦀❌❌✅🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| const.json | 4 | 6 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈🙈🙈🙈 |
| contains.json | 0 | 1 | 0 | 0 | 5 | 0 | 0 | 6 | 🦀🦀🦀🦀🙈🦀 |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 0 | 1 | 0 | 0 | 6 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🙈 |
| enum.json | 5 | 5 | 1 | 0 | 0 | 4 | 0 | 15 | 🙈🚧✅🙈🙈✅❌✅❌✅❌✅❌🙈🙈 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format.json | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 5 | 1 | 0 | 1 | 0 | 2 | 0 | 9 | ✅🤪✅❌❌🙈✅✅✅ |
| maxItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxProperties.json | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 🦀🦀🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minProperties.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minimum.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| multipleOf.json | 2 | 0 | 0 | 0 | 3 | 0 | 0 | 5 | 🦀🦀🦀✅✅ |
| not.json | 0 | 1 | 0 | 0 | 7 | 0 | 0 | 8 | 🦀🦀🦀🙈🦀🦀🦀🦀 |
| oneOf.json | 0 | 4 | 0 | 0 | 4 | 3 | 0 | 11 | 🦀🙈🦀❌❌❌🙈🦀🙈🙈🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| patternProperties.json | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| properties.json | 4 | 1 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| ref.json | 20 | 1 | 0 | 1 | 1 | 5 | 3 | 31 | 🙈✅✅❌✅✅🤪❌✅✅🦀❌✅❌✅✅✅❌✅✅🤯✅✅✅✅🤯🤯✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 4 | 5 | 0 | 1 | 1 | 0 | 0 | 11 | 🤪🙈🙈🙈🙈🙈🦀✅✅✅✅ |
| uniqueItems.json | 0 | 2 | 1 | 1 | 2 | 0 | 0 | 6 | 🦀🤪🙈🦀🚧🙈 |
| **Total** | **58** | **42** | **3** | **5** | **83** | **39** | **3** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 2 | 0 | 1 | 4 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri-template.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 3 | 3 | ❌❌❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| **Total** | **2** | **15** | **2** | **22** | **3** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 2 | 4 | 1 | 1 | 2 | 0 | 0 | 10 | 🙈✅✅🙈🦀🚧🙈🤪🙈🦀 |
| additionalProperties.json | 4 | 3 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈✅✅✅✅ |
| allOf.json | 2 | 2 | 0 | 0 | 6 | 2 | 0 | 12 | 🙈🙈🦀🦀❌❌🦀🦀✅✅🦀🦀 |
| anyOf.json | 1 | 1 | 0 | 0 | 4 | 2 | 0 | 8 | 🦀🙈🦀❌❌✅🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| const.json | 4 | 6 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈🙈🙈🙈 |
| contains.json | 0 | 1 | 0 | 0 | 6 | 0 | 0 | 7 | 🦀🦀🦀🦀🙈🦀🦀 |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependencies.json | 0 | 1 | 0 | 0 | 6 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🙈 |
| enum.json | 5 | 5 | 1 | 0 | 0 | 4 | 0 | 15 | 🙈🚧✅🙈🙈✅❌✅❌✅❌✅❌🙈🙈 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format.json | 0 | 0 | 0 | 0 | 17 | 0 | 0 | 17 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| if-then-else.json | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 5 | 1 | 0 | 1 | 0 | 2 | 0 | 9 | ✅🤪✅❌❌🙈✅✅✅ |
| maxItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxProperties.json | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 🦀🦀🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minProperties.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minimum.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| multipleOf.json | 2 | 0 | 0 | 0 | 3 | 0 | 0 | 5 | 🦀🦀🦀✅✅ |
| not.json | 0 | 1 | 0 | 0 | 7 | 0 | 0 | 8 | 🦀🦀🦀🙈🦀🦀🦀🦀 |
| oneOf.json | 0 | 4 | 0 | 0 | 4 | 3 | 0 | 11 | 🦀🙈🦀❌❌❌🙈🦀🙈🙈🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| patternProperties.json | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| properties.json | 4 | 1 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| ref.json | 24 | 1 | 0 | 1 | 1 | 5 | 3 | 35 | 🙈✅✅❌✅✅🤪❌✅✅🦀❌✅❌✅✅✅❌✅✅✅🤯✅✅✅✅🤯🤯✅✅✅✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | ❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 4 | 5 | 0 | 1 | 1 | 0 | 0 | 11 | 🤪🙈🙈🙈🙈🙈🦀✅✅✅✅ |
| uniqueItems.json | 0 | 2 | 1 | 1 | 2 | 0 | 0 | 6 | 🦀🤪🙈🦀🚧🙈 |
| **Total** | **62** | **42** | **3** | **5** | **104** | **39** | **3** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 2 | 0 | 1 | 4 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/content.json | 0 | 0 | 0 | 3 | 0 | 3 | 🦀🦀🦀 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 6 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/format/idn-email.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/iri.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/regex.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/time.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri-template.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 2 | 1 | 3 | ❌🦀🦀 |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| **Total** | **2** | **15** | **2** | **43** | **2** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 2 | 4 | 1 | 1 | 2 | 0 | 0 | 10 | 🙈✅✅🙈🦀🚧🙈🤪🙈🦀 |
| additionalProperties.json | 5 | 4 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈✅✅✅✅✅🙈 |
| allOf.json | 2 | 2 | 0 | 0 | 6 | 2 | 0 | 12 | 🙈🙈🦀🦀❌❌🦀🦀✅✅🦀🦀 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 1 | 1 | 0 | 0 | 4 | 2 | 0 | 8 | 🦀🙈🦀❌❌✅🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| const.json | 4 | 6 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈🙈🙈🙈 |
| contains.json | 0 | 1 | 0 | 0 | 6 | 0 | 0 | 7 | 🦀🦀🦀🦀🙈🦀🦀 |
| content.json | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| dependentSchemas.json | 0 | 1 | 0 | 0 | 3 | 0 | 0 | 4 | 🦀🦀🦀🙈 |
| enum.json | 5 | 5 | 1 | 0 | 0 | 5 | 0 | 16 | 🙈🚧✅🙈🙈✅❌✅❌✅❌✅❌🙈❌🙈 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format.json | 0 | 0 | 0 | 0 | 19 | 0 | 0 | 19 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| if-then-else.json | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 5 | 1 | 0 | 1 | 0 | 2 | 0 | 9 | ✅🤪✅❌❌🙈✅✅✅ |
| maxContains.json | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| maxItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxProperties.json | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 🦀🦀🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minContains.json | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 8 | 🦀🦀🦀🦀🦀🦀🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minProperties.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minimum.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| multipleOf.json | 2 | 0 | 0 | 0 | 3 | 0 | 0 | 5 | 🦀🦀🦀✅✅ |
| not.json | 0 | 1 | 0 | 0 | 8 | 0 | 0 | 9 | 🦀🦀🦀🙈🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 4 | 0 | 0 | 4 | 3 | 0 | 11 | 🦀🙈🦀❌❌❌🙈🦀🙈🙈🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| patternProperties.json | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| properties.json | 4 | 1 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| recursiveRef.json | 1 | 6 | 0 | 0 | 2 | 0 | 0 | 9 | 🙈🙈✅🙈🙈🙈🙈🦀🦀 |
| ref.json | 21 | 3 | 1 | 0 | 2 | 7 | 3 | 37 | 🙈✅✅❌✅🙈❌✅✅🦀❌✅❌🙈❌✅✅✅🦀❌✅🤯✅✅✅✅🤯🤯❌✅✅✅✅✅✅✅🚧 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 4 | 5 | 0 | 1 | 1 | 0 | 0 | 11 | 🤪🙈🙈🙈🙈🙈🦀✅✅✅✅ |
| unevaluatedItems.json | 1 | 11 | 3 | 1 | 10 | 0 | 0 | 26 | 🦀🦀🦀✅🙈🚧🦀🦀🙈🦀🚧🚧🙈🙈🙈🤪🦀🙈🙈🙈🙈🙈🦀🦀🦀🙈 |
| unevaluatedProperties.json | 7 | 33 | 0 | 0 | 4 | 0 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🙈🙈🙈🙈 |
| uniqueItems.json | 0 | 2 | 1 | 1 | 2 | 0 | 0 | 6 | 🦀🤪🙈🦀🚧🙈 |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **70** | **95** | **7** | **5** | **142** | **51** | **3** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| optional/bignum.json | 2 | 0 | 1 | 4 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 2 | 2 | ❌❌ |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 7 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/duration.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 6 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/format/idn-email.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/iri.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/regex.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/time.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri-template.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uuid.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| optional/no-schema.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/refOfUnknownKeyword.json | 3 | 0 | 0 | 0 | 2 | 5 | ✅❌✅✅❌ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| **Total** | **5** | **15** | **2** | **48** | **6** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 5 | 4 | 0 | 0 | 0 | 0 | 0 | 9 | 🙈🙈🙈✅✅✅✅✅🙈 |
| allOf.json | 2 | 2 | 0 | 0 | 6 | 2 | 0 | 12 | 🙈🙈🦀🦀❌❌🦀🦀✅✅🦀🦀 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | ❌❌❌❌ |
| anyOf.json | 1 | 1 | 0 | 0 | 4 | 2 | 0 | 8 | 🦀🙈🦀❌❌✅🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| const.json | 4 | 6 | 0 | 0 | 0 | 7 | 0 | 17 | 🙈❌❌❌✅✅❌❌❌❌✅✅🙈🙈🙈🙈🙈 |
| contains.json | 0 | 1 | 0 | 0 | 6 | 0 | 0 | 7 | 🦀🦀🦀🦀🙈🦀🦀 |
| content.json | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| dependentRequired.json | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| dependentSchemas.json | 0 | 1 | 0 | 0 | 3 | 0 | 0 | 4 | 🦀🦀🦀🙈 |
| dynamicRef.json | 4 | 9 | 0 | 0 | 3 | 5 | 0 | 21 | 🙈🙈🙈🙈🙈🙈✅✅✅🙈✅🦀🦀❌❌❌❌❌🙈🙈🦀 |
| enum.json | 5 | 5 | 1 | 0 | 0 | 5 | 0 | 16 | 🙈🚧✅🙈🙈✅❌✅❌✅❌✅❌🙈❌🙈 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format.json | 0 | 0 | 0 | 0 | 19 | 0 | 0 | 19 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| if-then-else.json | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 5 | 0 | 0 | 0 | 1 | 0 | 10 | ✅✅❌🙈✅🙈🙈🙈🙈✅ |
| maxContains.json | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| maxItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maxProperties.json | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 🦀🦀🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minContains.json | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 8 | 🦀🦀🦀🦀🦀🦀🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minLength.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minProperties.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minimum.json | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| multipleOf.json | 2 | 0 | 0 | 0 | 3 | 0 | 0 | 5 | 🦀🦀🦀✅✅ |
| not.json | 0 | 1 | 0 | 0 | 8 | 0 | 0 | 9 | 🦀🦀🦀🙈🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 4 | 0 | 0 | 4 | 3 | 0 | 11 | 🦀🙈🦀❌❌❌🙈🦀🙈🙈🦀 |
| pattern.json | 0 | 1 | 0 | 0 | 2 | 0 | 0 | 3 | 🦀🦀🙈 |
| patternProperties.json | 1 | 0 | 0 | 0 | 5 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀✅ |
| prefixItems.json | 1 | 0 | 1 | 1 | 0 | 1 | 0 | 4 | 🤪❌🚧✅ |
| properties.json | 4 | 1 | 0 | 0 | 0 | 1 | 0 | 6 | ✅🙈❌✅✅✅ |
| propertyNames.json | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| ref.json | 21 | 3 | 0 | 0 | 2 | 7 | 3 | 36 | 🙈✅✅❌✅🙈❌✅✅🦀❌✅❌🙈❌✅✅✅🦀❌✅🤯✅✅✅✅🤯🤯❌✅✅✅✅✅✅✅ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 15 | 0 | 15 | ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌ |
| required.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈✅✅🙈🙈 |
| type.json | 4 | 5 | 0 | 1 | 1 | 0 | 0 | 11 | 🤪🙈🙈🙈🙈🙈🦀✅✅✅✅ |
| unevaluatedItems.json | 2 | 12 | 3 | 1 | 11 | 0 | 0 | 29 | 🦀🦀🦀✅🙈🚧✅🙈🦀🚧🚧🙈🙈🙈🤪🦀🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🦀🙈 |
| unevaluatedProperties.json | 6 | 27 | 0 | 0 | 11 | 0 | 0 | 44 | 🦀🦀🦀🙈🦀✅✅🙈🙈✅✅🙈🙈🙈🦀🦀🦀🙈🙈🙈🙈🙈🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🙈🙈🙈🙈 |
| uniqueItems.json | 0 | 2 | 1 | 1 | 2 | 0 | 0 | 6 | 🦀🤪🙈🦀🚧🙈 |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **72** | **94** | **6** | **4** | **149** | **56** | **3** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| optional/bignum.json | 2 | 0 | 1 | 4 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 7 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 1 | 🚧 |
| optional/format-assertion.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/format/date-time.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/duration.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 6 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| optional/format/email.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/format/idn-email.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/iri.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/regex.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/time.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri-template.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/format/uuid.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 1 | 1 | ❌ |
| optional/no-schema.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 2 | 0 | 2 | 🦀🦀 |
| optional/refOfUnknownKeyword.json | 3 | 0 | 0 | 0 | 2 | 5 | ✅❌✅✅❌ |
| optional/unknownKeyword.json | 0 | 0 | 0 | 1 | 0 | 1 | 🦀 |
| **Total** | **5** | **15** | **2** | **50** | **6** | **78** |  |

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

### 🦀 `compile_fail` (714)

617 distinct first-line errors; most common:

- **8×** `g_52b6126a3075d4dc/g_52b6126a3075d4dc.go:19:10: invalid receiver type Root (pointer or interface type)`
- **8×** `g_5c57d6c018e6471b/g_5c57d6c018e6471b.go:19:10: invalid receiver type Root (pointer or interface type)`
- **8×** `g_fa90e59e503b70d8/g_fa90e59e503b70d8.go:19:10: invalid receiver type Root (pointer or interface type)`
- **3×** `g_4bfd06b7c3fb0fde/g_4bfd06b7c3fb0fde.go:19:10: invalid receiver type Root (pointer or interface type)`
- **3×** `g_5a3b4523bcab5264/g_5a3b4523bcab5264.go:19:10: invalid receiver type Root (pointer or interface type)`

Per group:

- `draft4/additionalItems.json::additionalItems as false without items` — `g_71f3c700bdbda0f1/g_71f3c700bdbda0f1.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/additionalItems.json::additionalItems with null instance elements` — `g_c5023116d0f1abaf/g_c5023116d0f1abaf.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/allOf.json::allOf simple types` — `g_416197cd38bbfe00/g_416197cd38bbfe00.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/allOf.json::allOf with one empty schema` — `g_11a0a77124e2ea52/g_11a0a77124e2ea52.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/allOf.json::allOf with two empty schemas` — `g_d5650613ba845b38/g_d5650613ba845b38.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/allOf.json::nested allOf, to check validation semantics` — `g_bfb35cbda6298915/g_bfb35cbda6298915.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `g_c05380c867ace78a/g_c05380c867ace78a.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/anyOf.json::anyOf` — `g_5be91225b5e148aa/g_5be91225b5e148aa.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/anyOf.json::anyOf with one empty schema` — `g_13232d8b9b03af37/g_13232d8b9b03af37.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/anyOf.json::nested anyOf, to check validation semantics` — `g_c9776d129d7eeaa6/g_c9776d129d7eeaa6.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/dependencies.json::dependencies` — `g_0cbc5562c15a4109/g_0cbc5562c15a4109.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/dependencies.json::multiple dependencies` — `g_7c8890eafd06a1c9/g_7c8890eafd06a1c9.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/dependencies.json::multiple dependencies subschema` — `g_9ddfd3ce9e774fe0/g_9ddfd3ce9e774fe0.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/dependencies.json::dependencies with escaped characters` — `g_c5087af45bcc59fd/g_c5087af45bcc59fd.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/format.json::email format` — `g_8e3bd08875ce2419/g_8e3bd08875ce2419.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/format.json::ipv4 format` — `g_104a0255f1de0984/g_104a0255f1de0984.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/format.json::ipv6 format` — `g_eab154f3c1e297e8/g_eab154f3c1e297e8.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/format.json::hostname format` — `g_6721f10638cacf96/g_6721f10638cacf96.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/format.json::date-time format` — `g_9d55e3d2b9b956ef/g_9d55e3d2b9b956ef.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/format.json::uri format` — `g_7c24122fa07b5c5e/g_7c24122fa07b5c5e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/maxItems.json::maxItems validation` — `g_78584f27190ab0d4/g_78584f27190ab0d4.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/maxLength.json::maxLength validation` — `g_692d08c92de63fef/g_692d08c92de63fef.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/maxProperties.json::maxProperties validation` — `g_816cbf91e47507c8/g_816cbf91e47507c8.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/maxProperties.json::maxProperties = 0 means the object is empty` — `g_2b7a71540599486e/g_2b7a71540599486e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/maximum.json::maximum validation` — `g_32909173e8045857/g_32909173e8045857.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/maximum.json::maximum validation with unsigned integer` — `g_7862cc1f900ab55f/g_7862cc1f900ab55f.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/maximum.json::maximum validation (explicit false exclusivity)` — `g_3bdc6f12c894a60a/g_3bdc6f12c894a60a.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/maximum.json::exclusiveMaximum validation` — `g_da30bb7ad3512e35/g_da30bb7ad3512e35.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/minItems.json::minItems validation` — `g_60e0497ee18fdee2/g_60e0497ee18fdee2.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/minLength.json::minLength validation` — `g_a18bdf33c74c1027/g_a18bdf33c74c1027.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/minProperties.json::minProperties validation` — `g_6ac7803a845bc656/g_6ac7803a845bc656.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/minimum.json::minimum validation` — `g_78ed24902f632584/g_78ed24902f632584.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/minimum.json::minimum validation (explicit false exclusivity)` — `g_a7b657866c101825/g_a7b657866c101825.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/minimum.json::exclusiveMinimum validation` — `g_18fb28ffac599d0b/g_18fb28ffac599d0b.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/minimum.json::minimum validation with signed integer` — `g_c3bdec30404daf33/g_c3bdec30404daf33.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/multipleOf.json::by int` — `g_da9b1f437dfd46fc/g_da9b1f437dfd46fc.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/multipleOf.json::by number` — `g_0ee0fb6d3b9e555e/g_0ee0fb6d3b9e555e.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/multipleOf.json::by small number` — `g_a702fa0daf534a70/g_a702fa0daf534a70.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/not.json::not` — `g_65a519144f597dd6/g_65a519144f597dd6.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/not.json::not multiple types` — `g_47524b3992cfeb8a/g_47524b3992cfeb8a.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/not.json::not more complex schema` — `g_675ef041affa93cd/g_675ef041affa93cd.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/not.json::forbid everything with empty schema` — `g_b3d543b020b0c094/g_b3d543b020b0c094.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/not.json::double negation` — `g_7f5a83567f496327/g_7f5a83567f496327.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/oneOf.json::oneOf` — `g_5e62f0510ff424c9/g_5e62f0510ff424c9.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/oneOf.json::oneOf with empty schema` — `g_9f05383cb29cd0a5/g_9f05383cb29cd0a5.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/oneOf.json::nested oneOf, to check validation semantics` — `g_778a0f7d867ab360/g_778a0f7d867ab360.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/pattern.json::pattern validation` — `g_72082a02f42050c9/g_72082a02f42050c9.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/pattern.json::pattern is not anchored` — `g_c22022404e08683a/g_c22022404e08683a.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/patternProperties.json::patternProperties validates properties matching a regex` — `g_0c512651b99ededc/g_0c512651b99ededc.go:19:10: invalid receiver type Root (pointer or interface type)`
- `draft4/patternProperties.json::multiple simultaneous patternProperties are validated` — `g_61c20535e0bd3585/g_61c20535e0bd3585.go:19:10: invalid receiver type Root (pointer or interface type)`
- … and 664 more

### 🤪 `mixed` (22)

9 distinct first-line errors; most common:

- **5×** `accepted '[false, false] from items array is not valid' though valid: false`
- **5×** `accepted 'wrong types' though valid: false`
- **4×** `valid items: json: cannot unmarshal number into .1 of type string`
- **2×** `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
- **2×** `accepted 'when if matches and it has unevaluated items' though valid: false`

Per group:

- `draft4/additionalItems.json::items validation adjusts the starting index for additionalItems` — `valid items: json: cannot unmarshal number into .1 of type string`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `draft4/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft4/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `draft6/additionalItems.json::items validation adjusts the starting index for additionalItems` — `valid items: json: cannot unmarshal number into .1 of type string`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `draft6/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft6/ref.json::$ref prevents a sibling $id from changing the base uri` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `draft6/type.json::integer type matches integers` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_fc6dcfb360fd15c0.Root`
  - cases: a float with zero fractional part is an integer (rejected), null is not an integer (accepted)
- `draft6/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `draft7/additionalItems.json::items validation adjusts the starting index for additionalItems` — `valid items: json: cannot unmarshal number into .1 of type string`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `draft7/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft7/ref.json::$ref prevents a sibling $id from changing the base uri` — `accepted '$ref resolves to /definitions/base_foo, data does not validate' though valid: false`
  - cases: $ref resolves to /definitions/base_foo, data does not validate (accepted), $ref resolves to /definitions/base_foo, data validates (rejected)
- `draft7/type.json::integer type matches integers` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_f0da557550a73401.Root`
  - cases: a float with zero fractional part is an integer (rejected), null is not an integer (accepted)
- `draft7/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `draft2019-09/additionalItems.json::items validation adjusts the starting index for additionalItems` — `valid items: json: cannot unmarshal number into .1 of type string`
  - cases: valid items (rejected), wrong type of second item (accepted)
- `draft2019-09/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft2019-09/type.json::integer type matches integers` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_f58790856d4975d2.Root`
  - cases: a float with zero fractional part is an integer (rejected), null is not an integer (accepted)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with if/then/else` — `accepted 'when if matches and it has unevaluated items' though valid: false`
  - cases: when if matches and it has unevaluated items (accepted), when if doesn't match and it has no unevaluated items (rejected)
- `draft2019-09/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)
- `draft2020-12/prefixItems.json::a schema given for prefixItems` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted), array with additional items (rejected)
- `draft2020-12/type.json::integer type matches integers` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_3abf6a12250ace01.Root`
  - cases: a float with zero fractional part is an integer (rejected), null is not an integer (accepted)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with if/then/else` — `accepted 'when if matches and it has unevaluated items' though valid: false`
  - cases: when if matches and it has unevaluated items (accepted), when if doesn't match and it has no unevaluated items (rejected)
- `draft2020-12/uniqueItems.json::uniqueItems with an array of items` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected)

### 🚧 `false_reject` (31)

15 distinct first-line errors; most common:

- **5×** `one of the enum is valid: Union does not contain array`
- **5×** `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
- **5×** `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
- **4×** `with additional items: json: cannot unmarshal number into .1 of type string`
- **2×** `unevaluatedItems doesn't apply: json: cannot unmarshal number into .1 of type string`

Per group:

- `draft4/additionalItems.json::additionalItems are allowed by default` — `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
  - cases: only the first item is validated (rejected)
- `draft4/enum.json::heterogeneous enum validation` — `one of the enum is valid: Union does not contain array`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft4/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft6/additionalItems.json::additionalItems are allowed by default` — `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
  - cases: only the first item is validated (rejected)
- `draft6/enum.json::heterogeneous enum validation` — `one of the enum is valid: Union does not contain array`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft6/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft7/additionalItems.json::additionalItems are allowed by default` — `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
  - cases: only the first item is validated (rejected)
- `draft7/enum.json::heterogeneous enum validation` — `one of the enum is valid: Union does not contain array`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft7/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft2019-09/additionalItems.json::additionalItems are allowed by default` — `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
  - cases: only the first item is validated (rejected)
- `draft2019-09/enum.json::heterogeneous enum validation` — `one of the enum is valid: Union does not contain array`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft2019-09/ref.json::$ref with $recursiveAnchor` — `extra items allowed for inner arrays: Union does not contain number`
  - cases: extra items allowed for inner arrays (rejected)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with items and additionalItems` — `unevaluatedItems doesn't apply: json: cannot unmarshal number into .1 of type string`
  - cases: unevaluatedItems doesn't apply (rejected)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with nested items and additionalItems` — `with additional items: json: cannot unmarshal number into .1 of type string`
  - cases: with additional items (rejected)
- `draft2019-09/unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems` — `with additional items: json: cannot unmarshal number into .1 of type string`
  - cases: with additional items (rejected)
- `draft2019-09/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft2020-12/enum.json::heterogeneous enum validation` — `one of the enum is valid: Union does not contain array`
  - cases: one of the enum is valid (rejected), valid object matches (rejected)
- `draft2020-12/prefixItems.json::additional items are allowed by default` — `only the first item is validated: json: cannot unmarshal string into .1 of type int64`
  - cases: only the first item is validated (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with items and prefixItems` — `unevaluatedItems doesn't apply: json: cannot unmarshal number into .1 of type string`
  - cases: unevaluatedItems doesn't apply (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with nested prefixItems and items` — `with additional items: json: cannot unmarshal number into .1 of type string`
  - cases: with additional items (rejected)
- `draft2020-12/unevaluatedItems.json::unevaluatedItems with nested unevaluatedItems` — `with additional items: json: cannot unmarshal number into .1 of type string`
  - cases: with additional items (rejected)
- `draft2020-12/uniqueItems.json::uniqueItems=false with an array of items` — `unique array extended from [false, true] is valid: json: cannot unmarshal string into .2 of type bool`
  - cases: unique array extended from [false, true] is valid (rejected), unique array extended from [true, false] is valid (rejected), non-unique array extended from [false, true] is valid (rejected), non-unique array extended from [true, false] is valid (rejected)
- `draft4/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_901944f879b9675b.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft6/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_fc6dcfb360fd15c0.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft6/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: json: cannot unmarshal number 1e+308 into Go value of type g_8aab9390429b3f7b.Root`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft7/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_f0da557550a73401.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft7/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: json: cannot unmarshal number 1e+308 into Go value of type g_f9faff6b210bd3f9.Root`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft2019-09/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_f58790856d4975d2.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft2019-09/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: json: cannot unmarshal number 1e+308 into Go value of type g_dd888e7211595b2a.Root`
  - cases: valid if optional overflow handling is implemented (rejected)
- `draft2020-12/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_3abf6a12250ace01.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft2020-12/optional/float-overflow.json::all integers are multiples of 0.5, if overflow is handled` — `valid if optional overflow handling is implemented: json: cannot unmarshal number 1e+308 into Go value of type g_1cb87ac0db86bbfd.Root`
  - cases: valid if optional overflow handling is implemented (rejected)

### 🙈 `false_accept` (385)

84 distinct first-line errors; most common:

- **18×** `accepted 'with unevaluated properties' though valid: false`
- **15×** `accepted 'does not match' though valid: false`
- **14×** `accepted 'with unevaluated items' though valid: false`
- **10×** `accepted 'ascii non-digits' though valid: false`
- **10×** `accepted 'both oneOf valid' though valid: false`

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
- `draft4/allOf.json::allOf` — `accepted 'mismatch second' though valid: false`
  - cases: mismatch second (accepted), mismatch first (accepted)
- `draft4/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch first allOf (accepted), mismatch second allOf (accepted), mismatch both (accepted)
- `draft4/anyOf.json::anyOf with base schema` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `draft4/default.json::the default keyword does not do anything if the property is missing` — `accepted 'an explicit property value is checked against maximum (failing)' though valid: false`
  - cases: an explicit property value is checked against maximum (failing) (accepted)
- `draft4/dependencies.json::dependent subschema incompatible with root` — `accepted 'matches root' though valid: false`
  - cases: matches root (accepted), matches both (accepted)
- `draft4/enum.json::simple enum validation` — `accepted 'something else is invalid' though valid: false`
  - cases: something else is invalid (accepted)
- `draft4/enum.json::enums in properties` — `accepted 'wrong foo value' though valid: false`
  - cases: wrong foo value (accepted), wrong bar value (accepted), missing required property is invalid (accepted), missing all properties is invalid (accepted)
- `draft4/enum.json::enum with escaped characters` — `accepted 'another string is invalid' though valid: false`
  - cases: another string is invalid (accepted)
- `draft4/enum.json::nul characters in strings` — `accepted 'do not match string lacking nul' though valid: false`
  - cases: do not match string lacking nul (accepted)
- `draft4/enum.json::characters with the same visual representation but different codepoint` — `accepted 'character looks the same but uses a different codepoint' though valid: false`
  - cases: character looks the same but uses a different codepoint (accepted)
- `draft4/enum.json::characters with the same visual representation, but different number of codepoints` — `accepted 'character looks the same but uses combining marks' though valid: false`
  - cases: character looks the same but uses combining marks (accepted)
- `draft4/items.json::items and subitems` — `accepted 'too many items' though valid: false`
  - cases: too many items (accepted), too many sub-items (accepted), wrong sub-item (accepted)
- `draft4/not.json::forbidden property` — `accepted 'property present' though valid: false`
  - cases: property present (accepted)
- `draft4/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft4/oneOf.json::oneOf complex types` — `accepted 'both oneOf valid (complex)' though valid: false`
  - cases: both oneOf valid (complex) (accepted)
- `draft4/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft4/oneOf.json::oneOf with missing optional property` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted), neither oneOf valid (accepted)
- `draft4/properties.json::properties, patternProperties, additionalProperties interaction` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty invalidates nonproperty (accepted)
- `draft4/ref.json::root pointer ref` — `accepted 'mismatch' though valid: false`
  - cases: mismatch (accepted), recursive mismatch (accepted)
- `draft4/required.json::required validation` — `accepted 'non-present required property is invalid' though valid: false`
  - cases: non-present required property is invalid (accepted)
- `draft4/required.json::required with escaped characters` — `accepted 'object with some properties missing is invalid' though valid: false`
  - cases: object with some properties missing is invalid (accepted)
- `draft4/required.json::required properties whose names are Javascript object property names` — `accepted 'none of the properties mentioned' though valid: false`
  - cases: none of the properties mentioned (accepted), __proto__ present (accepted), toString present (accepted), constructor present (accepted)
- `draft4/type.json::integer type matches integers` — `accepted 'null is not an integer' though valid: false`
  - cases: null is not an integer (accepted)
- `draft4/type.json::number type matches numbers` — `accepted 'null is not a number' though valid: false`
  - cases: null is not a number (accepted)
- `draft4/type.json::string type matches strings` — `accepted 'null is not a string' though valid: false`
  - cases: null is not a string (accepted)
- `draft4/type.json::object type matches objects` — `accepted 'null is not an object' though valid: false`
  - cases: null is not an object (accepted)
- `draft4/type.json::array type matches arrays` — `accepted 'null is not an array' though valid: false`
  - cases: null is not an array (accepted)
- `draft4/type.json::boolean type matches booleans` — `accepted 'null is not a boolean' though valid: false`
  - cases: null is not a boolean (accepted)
- `draft4/uniqueItems.json::uniqueItems with an array of items and additionalItems=false` — `accepted '[false, false] from items array is not valid' though valid: false`
  - cases: [false, false] from items array is not valid (accepted), [true, true] from items array is not valid (accepted), extra items are invalid even if unique (accepted)
- `draft4/uniqueItems.json::uniqueItems=false with an array of items and additionalItems=false` — `accepted 'extra items are invalid even if unique' though valid: false`
  - cases: extra items are invalid even if unique (accepted)
- `draft6/additionalItems.json::additionalItems as schema` — `accepted 'additional items do not match schema' though valid: false`
  - cases: additional items do not match schema (accepted)
- `draft6/additionalItems.json::array of items with no additionalItems permitted` — `accepted 'additional items are not permitted' though valid: false`
  - cases: additional items are not permitted (accepted)
- `draft6/additionalItems.json::additionalItems does not look in applicators, invalid case` — `accepted 'items defined in allOf are not examined' though valid: false`
  - cases: items defined in allOf are not examined (accepted)
- `draft6/additionalItems.json::additionalItems with heterogeneous array` — `accepted 'heterogeneous invalid instance' though valid: false`
  - cases: heterogeneous invalid instance (accepted)
- `draft6/additionalProperties.json::additionalProperties being false does not allow other properties` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted)
- `draft6/additionalProperties.json::non-ASCII pattern with additionalProperties` — `accepted 'not matching the pattern is invalid' though valid: false`
  - cases: not matching the pattern is invalid (accepted)
- `draft6/additionalProperties.json::additionalProperties with schema` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `draft6/allOf.json::allOf` — `accepted 'mismatch second' though valid: false`
  - cases: mismatch second (accepted), mismatch first (accepted)
- `draft6/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), mismatch first allOf (accepted), mismatch second allOf (accepted), mismatch both (accepted)
- `draft6/anyOf.json::anyOf with base schema` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `draft6/const.json::const validation` — `accepted 'another value is invalid' though valid: false`
  - cases: another value is invalid (accepted)
- `draft6/const.json::const with -2.0 matches integer and float types` — `accepted 'integer 2 is invalid' though valid: false`
  - cases: integer 2 is invalid (accepted), float 2.0 is invalid (accepted), float -2.00001 is invalid (accepted)
- `draft6/const.json::float and integers are equal up to 64-bit representation limits` — `accepted 'integer minus one is invalid' though valid: false`
  - cases: integer minus one is invalid (accepted), float minus one is invalid (accepted)
- … and 335 more

