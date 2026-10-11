# schemafy 0.6.0

schemafy is the original Rust JSON Schema → serde generator (2017), usually
used through the `schemafy!` proc macro that drops types for a schema file
straight into a module. Its design center is a hand-written draft-4 schema
you own and keep working: walk `properties` and `definitions`, name a struct
or an alias for each, and otherwise get out of the way. It models only
structure — "no checking such as `min_value` is done" — so validation
keywords become comments at best, unknown keywords are dropped, and anything
that isn't a plain object graph (`oneOf` without `$ref`s, a boolean schema,
a `$ref` it can't find) panics rather than degrading. `schemafy_lib` is the
library behind the macro, and the generator here calls its `Expander`
directly.

_generated 2026-10-11T18:21:50Z_

**325 of 1408 non-optional groups pass (23.1%)**; 301 optional groups reported separately

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

## Headline

### Groups

| Draft | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 44 | 79 | 2 | 3 | 9 | 0 | 23 | 160 | 27.5% |
| draft6 | 54 | 106 | 3 | 3 | 12 | 27 | 28 | 233 | 23.2% |
| draft7 | 66 | 115 | 3 | 3 | 12 | 27 | 32 | 258 | 25.6% |
| draft2019-09 | 81 | 160 | 3 | 3 | 21 | 44 | 61 | 373 | 21.7% |
| draft2020-12 | 80 | 168 | 3 | 3 | 22 | 42 | 66 | 384 | 20.8% |
| **Total** | **325** | **628** | **14** | **15** | **76** | **140** | **210** | **1408** | **23.1%** |

### Optional groups

| Draft | ✅ | 🙈 | 🚧 | 🦀 | ❌ | 🤯 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| draft4 | 7 | 30 | 1 | 0 | 0 | 1 | 39 | 17.9% |
| draft6 | 5 | 29 | 2 | 0 | 7 | 1 | 44 | 11.4% |
| draft7 | 8 | 45 | 2 | 0 | 8 | 1 | 64 | 12.5% |
| draft2019-09 | 9 | 48 | 2 | 4 | 8 | 5 | 76 | 11.8% |
| draft2020-12 | 9 | 50 | 2 | 5 | 7 | 5 | 78 | 11.5% |
| **Total** | **38** | **202** | **9** | **9** | **30** | **13** | **301** | **12.6%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 367 | 536 | 68.5% | 618 | 59.4% |
| draft6 | 441 | 658 | 67.0% | 841 | 52.4% |
| draft7 | 510 | 738 | 69.1% | 929 | 54.9% |
| draft2019-09 | 616 | 918 | 67.1% | 1261 | 48.9% |
| draft2020-12 | 641 | 955 | 67.1% | 1301 | 49.3% |
| **Total** | **2575** | **3805** | **67.7%** | **4950** | **52.0%** |

### Per-case match rate (optional)

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft4 | 169 | 388 | 43.6% | 391 | 43.2% |
| draft6 | 234 | 498 | 47.0% | 520 | 45.0% |
| draft7 | 414 | 898 | 46.1% | 922 | 44.9% |
| draft2019-09 | 471 | 1001 | 47.1% | 1043 | 45.2% |
| draft2020-12 | 473 | 1005 | 47.1% | 1047 | 45.2% |
| **Total** | **1761** | **3790** | **46.5%** | **3923** | **44.9%** |

## Chart

### draft4

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 5 | 0 | 0 | 0 | 0 | 9 | 🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 2 | 4 | 0 | 1 | 0 | 0 | 7 | 🤪🙈🙈🙈✅🙈✅ |
| allOf.json | 3 | 6 | 0 | 0 | 0 | 0 | 9 | ✅🙈🙈✅✅🙈🙈🙈🙈 |
| anyOf.json | 1 | 4 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅🙈 |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| dependencies.json | 0 | 5 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈🙈🙈 |
| enum.json | 3 | 1 | 0 | 0 | 1 | 11 | 16 | 🤯🤯🤯🙈🦀🤯🤯🤯🤯🤯🤯🤯🤯✅✅✅ |
| format.json | 6 | 0 | 0 | 0 | 0 | 0 | 6 | ✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 3 | 3 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| maximum.json | 0 | 4 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| minItems.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minLength.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minProperties.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| minimum.json | 0 | 4 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 1 | 5 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| oneOf.json | 0 | 7 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈🙈 |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈✅ |
| properties.json | 1 | 1 | 1 | 1 | 1 | 0 | 5 | 🚧🙈🦀✅🤪 |
| ref.json | 3 | 4 | 0 | 1 | 4 | 7 | 19 | 🤪🦀🙈🦀🙈✅🤯🤯✅✅🦀🦀🤯🤯🤯🤯🙈🙈🤯 |
| refRemote.json | 0 | 1 | 0 | 0 | 3 | 4 | 8 | 🤯🤯🤯🙈🦀🦀🦀🤯 |
| required.json | 1 | 2 | 1 | 0 | 0 | 0 | 4 | 🚧✅🙈🙈 |
| type.json | 7 | 4 | 0 | 0 | 0 | 0 | 11 | ✅✅✅✅✅✅🙈🙈✅🙈🙈 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **44** | **79** | **2** | **3** | **9** | **23** | **160** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 2 | 1 | 0 | 7 | 🚧✅✅✅🙈✅🙈 |
| optional/ecmascript-regex.json | 0 | 20 | 0 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| optional/float-overflow.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/email.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 2 | 🙈🙈 |
| optional/zeroTerminatedFloats.json | 1 | 0 | 0 | 0 | 1 | ✅ |
| **Total** | **7** | **30** | **1** | **1** | **39** |  |

