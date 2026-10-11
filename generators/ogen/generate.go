package main

// The generate step. `generate <out_dir>` reads the suite's groups on
// stdin, wraps each group's schema in an OpenAPI 3.1 document, hands that
// document to ogen's `gen` library, and writes `<out_dir>/<key>.go` for
// every group ogen could turn into a type.
//
// Why a library and not the CLI: `ogen` the command takes one document per
// invocation and writes a package to a directory. One spawn per group is
// 1,700 processes, each re-running goimports over a dozen files; and the
// CLI exits non-zero on the first document it can't handle, which would
// lose the rest of the batch. The library gives one fresh
// `gen.Generator` per group in one long-lived process.
//
// Why no batching: measured, ogen turns one of these documents into a
// package in 6–7 ms with client and server generation off (23 ms with its
// default feature set, which also renders the HTTP plumbing), so the whole
// five-draft suite is about 10 s of generation. Batching many components
// into one document would buy nothing and cost a lot: the components would
// land in one Go package, which this harness cannot attribute per group,
// and one bad schema would take its batch down. See the README.
//
// The fallback: a generation can still take the process down for real (a
// stack overflow on a recursive schema is fatal and unrecoverable in Go).
// So `generate` is a parent that writes a plan file and runs itself as a
// `worker` over it; the worker reports each group's status on stdout as it
// goes, and when it dies — or stalls past the per-group timeout — the
// parent records that one group as a `panic` and restarts the worker at the
// next index.
//
// The draft is ignored, and said so in the README: ogen has no dialect
// setting. An OpenAPI 3.1 document's schema objects *are* JSON Schema
// 2020-12 by definition, which is the only dialect ogen implements, and its
// parser never reads `$schema`.

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/format"
	"go/parser"
	"go/token"
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
	"sync"
	"time"

	"github.com/ogen-go/ogen"
	"github.com/ogen-go/ogen/gen"
)

// How long one group gets before the parent calls it a hang.
const generateTimeout = 60 * time.Second

// Caps on text that ends up in results.json.
const (
	maxMessageLen = 4000
	maxStderrTail = 1000
)

// A group as the harness sends it. The schema stays as raw bytes so the
// suite's digits reach ogen exactly as written.
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
	Key    string          `json:"key"`
	Schema json.RawMessage `json:"schema"`
}

