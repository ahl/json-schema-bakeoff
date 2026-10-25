# go-jsonschema v0.24.1

Go structs from JSON Schema, plus hand-written `UnmarshalJSON` methods that
re-check the schema's keywords while decoding. Its design center is a config
file with a known shape: a root that declares `type: object` becomes a tagged
struct, and `required`, `enum`, lengths, patterns and bounds are enforced in
the unmarshaller rather than left to the type system. The flip side is that a
root with no `type` keyword produces no root type at all, and the combinators
collapse into a merged type or `interface{}`.

_generated 2026-10-25T14:54:23Z_

**27 of 1408 non-optional groups pass (1.9%)**; 301 optional groups reported separately

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
| draft4 | 3 | 11 | 1 | 2 | 0 | 121 | 22 | 0 | 160 | 1.9% |
| draft6 | 3 | 10 | 1 | 3 | 0 | 172 | 44 | 0 | 233 | 1.3% |
| draft7 | 3 | 10 | 1 | 3 | 0 | 185 | 56 | 0 | 258 | 1.2% |
| draft2019-09 | 11 | 31 | 1 | 3 | 0 | 281 | 46 | 0 | 373 | 2.9% |
| draft2020-12 | 7 | 14 | 1 | 3 | 0 | 339 | 20 | 0 | 384 | 1.8% |
| **Total** | **27** | **76** | **5** | **14** | **0** | **1098** | **188** | **0** | **1408** | **1.9%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 9 | 6 | 4 | 0 | 0 | 19 | 1 | 0 | 39 | 23.1% |
| draft6 | 8 | 6 | 4 | 0 | 0 | 25 | 1 | 0 | 44 | 18.2% |
| draft7 | 8 | 7 | 4 | 0 | 0 | 39 | 6 | 0 | 64 | 12.5% |
| draft2019-09 | 9 | 6 | 4 | 0 | 0 | 54 | 3 | 0 | 76 | 11.8% |
| draft2020-12 | 8 | 6 | 4 | 0 | 0 | 56 | 4 | 0 | 78 | 10.3% |
| **Total** | **42** | **31** | **20** | **0** | **0** | **193** | **15** | **0** | **301** | **14.0%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 75 | 101 | 74.3% | 618 | 12.1% |
| draft6 | 75 | 102 | 73.5% | 841 | 8.9% |
| draft7 | 75 | 102 | 73.5% | 929 | 8.1% |
| draft2019-09 | 113 | 170 | 66.5% | 1261 | 9.0% |
| draft2020-12 | 90 | 124 | 72.6% | 1302 | 6.9% |
| **Total** | **428** | **599** | **71.5%** | **4951** | **8.6%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 40 | 63 | 63.5% | 394 | 10.2% |
| draft6 | 39 | 62 | 62.9% | 523 | 7.5% |
| draft7 | 40 | 64 | 62.5% | 933 | 4.3% |
| draft2019-09 | 40 | 63 | 63.5% | 1054 | 3.8% |
| draft2020-12 | 39 | 62 | 62.9% | 1058 | 3.7% |
| **Total** | **198** | **314** | **63.1%** | **3962** | **5.0%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 0 | 3 | 6 | 0 | 9 | ❌🦀❌🦀❌❌❌❌🦀 |
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| allOf.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| anyOf.json | 0 | 1 | 0 | 0 | 0 | 4 | 0 | 0 | 5 | 🦀🙈🦀🦀🦀 |
| default.json | 0 | 0 | 1 | 0 | 0 | 2 | 0 | 0 | 3 | 🦀🦀🚧 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| dependencies.json | 0 | 0 | 0 | 0 | 0 | 1 | 4 | 0 | 5 | ❌❌❌❌🦀 |
| enum.json | 1 | 0 | 0 | 0 | 0 | 15 | 0 | 0 | 16 | 🦀🦀🦀✅🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| format.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 1 | 0 | 0 | 0 | 0 | 2 | 3 | 0 | 6 | 🦀❌❌✅🦀❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| multipleOf.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| not.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 2 | 0 | 0 | 0 | 5 | 0 | 0 | 7 | 🦀🙈🦀🦀🙈🦀🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| patternProperties.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| properties.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| ref.json | 0 | 0 | 0 | 0 | 0 | 17 | 2 | 0 | 19 | 🦀🦀❌🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 5 | 3 | 0 | 8 | 🦀🦀🦀🦀❌❌❌🦀 |
| required.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| type.json | 1 | 8 | 0 | 2 | 0 | 0 | 0 | 0 | 11 | 🙈🙈🙈🙈🙈🙈🙈🤪✅🤪🙈 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 2 | 4 | 0 | 6 | 🦀❌❌🦀❌❌ |
| **Total** | **3** | **11** | **1** | **2** | **0** | **121** | **22** | **0** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 2 | 0 | 1 | 0 | 0 | 4 | 0 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/ecmascript-regex.json | 6 | 6 | 3 | 0 | 0 | 5 | 0 | 0 | 20 | ✅✅🚧🚧✅✅✅✅🚧🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/zeroTerminatedFloats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **9** | **6** | **4** | **0** | **0** | **19** | **1** | **0** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 0 | 4 | 6 | 0 | 10 | ❌🦀🦀❌🦀❌❌❌❌🦀 |
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| allOf.json | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| anyOf.json | 0 | 1 | 0 | 0 | 0 | 7 | 0 | 0 | 8 | 🦀🙈🦀🦀🦀🦀🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 0 | 17 | 0 | 0 | 17 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| contains.json | 0 | 0 | 0 | 0 | 0 | 1 | 5 | 0 | 6 | ❌❌❌❌🦀❌ |
| default.json | 0 | 0 | 1 | 0 | 0 | 2 | 0 | 0 | 3 | 🦀🦀🚧 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| dependencies.json | 0 | 0 | 0 | 0 | 0 | 1 | 6 | 0 | 7 | ❌❌❌❌❌❌🦀 |
| enum.json | 1 | 0 | 0 | 0 | 0 | 14 | 0 | 0 | 15 | 🦀🦀🦀✅🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 0 | 9 | 🦀❌🦀🦀❌❌✅🦀❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 2 | 1 | 0 | 3 | 🦀❌🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| multipleOf.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| not.json | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 8 | 🦀🦀🦀🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 2 | 0 | 0 | 0 | 9 | 0 | 0 | 11 | 🦀🙈🦀🦀🦀🦀🦀🦀🙈🦀🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| patternProperties.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| properties.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| ref.json | 0 | 0 | 0 | 0 | 0 | 29 | 2 | 0 | 31 | 🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 8 | 3 | 0 | 11 | 🦀🦀🦀🦀❌❌❌🦀🦀🦀🦀 |
| required.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| type.json | 1 | 7 | 0 | 3 | 0 | 0 | 0 | 0 | 11 | 🤪🙈🙈🙈🙈🙈🙈🤪✅🤪🙈 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 2 | 4 | 0 | 6 | 🦀❌❌🦀❌❌ |
| **Total** | **3** | **10** | **1** | **3** | **0** | **172** | **44** | **0** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 2 | 0 | 1 | 0 | 0 | 4 | 0 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/ecmascript-regex.json | 6 | 6 | 3 | 0 | 0 | 5 | 0 | 0 | 20 | ✅✅🚧🚧✅✅✅✅🚧🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 2 | 1 | 0 | 3 | ❌🦀🦀 |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| **Total** | **8** | **6** | **4** | **0** | **0** | **25** | **1** | **0** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 0 | 4 | 6 | 0 | 10 | ❌🦀🦀❌🦀❌❌❌❌🦀 |
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| allOf.json | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| anyOf.json | 0 | 1 | 0 | 0 | 0 | 7 | 0 | 0 | 8 | 🦀🙈🦀🦀🦀🦀🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 0 | 17 | 0 | 0 | 17 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| contains.json | 0 | 0 | 0 | 0 | 0 | 1 | 6 | 0 | 7 | ❌❌❌❌🦀❌❌ |
| default.json | 0 | 0 | 1 | 0 | 0 | 2 | 0 | 0 | 3 | 🦀🦀🚧 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| dependencies.json | 0 | 0 | 0 | 0 | 0 | 1 | 6 | 0 | 7 | ❌❌❌❌❌❌🦀 |
| enum.json | 1 | 0 | 0 | 0 | 0 | 14 | 0 | 0 | 15 | 🦀🦀🦀✅🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format.json | 0 | 0 | 0 | 0 | 0 | 17 | 0 | 0 | 17 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 1 | 11 | 0 | 12 | ❌❌❌❌❌❌🦀❌❌❌❌❌ |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 0 | 9 | 🦀❌🦀🦀❌❌✅🦀❌ |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 2 | 1 | 0 | 3 | 🦀❌🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| multipleOf.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| not.json | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 8 | 🦀🦀🦀🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 2 | 0 | 0 | 0 | 9 | 0 | 0 | 11 | 🦀🙈🦀🦀🦀🦀🦀🦀🙈🦀🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| patternProperties.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| properties.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 6 | ❌❌❌❌❌❌ |
| ref.json | 0 | 0 | 0 | 0 | 0 | 33 | 2 | 0 | 35 | 🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 8 | 3 | 0 | 11 | 🦀🦀🦀🦀❌❌❌🦀🦀🦀🦀 |
| required.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| type.json | 1 | 7 | 0 | 3 | 0 | 0 | 0 | 0 | 11 | 🤪🙈🙈🙈🙈🙈🙈🤪✅🤪🙈 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 2 | 4 | 0 | 6 | 🦀❌❌🦀❌❌ |
| **Total** | **3** | **10** | **1** | **3** | **0** | **185** | **56** | **0** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 2 | 0 | 1 | 0 | 0 | 4 | 0 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/content.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/cross-draft.json | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/ecmascript-regex.json | 6 | 6 | 3 | 0 | 0 | 5 | 0 | 0 | 20 | ✅✅🚧🚧✅✅✅✅🚧🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format/idn-email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/regex.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 3 | ❌❌❌ |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| **Total** | **8** | **7** | **4** | **0** | **0** | **39** | **6** | **0** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 0 | 0 | 0 | 0 | 0 | 4 | 6 | 0 | 10 | ❌🦀🦀❌🦀❌❌❌❌🦀 |
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| allOf.json | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| anyOf.json | 0 | 1 | 0 | 0 | 0 | 7 | 0 | 0 | 8 | 🦀🙈🦀🦀🦀🦀🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 0 | 17 | 0 | 0 | 17 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| contains.json | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| content.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| default.json | 0 | 0 | 1 | 0 | 0 | 2 | 0 | 0 | 3 | 🦀🦀🚧 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| dependentRequired.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| dependentSchemas.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| enum.json | 1 | 0 | 0 | 0 | 0 | 15 | 0 | 0 | 16 | 🦀🦀🦀✅🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format.json | 0 | 0 | 0 | 0 | 0 | 19 | 0 | 0 | 19 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 0 | 9 | 🦀❌🦀🦀❌❌✅🦀❌ |
| maxContains.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 2 | 1 | 0 | 3 | 🦀❌🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minContains.json | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 8 | 🦀🦀🦀🦀🦀🦀🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| multipleOf.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| not.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 2 | 0 | 0 | 0 | 9 | 0 | 0 | 11 | 🦀🙈🦀🦀🦀🦀🦀🦀🙈🦀🦀 |
| pattern.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| patternProperties.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| properties.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| recursiveRef.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| ref.json | 0 | 0 | 0 | 0 | 0 | 32 | 5 | 0 | 37 | 🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀❌ |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 12 | 3 | 0 | 15 | 🦀🦀🦀🦀🦀❌❌❌🦀🦀🦀🦀🦀🦀🦀 |
| required.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| type.json | 1 | 7 | 0 | 3 | 0 | 0 | 0 | 0 | 11 | 🤪🙈🙈🙈🙈🙈🙈🤪✅🤪🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 11 | 15 | 0 | 26 | 🦀🦀🦀🦀❌❌🦀🦀❌🦀❌❌❌❌❌❌🦀❌❌❌❌❌🦀🦀🦀❌ |
| unevaluatedProperties.json | 7 | 21 | 0 | 0 | 0 | 15 | 1 | 0 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈🙈🦀🦀🦀🦀🦀✅✅🙈🙈🙈🙈🙈🙈🙈❌🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 2 | 4 | 0 | 6 | 🦀❌❌🦀❌❌ |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 2 | 🦀✅ |
| **Total** | **11** | **31** | **1** | **3** | **0** | **281** | **46** | **0** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 2 | 0 | 1 | 0 | 0 | 4 | 0 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/cross-draft.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| optional/ecmascript-regex.json | 6 | 6 | 3 | 0 | 0 | 5 | 0 | 0 | 20 | ✅✅🚧🚧✅✅✅✅🚧🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/duration.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format/idn-email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/regex.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uuid.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/no-schema.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/refOfUnknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| **Total** | **9** | **6** | **4** | **0** | **0** | **54** | **3** | **0** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| allOf.json | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| anyOf.json | 0 | 1 | 0 | 0 | 0 | 7 | 0 | 0 | 8 | 🦀🙈🦀🦀🦀🦀🦀🦀 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 0 | 0 | 0 | 0 | 17 | 0 | 0 | 17 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| contains.json | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| content.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| default.json | 0 | 0 | 1 | 0 | 0 | 2 | 0 | 0 | 3 | 🦀🦀🚧 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| dependentRequired.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| dependentSchemas.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| dynamicRef.json | 0 | 2 | 0 | 0 | 0 | 13 | 6 | 0 | 21 | 🙈🙈❌🦀🦀❌🦀🦀🦀❌❌❌🦀🦀🦀🦀🦀🦀🦀❌🦀 |
| enum.json | 1 | 0 | 0 | 0 | 0 | 15 | 0 | 0 | 16 | 🦀🦀🦀✅🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| format.json | 0 | 0 | 0 | 0 | 0 | 19 | 0 | 0 | 19 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 12 | 0 | 0 | 12 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 1 | 1 | 0 | 0 | 0 | 8 | 0 | 0 | 10 | 🦀🦀🦀🙈✅🦀🦀🦀🦀🦀 |
| maxContains.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| maxItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| maxProperties.json | 0 | 0 | 0 | 0 | 0 | 2 | 1 | 0 | 3 | 🦀❌🦀 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| minContains.json | 0 | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 8 | 🦀🦀🦀🦀🦀🦀🦀🦀 |
| minItems.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minLength.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minProperties.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🦀❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| multipleOf.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| not.json | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 9 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| oneOf.json | 0 | 2 | 0 | 0 | 0 | 9 | 0 | 0 | 11 | 🦀🙈🦀🦀🦀🦀🦀🦀🙈🦀🦀 |
| pattern.json | 1 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 3 | 🦀🦀✅ |
| patternProperties.json | 0 | 1 | 0 | 0 | 0 | 5 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🙈 |
| prefixItems.json | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 | 🦀🦀🦀🦀 |
| properties.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| propertyNames.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| ref.json | 0 | 0 | 0 | 0 | 0 | 33 | 3 | 0 | 36 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀❌🦀🦀🦀🦀🦀🦀🦀 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 12 | 3 | 0 | 15 | 🦀🦀🦀🦀🦀❌❌❌🦀🦀🦀🦀🦀🦀🦀 |
| required.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| type.json | 1 | 7 | 0 | 3 | 0 | 0 | 0 | 0 | 11 | 🤪🙈🙈🙈🙈🙈🙈🤪✅🤪🙈 |
| unevaluatedItems.json | 0 | 0 | 0 | 0 | 0 | 29 | 0 | 0 | 29 | 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| unevaluatedProperties.json | 2 | 0 | 0 | 0 | 0 | 42 | 0 | 0 | 44 | 🦀🦀🦀🦀🦀✅✅🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀 |
| uniqueItems.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 2 | 🦀✅ |
| **Total** | **7** | **14** | **1** | **3** | **0** | **339** | **20** | **0** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/bignum.json | 2 | 0 | 1 | 0 | 0 | 4 | 0 | 0 | 7 | 🚧✅✅🦀🦀🦀🦀 |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/dependencies-compatibility.json | 0 | 0 | 0 | 0 | 0 | 7 | 0 | 0 | 7 | 🦀🦀🦀🦀🦀🦀🦀 |
| optional/dynamicRef.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 6 | 6 | 3 | 0 | 0 | 5 | 0 | 0 | 20 | ✅✅🚧🚧✅✅✅✅🚧🙈🦀🦀🦀🦀🦀🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format-assertion.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format/date-time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/date.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/duration.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ecmascript-regex.json | 0 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 6 | 🦀🦀🦀🦀🦀🦀 |
| optional/format/email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format/idn-email.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/idn-hostname.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/format/ipv4.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/ipv6.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/iri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/regex.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/relative-json-pointer.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/time.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/unknown.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri-reference.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri-template.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uri.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/format/uuid.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/no-schema.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/non-bmp-regex.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 0 | 2 | 🦀🦀 |
| optional/refOfUnknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 5 | 0 | 0 | 5 | 🦀🦀🦀🦀🦀 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| **Total** | **8** | **6** | **4** | **0** | **0** | **56** | **4** | **0** | **78** |  |

