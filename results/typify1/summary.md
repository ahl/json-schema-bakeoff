# Conformance summary — typify1 (typify 0.10.0-alpha.2)

_generated 2026-10-11T03:47:42Z_

**80 of 258 non-optional groups pass (31.0%)**

| | bucket | meaning |
|---|---|---|
| ✅ | `pass` | every case matched |
| 🙈 | `false_accept` | deserialize succeeded but `valid: false` (too loose) |
| 🚫 | `false_reject` | deserialize failed but `valid: true` (too strict) |
| 🔀 | `mixed` | both false_accept and false_reject cases in one group |
| 🔥 | `runtime_panic` | a test case panicked during deserialize |
| 🧱 | `compile_fail` | generated code doesn't compile |
| ❌ | `generation_error` | typify returned an error |
| 💥 | `generation_panic` | typify panicked |

## Headline

### Groups

| Draft | ✅ | 🙈 | 🚫 | 🔀 | ❌ | 💥 | Total | Pass rate |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| draft7 | 80 | 36 | 16 | 5 | 32 | 89 | 258 | 31.0% |
| **Total** | **80** | **36** | **16** | **5** | **32** | **89** | **258** | **31.0%** |

### Per-case match rate

| Draft | Matched | Reached run | Rate (reached) | Total cases | Rate (all) |
|---|---:|---:|---:|---:|---:|
| draft7 | 395 | 516 | 76.6% | 929 | 42.5% |
| **Total** | **395** | **516** | **76.6%** | **929** | **42.5%** |

## Chart

### draft7

| File | ✅ | 🙈 | 🚫 | 🔀 | ❌ | 💥 | Total | Chart |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| additionalItems.json | 3 | 0 | 1 | 0 | 6 | 0 | 10 | ❌✅✅❌🚫❌❌❌❌✅ |
| additionalProperties.json | 6 | 0 | 1 | 0 | 0 | 0 | 7 | 🚫✅✅✅✅✅✅ |
| allOf.json | 10 | 0 | 0 | 0 | 0 | 2 | 12 | ✅✅💥✅✅✅✅✅✅✅✅💥 |
| anyOf.json | 1 | 1 | 1 | 0 | 2 | 3 | 8 | 💥❌🚫💥💥🙈❌✅ |
| boolean_schema.json | 2 | 0 | 0 | 0 | 0 | 0 | 2 | ✅✅ |
| const.json | 0 | 17 | 0 | 0 | 0 | 0 | 17 | 🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 |
| contains.json | 0 | 0 | 0 | 0 | 7 | 0 | 7 | ❌❌❌❌❌❌❌ |
| default.json | 1 | 1 | 0 | 0 | 1 | 0 | 3 | ❌✅🙈 |
| definitions.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 💥 |
| dependencies.json | 1 | 6 | 0 | 0 | 0 | 0 | 7 | 🙈✅🙈🙈🙈🙈🙈 |
| enum.json | 11 | 0 | 2 | 0 | 0 | 2 | 15 | ✅💥✅✅💥✅✅✅✅✅🚫✅🚫✅✅ |
| exclusiveMaximum.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 💥 |
| exclusiveMinimum.json | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 💥 |
| format.json | 0 | 0 | 0 | 0 | 0 | 17 | 17 | 💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥 |
| if-then-else.json | 0 | 0 | 0 | 0 | 0 | 12 | 12 | 💥💥💥💥💥💥💥💥💥💥💥💥 |
| infinite-loop-detection.json | 1 | 0 | 0 | 0 | 0 | 0 | 1 | ✅ |
| items.json | 4 | 0 | 1 | 0 | 4 | 0 | 9 | 🚫❌✅✅❌❌✅✅❌ |
| maxItems.json | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🔀❌ |
| maxLength.json | 0 | 0 | 1 | 0 | 1 | 0 | 2 | 🚫❌ |
| maxProperties.json | 0 | 1 | 0 | 1 | 1 | 0 | 3 | 🔀❌🙈 |
| maximum.json | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 💥💥 |
| minItems.json | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🔀❌ |
| minLength.json | 0 | 0 | 1 | 0 | 1 | 0 | 2 | 🚫❌ |
| minProperties.json | 0 | 0 | 0 | 1 | 1 | 0 | 2 | 🔀❌ |
| minimum.json | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 💥💥 |
| multipleOf.json | 2 | 0 | 0 | 0 | 0 | 3 | 5 | 💥💥💥✅✅ |
| not.json | 4 | 0 | 0 | 0 | 0 | 4 | 8 | 💥💥💥✅✅✅✅💥 |
| oneOf.json | 2 | 4 | 0 | 0 | 0 | 5 | 11 | 💥🙈💥💥💥💥✅🙈🙈🙈✅ |
| pattern.json | 1 | 0 | 1 | 0 | 0 | 0 | 2 | 🚫✅ |
| patternProperties.json | 1 | 3 | 1 | 0 | 0 | 0 | 5 | 🚫🙈🙈🙈✅ |
| properties.json | 1 | 1 | 2 | 1 | 0 | 1 | 6 | 🚫🔀🙈💥✅🚫 |
| propertyNames.json | 3 | 1 | 1 | 0 | 0 | 1 | 6 | 🚫✅✅💥🙈✅ |
| ref.json | 13 | 1 | 0 | 0 | 1 | 20 | 35 | 💥💥❌💥✅✅💥💥✅✅✅✅💥💥💥💥💥✅💥💥💥💥✅✅✅✅💥💥💥💥💥💥✅✅🙈 |
| refRemote.json | 0 | 0 | 0 | 0 | 0 | 11 | 11 | 💥💥💥💥💥💥💥💥💥💥💥 |
| required.json | 2 | 0 | 2 | 0 | 0 | 1 | 5 | 🚫✅✅💥🚫 |
| type.json | 10 | 0 | 1 | 0 | 0 | 0 | 11 | 🚫✅✅✅✅✅✅✅✅✅✅ |
| uniqueItems.json | 1 | 0 | 0 | 0 | 5 | 0 | 6 | ❌❌❌✅❌❌ |
| **Total** | **80** | **36** | **16** | **5** | **32** | **89** | **258** |  |

