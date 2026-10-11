//go:build bakeoff_batch_deps

// The generated code's own dependencies, blank-imported so that `go mod
// tidy` keeps them in go.mod and their hashes in go.sum. Nothing compiles
// this file (the build tag is never set); it exists so that the batch
// module can require these from the module cache with the proxy off, the
// way rust-batch provides serde and regress to generated Rust.
//
// ogen writes code that imports into the *consuming* module, so these are
// the modules a real ogen user has in go.mod too:
//
//   - go-faster/jx is the JSON codec every generated Encode/Decode is
//     written against.
//   - go-faster/errors is what the decoders wrap errors with.
//   - ogen/validate holds the String/Int/Array validators `Validate`
//     calls, and ogen/ogenregex the ECMA-262 regexp engine `pattern`
//     compiles to. Both are packages of the ogen module itself, which the
//     tool pin already covers; they are listed so that `go mod tidy`
//     cannot decide the ogen module is only a tool dependency.
//   - ogen/json and ogen/uri render some `format` values (date, time,
//     duration, ...).
//   - google/uuid and shopspring/decimal are the types `format: uuid` and
//     some number formats render to.
//
// Add an entry here when a batch build reports a module the generated code
// imports and the batch module can't resolve; `build` matches imports
// against go.mod's require list, so no other code needs to change.

package main

import (
	_ "github.com/go-faster/errors"
	_ "github.com/go-faster/jx"
	_ "github.com/google/uuid"
	_ "github.com/ogen-go/ogen/json"
	_ "github.com/ogen-go/ogen/ogenregex"
	_ "github.com/ogen-go/ogen/uri"
	_ "github.com/ogen-go/ogen/validate"
	_ "github.com/shopspring/decimal"
)