## Failures

### ❌ `generation_error` (203)

25 distinct first-line errors; most common:

- **107×** `error parsing from file <schema>: cannot load schema`
- **55×** `schema has no root`
- **5×** `cannot add struct field: could not generate type with scope 'NodeSubtree': could not follow $ref "tree" to file "tree": cannot load schema`
- **5×** `could not follow $ref "folderInteger.json" to file "folderInteger.json": cannot load schema`
- **3×** `cannot add struct field: could not generate type with scope 'RootList': definition does not exist in schema: "baz/definitions/bar" (from ref "#/definitions/baz/…`

Per group:

- `draft4/additionalItems.json::additionalItems as schema` — `error parsing from file <schema>: cannot load schema`
- `draft4/additionalItems.json::array of items with no additionalItems permitted` — `error parsing from file <schema>: cannot load schema`
- `draft4/additionalItems.json::additionalItems are allowed by default` — `error parsing from file <schema>: cannot load schema`
- `draft4/additionalItems.json::additionalItems does not look in applicators, invalid case` — `error parsing from file <schema>: cannot load schema`
- `draft4/additionalItems.json::items validation adjusts the starting index for additionalItems` — `error parsing from file <schema>: cannot load schema`
- `draft4/additionalItems.json::additionalItems with heterogeneous array` — `error parsing from file <schema>: cannot load schema`
- `draft4/dependencies.json::dependencies` — `schema has no root`
- `draft4/dependencies.json::multiple dependencies` — `schema has no root`
- `draft4/dependencies.json::multiple dependencies subschema` — `schema has no root`
- `draft4/dependencies.json::dependencies with escaped characters` — `schema has no root`
- `draft4/items.json::an array of schemas for items` — `error parsing from file <schema>: cannot load schema`
- `draft4/items.json::items and subitems` — `error parsing from file <schema>: cannot load schema`
- `draft4/items.json::array-form items with null instance elements` — `error parsing from file <schema>: cannot load schema`
- `draft4/ref.json::relative pointer ref to array` — `error parsing from file <schema>: cannot load schema`
- `draft4/ref.json::Recursive references between schemas` — `cannot add struct field: could not generate type with scope 'NodeSubtree': could not follow $ref "tree" to file "tree": cannot load schema`
- `draft4/refRemote.json::base URI change - change folder` — `could not follow $ref "folderInteger.json" to file "folderInteger.json": cannot load schema`
- `draft4/refRemote.json::base URI change - change folder in subschema` — `cannot add struct field: could not generate type with scope 'RootList': definition does not exist in schema: "baz/definitions/bar" (from ref "#/definitions/baz/…`
- `draft4/refRemote.json::root ref in remote ref` — `cannot add struct field: could not generate type with scope 'RootName': could not follow $ref "draft4/name.json#/definitions/orNull" to file "draft4/name.json":…`
- `draft4/uniqueItems.json::uniqueItems with an array of items` — `error parsing from file <schema>: cannot load schema`
- `draft4/uniqueItems.json::uniqueItems with an array of items and additionalItems=false` — `error parsing from file <schema>: cannot load schema`
- `draft4/uniqueItems.json::uniqueItems=false with an array of items` — `error parsing from file <schema>: cannot load schema`
- `draft4/uniqueItems.json::uniqueItems=false with an array of items and additionalItems=false` — `error parsing from file <schema>: cannot load schema`
- `draft6/additionalItems.json::additionalItems as schema` — `error parsing from file <schema>: cannot load schema`
- `draft6/additionalItems.json::array of items with no additionalItems permitted` — `error parsing from file <schema>: cannot load schema`
- `draft6/additionalItems.json::additionalItems are allowed by default` — `error parsing from file <schema>: cannot load schema`
- `draft6/additionalItems.json::additionalItems does not look in applicators, invalid case` — `error parsing from file <schema>: cannot load schema`
- `draft6/additionalItems.json::items validation adjusts the starting index for additionalItems` — `error parsing from file <schema>: cannot load schema`
- `draft6/additionalItems.json::additionalItems with heterogeneous array` — `error parsing from file <schema>: cannot load schema`
- `draft6/boolean_schema.json::boolean schema 'true'` — `error parsing from file <schema>: cannot load schema`
- `draft6/boolean_schema.json::boolean schema 'false'` — `error parsing from file <schema>: cannot load schema`
- `draft6/contains.json::contains keyword validation` — `schema has no root`
- `draft6/contains.json::contains keyword with const keyword` — `schema has no root`
- `draft6/contains.json::contains keyword with boolean schema true` — `schema has no root`
- `draft6/contains.json::contains keyword with boolean schema false` — `schema has no root`
- `draft6/contains.json::contains with null instance elements` — `schema has no root`
- `draft6/dependencies.json::dependencies` — `schema has no root`
- `draft6/dependencies.json::dependencies with empty array` — `schema has no root`
- `draft6/dependencies.json::multiple dependencies` — `schema has no root`
- `draft6/dependencies.json::multiple dependencies subschema` — `schema has no root`
- `draft6/dependencies.json::dependencies with boolean subschemas` — `schema has no root`
- `draft6/dependencies.json::dependencies with escaped characters` — `schema has no root`
- `draft6/items.json::an array of schemas for items` — `error parsing from file <schema>: cannot load schema`
- `draft6/items.json::items with boolean schemas` — `error parsing from file <schema>: cannot load schema`
- `draft6/items.json::items and subitems` — `error parsing from file <schema>: cannot load schema`
- `draft6/items.json::array-form items with null instance elements` — `error parsing from file <schema>: cannot load schema`
- `draft6/maxItems.json::maxItems validation with a decimal` — `error parsing from file <schema>: cannot load schema`
- `draft6/maxLength.json::maxLength validation with a decimal` — `error parsing from file <schema>: cannot load schema`
- `draft6/maxProperties.json::maxProperties validation with a decimal` — `error parsing from file <schema>: cannot load schema`
- `draft6/minItems.json::minItems validation with a decimal` — `error parsing from file <schema>: cannot load schema`
- `draft6/minLength.json::minLength validation with a decimal` — `error parsing from file <schema>: cannot load schema`
- … and 153 more