// What the worker prints per group.
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

	workDir := filepath.Join(filepath.Dir(outDir), "ogen-plan")
	if err := os.RemoveAll(workDir); err != nil {
		fatalf("clearing %s: %v", workDir, err)
	}
	if err := os.MkdirAll(workDir, 0o755); err != nil {
		fatalf("creating %s: %v", workDir, err)
	}
	plan := make([]planEntry, 0, len(groups))
	for _, g := range groups {
		plan = append(plan, planEntry{Key: g.Key, Schema: g.Schema})
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
// group that kills it or stalls it.
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
		fmt.Fprintf(os.Stderr, "ogen: %s crashed the generator (%s); continuing\n",
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
				fmt.Fprintf(os.Stderr, "ogen: worker said %q\n", line)
				continue
			}
			switch {
			case wl.Ready != nil:
				ready = *wl.Ready == start
			case wl.Index != nil:
				i := *wl.Index
				if i != start+len(out) {
					fatalf("worker reported group %d, expected %d", i, start+len(out))
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
	// Drain whatever landed while we were deciding to give up: a status
	// that arrived late is still a real answer. This has to finish before
	// Wait, which closes the pipe StdoutPipe handed us — a Wait here would
	// race the scanner goroutine into "file already closed" and lose
	// whatever was still in the pipe buffer, turning a group that reported
	// `error` into a `panic`. The channel closes on its own: either the
	// worker finished its plan and exited, or it has just been killed.
	for line := range lines {
		var wl workerLine
		if err := json.Unmarshal([]byte(line), &wl); err == nil && wl.Index != nil &&
			*wl.Index == start+len(out) {
			out = append(out, status{Key: plan[*wl.Index].Key, Status: wl.Status, Message: wl.Message})
		}
	}
	waitErr := cmd.Wait()

	if start+len(out) < len(plan) && !ready && len(out) == 0 {
		// The worker never got as far as its first group: that is a bug in
		// this glue (bad binary, unreadable plan), not a bad schema.
		fatalf("the worker never started at %d: %v; stderr: %s", start, waitErr,
			tail(strings.TrimSpace(errText.String()), maxStderrTail))
	}
	how := "the generator exited without reporting this group"
	if timedOut {
		how = fmt.Sprintf("the generator did not finish within %s", generateTimeout)
	} else if waitErr != nil {
		how = fmt.Sprintf("the generator died: %v", waitErr)
	}
	if t := crashExcerpt(scrubStack(errText.String())); t != "" {
		how += "; stderr: " + t
	}
	return out, how
}

// ----------------------------------------------------------------- worker

// offlineTransport is the belt to ogen's braces. ogen resolves remote
// references only when `ParseOptions.AllowRemote` is set, which this glue
// leaves off, so `http://localhost:1234/...` already fails in-process with
// "external references are disabled". Replacing the default transport as
// well means that no code path anywhere in the library can reach the
// network even if that option's default ever changes.
type offlineTransport struct{}

func (offlineTransport) RoundTrip(req *http.Request) (*http.Response, error) {
	return nil, fmt.Errorf("%s: the bakeoff harness runs offline: remote references are not fetched", req.URL)
}

func cmdWorker(planPath, outDir, startArg string) {
	http.DefaultTransport = offlineTransport{}
	http.DefaultClient = &http.Client{Transport: offlineTransport{}}

	// Everything ogen writes is supposed to go through the in-memory
	// FileSystem below, but its writer has one path that doesn't: when
	// goimports rejects a rendered template, `gen.(*writer).Generate` does
	// `os.WriteFile(fileName+".dump", …)` with a bare base name, i.e.
	// relative to the working directory — which the harness sets to this
	// generator's directory. A stray `oas_schemas_gen.go.dump` there would
	// be hashed into the generator's source fingerprint and silently
	// invalidate every stored record for this row. So the worker moves to
	// the scratch directory next to the plan file, where a dump is
	// harmless (and still readable, under target/).
	if err := os.Chdir(filepath.Dir(planPath)); err != nil {
		fatalf("moving to the scratch directory %s: %v", filepath.Dir(planPath), err)
	}

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

// memFS collects what ogen writes instead of letting it write a package
// directory per group. WriteSource renders its templates in parallel, so
// this has to be safe for concurrent writers.
type memFS struct {
	mu    sync.Mutex
	files map[string][]byte
}

func (m *memFS) WriteFile(name string, content []byte) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.files[name] = content
	return nil
}

// generateOne runs ogen over one group. A returned error is a generation
// `error`; a recovered panic is a `panic`.
func generateOne(outDir string, entry planEntry) (st string, msg string) {
	outPath := filepath.Join(outDir, entry.Key+".go")
	// The scratch directory survives between runs, so a group that used to
	// generate and now doesn't must not leave its old module behind for a
	// reader to mistake for this run's output. (The build step is only
	// handed the keys that came back `ok`, so a stale file is never
	// compiled; this is about what a filtered run shows.)
	if err := os.Remove(outPath); err != nil && !os.IsNotExist(err) {
		return "error", fmt.Sprintf("clearing the previous output: %v", err)
	}
	defer func() {
		if r := recover(); r != nil {
			st = "panic"
			msg = fmt.Sprintf("%v\n%s", r, scrubStack(string(debug.Stack())))
		}
		msg = strings.ReplaceAll(msg, outPath, "<output>")
	}()

	doc, dropped := wrapDocument(entry.Schema)
	for _, k := range dropped {
		fmt.Fprintf(os.Stderr, "ogen: %s: the schema's top-level %q collides with the wrapper document and was not hoisted\n",
			entry.Key, k)
	}

	// ogen.Spec.UnmarshalJSON both fills the typed struct and keeps the
	// raw YAML node that its `$ref` resolver walks, so this is where a
	// schema ogen's own model can't hold (a boolean schema, `type` as a
	// list) is rejected.
	var spec ogen.Spec
	if err := json.Unmarshal(doc, &spec); err != nil {
		return "error", err.Error()
	}

	// Everything here is ogen's default except the feature set:
	//   - AllowRemote stays off (its default), so remote references fail
	//     in-process and nothing is fetched.
	//   - DisableAll turns off the six default features, all of which are
	//     client, server, OpenTelemetry or stub-handler generation. The
	//     type graph, the jx encoders and decoders and the validators are
	//     not features and are generated either way; what this buys is
	//     that ogen doesn't also render ~25 KB of net/http plumbing per
	//     group for the batch to compile. It changes which files ogen
	//     writes, not what it decides about the schema.
	g, err := gen.NewGenerator(&spec, gen.Options{
		Generator: gen.GenerateOptions{
			Features: &gen.FeatureOptions{DisableAll: true},
		},
	})
	if err != nil {
		return "error", err.Error()
	}

	fs := &memFS{files: map[string][]byte{}}
	if err := g.WriteSource(fs, entry.Key); err != nil {
		return "error", err.Error()
	}

	module, err := assemble(entry.Key, fs.files)
	if err != nil {
		return "error", err.Error()
	}
	if err := os.WriteFile(outPath, module, 0o644); err != nil {
		return "error", fmt.Sprintf("writing %s: %v", outPath, err)
	}
	return "ok", ""
}

// ------------------------------------------------------- the OpenAPI wrap

// Document members the wrapper owns; a schema whose top level uses one of
// these names does not get that member hoisted (see wrapDocument).
//
// This is every JSON field of `ogen.Spec`, not just the four the wrapper
// writes: hoisting a schema's top-level `servers` or `tags` into the
// document root would make ogen read it as part of the API description and
// report a type error that is the wrapper's fault, not the schema's. (And a
// top-level `swagger` would make ogen say the document is OpenAPI v2.)
// Nothing in the suite uses any of these eleven names; the guard is here so
// that if something ever does, the cost is one unresolvable pointer target
// and a line on stderr instead of a bogus generation error.
var reservedMembers = map[string]bool{
	"openapi":           true,
	"swagger":           true,
	"info":              true,
	"jsonSchemaDialect": true,
	"servers":           true,
	"paths":             true,
	"webhooks":          true,
	"components":        true,
	"security":          true,
	"tags":              true,
	"externalDocs":      true,
}

// wrapDocument turns one group's schema into an OpenAPI 3.1 document.
//
// ogen takes an OpenAPI document, not a bare schema, so the schema goes in
// as `components.schemas.Root` — verbatim: the bytes the harness sent are
// spliced into the document text, never re-serialized, so the suite's
// digits survive. 3.1 is the version to wrap in because a 3.1 schema object
// *is* a JSON Schema 2020-12 schema, so nothing about the schema has to be
// translated.
//
// Two things the wrapper has to add beyond `openapi`, `info` and `paths`:
//
// **An operation.** With `"paths": {}` ogen emits an empty
// `oas_schemas_gen.go`: it walks operations to find the types worth
// generating, and a component nothing references is not one. So the
// document carries a single synthetic operation, `POST /`, whose request
// body is `{"$ref": "#/components/schemas/Root"}`. That reference is the
// whole reason the type, its jx decoder and its validators exist, and it is
// also exactly the position a real ogen user's request body occupies.
//
// **The schema's own top-level members, hoisted.** A JSON Schema's `$ref`
// is a pointer into *its own document*: `{"$ref": "#/definitions/x"}` means
// the `definitions` next to it, and `{"$ref": "#"}` means the schema
// itself. ogen resolves a local reference with a `RootResolver` over the
// whole OpenAPI document, so under the naive wrapping those pointers aim at
// `#/definitions/x` of the *wrapper*, which does not exist — every group in
// the suite that uses `definitions` or `$defs` would be an error about the
// wrapper rather than a finding about ogen. So each top-level member of the
// schema is also copied, verbatim, to the document root, where the schema's
// own pointers expect to find it. `#/definitions/x`, `#/$defs/x`,
// `#/properties/foo` and a bare `#` then all resolve to what the suite
// meant. The members `ogen.Spec` reads for itself are never hoisted (see
// reservedMembers); nothing in the suite uses those names, and a group that
// did would be reported on stderr and simply lose that one pointer target.
//
// wrapDocument returns the document and the list of top-level member names
// it refused to hoist.
func wrapDocument(schema json.RawMessage) ([]byte, []string) {
	// A group always has a schema, but a missing one would splice nothing
	// and make the document itself invalid JSON — a confusing parse error
	// instead of a clear one.
	if len(bytes.TrimSpace(schema)) == 0 {
		schema = json.RawMessage("null")
	}
	var buf bytes.Buffer
	buf.WriteString(`{"openapi":"3.1.0",`)
	buf.WriteString(`"info":{"title":"bakeoff","version":"0"},`)
	buf.WriteString(`"paths":{"/":{"post":{"operationId":"check",`)
	buf.WriteString(`"requestBody":{"required":true,"content":{"application/json":`)
	buf.WriteString(`{"schema":{"$ref":"#/components/schemas/Root"}}}},`)
	buf.WriteString(`"responses":{"200":{"description":"ok"}}}}},`)
	buf.WriteString(`"components":{"schemas":{"Root":`)
	buf.Write(schema)
	buf.WriteString(`}}`)

	var dropped []string
	// A schema that isn't a JSON object (a boolean schema) has no members
	// to hoist; it is spliced in above and ogen rejects it on its own.
	var members map[string]json.RawMessage
	if err := json.Unmarshal(schema, &members); err == nil {
		names := make([]string, 0, len(members))
		for name := range members {
			names = append(names, name)
		}
		sort.Strings(names)
		for _, name := range names {
			if reservedMembers[name] {
				dropped = append(dropped, name)
				continue
			}
			key, err := json.Marshal(name)
			if err != nil {
				continue
			}
			buf.WriteByte(',')
			buf.Write(key)
			buf.WriteByte(':')
			buf.Write(members[name])
		}
	}
	buf.WriteByte('}')
	return buf.Bytes(), dropped
}

// ------------------------------------------------------- the Go module

// assemble folds the files ogen wrote for one document into a single
// self-contained Go file: `package <key>`, the union of the generated
// files' imports, every generated declaration verbatim, and the check
// entry point.
//
// ogen's output for one component spans several files — the structs in
// `oas_schemas_gen.go`, the jx encoder and decoder in `oas_json_gen.go`,
// `Validate` in `oas_validators_gen.go`, the compiled `pattern` regexps in
// `oas_cfg_gen.go`, plus `Equal`/`Hash` helpers and field defaults where
// the schema needs them — and the harness's contract is one module per
// group. Folding them into one file keeps each group's compile diagnostics
// attributable to its own key, which a shared package directory would not.
//
// Every file ogen writes goes in. With client and server generation off
// there is nothing else in the output: `oas_cfg_gen.go`, which normally
// holds the HTTP client's configuration, is reduced to the `regexMap` the
// validators index by pattern (and to a bare package clause when the
// schema has no `pattern` at all).
//
// Nothing in a declaration is rewritten. Only the per-file `package` clause
// and `import` block are removed, because Go allows one of each per file;
// their contents are merged back at the top.
func assemble(pkg string, files map[string][]byte) ([]byte, error) {
	names := make([]string, 0, len(files))
	for name := range files {
		names = append(names, name)
	}
	sort.Strings(names)

	type imp struct{ name, path string }
	var (
		imports []imp
		seen    = map[string]bool{}
		bound   = map[string]string{}
		types   = map[string]bool{}
		body    bytes.Buffer
	)
	for _, name := range names {
		src := files[name]
		fset := token.NewFileSet()
		f, err := parser.ParseFile(fset, name, src, parser.ParseComments|parser.SkipObjectResolution)
		if err != nil {
			// ogen renders through goimports, so this should not happen;
			// if it ever does, the unparseable text is the finding.
			return nil, fmt.Errorf("ogen wrote %s, which does not parse: %v", name, err)
		}
		// Cut the package clause and every import declaration out of the
		// file's text, leaving the declarations (and their comments) byte
		// for byte.
		type span struct{ lo, hi int }
		base := fset.File(f.Pos()).Base()
		cuts := []span{{0, int(f.Name.End()) - base}}
		// The first cut removes everything up to the package name, which
		// for ogen's output is just its `// Code generated` line. Assert
		// rather than assume: a build constraint up there would be
		// silently dropped, and a dropped `//go:build` is the kind of
		// thing that turns into a mystery two versions from now.
		if head := string(src[:cuts[0].hi]); strings.Contains(head, "//go:build") ||
			strings.Contains(head, "// +build") {
			return nil, fmt.Errorf("ogen wrote %s with a build constraint, which this merge would drop", name)
		}
		for _, decl := range f.Decls {
			gd, ok := decl.(*ast.GenDecl)
			if !ok {
				continue
			}
			if gd.Tok == token.TYPE {
				for _, spec := range gd.Specs {
					if ts, ok := spec.(*ast.TypeSpec); ok {
						types[ts.Name.Name] = true
					}
				}
				continue
			}
			if gd.Tok != token.IMPORT {
				continue
			}
			lo := int(gd.Pos()) - base
			if gd.Doc != nil {
				lo = int(gd.Doc.Pos()) - base
			}
			cuts = append(cuts, span{lo, int(gd.End()) - base})
			for _, spec := range gd.Specs {
				is := spec.(*ast.ImportSpec)
				path, err := strconv.Unquote(is.Path.Value)
				if err != nil {
					continue
				}
				alias := ""
				if is.Name != nil {
					alias = is.Name.Name
				}
				// An alias that just restates the package's own name is
				// the same import as the bare form. ogen's output is
				// goimports-normalized so this doesn't come up, but
				// emitting both `"fmt"` and `fmt "fmt"` would be
				// `fmt redeclared` and the diagnostic would point at
				// ogen rather than at this merge.
				if alias != "" && alias == path[strings.LastIndexByte(path, '/')+1:] {
					alias = ""
				}
				if key := alias + " " + path; !seen[key] {
					seen[key] = true
					// Two different paths bound to one name cannot both
					// live in one file, and renaming would mean rewriting
					// the bodies. Say so — and say it is this merge's
					// doing — rather than emitting a file whose
					// `redeclared` diagnostic would read like ogen's.
					// `github.com/ogen-go/ogen/json` next to
					// `encoding/json`, or `github.com/go-faster/errors`
					// next to stdlib `errors`, is how this would happen;
					// no group in the suite produces either pair.
					name := alias
					if name == "" {
						name = path[strings.LastIndexByte(path, '/')+1:]
					}
					if was, taken := bound[name]; taken && was != path {
						return nil, fmt.Errorf(
							"the bakeoff harness cannot merge ogen's files for this group: "+
								"they import both %q and %q, which bind the same name %s",
							was, path, name)
					}
					bound[name] = path
					imports = append(imports, imp{alias, path})
				}
			}
		}
		at := 0
		for _, c := range cuts {
			if c.lo > at {
				body.Write(src[at:c.lo])
			}
			if c.hi > at {
				at = c.hi
			}
		}
		if at < len(src) {
			body.Write(src[at:])
		}
		body.WriteString("\n")
	}

	var out bytes.Buffer
	fmt.Fprintf(&out, "// Code generated by ogen, DO NOT EDIT.\n\npackage %s\n\n", pkg)
	out.WriteString("import (\n")
	// The check entry point's own import, aliased so it can't collide with
	// whatever ogen imported.
	out.WriteString("\tbakeoffio \"io\"\n\n\tbakeoffjx \"github.com/go-faster/jx\"\n")
	for _, i := range imports {
		if i.name != "" {
			fmt.Fprintf(&out, "\t%s %q\n", i.name, i.path)
		} else {
			fmt.Fprintf(&out, "\t%q\n", i.path)
		}
	}
	out.WriteString(")\n")
	out.Write(body.Bytes())
	if alias := rootAlias(types); alias != "" {
		out.WriteString(alias)
	}
	out.WriteString(conformanceCheck)

	// gofmt for readability (the file is what a filtered run shows the
	// reader). A file gofmt rejects is kept as-is — that is a compile
	// failure to report, not something to hide — but every input here was
	// rendered through goimports, so a merged file that doesn't parse is
	// this merge's bug and says so on stderr.
	pretty, err := format.Source(out.Bytes())
	if err != nil {
		fmt.Fprintf(os.Stderr, "ogen: %s: the merged module does not parse (%v); writing it unformatted\n", pkg, err)
		return out.Bytes(), nil
	}
	return pretty, nil
}

// The type ogen names an inlined request body after: our synthetic
// operation's `operationId` is `check` (see wrapDocument), so ogen's
// identifier rules give `CheckReq`.
const inlinedRootType = "CheckReq"

// rootAlias is the declaration that gives the harness's `Root` name to the
// type ogen actually generated for the request body, when ogen declined to
// take the name from the component.
//
// Normally a `$ref` to `components/schemas/Root` produces `type Root …` and
// there is nothing to do. But when the component's schema is a sum type at
// its root (`{"oneOf": [...]}`), ogen's sum-type path loses the reference's
// name and names the type after the *operation* instead — `CheckReq`, from
// the `operationId` this wrapper chose. That is the wrapper's name leaking
// into the result: with a different `operationId` the type would be called
// something else, and the module would still fail to compile on `var v
// Root`. 40 of the suite's groups land there, so it has to be fixed rather
// than reported.
//
// An alias, not a definition: `type Root = CheckReq` carries the method set
// over, so `v.Decode` and `v.Validate` are ogen's own methods on ogen's own
// type. Nothing is renamed and nothing is synthesized.
//
// What this deliberately does *not* do is invent a root type when ogen
// emitted none. Two cases stay `compile_fail` with the compiler's own
// `undefined: Root`, which is the convention DESIGN.md sets and which the
// go-jsonschema row follows for the same situations:
//
//   - a root `$ref`, where ogen names the type after the *reference target*
//     (`{"$ref": "#/definitions/a"}` gives `type A`, no `Root`);
//   - a schema ogen gave up on silently, leaving a module with no schema
//     type in it at all.
func rootAlias(types map[string]bool) string {
	if types["Root"] || !types[inlinedRootType] {
		return ""
	}
	return "\n// Root is the name the bakeoff harness's check expects. ogen named this\n" +
		"// type after the wrapper document's operation rather than after the\n" +
		"// component, so the alias puts the contract's name back on ogen's own\n" +
		"// type; the method set, and therefore everything measured, is ogen's.\n" +
		"type Root = " + inlinedRootType + "\n"
}

// conformanceCheck is the per-module entry point the build step calls.
// It is ogen's own request-body path, lifted out of the generated
// `decodeCheckRequest` in `oas_request_decoders_gen.go`: decode the bytes
// with the generated jx decoder, insist that the whole document was
// consumed, then validate. Both steps count — a decode error and a
// validation error are each a reject — because ogen splits the work:
// `Decode` enforces the type and the required-field bitset, and
// `Validate` enforces lengths, bounds, patterns, `uniqueItems` and
// `format`. Calling only the decoder would measure half the tool.
//
// `Validate` is reached through an interface assertion rather than called
// directly because ogen emits `oas_validators_gen.go` only for types that
// have something to check; a `{"type": "string"}` root has no `Validate`
// method at all, and a direct call would not compile. The assertion is the
// same decision ogen's own template makes, made at compile time instead.
const conformanceCheck = `
// ConformanceCheck is the bakeoff harness's entry point.
func ConformanceCheck(data []byte) error {
	var v Root
	d := bakeoffjx.DecodeBytes(data)
	if err := v.Decode(d); err != nil {
		return err
	}
	if err := d.Skip(); err != bakeoffio.EOF {
		return bakeoffErrTrailing
	}
	if val, ok := any(&v).(interface{ Validate() error }); ok {
		return val.Validate()
	}
	return nil
}

type bakeoffError string

func (e bakeoffError) Error() string { return string(e) }

const bakeoffErrTrailing bakeoffError = "unexpected trailing data"
`

// crashExcerpt is the part of a dead worker's stderr worth storing. Keeping
// the last `maxStderrTail` bytes is wrong here: the Go runtime's banner
// ("fatal error: stack overflow") and the stack of the goroutine that died
// come first, and what follows is every other goroutine's stack, so a plain
// tail stores the idle runtime and drops the cause. So start at the banner
// where there is one and fall back to the tail when the worker died without
// one.
func crashExcerpt(stderr string) string {
	text := strings.TrimSpace(stderr)
	for at := 0; at < len(text); {
		line := text[at:]
		if i := strings.IndexByte(line, '\n'); i >= 0 {
			line = line[:i]
		}
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
		if frame, ok := strings.CutPrefix(line, "\t"); ok {
			frame, _, _ = strings.Cut(strings.TrimSpace(frame), " ")
			lines[i] = "\t" + filepath.Base(frame)
			continue
		}
		// Goroutine ids are scheduler-dependent, and ogen renders its
		// templates on an errgroup, so the goroutine a crash lands on is
		// not stable across runs either.
		lines[i] = goroutineID.ReplaceAllString(hexWord.ReplaceAllString(line, "0x?"), "goroutine ?")
	}
	return strings.Join(lines, "\n")
}

// A hexadecimal machine word as a stack trace prints one.
var hexWord = regexp.MustCompile(`0x[0-9a-f]+`)

// A goroutine id as a stack trace's header and `created by` lines print one.
var goroutineID = regexp.MustCompile(`goroutine \d+`)

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