## Failures

### 💥 `generation_panic` (89)

39 distinct first-line errors; most common:

- **30×** `not yet implemented: invalid (or unexpected) schema:`
- **12×** `not implemented: if/then/else schemas are not supported: {`
- **4×** `internal error: entered unreachable code at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typify-impl-0.10.0-alpha.2/src/enums.rs:76`
- **4×** `not yet implemented: unhandled not schema Object(`
- **2×** `called 'Result::unwrap()' on an 'Err' value: DuplicateItemName { kind: "property", type_name: "Root", name: "foo_bar", axis: Rust } at /Users/ahl/.cargo/registr…`

Per group:

- `draft7/allOf.json::allOf simple types` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/allOf.json::allOf combined with anyOf, oneOf` — `assertion failed: any_of.is_none() || one_of.is_none() at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typify-impl-0.10.0-alpha.2/src/merge.r…`
- `draft7/anyOf.json::anyOf` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/anyOf.json::anyOf with boolean schemas, some true` — `internal error: entered unreachable code at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typify-impl-0.10.0-alpha.2/src/enums.rs:76`
- `draft7/anyOf.json::anyOf with boolean schemas, all false` — `not yet implemented at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typify-impl-0.10.0-alpha.2/src/enums.rs:80`
- `draft7/definitions.json::validate definition against metaschema` — `external references are not supported: http://json-schema.org/draft-07/schema# at /Users/ahl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typify-impl-0.…`
- `draft7/enum.json::heterogeneous enum validation` — `multiple implied types for an un-typed enum {Boolean, Object, Array, Number, String} [Number(6), String("foo"), Array [], Bool(true), Object {"foo": Number(12)}…`
- `draft7/enum.json::enum with escaped characters` — `Failed to make unique variant names for [foo`
- `draft7/exclusiveMaximum.json::exclusiveMaximum validation` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/exclusiveMinimum.json::exclusiveMinimum validation` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::email format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::idn-email format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::regex format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::ipv4 format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::ipv6 format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::idn-hostname format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::hostname format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::date format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::date-time format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::time format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::json-pointer format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::relative-json-pointer format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::iri format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::iri-reference format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::uri format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::uri-reference format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/format.json::uri-template format` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/if-then-else.json::ignore if without then or else` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::ignore then without if` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::ignore else without if` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::if and then without else` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::if and else without then` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::validate against correct branch, then vs else` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::non-interference across combined schemas` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::if with boolean schema true` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::if with boolean schema false` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::if appears at the end when serialized (keyword processing sequence)` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::then: false fails when condition matches` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/if-then-else.json::else: false fails when condition does not match` — `not implemented: if/then/else schemas are not supported: {`
- `draft7/maximum.json::maximum validation` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/maximum.json::maximum validation with unsigned integer` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/minimum.json::minimum validation` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/minimum.json::minimum validation with signed integer` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/multipleOf.json::by int` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/multipleOf.json::by number` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/multipleOf.json::by small number` — `not yet implemented: invalid (or unexpected) schema:`
- `draft7/not.json::not` — `not yet implemented: unhandled not schema Object(`
- `draft7/not.json::not multiple types` — `not yet implemented: unhandled not schema Object(`
- `draft7/not.json::not more complex schema` — `not yet implemented: unhandled not schema Object(`
- `draft7/not.json::double negation` — `not yet implemented: unhandled not schema Object(`
- … and 39 more