### 🦀 `compile_fail` (1291)

1186 distinct first-line errors; most common:

- **8×** `g_d0b2c179917d66d6/g_d0b2c179917d66d6.go:9:8: undefined: Root`
- **8×** `g_debdfb0bc8ba4bfa/g_debdfb0bc8ba4bfa.go:9:8: undefined: Root`
- **8×** `g_f03a36338525c4d2/g_f03a36338525c4d2.go:9:8: undefined: Root`
- **3×** `g_32ae7c063d0757cc/g_32ae7c063d0757cc.go:9:8: undefined: Root`
- **3×** `g_3464ec40053b8e9f/g_3464ec40053b8e9f.go:9:8: undefined: Root`

Per group:

- `draft4/additionalItems.json::when items is schema, additionalItems does nothing` — `g_2ae117931cbed77e/g_2ae117931cbed77e.go:9:8: undefined: Root`
- `draft4/additionalItems.json::additionalItems as false without items` — `g_91752a5959f55a1a/g_91752a5959f55a1a.go:9:8: undefined: Root`
- `draft4/additionalItems.json::additionalItems with null instance elements` — `g_9cf72fcf495300ee/g_9cf72fcf495300ee.go:9:8: undefined: Root`
- `draft4/additionalProperties.json::additionalProperties being false does not allow other properties` — `g_bd5bcfbc6f8dfe9b/g_bd5bcfbc6f8dfe9b.go:9:8: undefined: Root`
- `draft4/additionalProperties.json::non-ASCII pattern with additionalProperties` — `g_fe0eb6d89a42d07c/g_fe0eb6d89a42d07c.go:9:8: undefined: Root`
- `draft4/additionalProperties.json::additionalProperties with schema` — `g_9c9c4578cb05e82e/g_9c9c4578cb05e82e.go:9:8: undefined: Root`
- `draft4/additionalProperties.json::additionalProperties can exist by itself` — `g_d48a2ab0145a1242/g_d48a2ab0145a1242.go:9:8: undefined: Root`
- `draft4/additionalProperties.json::additionalProperties are allowed by default` — `g_bfba291b878edcee/g_bfba291b878edcee.go:9:8: undefined: Root`
- `draft4/additionalProperties.json::additionalProperties does not look in applicators` — `g_c5a22626afe378bf/g_c5a22626afe378bf.go:9:8: undefined: Root`
- `draft4/additionalProperties.json::additionalProperties with null valued instance properties` — `g_313543d874e2f4f1/g_313543d874e2f4f1.go:9:8: undefined: Root`
- `draft4/allOf.json::allOf` — `g_e90641e929294886/g_e90641e929294886.go:9:8: undefined: Root`
- `draft4/allOf.json::allOf with base schema` — `g_91002da39f69f8dd/g_91002da39f69f8dd.go:9:8: undefined: Root`
- `draft4/allOf.json::allOf simple types` — `g_db2e13a567f3db88/g_db2e13a567f3db88.go:9:8: undefined: Root`
- `draft4/allOf.json::allOf with one empty schema` — `g_531b29c167ca743f/g_531b29c167ca743f.go:9:8: undefined: Root`
- `draft4/allOf.json::allOf with two empty schemas` — `g_9a124b19b5f17d91/g_9a124b19b5f17d91.go:9:8: undefined: Root`
- `draft4/allOf.json::allOf with the first empty schema` — `g_4bf88267b5fcf100/g_4bf88267b5fcf100.go:9:8: undefined: Root`
- `draft4/allOf.json::allOf with the last empty schema` — `g_2fa5fcb8f3cfbda4/g_2fa5fcb8f3cfbda4.go:9:8: undefined: Root`
- `draft4/allOf.json::nested allOf, to check validation semantics` — `g_faa334b5065a0e64/g_faa334b5065a0e64.go:9:8: undefined: Root`
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `g_9bbe2b4b9655a01f/g_9bbe2b4b9655a01f.go:9:8: undefined: Root`
- `draft4/anyOf.json::anyOf` — `g_68e72c9435a7a6cd/g_68e72c9435a7a6cd.go:9:8: undefined: Root`
- `draft4/anyOf.json::anyOf complex types` — `g_b9c214d4907dbc7f/g_b9c214d4907dbc7f.go:9:8: undefined: Root`
- `draft4/anyOf.json::anyOf with one empty schema` — `g_465cd6eebf7c02b4/g_465cd6eebf7c02b4.go:9:8: undefined: Root`
- `draft4/anyOf.json::nested anyOf, to check validation semantics` — `g_a0476a123f621645/g_a0476a123f621645.go:9:8: undefined: Root`
- `draft4/default.json::invalid type for default` — `g_1b6b5332f1c81610/g_1b6b5332f1c81610.go:9:8: undefined: Root`
- `draft4/default.json::invalid string value for default` — `g_d3c4f4448319d815/g_d3c4f4448319d815.go:9:8: undefined: Root`
- `draft4/definitions.json::validate definition against metaschema` — `g_e621137b73ddf419/g_e621137b73ddf419.go:9:8: undefined: Root`
- `draft4/dependencies.json::dependent subschema incompatible with root` — `g_d274dc016629f41b/g_d274dc016629f41b.go:9:8: undefined: Root`
- `draft4/enum.json::simple enum validation` — `g_03ba2e0273b8b7a4/g_03ba2e0273b8b7a4.go:9:8: undefined: Root`
- `draft4/enum.json::heterogeneous enum validation` — `g_e004e37998f254ae/g_e004e37998f254ae.go:9:8: undefined: Root`
- `draft4/enum.json::heterogeneous enum-with-null validation` — `g_c922b604a3f108b2/g_c922b604a3f108b2.go:9:8: undefined: Root`
- `draft4/enum.json::enum with escaped characters` — `g_b93862ea9306cd69/g_b93862ea9306cd69.go:9:8: undefined: Root`
- `draft4/enum.json::enum with false does not match 0` — `g_5e7c6e886cfe1b5c/g_5e7c6e886cfe1b5c.go:9:8: undefined: Root`
- `draft4/enum.json::enum with [false] does not match [0]` — `g_b9704e2524876b3a/g_b9704e2524876b3a.go:9:8: undefined: Root`
- `draft4/enum.json::enum with true does not match 1` — `g_d082377012835913/g_d082377012835913.go:9:8: undefined: Root`
- `draft4/enum.json::enum with [true] does not match [1]` — `g_1841fa8121f1f7fc/g_1841fa8121f1f7fc.go:9:8: undefined: Root`
- `draft4/enum.json::enum with 0 does not match false` — `g_244b45c631fe22bb/g_244b45c631fe22bb.go:9:8: undefined: Root`
- `draft4/enum.json::enum with [0] does not match [false]` — `g_e1cfd4b8c7806e78/g_e1cfd4b8c7806e78.go:9:8: undefined: Root`
- `draft4/enum.json::enum with 1 does not match true` — `g_cabbcbae211922a3/g_cabbcbae211922a3.go:9:8: undefined: Root`
- `draft4/enum.json::enum with [1] does not match [true]` — `g_56241582576d7a66/g_56241582576d7a66.go:9:8: undefined: Root`
- `draft4/enum.json::nul characters in strings` — `g_892cb66ea81a0f5e/g_892cb66ea81a0f5e.go:9:8: undefined: Root`
- `draft4/enum.json::characters with the same visual representation but different codepoint` — `g_40c459ff8d977fe3/g_40c459ff8d977fe3.go:9:8: undefined: Root`
- `draft4/enum.json::characters with the same visual representation, but different number of codepoints` — `g_b95843450e35b931/g_b95843450e35b931.go:9:8: undefined: Root`
- `draft4/format.json::email format` — `g_4cc93b415b496e2b/g_4cc93b415b496e2b.go:9:8: undefined: Root`
- `draft4/format.json::ipv4 format` — `g_510af5b211611c4b/g_510af5b211611c4b.go:9:8: undefined: Root`
- `draft4/format.json::ipv6 format` — `g_0cb470f40ea48aad/g_0cb470f40ea48aad.go:9:8: undefined: Root`
- `draft4/format.json::hostname format` — `g_68c07488f1d9c11b/g_68c07488f1d9c11b.go:9:8: undefined: Root`
- `draft4/format.json::date-time format` — `g_d2bd7e21bb1b8d54/g_d2bd7e21bb1b8d54.go:9:8: undefined: Root`
- `draft4/format.json::uri format` — `g_6bac4657aa959e29/g_6bac4657aa959e29.go:9:8: undefined: Root`
- `draft4/infinite-loop-detection.json::evaluating the same schema location against the same data location twice is not a sign of an infinite loop` — `g_19bf22974b211372/g_19bf22974b211372.go:11:8: undefined: Root`
- `draft4/items.json::a schema given for items` — `g_1a57970c3f190bec/g_1a57970c3f190bec.go:9:8: undefined: Root`
- … and 1241 more