### draft6

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 2 | 4 | 0 | 1 | 0 | 0 | 0 | 7 | 🤪🙈🙈🙈✅🙈✅ |
| allOf.json | 3 | 6 | 0 | 0 | 0 | 3 | 0 | 12 | ✅🙈🙈❌❌❌✅✅🙈🙈🙈🙈 |
| anyOf.json | 1 | 4 | 0 | 0 | 0 | 3 | 0 | 8 | 🙈🙈❌❌❌🙈✅🙈 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈🙈🙈✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 2 | 1 | 0 | 0 | 1 | 0 | 11 | 15 | 🤯🤯🤯🙈🦀🤯🤯🤯🤯🤯🤯🤯🤯✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 9 | ✅✅✅✅✅✅✅✅✅ |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 3 | 3 | 0 | 0 | 0 | 3 | 0 | 9 | 🙈🙈❌❌❌🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 1 | 5 | 0 | 0 | 0 | 2 | 0 | 8 | 🙈🙈🙈🙈🙈❌❌✅ |
| oneOf.json | 0 | 6 | 0 | 0 | 0 | 5 | 0 | 11 | 🙈🙈❌❌❌❌🙈🙈🙈❌🙈 |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈❌✅ |
| properties.json | 1 | 1 | 1 | 1 | 1 | 1 | 0 | 6 | 🚧🙈❌🦀✅🤪 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 8 | 4 | 0 | 1 | 6 | 2 | 10 | 31 | 🤪🦀🙈🦀🙈✅🤯🤯✅✅❌❌🦀🦀🤯🤯🤯🤯🤯🤯🦀✅✅✅✅✅🦀🤯🙈🙈🤯 |
| refRemote.json | 0 | 1 | 0 | 0 | 4 | 0 | 6 | 11 | 🤯🤯🤯🙈🦀🦀🦀🤯🤯🦀🤯 |
| required.json | 2 | 2 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🙈 |
| type.json | 6 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅🙈🙈✅🙈🙈 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **54** | **106** | **3** | **3** | **12** | **27** | **28** | **233** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 0 | 1 | 2 | 0 | 7 | 🚧✅✅✅❌✅❌ |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈❌❌❌❌❌ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 2 | 0 | 0 | 1 | 3 | 🤯🙈🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **5** | **29** | **2** | **7** | **1** | **44** |  |

### draft7

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 2 | 4 | 0 | 1 | 0 | 0 | 0 | 7 | 🤪🙈🙈🙈✅🙈✅ |
| allOf.json | 3 | 6 | 0 | 0 | 0 | 3 | 0 | 12 | ✅🙈🙈❌❌❌✅✅🙈🙈🙈🙈 |
| anyOf.json | 1 | 4 | 0 | 0 | 0 | 3 | 0 | 8 | 🙈🙈❌❌❌🙈✅🙈 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 2 | 1 | 0 | 0 | 1 | 0 | 11 | 15 | 🤯🤯🤯🙈🦀🤯🤯🤯🤯🤯🤯🤯🤯✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 17 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 3 | 3 | 0 | 0 | 0 | 3 | 0 | 9 | 🙈🙈❌❌❌🙈✅✅✅ |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 1 | 5 | 0 | 0 | 0 | 2 | 0 | 8 | 🙈🙈🙈🙈🙈❌❌✅ |
| oneOf.json | 0 | 6 | 0 | 0 | 0 | 5 | 0 | 11 | 🙈🙈❌❌❌❌🙈🙈🙈❌🙈 |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈❌✅ |
| properties.json | 1 | 1 | 1 | 1 | 1 | 1 | 0 | 6 | 🚧🙈❌🦀✅🤪 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 8 | 4 | 0 | 1 | 6 | 2 | 14 | 35 | 🤪🦀🙈🦀🙈✅🤯🤯✅✅❌❌🦀🦀🤯🤯🤯🤯🤯🤯🤯🦀✅✅✅✅✅🦀🤯🤯🤯🤯🙈🙈🤯 |
| refRemote.json | 0 | 1 | 0 | 0 | 4 | 0 | 6 | 11 | 🤯🤯🤯🙈🦀🦀🦀🤯🤯🦀🤯 |
| required.json | 2 | 2 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🙈 |
| type.json | 6 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅🙈🙈✅🙈🙈 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| **Total** | **66** | **115** | **3** | **3** | **12** | **27** | **32** | **258** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---|
| optional/bignum.json | 4 | 0 | 1 | 2 | 0 | 7 | 🚧✅✅✅❌✅❌ |
| optional/content.json | 0 | 3 | 0 | 0 | 0 | 3 | 🙈🙈🙈 |
| optional/cross-draft.json | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈❌❌❌❌❌ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 2 | 0 | 0 | 1 | 3 | 🤯🙈🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/unknownKeyword.json | 0 | 1 | 0 | 0 | 0 | 1 | 🙈 |
| **Total** | **8** | **45** | **2** | **8** | **1** | **64** |  |

