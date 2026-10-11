//go:build bakeoff_batch_deps

// The generated code's own dependencies, blank-imported so that `go mod
// tidy` keeps them in go.mod and their hashes in go.sum. Nothing compiles
// this file (the build tag is never set); it exists so that the batch
// module can require these from the module cache with the proxy off, the
// way rust-batch provides serde and regress to generated Rust.
//
//   - pkg/types holds SerializableDate/SerializableTime, which `format:
//     date` and `format: time` render to.
//   - mapstructure is what a typed `additionalProperties` decodes through.
//     The tool emits the import without depending on the module itself, so
//     this is the only place it is pinned.
//
// Add an entry here when a batch build reports a module the generated code
// imports and the batch module can't resolve; `build` matches imports
// against go.mod's require list, so no other code needs to change.

package main

import (
	_ "github.com/atombender/go-jsonschema/pkg/types"
	_ "github.com/go-viper/mapstructure/v2"
)