### 🤪 `mixed` (14)

14 distinct first-line errors; most common:

- **1×** `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_8a2e24b8c8ac6f3d.Root`
- **1×** `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_a4bef37fb10b78e8.Root`
- **1×** `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_c5a49de2ae923598.Root`
- **1×** `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_e29c334971ec538a.Root`
- **1×** `an integer is valid: json: cannot unmarshal number into Go value of type g_50d3d18df2c3a7b9.Root`

Per group:

- `draft4/type.json::multiple types can be specified in an array` — `an integer is valid: json: cannot unmarshal number into Go value of type g_50d3d18df2c3a7b9.Root`
  - cases: an integer is valid (rejected), null is invalid (accepted)
- `draft4/type.json::type: array or object` — `array is valid: json: cannot unmarshal array into Go value of type g_83a2cf302e494b50.Root`
  - cases: array is valid (rejected), null is invalid (accepted)
- `draft6/type.json::integer type matches integers` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_a4bef37fb10b78e8.Root`
  - cases: a float with zero fractional part is an integer (rejected), null is not an integer (accepted)
- `draft6/type.json::multiple types can be specified in an array` — `an integer is valid: json: cannot unmarshal number into Go value of type g_717f0f0e74358c43.Root`
  - cases: an integer is valid (rejected), null is invalid (accepted)
- `draft6/type.json::type: array or object` — `array is valid: json: cannot unmarshal array into Go value of type g_8d846c6915be2bbd.Root`
  - cases: array is valid (rejected), null is invalid (accepted)
- `draft7/type.json::integer type matches integers` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_c5a49de2ae923598.Root`
  - cases: a float with zero fractional part is an integer (rejected), null is not an integer (accepted)
