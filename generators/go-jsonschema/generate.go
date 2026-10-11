package main

// The generate step. `generate <out_dir>` reads the suite's groups on
// stdin, hands each one to go-jsonschema's `pkg/generator` library, and
// writes `<out_dir>/<key>.go` for every schema the tool could handle.
//
// Why a library and not the CLI: the CLI's `DoFile` loop aborts the whole
// process on the first schema that errors, so a single invocation over
// hundreds of the suite's schemas would report one error and lose the rest.
// With the library each schema gets its own `generator.Generator` inside
// one long-lived process — no process per group — and an error or a
// recovered panic is attributed to the schema that produced it.
//
// The fallback: a generation can still take the process down for real (a
// stack overflow on a recursive schema is fatal and unrecoverable in Go).
// So `generate` is a parent that writes a plan file and runs itself as a
// `worker` over it; the worker reports each schema's status on stdout as it
// goes, and when it dies — or stalls past the per-schema timeout — the
// parent records that one schema as a `panic` and restarts the worker at
// the next index. The degenerate case is one invocation per schema, which
// is exactly the per-schema fallback, but only for as many schemas as
// actually crash.
//
// Root type naming: the tool names the root type after the schema's file
// name unless a `--schema-root-type` mapping matches the schema's `$id`, so
// each schema is written to `<work>/<key>/Root` (no extension) *and* given
// a mapping with an empty schema ID and `RootType: "Root"`. Between the two
// the root type is `Root` for every schema that has a root type at all.
//
// The draft is ignored, and said so in the README: go-jsonschema has no
// dialect switch and never reads `$schema` at all — its schema model has no
// field for it — so it takes `$defs` where present and falls back to
// `definitions`, for every draft alike. The schema bytes are written through
// untouched, `$schema` and all.

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"runtime/debug"
	"sort"
	"strconv"
	"strings"
	"time"

	"github.com/atombender/go-jsonschema/pkg/generator"
)

// How long one schema gets before the parent calls it a hang.
const generateTimeout = 60 * time.Second

// Caps on text that ends up in results.json.
const (
	maxMessageLen = 4000
	maxStderrTail = 1000
)

// A group as the harness sends it. The schema stays as raw bytes so the
// suite's digits reach the tool exactly as written.
type group struct {
	Key    string          `json:"key"`
	Draft  string          `json:"draft"`
	Schema json.RawMessage `json:"schema"`
}

// One line of the generate status array.
type status struct {
	Key     string `json:"key"`
	Status  string `json:"status"`
	Message string `json:"message,omitempty"`
}

// An entry in the plan file the parent hands the worker.
type planEntry struct {
	Key  string `json:"key"`
	File string `json:"file"`
}

// What the worker prints per schema.
type workerLine struct {
	Ready   *int   `json:"ready,omitempty"`
	Index   *int   `json:"i,omitempty"`
	Status  string `json:"status,omitempty"`
	Message string `json:"message,omitempty"`
}

func cmdGenerate(outDir string) {
	var groups []group
	if err := json.Unmarshal(readStdin(), &groups); err != nil {
		fatalf("parsing the group array on stdin: %v", err)
	}
	if err := os.MkdirAll(outDir, 0o755); err != nil {
		fatalf("creating %s: %v", outDir, err)
	}

	// Each schema gets its own directory so that the file can be called
	// `Root`: that name is what the tool derives the root type from.
	workDir := filepath.Join(filepath.Dir(outDir), "go-jsonschema-schemas")
	if err := os.RemoveAll(workDir); err != nil {
		fatalf("clearing %s: %v", workDir, err)
	}
	plan := make([]planEntry, 0, len(groups))
	for _, g := range groups {
		dir := filepath.Join(workDir, g.Key)
		if err := os.MkdirAll(dir, 0o755); err != nil {
			fatalf("creating %s: %v", dir, err)
		}
		file := filepath.Join(dir, "Root")
		if err := os.WriteFile(file, g.Schema, 0o644); err != nil {
			fatalf("writing %s: %v", file, err)
		}
		plan = append(plan, planEntry{Key: g.Key, File: file})
	}
	planPath := filepath.Join(workDir, "plan.json")
	planText, err := json.Marshal(plan)
	if err != nil {
		fatalf("marshalling the plan: %v", err)
	}
	if err := os.WriteFile(planPath, planText, 0o644); err != nil {
		fatalf("writing %s: %v", planPath, err)
	}

	statuses := runWorkers(planPath, outDir, plan)
	text, err := json.Marshal(statuses)
	if err != nil {
		fatalf("marshalling the status array: %v", err)
	}
	writeAll(os.Stdout, append(text, '\n'))
}