### draft2019-09

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 4 | 6 | 0 | 0 | 0 | 0 | 0 | 10 | 🙈🙈✅🙈✅✅🙈🙈🙈✅ |
| additionalProperties.json | 2 | 6 | 0 | 1 | 0 | 0 | 0 | 9 | 🤪🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 3 | 6 | 0 | 0 | 0 | 3 | 0 | 12 | ✅🙈🙈❌❌❌✅✅🙈🙈🙈🙈 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 4 | 🤯🤯🤯🤯 |
| anyOf.json | 1 | 4 | 0 | 0 | 0 | 3 | 0 | 8 | 🙈🙈❌❌❌🙈✅🙈 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| enum.json | 2 | 2 | 0 | 0 | 1 | 0 | 11 | 16 | 🤯🤯🤯🙈🦀🤯🤯🤯🤯🤯🤯🤯🤯✅🙈✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 3 | 2 | 0 | 0 | 1 | 3 | 0 | 9 | 🙈🙈❌❌❌🦀✅✅✅ |
| maxContains.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈🙈🙈🙈 |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minContains.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 8 | ✅🙈🙈🙈🙈🙈✅🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 1 | 5 | 0 | 0 | 0 | 3 | 0 | 9 | 🙈🙈🙈🙈🙈❌❌✅❌ |
| oneOf.json | 0 | 6 | 0 | 0 | 0 | 5 | 0 | 11 | 🙈🙈❌❌❌❌🙈🙈🙈❌🙈 |
| pattern.json | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈✅ |
| patternProperties.json | 1 | 3 | 0 | 0 | 0 | 1 | 0 | 5 | 🙈🙈🙈❌✅ |
| properties.json | 1 | 1 | 1 | 1 | 1 | 1 | 0 | 6 | 🚧🙈❌🦀✅🤪 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| recursiveRef.json | 0 | 5 | 0 | 0 | 0 | 0 | 4 | 9 | 🙈🤯🤯🤯🤯🙈🙈🙈🙈 |
| ref.json | 1 | 1 | 0 | 1 | 13 | 0 | 21 | 37 | 🤪🦀🙈🦀🤯🦀🤯✅🦀🤯🤯🦀🦀🤯🤯🤯🤯🤯🤯🤯🤯🦀🦀🦀🦀🦀🦀🦀🤯🤯🤯🤯🤯🤯🤯🤯🤯 |
| refRemote.json | 0 | 1 | 0 | 0 | 4 | 0 | 10 | 15 | 🤯🤯🤯🤯🙈🦀🦀🦀🤯🤯🦀🤯🤯🤯🤯 |
| required.json | 2 | 2 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🙈 |
| type.json | 6 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅🙈🙈✅🙈🙈 |
| unevaluatedItems.json | 7 | 7 | 0 | 0 | 0 | 10 | 2 | 26 | ✅🙈🙈✅🙈✅🙈🙈❌❌✅✅❌❌❌🙈❌🤯🤯❌❌❌✅✅🙈❌ |
| unevaluatedProperties.json | 6 | 23 | 0 | 0 | 0 | 7 | 8 | 44 | ✅🙈🙈🙈🙈✅✅🙈🤯🤯🤯🙈🙈🙈🙈🙈🙈🙈❌🤯🤯🤯❌❌🤯✅🤯🙈🙈🙈🙈❌❌🙈❌❌✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 4 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅🙈 |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **81** | **160** | **3** | **3** | **21** | **44** | **61** | **373** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/bignum.json | 4 | 0 | 1 | 0 | 2 | 0 | 7 | 🚧✅✅✅❌✅❌ |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 1 | 1 | 2 | 🤯❌ |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈❌❌❌❌❌ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/duration.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uuid.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/no-schema.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 0 | 0 | 0 | 4 | 0 | 1 | 5 | 🦀🦀🦀🤯🦀 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| **Total** | **9** | **48** | **2** | **4** | **8** | **5** | **76** |  |

### draft2020-12