- `draft7/type.json::multiple types can be specified in an array` — `an integer is valid: json: cannot unmarshal number into Go value of type g_ae071fa272594ea5.Root`
  - cases: an integer is valid (rejected), null is invalid (accepted)
- `draft7/type.json::type: array or object` — `array is valid: json: cannot unmarshal array into Go value of type g_6f74f6a5a5f1cf85.Root`
  - cases: array is valid (rejected), null is invalid (accepted)
- `draft2019-09/type.json::integer type matches integers` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_e29c334971ec538a.Root`
  - cases: a float with zero fractional part is an integer (rejected), null is not an integer (accepted)
- `draft2019-09/type.json::multiple types can be specified in an array` — `an integer is valid: json: cannot unmarshal number into Go value of type g_f07938dc3891d236.Root`
  - cases: an integer is valid (rejected), null is invalid (accepted)
- `draft2019-09/type.json::type: array or object` — `array is valid: json: cannot unmarshal array into Go value of type g_74485c00253a0754.Root`
  - cases: array is valid (rejected), null is invalid (accepted)
- `draft2020-12/type.json::integer type matches integers` — `a float with zero fractional part is an integer: json: cannot unmarshal number 1.0 into Go value of type g_8a2e24b8c8ac6f3d.Root`
  - cases: a float with zero fractional part is an integer (rejected), null is not an integer (accepted)
- `draft2020-12/type.json::multiple types can be specified in an array` — `an integer is valid: json: cannot unmarshal number into Go value of type g_75b32d9ca064f40c.Root`
  - cases: an integer is valid (rejected), null is invalid (accepted)
- `draft2020-12/type.json::type: array or object` — `array is valid: json: cannot unmarshal array into Go value of type g_9b25990076d3d748.Root`
  - cases: array is valid (rejected), null is invalid (accepted)

### 🚧 `false_reject` (25)

9 distinct first-line errors; most common:

- **5×** `Line tabulation matches: field  pattern match: must match ^\s$`
- **5×** `matches: field  pattern match: must match ^\cC$`
- **5×** `matches: field  pattern match: must match ^\cc$`
- **5×** `missing properties are not filled in with the default: field alpha: must be <= 3`
- **1×** `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_672219b78d62416f.Root`

Per group:

- `draft4/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: field alpha: must be <= 3`
  - cases: missing properties are not filled in with the default (rejected)
- `draft6/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: field alpha: must be <= 3`
  - cases: missing properties are not filled in with the default (rejected)
- `draft7/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: field alpha: must be <= 3`
  - cases: missing properties are not filled in with the default (rejected)
- `draft2019-09/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: field alpha: must be <= 3`
  - cases: missing properties are not filled in with the default (rejected)
- `draft2020-12/default.json::the default keyword does not do anything if the property is missing` — `missing properties are not filled in with the default: field alpha: must be <= 3`
  - cases: missing properties are not filled in with the default (rejected)
- `draft4/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_672219b78d62416f.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft4/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and upper letter` — `matches: field  pattern match: must match ^\cC$`
  - cases: matches (rejected)
- `draft4/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and lower letter` — `matches: field  pattern match: must match ^\cc$`
  - cases: matches (rejected)
- `draft4/optional/ecmascript-regex.json::ECMA 262 \s matches whitespace` — `Line tabulation matches: field  pattern match: must match ^\s$`
  - cases: Line tabulation matches (rejected), latin-1 non-breaking-space matches (rejected), zero-width whitespace matches (rejected), paragraph separator matches (line terminator) (rejected), EM SPACE matches (Space_Separator) (rejected)
- `draft6/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_a4bef37fb10b78e8.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft6/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and upper letter` — `matches: field  pattern match: must match ^\cC$`
  - cases: matches (rejected)
- `draft6/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and lower letter` — `matches: field  pattern match: must match ^\cc$`
  - cases: matches (rejected)
- `draft6/optional/ecmascript-regex.json::ECMA 262 \s matches whitespace` — `Line tabulation matches: field  pattern match: must match ^\s$`
  - cases: Line tabulation matches (rejected), latin-1 non-breaking-space matches (rejected), zero-width whitespace matches (rejected), paragraph separator matches (line terminator) (rejected), EM SPACE matches (Space_Separator) (rejected)
- `draft7/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_c5a49de2ae923598.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft7/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and upper letter` — `matches: field  pattern match: must match ^\cC$`
  - cases: matches (rejected)
- `draft7/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and lower letter` — `matches: field  pattern match: must match ^\cc$`
  - cases: matches (rejected)
- `draft7/optional/ecmascript-regex.json::ECMA 262 \s matches whitespace` — `Line tabulation matches: field  pattern match: must match ^\s$`
  - cases: Line tabulation matches (rejected), latin-1 non-breaking-space matches (rejected), zero-width whitespace matches (rejected), paragraph separator matches (line terminator) (rejected), EM SPACE matches (Space_Separator) (rejected)
- `draft2019-09/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_e29c334971ec538a.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft2019-09/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and upper letter` — `matches: field  pattern match: must match ^\cC$`
  - cases: matches (rejected)
- `draft2019-09/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and lower letter` — `matches: field  pattern match: must match ^\cc$`
  - cases: matches (rejected)
- `draft2019-09/optional/ecmascript-regex.json::ECMA 262 \s matches whitespace` — `Line tabulation matches: field  pattern match: must match ^\s$`
  - cases: Line tabulation matches (rejected), latin-1 non-breaking-space matches (rejected), zero-width whitespace matches (rejected), paragraph separator matches (line terminator) (rejected), EM SPACE matches (Space_Separator) (rejected)
- `draft2020-12/optional/bignum.json::integer` — `a bignum is an integer: json: cannot unmarshal number 12345678910111213141516171819202122232425262728293031 into Go value of type g_8a2e24b8c8ac6f3d.Root`
  - cases: a bignum is an integer (rejected), a negative bignum is an integer (rejected)
- `draft2020-12/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and upper letter` — `matches: field  pattern match: must match ^\cC$`
  - cases: matches (rejected)
- `draft2020-12/optional/ecmascript-regex.json::ECMA 262 regex escapes control codes with \c and lower letter` — `matches: field  pattern match: must match ^\cc$`
  - cases: matches (rejected)
- `draft2020-12/optional/ecmascript-regex.json::ECMA 262 \s matches whitespace` — `Line tabulation matches: field  pattern match: must match ^\s$`
  - cases: Line tabulation matches (rejected), latin-1 non-breaking-space matches (rejected), zero-width whitespace matches (rejected), paragraph separator matches (line terminator) (rejected), EM SPACE matches (Space_Separator) (rejected)

### 🙈 `false_accept` (107)

28 distinct first-line errors; most common:

- **10×** `accepted 'ascii non-digits' though valid: false`
- **10×** `accepted 'literal unicode character in json string' though valid: false`
- **7×** `accepted 'with unevaluated properties' though valid: false`
- **5×** `accepted 'Line tabulation does not match' though valid: false`
- **5×** `accepted 'an integer is not null' though valid: false`

Per group:

- `draft4/anyOf.json::anyOf with base schema` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `draft4/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft4/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
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
- `draft4/type.json::null type matches only the null object` — `accepted 'an integer is not null' though valid: false`
  - cases: an integer is not null (accepted), a float is not null (accepted), zero is not null (accepted), a string is not null (accepted), an empty string is not null (accepted), an object is not null (accepted), … and 3 more
