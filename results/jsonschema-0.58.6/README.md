# jsonschema 0.58.6

A Rust validator, not a generator: it interprets the schema at run time, so
it can enforce every keyword, which a type generator structurally can't. It's
the reference row: the ceiling the suite allows, a check on the harness, and
a price tag on the policies the generators live with. Remote refs come from
`suite/remotes/`, the dialect from the suite directory, and `format` is
asserted in every draft (the library default would stop at draft 7).

_generated 2026-10-11T21:59:49Z_

**1389 of 1408 non-optional groups pass (98.7%)**; 301 optional groups reported separately

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
| draft4 | 160 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 160 | 100.0% |
| draft6 | 233 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 233 | 100.0% |
| draft7 | 258 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 258 | 100.0% |
| draft2019-09 | 373 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 373 | 100.0% |
| draft2020-12 | 365 | 0 | 19 | 0 | 0 | 0 | 0 | 0 | 384 | 95.1% |
| **Total** | **1389** | **0** | **19** | **0** | **0** | **0** | **0** | **0** | **1408** | **98.7%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 39 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 39 | 100.0% |
| draft6 | 44 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 44 | 100.0% |
| draft7 | 64 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 64 | 100.0% |
| draft2019-09 | 76 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 76 | 100.0% |
| draft2020-12 | 78 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 78 | 100.0% |
| **Total** | **301** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **301** | **100.0%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 618 | 618 | 100.0% | 618 | 100.0% |
| draft6 | 841 | 841 | 100.0% | 841 | 100.0% |
| draft7 | 929 | 929 | 100.0% | 929 | 100.0% |
| draft2019-09 | 1261 | 1261 | 100.0% | 1261 | 100.0% |
| draft2020-12 | 1282 | 1301 | 98.5% | 1301 | 98.5% |
| **Total** | **4931** | **4950** | **99.6%** | **4950** | **99.6%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 391 | 391 | 100.0% | 391 | 100.0% |
| draft6 | 520 | 520 | 100.0% | 520 | 100.0% |
| draft7 | 922 | 922 | 100.0% | 922 | 100.0% |
| draft2019-09 | 1043 | 1043 | 100.0% | 1043 | 100.0% |
| draft2020-12 | 1047 | 1047 | 100.0% | 1047 | 100.0% |
| **Total** | **3923** | **3923** | **100.0%** | **3923** | **100.0%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| enum.json | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| format.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| maxItems.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| maxLength.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| maxProperties.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maximum.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| minItems.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| minLength.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| minProperties.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| minimum.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| oneOf.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| pattern.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| patternProperties.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| properties.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| ref.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| required.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| **Total** | **160** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/email.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/hostname.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ipv4.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ipv6.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/id.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/zeroTerminatedFloats.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **39** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| enum.json | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| patternProperties.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| properties.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| propertyNames.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| ref.json | 31 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 31 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| **Total** | **233** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/email.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/hostname.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ipv4.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ipv6.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/json-pointer.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-template.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/id.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/unknownKeyword.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **44** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| definitions.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependencies.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| enum.json | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| patternProperties.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| properties.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| propertyNames.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| ref.json | 35 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 35 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| **Total** | **258** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/content.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| optional/cross-draft.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ecmascript-regex.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| optional/format/email.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/format/idn-email.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/idn-hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/format/ipv4.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ipv6.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/iri-reference.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/iri.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/json-pointer.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/regex.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/relative-json-pointer.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/time.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-template.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/id.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/unknownKeyword.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **64** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| additionalProperties.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| defs.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependentRequired.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| dependentSchemas.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| enum.json | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| maxContains.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minContains.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| patternProperties.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| properties.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| propertyNames.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| recursiveRef.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| ref.json | 37 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 37 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 26 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 26 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedProperties.json | 44 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 44 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| **Total** | **373** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/bignum.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/cross-draft.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/dependencies-compatibility.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/duration.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ecmascript-regex.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| optional/format/email.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/format/idn-email.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/idn-hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/format/ipv4.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ipv6.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/iri-reference.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/iri.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/json-pointer.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/regex.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/relative-json-pointer.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/time.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-template.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uuid.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/id.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/no-schema.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/refOfUnknownKeyword.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| optional/unknownKeyword.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **76** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| allOf.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| anchor.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| anyOf.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| contains.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| defs.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| dependentRequired.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| dependentSchemas.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| dynamicRef.json | 21 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 21 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| enum.json | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 16 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| exclusiveMaximum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| exclusiveMinimum.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| format.json | 0 | 0 | 19 | 0 | 0 | 0 | 0 | 0 | 19 | 🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧 |
| if-then-else.json | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 10 | ✅✅✅✅✅✅✅✅✅✅ |
| maxContains.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| maxItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| maxProperties.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| maximum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minContains.json | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | ✅✅✅✅✅✅✅✅ |
| minItems.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minLength.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minProperties.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| minimum.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| multipleOf.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| not.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| oneOf.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| pattern.json | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅✅ |
| patternProperties.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| prefixItems.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| properties.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| propertyNames.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| ref.json | 36 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 36 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| refRemote.json | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 15 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| required.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| type.json | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedItems.json | 29 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 29 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| unevaluatedProperties.json | 44 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 44 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| vocabulary.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| **Total** | **365** | **0** | **19** | **0** | **0** | **0** | **0** | **0** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤪 | 💀 | 🦀 | ❌ | 🤯 | Total | Per group |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/bignum.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/cross-draft.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/dependencies-compatibility.json | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | ✅✅✅✅✅✅✅ |
| optional/dynamicRef.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/ecmascript-regex.json | 20 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format-assertion.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/format/date-time.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/duration.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ecmascript-regex.json | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| optional/format/email.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/format/idn-email.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/idn-hostname.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/format/ipv4.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/ipv6.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/iri-reference.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/iri.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/json-pointer.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/regex.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/relative-json-pointer.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/time.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-template.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uuid.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/id.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/no-schema.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/non-bmp-regex.json | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| optional/refOfUnknownKeyword.json | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 | ✅✅✅✅✅ |
| optional/unknownKeyword.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **78** | **0** | **0** | **0** | **0** | **0** | **0** | **0** | **78** |  |