### ❌ `generation_error` (32)

7 distinct first-line errors; most common:

- **21×** `invalid schema for Root: unhandled array validation ArrayValidation {`
- **4×** `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- **2×** `'Root' flattens the property 'subtype_0', whose type does not serialize as an object; serde can flatten only a struct, an enum, or a map`
- **2×** `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '1.0', expected u32`
- **1×** `invalid schema for item: unhandled array validation ArrayValidation {`

Per group:

- `draft7/additionalItems.json::additionalItems as schema` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/additionalItems.json::array of items with no additionalItems permitted` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/additionalItems.json::additionalItems are allowed by default` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/additionalItems.json::additionalItems does not look in applicators, invalid case` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/additionalItems.json::items validation adjusts the starting index for additionalItems` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/additionalItems.json::additionalItems with heterogeneous array` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/anyOf.json::anyOf with base schema` — `'Root' flattens the property 'subtype_0', whose type does not serialize as an object; serde can flatten only a struct, an enum, or a map`
- `draft7/anyOf.json::anyOf with one empty schema` — `'Root' flattens the property 'subtype_0', whose type does not serialize as an object; serde can flatten only a struct, an enum, or a map`
- `draft7/contains.json::contains keyword validation` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/contains.json::contains keyword with const keyword` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/contains.json::contains keyword with boolean schema true` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/contains.json::contains keyword with boolean schema false` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/contains.json::items + contains` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/contains.json::contains with false if subschema` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/contains.json::contains with null instance elements` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/default.json::invalid type for default` — `value does not conform to the given schema`
- `draft7/items.json::an array of schemas for items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/items.json::items with boolean schemas` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/items.json::items and subitems` — `invalid schema for item: unhandled array validation ArrayValidation {`
- `draft7/items.json::array-form items with null instance elements` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/maxItems.json::maxItems validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `draft7/maxLength.json::maxLength validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `draft7/maxProperties.json::maxProperties validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `draft7/minItems.json::minItems validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '1.0', expected u32`
- `draft7/minLength.json::minLength validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '2.0', expected u32`
- `draft7/minProperties.json::minProperties validation with a decimal` — `schema did not parse as schemars::schema::RootSchema: invalid type: floating point '1.0', expected u32`
- `draft7/ref.json::relative pointer ref to array` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/uniqueItems.json::uniqueItems validation` — `type 'serde_json::Value' (id '1') cannot implement the required trait 'Ord'`
- `draft7/uniqueItems.json::uniqueItems with an array of items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/uniqueItems.json::uniqueItems with an array of items and additionalItems=false` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/uniqueItems.json::uniqueItems=false with an array of items` — `invalid schema for Root: unhandled array validation ArrayValidation {`
- `draft7/uniqueItems.json::uniqueItems=false with an array of items and additionalItems=false` — `invalid schema for Root: unhandled array validation ArrayValidation {`

### 🔀 `mixed` (5)

- `draft7/maxItems.json::maxItems validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores non-arrays (rejected)
- `draft7/maxProperties.json::maxProperties validation` — `accepted 'too long is invalid' though valid: false`
  - cases: too long is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft7/minItems.json::minItems validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores non-arrays (rejected)
- `draft7/minProperties.json::minProperties validation` — `accepted 'too short is invalid' though valid: false`
  - cases: too short is invalid (accepted), ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores booleans (rejected)
- `draft7/properties.json::properties, patternProperties, additionalProperties interaction` — `accepted 'property invalidates property' though valid: false`
  - cases: property invalidates property (accepted), patternProperty invalidates property (accepted), patternProperty validates nonproperty (rejected)

### 🚫 `false_reject` (16)

12 distinct first-line errors; most common:

- **2×** `ignores arrays: invalid length 0, expected struct Root with 2 elements`
- **2×** `ignores arrays: invalid length 0, expected struct Root with 3 elements`
- **2×** `ignores arrays: invalid type: sequence, expected a map`
- **2×** `ignores non-arrays: invalid type: map, expected a sequence`
- **1×** `[0.0] is valid: invalid value`

Per group:

- `draft7/additionalItems.json::additionalItems as false without items` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected)
- `draft7/additionalProperties.json::additionalProperties being false does not allow other properties` — `ignores arrays: invalid length 3, expected fewer elements in array`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), patternProperties are not additional properties (rejected)
- `draft7/anyOf.json::anyOf with boolean schemas, all true` — `any value is valid: invalid type: string "foo", expected struct Root`
  - cases: any value is valid (rejected)
- `draft7/enum.json::enum with [0] does not match [false]` — `[0.0] is valid: invalid value`
  - cases: [0.0] is valid (rejected)
- `draft7/enum.json::enum with [1] does not match [true]` — `[1.0] is valid: invalid value`
  - cases: [1.0] is valid (rejected)
- `draft7/items.json::a schema given for items` — `ignores non-arrays: invalid type: map, expected a sequence`
  - cases: ignores non-arrays (rejected), JavaScript pseudo-array is valid (rejected)
- `draft7/maxLength.json::maxLength validation` — `ignores non-strings: invalid type: integer '100', expected a string`
  - cases: ignores non-strings (rejected)
- `draft7/minLength.json::minLength validation` — `ignores non-strings: invalid type: integer '1', expected a string`
  - cases: ignores non-strings (rejected)
- `draft7/pattern.json::pattern validation` — `ignores booleans: invalid type: boolean 'true', expected a string`
  - cases: ignores booleans (rejected), ignores integers (rejected), ignores floats (rejected), ignores objects (rejected), ignores arrays (rejected), ignores null (rejected)
- `draft7/patternProperties.json::patternProperties validates properties matching a regex` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected)
- `draft7/properties.json::object properties validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft7/properties.json::properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft7/propertyNames.json::propertyNames validation` — `ignores arrays: invalid type: sequence, expected a map`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores booleans (rejected)
- `draft7/required.json::required validation` — `ignores arrays: invalid length 0, expected struct Root with 2 elements`
  - cases: ignores arrays (rejected), ignores strings (rejected), ignores other non-objects (rejected), ignores null (rejected), ignores boolean (rejected)
- `draft7/required.json::required properties whose names are Javascript object property names` — `ignores arrays: invalid length 0, expected struct Root with 3 elements`
  - cases: ignores arrays (rejected), ignores other non-objects (rejected)
- `draft7/type.json::integer type matches integers` — `a float with zero fractional part is an integer: invalid type: floating point '1.0', expected i64`
  - cases: a float with zero fractional part is an integer (rejected)

### 🙈 `false_accept` (36)

33 distinct first-line errors; most common:

- **2×** `accepted 'both oneOf valid' though valid: false`
- **2×** `accepted 'both valid - invalid' though valid: false`
- **2×** `accepted 'missing dependency' though valid: false`
- **1×** `accepted '[0] is invalid' though valid: false`
- **1×** `accepted '[1] is invalid' though valid: false`

Per group:

- `draft7/anyOf.json::anyOf complex types` — `accepted 'neither anyOf valid (complex)' though valid: false`
  - cases: neither anyOf valid (complex) (accepted)
- `draft7/const.json::const validation` — `accepted 'another value is invalid' though valid: false`
  - cases: another value is invalid (accepted), another type is invalid (accepted)
- `draft7/const.json::const with object` — `accepted 'another object is invalid' though valid: false`
  - cases: another object is invalid (accepted), another type is invalid (accepted)
- `draft7/const.json::const with array` — `accepted 'another array item is invalid' though valid: false`
  - cases: another array item is invalid (accepted), array with additional items is invalid (accepted)
- `draft7/const.json::const with null` — `accepted 'not null is invalid' though valid: false`
  - cases: not null is invalid (accepted)
- `draft7/const.json::const with false does not match 0` — `accepted 'integer zero is invalid' though valid: false`
  - cases: integer zero is invalid (accepted), float zero is invalid (accepted)
- `draft7/const.json::const with true does not match 1` — `accepted 'integer one is invalid' though valid: false`
  - cases: integer one is invalid (accepted), float one is invalid (accepted)
- `draft7/const.json::const with [false] does not match [0]` — `accepted '[0] is invalid' though valid: false`
  - cases: [0] is invalid (accepted), [0.0] is invalid (accepted)
- `draft7/const.json::const with [true] does not match [1]` — `accepted '[1] is invalid' though valid: false`
  - cases: [1] is invalid (accepted), [1.0] is invalid (accepted)
- `draft7/const.json::const with {"a": false} does not match {"a": 0}` — `accepted '{"a": 0} is invalid' though valid: false`
  - cases: {"a": 0} is invalid (accepted), {"a": 0.0} is invalid (accepted)
- `draft7/const.json::const with {"a": true} does not match {"a": 1}` — `accepted '{"a": 1} is invalid' though valid: false`
  - cases: {"a": 1} is invalid (accepted), {"a": 1.0} is invalid (accepted)
- `draft7/const.json::const with 0 does not match other zero-like types` — `accepted 'false is invalid' though valid: false`
  - cases: false is invalid (accepted), empty object is invalid (accepted), empty array is invalid (accepted), empty string is invalid (accepted)
- `draft7/const.json::const with 1 does not match true` — `accepted 'true is invalid' though valid: false`
  - cases: true is invalid (accepted)
- `draft7/const.json::const with -2.0 matches integer and float types` — `accepted 'integer 2 is invalid' though valid: false`
  - cases: integer 2 is invalid (accepted), float 2.0 is invalid (accepted), float -2.00001 is invalid (accepted)
- `draft7/const.json::float and integers are equal up to 64-bit representation limits` — `accepted 'integer minus one is invalid' though valid: false`
  - cases: integer minus one is invalid (accepted), float minus one is invalid (accepted)
- `draft7/const.json::nul characters in strings` — `accepted 'do not match string lacking nul' though valid: false`
  - cases: do not match string lacking nul (accepted)
- `draft7/const.json::characters with the same visual representation but different codepoint` — `accepted 'character looks the same but uses a different codepoint' though valid: false`
  - cases: character looks the same but uses a different codepoint (accepted)
- `draft7/const.json::characters with the same visual representation, but different number of codepoints` — `accepted 'character looks the same but uses combining marks' though valid: false`
  - cases: character looks the same but uses combining marks (accepted)
- `draft7/default.json::the default keyword does not do anything if the property is missing` — `accepted 'an explicit property value is checked against maximum (failing)' though valid: false`
  - cases: an explicit property value is checked against maximum (failing) (accepted)
- `draft7/dependencies.json::dependencies` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted)
- `draft7/dependencies.json::multiple dependencies` — `accepted 'missing dependency' though valid: false`
  - cases: missing dependency (accepted), missing other dependency (accepted), missing both dependencies (accepted)