- `draft4/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft6/anyOf.json::anyOf with base schema` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `draft6/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft6/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft6/type.json::number type matches numbers` — `accepted 'null is not a number' though valid: false`
  - cases: null is not a number (accepted)
- `draft6/type.json::string type matches strings` — `accepted 'null is not a string' though valid: false`
  - cases: null is not a string (accepted)
- `draft6/type.json::object type matches objects` — `accepted 'null is not an object' though valid: false`
  - cases: null is not an object (accepted)
- `draft6/type.json::array type matches arrays` — `accepted 'null is not an array' though valid: false`
  - cases: null is not an array (accepted)
- `draft6/type.json::boolean type matches booleans` — `accepted 'null is not a boolean' though valid: false`
  - cases: null is not a boolean (accepted)
- `draft6/type.json::null type matches only the null object` — `accepted 'an integer is not null' though valid: false`
  - cases: an integer is not null (accepted), a float is not null (accepted), zero is not null (accepted), a string is not null (accepted), an empty string is not null (accepted), an object is not null (accepted), … and 3 more
- `draft6/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft7/anyOf.json::anyOf with base schema` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `draft7/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft7/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft7/type.json::number type matches numbers` — `accepted 'null is not a number' though valid: false`
  - cases: null is not a number (accepted)
