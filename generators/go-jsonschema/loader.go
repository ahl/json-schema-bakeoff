package main

// The schema loader each generation runs with. go-jsonschema loads the root
// schema and every `$ref` that names another document through
// `generator.Config.Loader`; left nil, that is a cache over a file loader
// and an HTTP loader. Here the same two slots hold a file loader that reads
// the root and nothing else, and an `http(s)://` loader that serves the
// group's own `externalSchemas` and nothing else. A URI with any other
// scheme gets the tool's own unsupported-format error.
//
// The tool asks for a reference by its text before the `#`, exactly as the
// schema wrote it: nothing resolves it against an `$id` or against the
// document it came from first. So a lookup matches only an absolute ref
// whose spelling is the retrieval URI, and a relative ref (`./bar.json`)
// arrives scheme-less and is refused like any other file.

import (
	"bytes"
	"encoding/json"
	"fmt"

	"github.com/atombender/go-jsonschema/pkg/schemas"
)

// newLoader is the loader for one group: the tool's own multi-loader and
// cache, the way the default is built, over a file loader that reads only
// the root and an HTTP loader that reads only the group's external schemas.
// It is built per group because the same URI can name different documents
// in different groups.
func newLoader(root string, external map[string]json.RawMessage, yamlExtensions []string) schemas.Loader {
	files := rootLoader{root: root, files: schemas.NewFileLoader(nil, yamlExtensions)}
	remotes := externalLoader(external)
	return schemas.NewCachedLoader(schemas.MultiLoader{
		schemas.RefTypeFile:  files,
		schemas.RefTypeHTTP:  remotes,
		schemas.RefTypeHTTPS: remotes,
	}, map[string]*schemas.Schema{})
}

// rootLoader reads the root schema's file, the one path read from disk.
type rootLoader struct {
	root  string
	files *schemas.FileLoader
}

func (l rootLoader) Load(uri, parentURI string) (*schemas.Schema, error) {
	if uri != l.root || parentURI != "" {
		return nil, fmt.Errorf("refusing to read %q from disk: only the schema "+
			"itself is read from a file", uri)
	}
	return l.files.Load(uri, parentURI)
}

// externalLoader serves the group's `externalSchemas`, keyed by retrieval
// URI and held as the harness's bytes.
type externalLoader map[string]json.RawMessage

func (l externalLoader) Load(uri, _ string) (*schemas.Schema, error) {
	raw, ok := l[uri]
	if !ok {
		return nil, fmt.Errorf("refusing to load %q: not one of this group's "+
			"external schemas", uri)
	}
	// The same decoder the tool's file and HTTP loaders use.
	schema, err := schemas.FromJSONReader(bytes.NewReader(raw))
	if err != nil {
		return nil, fmt.Errorf("error parsing external schema %s: %w", uri, err)
	}
	return schema, nil
}