| File | ✅ | 🙈 | 🚧 | 🤪 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| additionalProperties.json | 2 | 6 | 0 | 1 | 0 | 0 | 0 | 9 | 🤪🙈🙈🙈✅🙈✅🙈🙈 |
| allOf.json | 3 | 6 | 0 | 0 | 0 | 3 | 0 | 12 | ✅🙈🙈❌❌❌✅✅🙈🙈🙈🙈 |
| anchor.json | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 4 | 🤯🤯🤯🤯 |
| anyOf.json | 1 | 4 | 0 | 0 | 0 | 3 | 0 | 8 | 🙈🙈❌❌❌🙈✅🙈 |
| boolean_schema.json | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | ❌❌ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 1 | 6 | 0 | 0 | 0 | 0 | 0 | 7 | 🙈🙈🙈🙈🙈🙈✅ |
| content.json | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | ✅✅✅✅ |
| default.json | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | ✅✅🙈 |
| defs.json | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| dependentRequired.json | 1 | 3 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈✅🙈🙈 |
| dependentSchemas.json | 0 | 4 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈🙈🙈 |
| dynamicRef.json | 0 | 5 | 0 | 0 | 2 | 0 | 14 | 21 | 🙈🙈🦀🤯🤯🤯🤯🤯🤯🤯🤯🙈🙈🤯🤯🤯🤯🤯🙈🦀🤯 |
| enum.json | 2 | 2 | 0 | 0 | 1 | 0 | 11 | 16 | 🤯🤯🤯🙈🦀🤯🤯🤯🤯🤯🤯🤯🤯✅🙈✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | ❌ |
| format.json | 19 | 0 | 0 | 0 | 0 | 0 | 0 | 19 | ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅ |
| if-then-else.json | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 12 | ✅✅✅🙈🙈🙈✅🙈🙈🙈🙈🙈 |
| infinite-loop-detection.json | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| items.json | 2 | 3 | 0 | 0 | 0 | 5 | 0 | 10 | 🙈❌❌❌✅❌🙈🙈❌✅ |
| maxContains.json | 1 | 4 | 0 | 0 | 0 | 0 | 0 | 5 | ✅🙈🙈🙈🙈 |
| maxItems.json | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxLength.json | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 2 | 🙈❌ |
| maxProperties.json | 0 | 2 | 0 | 0 | 0 | 1 | 0 | 3 | 🙈❌🙈 |
| maximum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minContains.json | 2 | 6 | 0 | 0 | 0 | 0 | 0 | 8 | ✅🙈🙈🙈🙈🙈✅🙈 |
| minItems.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minLength.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minProperties.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| minimum.json | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| multipleOf.json | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 5 | 🙈🙈🙈✅✅ |
| not.json | 1 | 5 | 0 | 0 | 0 | 3 | 0 | 9 | 🙈🙈🙈🙈🙈❌❌✅❌ |
| oneOf.json | 0 | 6 | 0 | 0 | 0 | 5 | 0 | 11 | 🙈🙈❌❌❌❌🙈🙈🙈❌🙈 |
| pattern.json | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 3 | 🙈✅🙈 |
| patternProperties.json | 2 | 3 | 0 | 0 | 0 | 1 | 0 | 6 | 🙈🙈🙈❌✅✅ |
| prefixItems.json | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 4 | 🙈🙈✅✅ |
| properties.json | 1 | 1 | 1 | 1 | 1 | 1 | 0 | 6 | 🚧🙈❌🦀✅🤪 |
| propertyNames.json | 1 | 5 | 0 | 0 | 0 | 0 | 0 | 6 | 🙈🙈✅🙈🙈🙈 |
| ref.json | 1 | 1 | 0 | 1 | 13 | 0 | 20 | 36 | 🤪🦀🙈🦀🤯🦀🤯✅🦀🤯🤯🦀🦀🤯🤯🤯🤯🤯🤯🤯🤯🦀🦀🦀🦀🦀🦀🦀🤯🤯🤯🤯🤯🤯🤯🤯 |
| refRemote.json | 0 | 1 | 0 | 0 | 4 | 0 | 10 | 15 | 🤯🤯🤯🤯🙈🦀🦀🦀🤯🤯🦀🤯🤯🤯🤯 |
| required.json | 2 | 2 | 1 | 0 | 0 | 0 | 0 | 5 | 🚧✅✅🙈🙈 |
| type.json | 6 | 4 | 1 | 0 | 0 | 0 | 0 | 11 | 🚧✅✅✅✅✅🙈🙈✅🙈🙈 |
| unevaluatedItems.json | 5 | 17 | 0 | 0 | 0 | 4 | 3 | 29 | ✅🙈🙈✅🙈❌🙈🙈❌❌✅🙈🙈🙈🙈❌🤯🤯🤯🙈🙈🙈🙈🙈🙈✅✅🙈🙈 |
| unevaluatedProperties.json | 9 | 25 | 0 | 0 | 0 | 7 | 3 | 44 | ✅🙈🙈🙈🙈✅✅🙈🙈✅✅🙈🙈🙈🙈🙈🙈🙈❌🤯🤯🤯❌❌✅✅🙈🙈🙈🙈🙈❌❌🙈❌❌✅✅🙈🙈🙈🙈🙈🙈 |
| uniqueItems.json | 2 | 2 | 0 | 0 | 0 | 2 | 0 | 6 | 🙈🙈❌✅✅❌ |
| vocabulary.json | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 | ❌✅ |
| **Total** | **80** | **168** | **3** | **3** | **22** | **42** | **66** | **384** |  |

#### optional

| File | ✅ | 🙈 | 🚧 | 🦀 | ❌ | 🤯 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| optional/anchor.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/bignum.json | 4 | 0 | 1 | 0 | 2 | 0 | 7 | 🚧✅✅✅❌✅❌ |
| optional/cross-draft.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/dependencies-compatibility.json | 1 | 6 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| optional/dynamicRef.json | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 🦀 |
| optional/ecmascript-regex.json | 0 | 15 | 0 | 0 | 5 | 0 | 20 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈❌❌❌❌❌ |
| optional/float-overflow.json | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 🚧 |
| optional/format-assertion.json | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/date-time.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/date.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/duration.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ecmascript-regex.json | 3 | 3 | 0 | 0 | 0 | 0 | 6 | 🙈🙈🙈✅✅✅ |
| optional/format/email.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/idn-email.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/idn-hostname.json | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/format/ipv4.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/ipv6.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/iri.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/regex.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/relative-json-pointer.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/time.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/unknown.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| optional/format/uri-reference.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri-template.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uri.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/format/uuid.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/id.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| optional/no-schema.json | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 🙈 |
| optional/non-bmp-regex.json | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 🙈🙈 |
| optional/refOfUnknownKeyword.json | 0 | 0 | 0 | 4 | 0 | 1 | 5 | 🦀🦀🦀🤯🦀 |
| optional/unknownKeyword.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 🤯 |
| **Total** | **9** | **50** | **2** | **5** | **7** | **5** | **78** |  |

## Failures

### 🤯 `generation_panic` (223)

100 distinct first-line errors; most common:

- **10×** `Expected string for enum got '1' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- **10×** `Expected string for enum got '6' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- **8×** `Expected definition: '#/$defs/bar' $defs at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`
- **5×** `Expected definition: '#foo' #foo at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`
- **5×** `Expected definition: 'list' list at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`

Per group:

- `draft4/definitions.json::validate definition against metaschema` — `Expected definition: 'http://json-schema.org/draft-04/schema#' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/l…`
- `draft4/enum.json::simple enum validation` — `Expected string for enum got '1' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::heterogeneous enum validation` — `Expected string for enum got '6' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::heterogeneous enum-with-null validation` — `Expected string for enum got '6' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::enum with false does not match 0` — `Expected string for enum got 'false' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::enum with [false] does not match [0]` — `Expected string for enum got '[false]' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::enum with true does not match 1` — `Expected string for enum got 'true' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::enum with [true] does not match [1]` — `Expected string for enum got '[true]' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::enum with 0 does not match false` — `Expected string for enum got '0' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::enum with [0] does not match [false]` — `Expected string for enum got '[0]' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::enum with 1 does not match true` — `Expected string for enum got '1' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/enum.json::enum with [1] does not match [true]` — `Expected string for enum got '[1]' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft4/ref.json::$ref prevents a sibling id from changing the base uri` — `Expected definition: 'foo.json' foo.json at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`
- `draft4/ref.json::remote ref, containing refs itself` — `Expected definition: 'http://json-schema.org/draft-04/schema#' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/l…`
- `draft4/ref.json::Location-independent identifier` — `Expected definition: '#foo' #foo at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`
- `draft4/ref.json::Location-independent identifier with base URI change in subschema` — `Expected definition: 'http://localhost:1234/nested.json#foo' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib…`
- `draft4/ref.json::naive replacement of $ref with its destination is not correct` — `Expected string for enum got '{"$ref":"#/definitions/a_string"}' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.r…`
- `draft4/ref.json::id must be resolved against nearest parent, not just immediate parent` — `Expected definition: 'http://example.com/b/d.json' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`
- `draft4/ref.json::empty tokens in $ref json-pointer` — `Ident is not allowed to be empty; use Option<Ident> at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:595`
- `draft4/refRemote.json::remote ref` — `Expected definition: 'http://localhost:1234/integer.json' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs…`
- `draft4/refRemote.json::fragment within remote ref` — `Expected definition: 'http://localhost:1234/draft4/subSchemas.json#/definitions/integer' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557…`
- `draft4/refRemote.json::ref within remote ref` — `Expected definition: 'http://localhost:1234/draft4/subSchemas.json#/definitions/refToInteger' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b…`
- `draft4/refRemote.json::Location-independent identifier in remote ref` — `Expected definition: 'http://localhost:1234/draft4/locationIndependentIdentifier.json#/definitions/refToInteger' http: at /Users/ahl/.cargo/registry/src/index.c…`
- `draft6/definitions.json::validate definition against metaschema` — `Expected definition: 'http://json-schema.org/draft-06/schema#' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/l…`
- `draft6/enum.json::simple enum validation` — `Expected string for enum got '1' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::heterogeneous enum validation` — `Expected string for enum got '6' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::heterogeneous enum-with-null validation` — `Expected string for enum got '6' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::enum with false does not match 0` — `Expected string for enum got 'false' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::enum with [false] does not match [0]` — `Expected string for enum got '[false]' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::enum with true does not match 1` — `Expected string for enum got 'true' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::enum with [true] does not match [1]` — `Expected string for enum got '[true]' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::enum with 0 does not match false` — `Expected string for enum got '0' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::enum with [0] does not match [false]` — `Expected string for enum got '[0]' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::enum with 1 does not match true` — `Expected string for enum got '1' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/enum.json::enum with [1] does not match [true]` — `Expected string for enum got '[1]' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:696`
- `draft6/ref.json::$ref prevents a sibling $id from changing the base uri` — `Expected definition: 'foo.json' foo.json at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`
- `draft6/ref.json::remote ref, containing refs itself` — `Expected definition: 'http://json-schema.org/draft-06/schema#' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/l…`
- `draft6/ref.json::Location-independent identifier` — `Expected definition: '#foo' #foo at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`
- `draft6/ref.json::Reference an anchor with a non-relative URI` — `Expected definition: 'https://example.com/schema-with-anchor#foo' https: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/s…`
- `draft6/ref.json::Location-independent identifier with base URI change in subschema` — `Expected definition: 'http://localhost:1234/nested.json#foo' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib…`
- `draft6/ref.json::naive replacement of $ref with its destination is not correct` — `Expected string for enum got '{"$ref":"#/definitions/a_string"}' at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.r…`
- `draft6/ref.json::refs with relative uris and defs` — `Expected definition: 'schema-relative-uri-defs2.json' schema-relative-uri-defs2.json at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy…`
- `draft6/ref.json::relative refs with absolute uris and defs` — `Expected definition: 'schema-refs-absolute-uris-defs2.json' schema-refs-absolute-uris-defs2.json at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b5…`
- `draft6/ref.json::ref with absolute-path-reference` — `Expected definition: '/absref/foobar.json'  at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:410`
- `draft6/ref.json::empty tokens in $ref json-pointer` — `Ident is not allowed to be empty; use Option<Ident> at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:595`
- `draft6/refRemote.json::remote ref` — `Expected definition: 'http://localhost:1234/integer.json' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs…`
- `draft6/refRemote.json::fragment within remote ref` — `Expected definition: 'http://localhost:1234/draft6/subSchemas.json#/definitions/integer' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557…`
- `draft6/refRemote.json::ref within remote ref` — `Expected definition: 'http://localhost:1234/draft6/subSchemas.json#/definitions/refToInteger' http: at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b…`
- `draft6/refRemote.json::remote ref with ref to definitions` — `Expected definition: 'draft6/ref-and-definitions.json' draft6 at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/schemafy_lib-0.6.0/src/lib.rs:4…`
- `draft6/refRemote.json::Location-independent identifier in remote ref` — `Expected definition: 'http://localhost:1234/draft6/locationIndependentIdentifier.json#/definitions/refToInteger' http: at /Users/ahl/.cargo/registry/src/index.c…`
- … and 173 more