- `draft7/type.json::string type matches strings` — `accepted 'null is not a string' though valid: false`
  - cases: null is not a string (accepted)
- `draft7/type.json::object type matches objects` — `accepted 'null is not an object' though valid: false`
  - cases: null is not an object (accepted)
- `draft7/type.json::array type matches arrays` — `accepted 'null is not an array' though valid: false`
  - cases: null is not an array (accepted)
- `draft7/type.json::boolean type matches booleans` — `accepted 'null is not a boolean' though valid: false`
  - cases: null is not a boolean (accepted)
- `draft7/type.json::null type matches only the null object` — `accepted 'an integer is not null' though valid: false`
  - cases: an integer is not null (accepted), a float is not null (accepted), zero is not null (accepted), a string is not null (accepted), an empty string is not null (accepted), an object is not null (accepted), … and 3 more
- `draft7/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft2019-09/anyOf.json::anyOf with base schema` — `accepted 'both anyOf invalid' though valid: false`
  - cases: both anyOf invalid (accepted)
- `draft2019-09/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft2019-09/oneOf.json::oneOf with required` — `accepted 'both invalid - invalid' though valid: false`
  - cases: both invalid - invalid (accepted), both valid - invalid (accepted)
- `draft2019-09/type.json::number type matches numbers` — `accepted 'null is not a number' though valid: false`
  - cases: null is not a number (accepted)