- `draft7/dependencies.json::multiple dependencies subschema` — `accepted 'wrong type' though valid: false`
  - cases: wrong type (accepted), wrong type other (accepted), wrong type both (accepted)
- `draft7/dependencies.json::dependencies with boolean subschemas` — `accepted 'object with property having schema false is invalid' though valid: false`
  - cases: object with property having schema false is invalid (accepted), object with both properties is invalid (accepted)
- `draft7/dependencies.json::dependencies with escaped characters` — `accepted 'invalid object 1' though valid: false`
  - cases: invalid object 1 (accepted), invalid object 2 (accepted), invalid object 3 (accepted), invalid object 4 (accepted)
- `draft7/dependencies.json::dependent subschema incompatible with root` — `accepted 'matches root' though valid: false`
  - cases: matches root (accepted), matches both (accepted)
- `draft7/maxProperties.json::maxProperties = 0 means the object is empty` — `accepted 'one property is invalid' though valid: false`
  - cases: one property is invalid (accepted)
- `draft7/oneOf.json::oneOf with base schema` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft7/oneOf.json::oneOf with empty schema` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `draft7/oneOf.json::oneOf with required` — `accepted 'both valid - invalid' though valid: false`
  - cases: both valid - invalid (accepted)
- `draft7/oneOf.json::oneOf with missing optional property` — `accepted 'both oneOf valid' though valid: false`
  - cases: both oneOf valid (accepted)
- `draft7/patternProperties.json::multiple simultaneous patternProperties are validated` — `accepted 'an invalid due to one is invalid' though valid: false`
  - cases: an invalid due to one is invalid (accepted), an invalid due to the other is invalid (accepted), an invalid due to both is invalid (accepted)
- `draft7/patternProperties.json::regexes are not anchored by default and are case sensitive` — `accepted 'recognized members are accounted for' though valid: false`
  - cases: recognized members are accounted for (accepted), regexes are case sensitive, 2 (accepted)
- `draft7/patternProperties.json::patternProperties with boolean schemas` — `accepted 'object with property matching schema false is invalid' though valid: false`
  - cases: object with property matching schema false is invalid (accepted), object with both properties is invalid (accepted), object with a property matching both true and false is invalid (accepted)
- `draft7/properties.json::properties with boolean schema` — `accepted 'only 'false' property present is invalid' though valid: false`
  - cases: only 'false' property present is invalid (accepted), both properties present is invalid (accepted)
- `draft7/propertyNames.json::propertyNames with const` — `accepted 'object with any other property is invalid' though valid: false`
  - cases: object with any other property is invalid (accepted)
- `draft7/ref.json::empty tokens in $ref json-pointer` — `accepted 'non-number is invalid' though valid: false`
  - cases: non-number is invalid (accepted)

