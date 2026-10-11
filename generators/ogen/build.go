package main

// The build step. `build <gen_dir> <batch_dir>` reads the module keys on
// stdin, assembles a single Go module at `<batch_dir>` with one package
// directory per key plus a `main.go` runner that imports the live ones,
// and converges: `go build`, blame each diagnostic on the package
// directory it names, drop those modules, build again. The shape is
// go-jsonschema's and quicktype's go target's — a compiler only reports so
// much before it gives up, so one pass can't find every broken module.

import (
	"encoding/json"
	"fmt"
	"go/parser"
	"go/token"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

// Module path of the batch module; package dirs hang off it.
const batchModule = "bakeoff"

// How many compile passes we'll spend shaking out broken modules.
const maxPasses = 10

// Cap on stored diagnostic text per module; these end up in results.json.
const maxDiagLen = 4000

func cmdBuild(genDir, batchDir string) {
	var raw []string
	if err := json.Unmarshal(readStdin(), &raw); err != nil {
		fatalf("parsing the key array on stdin: %v", err)
	}
	// Keys are content-derived, so the same schema in two suite files is
	// one key; a repeat would be a duplicate import and a duplicate map
	// literal key, both compile errors.
	keys := make([]string, 0, len(raw))
	seen := map[string]bool{}
	for _, k := range raw {
		if !seen[k] {
			seen[k] = true
			keys = append(keys, k)
		}
	}

	// Start clean: a package directory left over from an earlier batch
	// wouldn't be imported, but it would still be in the way.
	if err := os.RemoveAll(batchDir); err != nil {
		fatalf("clearing %s: %v", batchDir, err)
	}
	if err := os.MkdirAll(batchDir, 0o755); err != nil {
		fatalf("creating %s: %v", batchDir, err)
	}

	foreign := map[string]bool{}
	for _, key := range keys {
		path := filepath.Join(genDir, key+".go")
		code, err := os.ReadFile(path)
		if err != nil {
			fatalf("reading the generated module for %s: %v", key, err)
		}
		for _, imp := range foreignImports(path, code) {
			foreign[imp] = true
		}
		dir := filepath.Join(batchDir, key)
		if err := os.MkdirAll(dir, 0o755); err != nil {
			fatalf("creating %s: %v", dir, err)
		}
		if err := os.WriteFile(filepath.Join(dir, key+".go"), code, 0o644); err != nil {
			fatalf("writing the module for %s: %v", key, err)
		}
	}
	writeGoMod(batchDir, foreign)

	alive := append([]string(nil), keys...)
	failures := map[string]string{}
	// Not nil: the harness wants an array, and a nil slice marshals as null.
	runner := []string{}
	for pass := 0; pass < maxPasses; pass++ {
		if err := os.WriteFile(filepath.Join(batchDir, "main.go"), []byte(runnerSource(alive)), 0o644); err != nil {
			fatalf("writing the runner: %v", err)
		}
		if len(alive) == 0 {
			break
		}
		cmd := exec.Command("go", "build", "-o", "runner-bin", ".")
		cmd.Dir = batchDir
		cmd.Env = goEnv()
		out, err := cmd.CombinedOutput()
		if err == nil {
			runner = []string{filepath.Join(batchDir, "runner-bin")}
			break
		}
		if _, ok := err.(*exec.ExitError); !ok {
			fatalf("running go build: %v", err)
		}
		errors, unattributed := attribute(string(out), alive)
		if len(errors) == 0 {
			// Nothing to drop, so this is our bug (bad runner, bad go.mod)
			// rather than a bad schema.
			fatalf("go build failed with no attributable errors:\n%s", strings.Join(unattributed, "\n"))
		}
		fmt.Fprintf(os.Stderr, "ogen: dropping %d module(s) after pass %d\n", len(errors), pass+1)
		next := alive[:0:0]
		for _, key := range alive {
			if diag, bad := errors[key]; bad {
				if _, already := failures[key]; !already {
					failures[key] = truncate(strings.TrimRight(diag, "\n"), maxDiagLen)
				}
				continue
			}
			next = append(next, key)
		}
		alive = next
	}
	if len(alive) > 0 && len(runner) == 0 {
		fatalf("go build still failing after %d passes (%d modules left)", maxPasses, len(alive))
	}

	outcomes := make(map[string]any, len(keys))
	for _, key := range keys {
		if diag, bad := failures[key]; bad {
			outcomes[key] = map[string]string{"fail": diag}
		} else {
			outcomes[key] = "ok"
		}
	}
	text, err := json.Marshal(map[string]any{"outcomes": outcomes, "runner": runner})
	if err != nil {
		fatalf("marshalling the build result: %v", err)
	}
	writeAll(os.Stdout, append(text, '\n'))
}

// foreignImports is the non-stdlib imports of one generated file. A path
// whose first element has a dot in it is a module path, which is the same
// rule the go command uses.
func foreignImports(path string, code []byte) []string {
	f, err := parser.ParseFile(token.NewFileSet(), path, code, parser.ImportsOnly)
	if err != nil {
		// Unparseable output is a compile failure, found a pass later and
		// attributed to its own key; nothing to collect here.
		return nil
	}
	var out []string
	for _, spec := range f.Imports {
		p, err := strconv.Unquote(spec.Path.Value)
		if err != nil {
			continue
		}
		head, _, _ := strings.Cut(p, "/")
		if strings.Contains(head, ".") {
			out = append(out, p)
		}
	}
	return out
}

// writeGoMod writes the batch module's go.mod, requiring whatever modules
// the generated code imports.
//
// Every ogen module needs at least `github.com/go-faster/jx` (the JSON
// codec its decoders are written against) and usually
// `github.com/ogen-go/ogen/validate` and `github.com/go-faster/errors`;
// `ogenregex`, `github.com/google/uuid`, `github.com/shopspring/decimal`
// and `net/netip` show up as schemas reach patterns, formats and numbers.
// A real user of ogen has all of those in go.mod — `ogen` writes them into
// the consuming module itself — so the batch does too rather than
// reporting a compile failure that is really this harness's environment.
// The versions come from this directory's own go.mod (see batchdeps.go for
// how they get there) and the hashes from its go.sum, so `GOPROXY=off`
// still resolves from the module cache.
func writeGoMod(batchDir string, foreign map[string]bool) {
	pins := modulePins()
	requires := map[string]string{}
	for imp := range foreign {
		module, version := longestPin(pins, imp)
		if module == "" {
			// Not a module this glue pins. With GOPROXY=off the build
			// would then report `no required module provides package …`
			// against the module that imported it, and `attribute` would
			// blame that key — turning this harness's incomplete pin list
			// into a `compile_fail` that reads like ogen's fault.
			// DESIGN.md says an environment failure must be a hard exit
			// instead, so it is one. (go-jsonschema's build warns and
			// carries on here; this is the deliberate difference.)
			fatalf("the generated code imports %s, which this generator's go.mod does not pin; "+
				"add it to batchdeps.go and re-run `go mod tidy`", imp)
		}
		requires[module] = version
	}

	mod := fmt.Sprintf("module %s\n\ngo %s\n", batchModule, glueGoDirective())
	if len(requires) > 0 {
		paths := make([]string, 0, len(requires))
		for path := range requires {
			paths = append(paths, path)
		}
		sort.Strings(paths)
		var block strings.Builder
		for _, path := range paths {
			fmt.Fprintf(&block, "\t%s %s\n", path, requires[path])
		}
		mod = fmt.Sprintf("module %s\n\ngo %s\n\nrequire (\n%s)\n", batchModule, glueGoDirective(), block.String())
	}
	// The batch always borrows this module's go.sum: even a batch whose
	// surviving modules happen to need nothing is one dropped module away
	// from needing jx, and a missing go.sum is a hard failure rather than
	// an attributable one.
	sum, err := os.ReadFile("go.sum")
	if err != nil {
		fatalf("reading go.sum for the batch module: %v", err)
	}
	if err := os.WriteFile(filepath.Join(batchDir, "go.sum"), sum, 0o644); err != nil {
		fatalf("writing the batch module's go.sum: %v", err)
	}
	if err := os.WriteFile(filepath.Join(batchDir, "go.mod"), []byte(mod), 0o644); err != nil {
		fatalf("writing the batch module's go.mod: %v", err)
	}
}

// longestPin finds the pinned module that owns an import path: the longest
// require path that is a prefix of it.
func longestPin(pins map[string]string, imp string) (string, string) {
	best, version := "", ""
	for path, v := range pins {
		if imp != path && !strings.HasPrefix(imp, path+"/") {
			continue
		}
		if len(path) > len(best) {
			best, version = path, v
		}
	}
	return best, version
}

// goEnv is the environment for building the batch module: no workspace, no
// network, no toolchain download. `GOPROXY=off` still resolves what the
// module cache already holds, which is where ogen and its dependencies are
// after this generator's first build.
func goEnv() []string {
	env := os.Environ()
	return append(env,
		"GOWORK=off",
		"GOFLAGS=-mod=mod",
		"GOPROXY=off",
		"GO111MODULE=on",
		"GOTOOLCHAIN=local",
	)
}

// Diagnostics come as `# bakeoff/g_abc` package headers and
// `g_abc/g_abc.go:12:3: message` spans; both name the package directory,
// which is the module key. Only keys we wrote count — the toolchain
// happily points at its own sources.
var spanRe = regexp.MustCompile(`^\.?/?([A-Za-z0-9_]+)/[^:]+\.go:\d+(?::\d+)?:`)
var headerRe = regexp.MustCompile(`^#\s+(\S+)`)

func attribute(output string, alive []string) (map[string]string, []string) {
	live := map[string]bool{}
	for _, k := range alive {
		live[k] = true
	}
	errors := map[string]string{}
	var unattributed []string
	current := ""
	for _, raw := range strings.Split(output, "\n") {
		line := strings.TrimRight(raw, " \t\r")
		if line == "" {
			continue
		}
		if m := headerRe.FindStringSubmatch(line); m != nil {
			key := strings.TrimPrefix(m[1], batchModule+"/")
			current = ""
			if live[key] {
				current = key
			}
			continue
		}
		key := current
		if m := spanRe.FindStringSubmatch(strings.TrimSpace(line)); m != nil && live[m[1]] {
			key = m[1]
		}
		switch {
		case key != "":
			errors[key] += strings.TrimLeft(line, " \t") + "\n"
		case strings.HasPrefix(strings.TrimSpace(line), "too many errors"),
			strings.HasPrefix(strings.TrimSpace(line), "note:"):
		default:
			unattributed = append(unattributed, line)
		}
	}
	return errors, unattributed
}

// runnerSource is the batch module's `main.go`: the harness's runner
// protocol over a dispatch table of the live modules' ConformanceCheck.
func runnerSource(alive []string) string {
	var imports, table strings.Builder
	for _, key := range alive {
		fmt.Fprintf(&imports, "\t%s \"%s/%s\"\n", key, batchModule, key)
		fmt.Fprintf(&table, "\t%q: %s.ConformanceCheck,\n", key, key)
	}
	return `// Generated by the bakeoff harness. See DESIGN.md for the runner protocol.
package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strconv"
` + imports.String() + `)

// A case's data stays as raw bytes so that the generated jx decoder sees
// the digits the suite wrote: decoding to interface{} and re-encoding would
// round every number through a float64.
type conformanceCase struct {
	Key  string          ` + "`json:\"key\"`" + `
	Data json.RawMessage ` + "`json:\"data\"`" + `
}

var modules = map[string]func([]byte) error{
` + table.String() + `}

// Go has no exceptions for ordinary failures, so a returned error is a
// reject and only a recovered panic is a panic. A stack overflow is fatal
// and unrecoverable; the harness notices the dead process and restarts us
// past the offending case.
func runOne(check func([]byte) error, data []byte) (outcome any) {
	defer func() {
		if r := recover(); r != nil {
			outcome = map[string]string{"panic": fmt.Sprint(r)}
		}
	}()
	if err := check(data); err != nil {
		return map[string]string{"reject": err.Error()}
	}
	return "accept"
}

func main() {
	if len(os.Args) != 3 {
		fmt.Fprintln(os.Stderr, "usage: runner <cases.json> <start-index>")
		os.Exit(2)
	}
	text, err := os.ReadFile(os.Args[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, "read cases file:", err)
		os.Exit(2)
	}
	start, err := strconv.Atoi(os.Args[2])
	if err != nil {
		fmt.Fprintln(os.Stderr, "parse start index:", err)
		os.Exit(2)
	}
	var cases []conformanceCase
	if err := json.Unmarshal(text, &cases); err != nil {
		fmt.Fprintln(os.Stderr, "parse cases file:", err)
		os.Exit(2)
	}

	emit(map[string]any{"ready": start})
	for i := start; i < len(cases); i++ {
		c := cases[i]
		var outcome any
		if check, ok := modules[c.Key]; ok {
			outcome = runOne(check, c.Data)
		} else {
			outcome = map[string]string{"panic": "module not compiled"}
		}
		// The absolute index goes on the wire so stray output can't shift
		// the harness's cursor.
		emit(map[string]any{"case": map[string]any{"i": i, "r": outcome}})
	}
}

// emit writes one protocol line, looping over short writes: a single write
// to a pipe can stop at 64 KiB, and half a line of JSON is a line the
// harness can't parse. os.Stdout is unbuffered, but sync anyway so a case
// that kills the process can't swallow the line before it.
func emit(v any) {
	line, err := json.Marshal(v)
	if err != nil {
		return
	}
	line = append(line, '\n')
	for len(line) > 0 {
		n, err := os.Stdout.Write(line)
		if n > 0 {
			line = line[n:]
			continue
		}
		// Give up on an error, and also on a (0, nil) write: a conforming
		// Write never returns one, but spinning on it would hang the
		// harness instead of letting it restart us.
		_ = err
		return
	}
	_ = os.Stdout.Sync()
}
`
}
