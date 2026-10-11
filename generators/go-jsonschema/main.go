// Command bakeoff-go-jsonschema is the bakeoff glue for go-jsonschema
// (github.com/omissis/go-jsonschema, module path
// github.com/atombender/go-jsonschema). It implements the three commands of
// the generator contract in DESIGN.md:
//
//	version                                 → "go-jsonschema vX.Y.Z"
//	generate <out_dir>                      → stdin [{key, draft, schema}];
//	                                          writes <out_dir>/<key>.go;
//	                                          stdout [{key, status, message?}]
//	build <gen_dir> <batch_dir>             → stdin [key]; stdout
//	                                          {outcomes, runner}
//
// plus one internal command, `worker`, which is how `generate` survives a
// generation that takes the process down (see generate.go).
//
// The tool is used as a library (`pkg/generator`), not as a subprocess: the
// CLI in its main.go is a thin cobra wrapper that aborts the whole run on
// the first schema that fails, which would be useless here. The library
// gives one fresh generator per schema in one long-lived process, so an
// error or a panic is attributed to the schema that caused it.
//
// The version comes from the pinned module and nowhere else: the build info
// of this very binary, falling back to the require line in go.mod.
package main

import (
	"encoding/json"
	"fmt"
	"os"
	"runtime/debug"
	"strings"
)

// The module that is pinned in go.mod and whose version we report.
const toolModule = "github.com/atombender/go-jsonschema"

func main() {
	args := os.Args[1:]
	if len(args) == 0 {
		usage()
	}
	switch args[0] {
	case "version":
		writeAll(os.Stdout, []byte("go-jsonschema "+toolVersion()+"\n"))
	case "generate":
		if len(args) != 2 {
			usage()
		}
		cmdGenerate(args[1])
	case "worker":
		if len(args) != 4 {
			usage()
		}
		cmdWorker(args[1], args[2], args[3])
	case "build":
		if len(args) != 3 {
			usage()
		}
		cmdBuild(args[1], args[2])
	default:
		usage()
	}
}

func usage() {
	fmt.Fprint(os.Stderr, `usage:
  go run . version
  go run . generate <out_dir>              # stdin: [{key, draft, schema}]
  go run . build <gen_dir> <batch_dir>     # stdin: [key]
  go run . worker <plan.json> <out_dir> <start>   # internal
`)
	os.Exit(2)
}

// toolVersion is the pinned version of go-jsonschema, read from this
// binary's own build info so that go.mod stays the single source of truth.
// `go run` records the module graph, so the lookup normally succeeds; the
// go.mod fallback covers a build that strips it.
func toolVersion() string {
	if bi, ok := debug.ReadBuildInfo(); ok {
		for _, dep := range bi.Deps {
			if dep != nil && dep.Path == toolModule {
				if dep.Replace != nil && dep.Replace.Version != "" {
					return dep.Replace.Version
				}
				if dep.Version != "" {
					return dep.Version
				}
			}
		}
	}
	if v, ok := modulePins()[toolModule]; ok {
		return v
	}
	fatalf("no require line for %s in go.mod", toolModule)
	return ""
}

// modulePins is every module version this glue requires, read from go.mod:
// the tool itself plus the modules the *generated* code can import (see
// batchdeps.go), which the batch module borrows. Both `require path vX` and
// the parenthesized block are recognized, and `// indirect` is ignored.
func modulePins() map[string]string {
	pins := map[string]string{}
	for _, line := range goModLines() {
		fields := strings.Fields(strings.TrimPrefix(line, "require "))
		if len(fields) >= 2 && strings.HasPrefix(fields[1], "v") && strings.Contains(fields[0], ".") {
			pins[fields[0]] = fields[1]
		}
	}
	return pins
}

// glueGoDirective is this module's own `go` directive, which a batch with
// dependencies adopts.
func glueGoDirective() string {
	for _, line := range goModLines() {
		if v, ok := strings.CutPrefix(line, "go "); ok {
			return strings.TrimSpace(v)
		}
	}
	fatalf("no go directive in go.mod")
	return ""
}

// goModLines is go.mod's content, comments and blanks stripped. The file
// is read from the working directory, which the harness sets to this
// generator's directory.
func goModLines() []string {
	text, err := os.ReadFile("go.mod")
	if err != nil {
		fatalf("reading go.mod: %v", err)
	}
	var out []string
	for _, raw := range strings.Split(string(text), "\n") {
		line := strings.TrimSpace(raw)
		if i := strings.Index(line, "//"); i >= 0 {
			line = strings.TrimSpace(line[:i])
		}
		if line != "" && line != ")" {
			out = append(out, line)
		}
	}
	return out
}

// writeAll writes every byte, looping over short writes. os.File.Write
// already loops internally, but the protocol payloads here are unbounded
// (a build result can carry hundreds of 4 KB diagnostics) and DESIGN.md
// asks for the loop explicitly.
func writeAll(f *os.File, b []byte) {
	for len(b) > 0 {
		n, err := f.Write(b)
		if n > 0 {
			b = b[n:]
			continue
		}
		if err != nil {
			fatalf("writing to %s: %v", f.Name(), err)
		}
	}
}

// emitJSON writes one JSON value followed by a newline, flushed.
func emitJSON(f *os.File, v any) {
	line, err := json.Marshal(v)
	if err != nil {
		fatalf("marshalling a protocol line: %v", err)
	}
	writeAll(f, append(line, '\n'))
	_ = f.Sync()
}

func readStdin() []byte {
	var sb strings.Builder
	buf := make([]byte, 1<<16)
	for {
		n, err := os.Stdin.Read(buf)
		if n > 0 {
			sb.Write(buf[:n])
		}
		if err != nil {
			break
		}
	}
	return []byte(sb.String())
}

func fatalf(format string, args ...any) {
	fmt.Fprintf(os.Stderr, "go-jsonschema glue: "+format+"\n", args...)
	os.Exit(1)
}

// truncate caps text that ends up in results.json.
func truncate(s string, max int) string {
	if len(s) <= max {
		return s
	}
	return s[:max] + "\n... (truncated)"
}