### ❌ `generation_error` (170)

9 distinct first-line errors; most common:

- **95×** `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- **32×** `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- **12×** `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- **9×** `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- **6×** `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected one or many`

Per group:

- `draft6/allOf.json::allOf with boolean schemas, all true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/allOf.json::allOf with boolean schemas, some false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/allOf.json::allOf with boolean schemas, all false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft6/anyOf.json::anyOf with boolean schemas, all true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/anyOf.json::anyOf with boolean schemas, some true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/anyOf.json::anyOf with boolean schemas, all false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft6/boolean_schema.json::boolean schema 'true'` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/boolean_schema.json::boolean schema 'false'` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft6/exclusiveMaximum.json::exclusiveMaximum validation` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '3.0', expected a boolean`
- `draft6/exclusiveMinimum.json::exclusiveMinimum validation` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '1.1', expected a boolean`
- `draft6/items.json::items with boolean schema (true)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected one or many`
- `draft6/items.json::items with boolean schema (false)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- `draft6/items.json::items with boolean schemas` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/maxItems.json::maxItems validation with a decimal` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `draft6/maxLength.json::maxLength validation with a decimal` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `draft6/maxProperties.json::maxProperties validation with a decimal` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `draft6/not.json::forbid everything with boolean schema true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/not.json::allow everything with boolean schema false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft6/oneOf.json::oneOf with boolean schemas, all true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/oneOf.json::oneOf with boolean schemas, one true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/oneOf.json::oneOf with boolean schemas, more than one true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/oneOf.json::oneOf with boolean schemas, all false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft6/oneOf.json::oneOf with missing optional property` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/patternProperties.json::patternProperties with boolean schemas` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft6/properties.json::properties with boolean schema` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft6/ref.json::$ref to boolean schema true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft6/ref.json::$ref to boolean schema false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft7/allOf.json::allOf with boolean schemas, all true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/allOf.json::allOf with boolean schemas, some false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/allOf.json::allOf with boolean schemas, all false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft7/anyOf.json::anyOf with boolean schemas, all true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/anyOf.json::anyOf with boolean schemas, some true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/anyOf.json::anyOf with boolean schemas, all false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft7/boolean_schema.json::boolean schema 'true'` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/boolean_schema.json::boolean schema 'false'` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft7/exclusiveMaximum.json::exclusiveMaximum validation` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '3.0', expected a boolean`
- `draft7/exclusiveMinimum.json::exclusiveMinimum validation` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '1.1', expected a boolean`
- `draft7/items.json::items with boolean schema (true)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected one or many`
- `draft7/items.json::items with boolean schema (false)` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected one or many`
- `draft7/items.json::items with boolean schemas` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/maxItems.json::maxItems validation with a decimal` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `draft7/maxLength.json::maxLength validation with a decimal` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `draft7/maxProperties.json::maxProperties validation with a decimal` — `schema did not parse as schemafy_lib::Schema: invalid type: floating point '2.0', expected i64`
- `draft7/not.json::forbid everything with boolean schema true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/not.json::allow everything with boolean schema false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft7/oneOf.json::oneOf with boolean schemas, all true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/oneOf.json::oneOf with boolean schemas, one true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/oneOf.json::oneOf with boolean schemas, more than one true` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- `draft7/oneOf.json::oneOf with boolean schemas, all false` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'false', expected struct Schema`
- `draft7/oneOf.json::oneOf with missing optional property` — `schema did not parse as schemafy_lib::Schema: invalid type: boolean 'true', expected struct Schema`
- … and 120 more

### 🦀 `compile_fail` (85)

21 distinct first-line errors; most common:

- **12×** `error[E0425]: cannot find type 'Bar' in this scope`
- **6×** `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- **5×** `error[E0124]: field 'foo_bar' is already declared`
- **5×** `error[E0425]: cannot find type 'Foo22Bar' in this scope`
- **5×** `error[E0425]: cannot find type 'Foo' in this scope`

Per group:

- `draft4/enum.json::enum with escaped characters` — `error[E0428]: the name 'FooBar' is defined multiple times`
- `draft4/properties.json::properties with escaped characters` — `error[E0124]: field 'foo_bar' is already declared`
- `draft4/ref.json::relative pointer ref to object` — `error[E0425]: cannot find type 'Foo' in this scope`
- `draft4/ref.json::escaped pointer ref` — `error[E0425]: cannot find type 'Percent25Field' in this scope`
- `draft4/ref.json::Recursive references between schemas` — `error[E0425]: cannot find type 'Tree' in this scope`
- `draft4/ref.json::refs with quote` — `error[E0425]: cannot find type 'Foo22Bar' in this scope`
- `draft4/refRemote.json::base URI change - change folder` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `draft4/refRemote.json::base URI change - change folder in subschema` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `draft4/refRemote.json::root ref in remote ref` — `error[E0425]: cannot find type 'OrNull' in this scope`
- `draft6/enum.json::enum with escaped characters` — `error[E0428]: the name 'FooBar' is defined multiple times`
- `draft6/properties.json::properties with escaped characters` — `error[E0124]: field 'foo_bar' is already declared`
- `draft6/ref.json::relative pointer ref to object` — `error[E0425]: cannot find type 'Foo' in this scope`
- `draft6/ref.json::escaped pointer ref` — `error[E0425]: cannot find type 'Percent25Field' in this scope`
- `draft6/ref.json::Recursive references between schemas` — `error[E0425]: cannot find type 'Tree' in this scope`
- `draft6/ref.json::refs with quote` — `error[E0425]: cannot find type 'Foo22Bar' in this scope`
- `draft6/ref.json::simple URN base URI with $ref via the URN` — `error[E0425]: cannot find type 'UrnUuidDeadbeef1234FfffFfff4321Feebdaed' in this scope`
- `draft6/ref.json::URN base URI with URN and anchor ref` — `error[E0425]: cannot find type 'Something' in this scope`
- `draft6/refRemote.json::base URI change - change folder` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `draft6/refRemote.json::base URI change - change folder in subschema` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `draft6/refRemote.json::root ref in remote ref` — `error[E0425]: cannot find type 'OrNull' in this scope`
- `draft6/refRemote.json::retrieved nested refs resolve relative to their URI not $id` — `error[E0425]: cannot find type 'FooRefStringJson' in this scope`
- `draft7/enum.json::enum with escaped characters` — `error[E0428]: the name 'FooBar' is defined multiple times`
- `draft7/properties.json::properties with escaped characters` — `error[E0124]: field 'foo_bar' is already declared`
- `draft7/ref.json::relative pointer ref to object` — `error[E0425]: cannot find type 'Foo' in this scope`
- `draft7/ref.json::escaped pointer ref` — `error[E0425]: cannot find type 'Percent25Field' in this scope`
- `draft7/ref.json::Recursive references between schemas` — `error[E0425]: cannot find type 'Tree' in this scope`
- `draft7/ref.json::refs with quote` — `error[E0425]: cannot find type 'Foo22Bar' in this scope`
- `draft7/ref.json::simple URN base URI with $ref via the URN` — `error[E0425]: cannot find type 'UrnUuidDeadbeef1234FfffFfff4321Feebdaed' in this scope`
- `draft7/ref.json::URN base URI with URN and anchor ref` — `error[E0425]: cannot find type 'Something' in this scope`
- `draft7/refRemote.json::base URI change - change folder` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `draft7/refRemote.json::base URI change - change folder in subschema` — `error[E0425]: cannot find type 'FolderIntegerJson' in this scope`
- `draft7/refRemote.json::root ref in remote ref` — `error[E0425]: cannot find type 'OrNull' in this scope`
- `draft7/refRemote.json::retrieved nested refs resolve relative to their URI not $id` — `error[E0425]: cannot find type 'FooRefStringJson' in this scope`
- `draft2019-09/enum.json::enum with escaped characters` — `error[E0428]: the name 'FooBar' is defined multiple times`
- `draft2019-09/infinite-loop-detection.json::evaluating the same schema location against the same data location twice is not a sign of an infinite loop` — `error[E0425]: cannot find type 'Int' in this scope`
- `draft2019-09/items.json::items and subitems` — `error[E0425]: cannot find type 'Item' in this scope`
- `draft2019-09/properties.json::properties with escaped characters` — `error[E0124]: field 'foo_bar' is already declared`
- `draft2019-09/ref.json::relative pointer ref to object` — `error[E0425]: cannot find type 'Foo' in this scope`
- `draft2019-09/ref.json::escaped pointer ref` — `error[E0425]: cannot find type 'Percent25Field' in this scope`
- `draft2019-09/ref.json::ref applies alongside sibling keywords` — `error[E0425]: cannot find type 'Reffed' in this scope`
- `draft2019-09/ref.json::property named $ref, containing an actual $ref` — `error[E0425]: cannot find type 'IsString' in this scope`
- `draft2019-09/ref.json::Recursive references between schemas` — `error[E0425]: cannot find type 'Node' in this scope`
- `draft2019-09/ref.json::refs with quote` — `error[E0425]: cannot find type 'Foo22Bar' in this scope`
- `draft2019-09/ref.json::simple URN base URI with $ref via the URN` — `error[E0425]: cannot find type 'UrnUuidDeadbeef1234FfffFfff4321Feebdaed' in this scope`
- `draft2019-09/ref.json::simple URN base URI with JSON pointer` — `error[E0425]: cannot find type 'Bar' in this scope`
- `draft2019-09/ref.json::URN base URI with NSS` — `error[E0425]: cannot find type 'Bar' in this scope`
- `draft2019-09/ref.json::URN base URI with r-component` — `error[E0425]: cannot find type 'Bar' in this scope`
- `draft2019-09/ref.json::URN base URI with q-component` — `error[E0425]: cannot find type 'Bar' in this scope`
- `draft2019-09/ref.json::URN base URI with URN and JSON pointer ref` — `error[E0425]: cannot find type 'Bar' in this scope`
- `draft2019-09/ref.json::URN base URI with URN and anchor ref` — `error[E0425]: cannot find type 'Something' in this scope`
- … and 35 more

### 🤪 `mixed` (15)

3 distinct first-line errors; most common:

- **5×** `accepted 'an additional property is invalid' though valid: false`
- **5×** `ignores arrays: invalid length 0, expected struct Root with 3 elements`
- **5×** `match: invalid type: boolean 'false', expected struct Root`

Per group:

- `draft4/additionalProperties.json::additionalProperties being false does not allow other properties` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft4/properties.json::properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), toString not valid (accepted)
- `draft4/ref.json::root pointer ref` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected), mismatch (accepted), recursive mismatch (accepted)
- `draft6/additionalProperties.json::additionalProperties being false does not allow other properties` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft6/properties.json::properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), toString not valid (accepted)
- `draft6/ref.json::root pointer ref` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected), mismatch (accepted), recursive mismatch (accepted)
- `draft7/additionalProperties.json::additionalProperties being false does not allow other properties` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft7/properties.json::properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), toString not valid (accepted)
- `draft7/ref.json::root pointer ref` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected), mismatch (accepted), recursive mismatch (accepted)
- `draft2019-09/additionalProperties.json::additionalProperties being false does not allow other properties` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft2019-09/properties.json::properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), toString not valid (accepted)
- `draft2019-09/ref.json::root pointer ref` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected), mismatch (accepted), recursive mismatch (accepted)
- `draft2020-12/additionalProperties.json::additionalProperties being false does not allow other properties` — `accepted 'an additional property is invalid' though valid: false`
  - cases: an additional property is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft2020-12/properties.json::properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected), toString not valid (accepted)
- `draft2020-12/ref.json::root pointer ref` — `match: invalid type: boolean 'false', expected struct Root`
  - cases: match (rejected), recursive match (rejected), mismatch (accepted), recursive mismatch (accepted)

### 🚧 `false_reject` (23)

4 distinct first-line errors; most common:

- **10×** `ignores arrays: invalid length 0, expected struct Root with 2 elements`
- **5×** `a bignum is an integer: invalid type: floating point '1.2345678910111214e+52', expected i64`
- **4×** `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
- **4×** `valid if optional overflow handling is implemented: invalid type: floating point '1e+308', expected i64`