- `draft2019-09/type.json::string type matches strings` — `accepted 'null is not a string' though valid: false`
  - cases: null is not a string (accepted)
- `draft2019-09/type.json::object type matches objects` — `accepted 'null is not an object' though valid: false`
  - cases: null is not an object (accepted)
- `draft2019-09/type.json::array type matches arrays` — `accepted 'null is not an array' though valid: false`
  - cases: null is not an array (accepted)
- `draft2019-09/type.json::boolean type matches booleans` — `accepted 'null is not a boolean' though valid: false`
  - cases: null is not a boolean (accepted)
- `draft2019-09/type.json::null type matches only the null object` — `accepted 'an integer is not null' though valid: false`
  - cases: an integer is not null (accepted), a float is not null (accepted), zero is not null (accepted), a string is not null (accepted), an empty string is not null (accepted), an object is not null (accepted), … and 3 more
- `draft2019-09/type.json::type: array, object or null` — `accepted 'number is invalid' though valid: false`
  - cases: number is invalid (accepted), string is invalid (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties schema` — `accepted 'with invalid unevaluated properties' though valid: false`
  - cases: with invalid unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties false` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with adjacent properties` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with adjacent patternProperties` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with nested properties` — `accepted 'with additional properties' though valid: false`
  - cases: with additional properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with nested patternProperties` — `accepted 'with additional properties' though valid: false`
  - cases: with additional properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with anyOf` — `accepted 'when one matches and has unevaluated properties' though valid: false`
  - cases: when one matches and has unevaluated properties (accepted), when two match and has unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with oneOf` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- `draft2019-09/unevaluatedProperties.json::unevaluatedProperties with not` — `accepted 'with unevaluated properties' though valid: false`
  - cases: with unevaluated properties (accepted)
- … and 57 more