// runWorkers drives the worker over the whole plan, restarting it past any
// schema that kills it or stalls it.
func runWorkers(planPath, outDir string, plan []planEntry) []status {
	self, err := os.Executable()
	if err != nil {
		fatalf("finding my own binary: %v", err)
	}
	statuses := make([]status, 0, len(plan))
	for len(statuses) < len(plan) {
		start := len(statuses)
		done, died := runWorker(self, planPath, outDir, plan, start)
		statuses = append(statuses, done...)
		if len(statuses) >= len(plan) {
			break
		}
		// The schema at the cursor took the worker down (or hung).
		statuses = append(statuses, status{
			Key:     plan[len(statuses)].Key,
			Status:  "panic",
			Message: truncate(died, maxMessageLen),
		})
		fmt.Fprintf(os.Stderr, "go-jsonschema: %s crashed the generator (%s); continuing\n",
			plan[len(statuses)-1].Key, firstLine(died))
	}
	return statuses
}

// runWorker runs one worker from `start` and returns the statuses it
// reported, in order, plus a description of how it ended if it stopped
// early.
func runWorker(self, planPath, outDir string, plan []planEntry, start int) ([]status, string) {
	cmd := exec.Command(self, "worker", planPath, outDir, strconv.Itoa(start))
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		fatalf("worker stdout: %v", err)
	}
	var errText strings.Builder
	cmd.Stderr = io.MultiWriter(os.Stderr, &errText)
	if err := cmd.Start(); err != nil {
		fatalf("starting the worker: %v", err)
	}

	lines := make(chan string, 1024)
	go func() {
		defer close(lines)
		scanner := bufio.NewScanner(stdout)
		scanner.Buffer(make([]byte, 0, 64*1024), 16*1024*1024)
		for scanner.Scan() {
			lines <- scanner.Text()
		}
	}()

	var (
		out      []status
		ready    bool
		timedOut bool
	)
	for len(out) < len(plan)-start {
		select {
		case line, ok := <-lines:
			if !ok {
				goto reap
			}
			var wl workerLine
			if err := json.Unmarshal([]byte(line), &wl); err != nil {
				// Stray output: the tool's own chatter, not the protocol.
				fmt.Fprintf(os.Stderr, "go-jsonschema: worker said %q\n", line)
				continue
			}
			switch {
			case wl.Ready != nil:
				ready = *wl.Ready == start
			case wl.Index != nil:
				i := *wl.Index
				if i != start+len(out) {
					fatalf("worker reported schema %d, expected %d", i, start+len(out))
				}
				out = append(out, status{Key: plan[i].Key, Status: wl.Status, Message: wl.Message})
			}
		case <-time.After(generateTimeout):
			timedOut = true
			goto reap
		}
	}

reap:
	if timedOut {
		_ = cmd.Process.Kill()
	}
	waitErr := cmd.Wait()
	// Drain whatever landed while we were deciding to give up: a status
	// that arrived late is still a real answer.
	for line := range lines {
		var wl workerLine
		if err := json.Unmarshal([]byte(line), &wl); err == nil && wl.Index != nil &&
			*wl.Index == start+len(out) {
			out = append(out, status{Key: plan[*wl.Index].Key, Status: wl.Status, Message: wl.Message})
		}
	}

	if start+len(out) < len(plan) && !ready && len(out) == 0 {
		// The worker never got as far as its first schema: that is a bug in
		// this glue (bad binary, unreadable plan), not a bad schema.
		fatalf("the worker never started at %d: %v; stderr: %s", start, waitErr,
			tail(strings.TrimSpace(errText.String()), maxStderrTail))
	}
	how := "the generator exited without reporting this schema"
	if timedOut {
		how = fmt.Sprintf("the generator did not finish within %s", generateTimeout)
	} else if waitErr != nil {
		how = fmt.Sprintf("the generator died: %v", waitErr)
	}
	// A worker that died for real left its runtime stack on stderr, which is
	// the most useful thing we have; scrub it the way a recovered panic is
	// scrubbed, so the stored message doesn't depend on the machine.
	if t := crashExcerpt(scrubStack(errText.String())); t != "" {
		how += "; stderr: " + t
	}
	return out, how
}

// ----------------------------------------------------------------- worker