Per group:

- `draft4/properties.json::object properties validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft4/required.json::required validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft6/properties.json::object properties validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft6/required.json::required validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft6/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft7/properties.json::object properties validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft7/required.json::required validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft7/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft2019-09/properties.json::object properties validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft2019-09/required.json::required validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft2019-09/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
- `draft2020-12/properties.json::object properties validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft2020-12/required.json::required validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft2020-12/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)
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

### 🙈 `false_accept` (830)

186 distinct first-line errors; most common:

- **27×** `accepted 'too short is invalid' though valid: false`
- **15×** `accepted 'anything non-null is invalid' though valid: false`
- **15×** `accepted 'does not match' though valid: false`
- **15×** `accepted 'mismatch base schema' though valid: false`
- **15×** `accepted 'number is invalid' though valid: false`

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
- `draft4/additionalProperties.json::non-ASCII pattern with additionalProperties` — `accepted 'not matching the pattern is invalid' though valid: false`
  - cases: not matching the pattern is invalid (accepted)
- `draft4/additionalProperties.json::additionalProperties with schema` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `draft4/additionalProperties.json::additionalProperties can exist by itself` — `accepted 'an additional invalid property is invalid' though valid: false`
  - cases: an additional invalid property is invalid (accepted)
- `draft4/additionalProperties.json::additionalProperties does not look in applicators` — `accepted 'properties defined in allOf are not examined' though valid: false`
  - cases: properties defined in allOf are not examined (accepted)
- `draft4/allOf.json::allOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted)
- `draft4/allOf.json::allOf simple types` — `accepted 'mismatch one' though valid: false`
  - cases: mismatch one (accepted)
- `draft4/allOf.json::allOf with the first empty schema` — `accepted 'string is invalid' though valid: false`
  - cases: string is invalid (accepted)
- `draft4/allOf.json::allOf with the last empty schema` — `accepted 'string is invalid' though valid: false`
  - cases: string is invalid (accepted)
- `draft4/allOf.json::nested allOf, to check validation semantics` — `accepted 'anything non-null is invalid' though valid: false`
  - cases: anything non-null is invalid (accepted)
- `draft4/allOf.json::allOf combined with anyOf, oneOf` — `accepted 'allOf: false, anyOf: false, oneOf: false' though valid: false`
  - cases: allOf: false, anyOf: false, oneOf: false (accepted), allOf: false, anyOf: false, oneOf: true (accepted), allOf: false, anyOf: true, oneOf: false (accepted), allOf: false, anyOf: true, oneOf: true (accepted), allOf: true, anyOf: false, oneOf: false (accepted), allOf: true, anyOf: false, oneOf: true (accepted), … and 1 more
- `draft4/anyOf.json::anyOf` — `accepted 'neither anyOf valid' though valid: false`
  - cases: neither anyOf valid (accepted)
- `draft4/anyOf.json::anyOf with base schema` — `accepted 'mismatch base schema' though valid: false`
  - cases: mismatch base schema (accepted), both anyOf invalid (accepted)
- `draft4/anyOf.json::anyOf complex types` — `accepted 'neither anyOf valid (complex)' though valid: false`
  - cases: neither anyOf valid (complex) (accepted)
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
- `draft4/enum.json::enums in properties` — `accepted 'wrong foo value' though valid: false`
  - cases: wrong foo value (accepted), wrong bar value (accepted)
- `draft4/items.json::a schema given for items` — `accepted 'wrong type of items' though valid: false`
  - cases: wrong type of items (accepted)
- `draft4/items.json::an array of schemas for items` — `accepted 'wrong types' though valid: false`
  - cases: wrong types (accepted)
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
- … and 780 more

