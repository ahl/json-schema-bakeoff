package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/atombender/go-jsonschema/pkg/generator"
)

// generate runs the tool over one schema with the loader generateOne uses
// and returns the source or the error.
func generate(t *testing.T, schema string, external map[string]json.RawMessage) (string, error) {
	t.Helper()
	dir := t.TempDir()
	root := filepath.Join(dir, "Root")
	if err := os.WriteFile(root, []byte(schema), 0o644); err != nil {
		t.Fatal(err)
	}
	out := filepath.Join(dir, "out.go")
	gen, err := generator.New(generator.Config{
		Warner:             func(string) {},
		DefaultPackageName: "p",
		DefaultOutputName:  out,
		SchemaMappings: []generator.SchemaMapping{{
			PackageName: "p", OutputName: out, RootType: "Root",
		}},
		YAMLExtensions: yamlExtensions,
		Tags:           []string{"json"},
		Loader:         newLoader(root, external, yamlExtensions),
	})
	if err != nil {
		t.Fatal(err)
	}
	if err := gen.DoFile(root); err != nil {
		return "", err
	}
	sources, err := gen.Sources()
	if err != nil {
		return "", err
	}
	return string(sources[out]), nil
}

const refSchema = `{
	"type": "object",
	"properties": {"a": {"$ref": "http://localhost:1234/thing.json"}}
}`

func TestSuppliedRefResolves(t *testing.T) {
	src, err := generate(t, refSchema, map[string]json.RawMessage{
		"http://localhost:1234/thing.json": json.RawMessage(
			`{"type": "object", "properties": {"zork": {"type": "integer"}}}`),
	})
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(src, "Zork") {
		t.Fatalf("the external schema's property is missing:\n%s", src)
	}
}

func TestUnsuppliedRefRefused(t *testing.T) {
	_, err := generate(t, refSchema, map[string]json.RawMessage{
		"http://localhost:1234/other.json": json.RawMessage(`{"type": "integer"}`),
	})
	if err == nil || !strings.Contains(err.Error(),
		`refusing to load "http://localhost:1234/thing.json": not one of this group's external schemas`) {
		t.Fatalf("expected a refusal, got %v", err)
	}
}

func TestFileRefRefused(t *testing.T) {
	_, err := generate(t, `{
		"type": "object",
		"properties": {"a": {"$ref": "Root"}}
	}`, nil)
	if err == nil || !strings.Contains(err.Error(), `refusing to read "Root" from disk`) {
		t.Fatalf("expected a refusal, got %v", err)
	}
}

// The same URI names a different document in each group; each generation
// sees only its own.
func TestScopeIsPerItem(t *testing.T) {
	a, err := generate(t, refSchema, map[string]json.RawMessage{
		"http://localhost:1234/thing.json": json.RawMessage(
			`{"type": "object", "properties": {"alpha": {"type": "integer"}}}`),
	})
	if err != nil {
		t.Fatal(err)
	}
	b, err := generate(t, refSchema, map[string]json.RawMessage{
		"http://localhost:1234/thing.json": json.RawMessage(
			`{"type": "object", "properties": {"beta": {"type": "string"}}}`),
	})
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(a, "Alpha") || strings.Contains(a, "Beta") ||
		!strings.Contains(b, "Beta") || strings.Contains(b, "Alpha") {
		t.Fatalf("documents leaked between groups:\n%s\n---\n%s", a, b)
	}
}