// offlineTransport is how the no-network policy is enforced: the tool's
// schema loader fetches `http(s)://` references with `&http.Client{}`,
// which uses http.DefaultTransport, so replacing it makes every remote
// reference fail immediately and in-process. The suite's
// `http://localhost:1234/...` remotes are therefore a generation error,
// which is the point — nothing is ever fetched, and the cost of that
// policy is recorded rather than papered over.
type offlineTransport struct{}

var errNoNetwork = errors.New("the bakeoff harness runs offline: remote references are not fetched")

func (offlineTransport) RoundTrip(req *http.Request) (*http.Response, error) {
	return nil, fmt.Errorf("%s: %w", req.URL, errNoNetwork)
}

func cmdWorker(planPath, outDir, startArg string) {
	http.DefaultTransport = offlineTransport{}
	http.DefaultClient = &http.Client{Transport: offlineTransport{}}

	start, err := strconv.Atoi(startArg)
	if err != nil {
		fatalf("parsing the start index %q: %v", startArg, err)
	}
	text, err := os.ReadFile(planPath)
	if err != nil {
		fatalf("reading %s: %v", planPath, err)
	}
	var plan []planEntry
	if err := json.Unmarshal(text, &plan); err != nil {
		fatalf("parsing %s: %v", planPath, err)
	}

	emitJSON(os.Stdout, map[string]any{"ready": start})
	for i := start; i < len(plan); i++ {
		st, msg := generateOne(outDir, plan[i])
		emitJSON(os.Stdout, map[string]any{
			"i":       i,
			"status":  st,
			"message": truncate(msg, maxMessageLen),
		})
	}
}

// generateOne runs the tool over one schema. A returned error is a
// generation `error`; a recovered panic is a `panic`. Everything the tool
// is configured with here is its default except the package name, the
// output file and the root type name, which the contract fixes.
func generateOne(outDir string, entry planEntry) (st string, msg string) {
	// The tool names the schema file in most of its messages, and that
	// path is an absolute one under target/. Stored results are committed,
	// so the message must not depend on where the repo lives.
	defer func() {
		if r := recover(); r != nil {
			st = "panic"
			msg = fmt.Sprintf("%v\n%s", r, scrubStack(string(debug.Stack())))
		}
		msg = strings.ReplaceAll(msg, entry.File, "<schema>")
		msg = strings.ReplaceAll(msg, filepath.Join(outDir, entry.Key+".go"), "<output>")
		// A relative `$ref` resolves against the schema's directory, so a
		// failure can name a sibling file too; leave those as <key>/<name>.
		msg = strings.ReplaceAll(msg, filepath.Dir(filepath.Dir(entry.File))+string(filepath.Separator), "")
	}()

	outPath := filepath.Join(outDir, entry.Key+".go")
	cfg := generator.Config{
		Warner: func(message string) {
			fmt.Fprintf(os.Stderr, "go-jsonschema: %s: warning: %s\n", entry.Key, message)
		},
		DefaultPackageName: entry.Key,
		DefaultOutputName:  outPath,
		// The mapping matches a schema with no `$id` (most of the suite)
		// and fixes its root type name; a schema that does declare an `$id`
		// falls through to the file-name rule, and the file is called
		// `Root`. Either way the root type is `Root`.
		SchemaMappings: []generator.SchemaMapping{{
			SchemaID:    "",
			PackageName: entry.Key,
			OutputName:  outPath,
			RootType:    "Root",
		}},
		// The CLI's defaults, spelled out: YAML is recognized by
		// extension (nothing here has one), all three struct tags are
		// emitted, and no non-stdlib imports are allowed.
		YAMLExtensions: []string{".yml", ".yaml"},
		Tags:           []string{"json", "yaml", "mapstructure"},
		ExtraImports:   false,
	}

	gen, err := generator.New(cfg)
	if err != nil {
		return "error", err.Error()
	}
	if err := gen.DoFile(entry.File); err != nil {
		return "error", err.Error()
	}
	sources, err := gen.Sources()
	if err != nil {
		return "error", err.Error()
	}

	code, ok := sources[outPath]
	if !ok {
		// Only one output is configured, so this should not happen; if the
		// tool ever writes somewhere else, say what it wrote rather than
		// silently producing nothing.
		names := make([]string, 0, len(sources))
		for name := range sources {
			names = append(names, name)
		}
		sort.Strings(names)
		return "error", fmt.Sprintf("the tool wrote no source for %s (it wrote %s)",
			outPath, strings.Join(names, ", "))
	}
	module, err := withCheck(code)
	if err != nil {
		return "error", err.Error()
	}
	if err := os.WriteFile(outPath, module, 0o644); err != nil {
		return "error", fmt.Sprintf("writing %s: %v", outPath, err)
	}
	return "ok", ""
}