## Failures

### 🚧 `false_reject` (19)

19 distinct first-line errors; most common:

- **1×** `invalid date string is only an annotation by default: "06/19/1963" is not a "date"`
- **1×** `invalid date-time string is only an annotation by default: "1990-02-31T15:59:60.123-08:00" is not a "date-time"`
- **1×** `invalid duration string is only an annotation by default: "PT1D" is not a "duration"`
- **1×** `invalid email string is only an annotation by default: "2962" is not a "email"`
- **1×** `invalid hostname string is only an annotation by default: "-a-host-name-that-starts-with--" is not a "hostname"`

Per group:

- `draft2020-12/format.json::email format` — `invalid email string is only an annotation by default: "2962" is not a "email"`
  - cases: invalid email string is only an annotation by default (rejected)
- `draft2020-12/format.json::idn-email format` — `invalid idn-email string is only an annotation by default: "2962" is not a "idn-email"`
  - cases: invalid idn-email string is only an annotation by default (rejected)
- `draft2020-12/format.json::regex format` — `invalid regex string is only an annotation by default: "^(abc]" is not a "regex"`
  - cases: invalid regex string is only an annotation by default (rejected)
- `draft2020-12/format.json::ipv4 format` — `invalid ipv4 string is only an annotation by default: "127.0.0.0.1" is not a "ipv4"`
  - cases: invalid ipv4 string is only an annotation by default (rejected)
- `draft2020-12/format.json::ipv6 format` — `invalid ipv6 string is only an annotation by default: "12345::" is not a "ipv6"`
  - cases: invalid ipv6 string is only an annotation by default (rejected)
- `draft2020-12/format.json::idn-hostname format` — `invalid idn-hostname string is only an annotation by default: "〮실례.테스트" is not a "idn-hostname"`
  - cases: invalid idn-hostname string is only an annotation by default (rejected)
- `draft2020-12/format.json::hostname format` — `invalid hostname string is only an annotation by default: "-a-host-name-that-starts-with--" is not a "hostname"`
  - cases: invalid hostname string is only an annotation by default (rejected)
- `draft2020-12/format.json::date format` — `invalid date string is only an annotation by default: "06/19/1963" is not a "date"`
  - cases: invalid date string is only an annotation by default (rejected)
- `draft2020-12/format.json::date-time format` — `invalid date-time string is only an annotation by default: "1990-02-31T15:59:60.123-08:00" is not a "date-time"`
  - cases: invalid date-time string is only an annotation by default (rejected)
- `draft2020-12/format.json::time format` — `invalid time string is only an annotation by default: "08:30:06 PST" is not a "time"`
  - cases: invalid time string is only an annotation by default (rejected)
- `draft2020-12/format.json::json-pointer format` — `invalid json-pointer string is only an annotation by default: "/foo/bar~" is not a "json-pointer"`
  - cases: invalid json-pointer string is only an annotation by default (rejected)
- `draft2020-12/format.json::relative-json-pointer format` — `invalid relative-json-pointer string is only an annotation by default: "/foo/bar" is not a "relative-json-pointer"`
  - cases: invalid relative-json-pointer string is only an annotation by default (rejected)
- `draft2020-12/format.json::iri format` — `invalid iri string is only an annotation by default: "http://2001:0db8:85a3:0000:0000:8a2e:0370:7334" is not a "iri"`
  - cases: invalid iri string is only an annotation by default (rejected)
- `draft2020-12/format.json::iri-reference format` — `invalid iri-reference string is only an annotation by default: "\\\\WINDOWS\\filëßåré" is not a "iri-reference"`
  - cases: invalid iri-reference string is only an annotation by default (rejected)
- `draft2020-12/format.json::uri format` — `invalid uri string is only an annotation by default: "//foo.bar/?baz=qux#quux" is not a "uri"`
  - cases: invalid uri string is only an annotation by default (rejected)
- `draft2020-12/format.json::uri-reference format` — `invalid uri-reference string is only an annotation by default: "\\\\WINDOWS\\fileshare" is not a "uri-reference"`
  - cases: invalid uri-reference string is only an annotation by default (rejected)
- `draft2020-12/format.json::uri-template format` — `invalid uri-template string is only an annotation by default: "http://example.com/dictionary/{term:1}/{term" is not a "uri-template"`
  - cases: invalid uri-template string is only an annotation by default (rejected)
- `draft2020-12/format.json::uuid format` — `invalid uuid string is only an annotation by default: "2eb8aa08-aa98-11ea-b4aa-73b441d1638" is not a "uuid"`
  - cases: invalid uuid string is only an annotation by default (rejected)
- `draft2020-12/format.json::duration format` — `invalid duration string is only an annotation by default: "PT1D" is not a "duration"`
  - cases: invalid duration string is only an annotation by default (rejected)