// withCheck splices the check entry point's import in after the package
// clause and appends the function. Go wants every import before the first
// declaration, so the import can't simply ride along at the end of the
// file; nothing else about the tool's output is touched.
func withCheck(code []byte) ([]byte, error) {
	lines := strings.Split(string(code), "\n")
	for i, line := range lines {
		if !strings.HasPrefix(line, "package ") {
			continue
		}
		spliced := make([]string, 0, len(lines)+2)
		spliced = append(spliced, lines[:i+1]...)
		spliced = append(spliced, "", `import bakeoffjson "encoding/json"`)
		spliced = append(spliced, lines[i+1:]...)
		return []byte(strings.Join(spliced, "\n") + conformanceCheck), nil
	}
	return nil, errors.New("the tool's output has no package clause")
}

// conformanceCheck is the per-module entry point the build step calls,
// appended to whatever the tool rendered. Keeping it in the module (rather
// than in the runner) means a module whose root type came out unusable —
// or never came out at all, which is what happens when the schema's root
// has no `type` keyword — fails to compile on its own, and the diagnostic
// is attributed to its own key.
//
// `encoding/json` is imported under an alias (spliced in by `withCheck`)
// because the generated file usually imports it already, and a second
// plain import of the same path would be a redeclaration. `var v Root`
// works whether the root type is a struct or an alias of a scalar or
// slice, and unmarshalling into its address is what triggers the generated
// `UnmarshalJSON` (with its required-field, enum, bound and pattern
// checks) where the tool emitted one.
const conformanceCheck = `
// ConformanceCheck is the bakeoff harness's entry point.
func ConformanceCheck(data []byte) error {
	var v Root
	return bakeoffjson.Unmarshal(data, &v)
}
`

// crashExcerpt is the part of a dead worker's stderr worth storing. Keeping
// the last `maxStderrTail` bytes is wrong here: the Go runtime's banner
// ("fatal error: stack overflow") and the stack of the goroutine that died
// come first, and what follows is every other goroutine's stack, so a plain
// tail stores the idle runtime and drops the cause. So start at the banner
// where there is one — the tool's own per-schema warnings precede it — and
// fall back to the tail when the worker died without one.
func crashExcerpt(stderr string) string {
	text := strings.TrimSpace(stderr)
	for at := 0; at < len(text); {
		line := text[at:]
		if i := strings.IndexByte(line, '\n'); i >= 0 {
			line = line[:i]
		}
		// The banners the runtime writes: an unrecovered panic, a fatal
		// error (stack overflow, out of memory), or a signal it reports
		// itself ("SIGQUIT: quit" for the kill the stall timeout sends).
		if strings.HasPrefix(line, "panic:") || strings.HasPrefix(line, "fatal error:") ||
			strings.HasPrefix(line, "SIG") {
			return truncate(text[at:], maxStderrTail)
		}
		at += len(line) + 1
	}
	return tail(text, maxStderrTail)
}

// scrubStack makes a Go stack trace fit to store in results.json. A raw
// trace is machine- and run-specific twice over: the frame lines carry
// absolute paths into the Go installation, the module cache and this
// directory, and the function lines carry the argument words as raw
// addresses that differ on every run. Keeping the function name, the file's
// base name and the line number leaves everything a reader needs and
// nothing that makes a committed result depend on where it was produced.
func scrubStack(s string) string {
	lines := strings.Split(s, "\n")
	for i, line := range lines {
		// "\t/abs/path/to/stack.go:26 +0x64" → "\tstack.go:26"
		if frame, ok := strings.CutPrefix(line, "\t"); ok {
			frame, _, _ = strings.Cut(strings.TrimSpace(frame), " ")
			lines[i] = "\t" + filepath.Base(frame)
			continue
		}
		lines[i] = hexWord.ReplaceAllString(line, "0x?")
	}
	return strings.Join(lines, "\n")
}

// A hexadecimal machine word as a stack trace prints one.
var hexWord = regexp.MustCompile(`0x[0-9a-f]+`)

func firstLine(s string) string {
	if i := strings.IndexByte(s, '\n'); i >= 0 {
		return s[:i]
	}
	return s
}

func tail(s string, max int) string {
	if len(s) <= max {
		return s
	}
	return "..." + s[len(s)-max:]
}
