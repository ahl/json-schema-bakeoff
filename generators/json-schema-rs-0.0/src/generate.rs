//! Feed a schema to json-schema-rs and capture code, error, or panic.

use json_schema_rs::sanitizers::sanitize_struct_name;
use json_schema_rs::{
    CodeGenSettings, DedupeMode, JsonSchema, JsonSchemaSettings, SpecVersion, generate_rust,
};
use std::collections::{BTreeSet, HashMap};
use std::sync::mpsc::RecvTimeoutError;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

/// Identity string: `json-schema-rs <workspace version>+<short sha>`, both
/// read out of Cargo.lock by build.rs.
pub const VERSION: &str =
    concat!("json-schema-rs ", env!("JSR_CRATE_VERSION"), "+", env!("JSR_GIT_SHORT"));

/// Lines appended to the batch crate's `[dependencies]` table so the
/// generated code resolves.
///
/// Generated items carry `#[derive(json_schema_rs_macro::ToJsonSchema)]` and
/// `#[json_schema(...)]` helper attributes; the derive expands to
/// `impl ::json_schema_rs::ToJsonSchema`, and non-string enums call
/// `json_schema_rs::json_equal`, so both crates are needed. Both are pinned
/// to the same commit this integration's `json-schema-rs` comes from (the
/// macro crate depends on the library by workspace path, so cargo resolves
/// one copy of each).
///
/// The 0.0.6 integration needed a `syn` build-dependency to switch on syn's
/// `full` feature for the macro crate; at this commit the macro crate asks
/// for `full` itself, so nothing else is needed.
pub const BATCH_DEPS: &str = concat!(
    "json-schema-rs = { git = \"https://github.com/goddtriffin/json-schema-rs.git\", rev = \"",
    env!("JSR_GIT_SHA"),
    "\" }\njson-schema-rs-macro = { git = \"https://github.com/goddtriffin/json-schema-rs.git\", rev = \"",
    env!("JSR_GIT_SHA"),
    "\" }\n"
);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GenOutcome {
    /// Generated Rust source. `root` is the type to deserialize into; it
    /// refers to an item in `code`, which is a complete module body. `rule`
    /// says how `root` was found (see `root_type_name`).
    Code { code: String, root: String, rule: &'static str },
    Error(String),
    Panic(String),
}

/// The suite directory name as a `SpecVersion`.
///
/// It goes in as `JsonSchemaSettings::default_spec_version`: the draft for a
/// document whose root has no `$schema`. That is the setting the crate
/// documents for exactly this use (its own suite and Bowtie harnesses pass
/// the suite directory's draft there), and it is what the jsonschema row
/// does too (`with_draft` sets the root's draft and lets embedded resources
/// follow their own `$schema`). Drafts 4 through 7 in the suite never carry a
/// root `$schema`; 2019-09 and 2020-12 nearly always carry the directory's
/// own, so the directory decides every root either way. The other knob,
/// `forced_spec_version`, would also override an embedded resource's
/// `$schema`, which the spec says it must not.
///
/// All five suite directories map onto a variant, so the `Err` arm is for a
/// draft this integration has never seen: the group is recorded as an
/// `error` naming it rather than generated against a guessed dialect.
pub fn spec_version(draft: Option<&str>) -> Result<SpecVersion, String> {
    let Some(draft) = draft else {
        return Err("no draft for this group: nothing to select a dialect with".to_string());
    };
    match draft {
        "draft4" => Ok(SpecVersion::Draft04),
        "draft6" => Ok(SpecVersion::Draft06),
        "draft7" => Ok(SpecVersion::Draft07),
        "draft2019-09" => Ok(SpecVersion::Draft201909),
        "draft2020-12" => Ok(SpecVersion::Draft202012),
        other => Err(format!("no json-schema-rs SpecVersion for suite draft {other:?}")),
    }
}

/// Run json-schema-rs on a schema.
///
/// Runs on a dedicated thread with a large stack and catches panics. A
/// generation that exceeds `timeout` is abandoned (the thread is leaked) and
/// recorded as a panic. Finding the root type runs afterwards on the calling
/// thread, outside the catch: a panic there is this glue's bug, and it should
/// take the generate process down rather than be recorded against the tool.
pub fn generate(schema: &serde_json::Value, spec: SpecVersion, timeout: Duration) -> GenOutcome {
    let schema_value = schema.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("generate".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let _ = tx.send(generate_inner(schema_value, spec));
        })
        .expect("spawn generation thread");
    let code = match rx.recv_timeout(timeout) {
        Ok(Ok(code)) => code,
        Ok(Err(outcome)) => return outcome,
        Err(RecvTimeoutError::Timeout) => {
            return GenOutcome::Panic(format!("generation did not finish within {timeout:?}"));
        }
        // generate_inner catches every unwind, so this is a crash of the
        // thread itself (e.g. a panic while panicking).
        Err(RecvTimeoutError::Disconnected) => {
            return GenOutcome::Panic("generation thread exited without a result".to_string());
        }
    };
    match root_type_name(schema, &code) {
        Some((root, rule)) => GenOutcome::Code { code, root, rule },
        // Shouldn't happen: every successful generation emits a root item or
        // a root type alias. Recorded as an error rather than guessed at.
        None => GenOutcome::Error(
            "generated code declares no struct, enum or type alias to use as the root type"
                .to_string(),
        ),
    }
}

/// The generated module body, or the outcome to record instead.
fn generate_inner(schema_value: serde_json::Value, spec: SpecVersion) -> Result<String, GenOutcome> {
    // Panics are captured by a hook installed once (see `install_hook`), and
    // parsing is inside the catch as well as generation: at this commit
    // parsing does real work (dialect resolution, draft-4 normalisation).
    install_hook();
    *CAPTURED.lock().unwrap_or_else(|e| e.into_inner()) = None;
    IN_GENERATION.fetch_add(1, Ordering::SeqCst);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // The schema goes in as-is, with the suite's draft as the default
        // dialect. Everything else is `JsonSchemaSettings` defaults: lenient
        // parsing (unknown keywords kept in `extensions`, not rejected) and
        // `UnknownSchemaUri::Fallback`.
        let settings = JsonSchemaSettings::builder().default_spec_version(spec).build();
        let parsed = JsonSchema::new_from_serde_value(&schema_value, &settings).map_err(|e| e.to_string())?;
        // `DedupeMode::Disabled` rather than the default `Full`. With one
        // schema per call no mode ever produces a shared buffer (only items
        // used by two or more schemas are shared), and serde behavior is the
        // same either way: across the suite the two modes' output differs in
        // 9 groups, and only by merging structurally identical items. But a
        // merge can fold the root into an identical `$defs` entry and give
        // the root that entry's name, which is the one thing `root_type_name`
        // relies on not happening. Everything else is the default:
        // `ModelNameSource::TitleFirst`, and no `unknown_properties`
        // override, so each object follows its schema's
        // `additionalProperties`.
        let code_settings = CodeGenSettings::builder()
            .dedupe_mode(DedupeMode::Disabled)
            .build();
        let output =
            generate_rust(std::slice::from_ref(&parsed), &code_settings).map_err(|e| e.to_string())?;
        if output.shared.is_some() {
            return Err("generate_rust produced a shared buffer in DedupeMode::Disabled".to_string());
        }
        Ok::<_, String>(String::from_utf8_lossy(&output.per_schema[0]).into_owned())
    }));
    IN_GENERATION.fetch_sub(1, Ordering::SeqCst);

    match result {
        Ok(Ok(code)) => Ok(code),
        Ok(Err(e)) => Err(GenOutcome::Error(e)),
        Err(_) => {
            let msg = CAPTURED
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take()
                .unwrap_or_else(|| "unknown panic".to_string());
            Err(GenOutcome::Panic(msg))
        }
    }
}

/// Message from the most recent panic, filled in by the hook below.
static CAPTURED: Mutex<Option<String>> = Mutex::new(None);

/// Nonzero while a generation is in flight, so the hook only intercepts
/// panics that could plausibly be the tool's. A generation that times out
/// never decrements it (its thread never returns), which is the one case
/// where a later panic of our own stays quiet — after a hang we already
/// know something is wrong.
static IN_GENERATION: AtomicUsize = AtomicUsize::new(0);

/// Replace the default panic hook, once, with one that records a
/// generation's panic message instead of printing it. Installed once rather
/// than per generation because a per-generation hook needs a process-global
/// lock held across the generation, and one abandoned (timed-out) thread
/// would then wedge every later group. Panics from anywhere else still go to
/// the default hook.
fn install_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if IN_GENERATION.load(Ordering::SeqCst) == 0 {
                default(info);
                return;
            }
            let msg = info
                .payload()
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| info.payload().downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "non-string panic payload".to_string());
            let loc = info
                .location()
                .map(|l| format!(" at {}:{}", l.file(), l.line()))
                .unwrap_or_default();
            *CAPTURED.lock().unwrap_or_else(|e| e.into_inner()) = Some(format!("{msg}{loc}"));
        }));
    });
}

/// Name of the type the root instance deserializes into, and the rule that
/// found it.
///
/// There is no "call the root `Root`" setting, and the generated code does
/// not mark the root. What the crate guarantees (rust_backend.rs,
/// `render_buffer` / `ordered_items`):
///
/// - A root that is not a struct, enum or union (a scalar, an array, a map,
///   `serde_json::Value`) gets a `pub type <Name> = ...;` alias, emitted
///   last, and that is the only `pub type` the backend ever writes
///   (rule `alias`).
/// - Otherwise the root is a named item, and items are rendered in
///   depth-first post-order from the root, then from each `$defs` /
///   `definitions` entry. So the root closes the first run: it is an item
///   `i` such that the items reachable from it are exactly items `0..=i`.
///   Items before the root can qualify too (a leaf first item always does);
///   the root is the last item of the first run, so the last qualifying item
///   wins (rule `position`).
///
/// The one way an item after the root qualifies is a definition the root
/// doesn't reach that refers back to the root. A definition's node is named
/// for its key (the best-ranked name proposal), so a qualifying item named
/// for a definition key is passed over, unless it is the root's own `$ref`
/// target (the root and a `$ref`-only root's target are one node, named for
/// the key) or nothing earlier qualifies (rule `position-past-def` when one
/// was passed over).
pub fn root_type_name(schema: &serde_json::Value, code: &str) -> Option<(String, &'static str)> {
    if let Some(alias) = code.lines().find_map(|l| decl_name(l, "pub type ")) {
        return Some((alias.to_string(), "alias"));
    }
    let items = items(code);
    let index: HashMap<&str, usize> = items.iter().enumerate().map(|(i, (n, _))| (n.as_str(), i)).collect();
    let edges: Vec<BTreeSet<usize>> = items
        .iter()
        .map(|(_, body)| {
            let body = mask_variant_names(body);
            idents(&body).filter_map(|w| index.get(w).copied()).collect()
        })
        .collect();
    let closes_first_run = |i: usize| -> bool {
        let mut seen = BTreeSet::from([i]);
        let mut stack = vec![i];
        while let Some(n) = stack.pop() {
            for &m in &edges[n] {
                if m > i {
                    return false;
                }
                if seen.insert(m) {
                    stack.push(m);
                }
            }
        }
        seen.len() == i + 1
    };
    let candidates: Vec<usize> = (0..items.len()).filter(|&i| closes_first_run(i)).collect();
    let (defs, own) = definition_names(schema);
    let mut passed_over = false;
    for (k, &i) in candidates.iter().enumerate().rev() {
        let name = &items[i].0;
        if k > 0 && defs.contains(name) && !own.contains(name) {
            passed_over = true;
            continue;
        }
        return Some((name.clone(), if passed_over { "position-past-def" } else { "position" }));
    }
    None
}

/// The item names the root's `$defs` / `definitions` keys sanitize to, and
/// the subset that is the root's own `$ref` target: `#/$defs/<key>`,
/// `#/definitions/<key>`, or an entry whose `$id` (`id` in draft 4) is the
/// `$ref` as written.
fn definition_names(schema: &serde_json::Value) -> (BTreeSet<String>, BTreeSet<String>) {
    let (mut all, mut own) = (BTreeSet::new(), BTreeSet::new());
    let root_ref = schema.get("$ref").and_then(|r| r.as_str());
    for container in ["$defs", "definitions"] {
        let Some(map) = schema.get(container).and_then(|m| m.as_object()) else { continue };
        for (key, def) in map {
            let name = sanitize_struct_name(key);
            let escaped = key.replace('~', "~0").replace('/', "~1");
            let by_pointer = root_ref == Some(format!("#/{container}/{escaped}").as_str());
            let by_id = root_ref.is_some()
                && ["$id", "id"].iter().any(|k| def.get(*k).and_then(|v| v.as_str()) == root_ref);
            if by_pointer || by_id {
                own.insert(name.clone());
            }
            all.insert(name);
        }
    }
    (all, own)
}

/// An item's text with the names it declares rather than refers to blanked
/// out: enum variant names (a line inside an item that starts with four
/// spaces and an uppercase identifier; fields start with `pub` or `#`) and
/// the variants named as `Self::Variant` in the hand-written impls. A
/// variant named like another item is not a reference to it.
fn mask_variant_names(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    for line in body.lines() {
        let mut line = line.to_string();
        if let Some(rest) = line.strip_prefix("    ")
            && rest.starts_with(|c: char| c.is_ascii_uppercase())
        {
            let n = rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(rest.len());
            line = format!("    _{}", &rest[n..]);
        }
        while let Some(at) = line.find("Self::") {
            let rest = &line[at + 6..];
            let n = rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(rest.len());
            line = format!("{}_{}", &line[..at], &rest[n..]);
        }
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// The top-level `pub struct` / `pub enum` items in `code`, in order, each
/// with its text up to the next item (attributes of the next item and `impl`
/// blocks included, which is harmless: they name no other item).
fn items(code: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for line in code.lines() {
        if let Some(name) = decl_name(line, "pub struct ").or_else(|| decl_name(line, "pub enum ")) {
            out.push((name.to_string(), String::new()));
        }
        if let Some((_, body)) = out.last_mut() {
            body.push_str(line);
            body.push('\n');
        }
    }
    out
}

/// Identifiers in Rust source, skipping comments and string literals (a
/// title or a rename in a `#[json_schema(...)]` / `#[serde(...)]` attribute
/// can spell another item's name).
fn idents(src: &str) -> impl Iterator<Item = &str> {
    let bytes = src.as_bytes();
    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'/' && bytes.get(i + 1) == Some(&b'/') {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
        } else if c == b'r' && matches!(bytes.get(i + 1), Some(b'"' | b'#')) && (i == 0 || !is_ident(bytes[i - 1])) {
            // Raw string: r"..." or r#"..."#.
            let mut j = i + 1;
            let mut hashes = 0;
            while bytes.get(j) == Some(&b'#') {
                hashes += 1;
                j += 1;
            }
            if bytes.get(j) != Some(&b'"') {
                i += 1;
                continue;
            }
            j += 1;
            let close: Vec<u8> = std::iter::once(b'"').chain(std::iter::repeat_n(b'#', hashes)).collect();
            while j < bytes.len() && !bytes[j..].starts_with(&close) {
                j += 1;
            }
            i = j + close.len();
        } else if c == b'"' {
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                i += if bytes[i] == b'\\' { 2 } else { 1 };
            }
            i += 1;
        } else if is_ident(c) {
            let start = i;
            while i < bytes.len() && is_ident(bytes[i]) {
                i += 1;
            }
            spans.push((start, i));
        } else {
            i += 1;
        }
    }
    spans.into_iter().map(move |(a, b)| &src[a..b])
}

fn is_ident(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

/// The identifier in a one-line item declaration, e.g. `pub struct Foo {`.
fn decl_name<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(keyword)?;
    let name = rest
        .split([' ', '{', '(', '<', ';', '='])
        .next()
        .filter(|s| !s.is_empty())?;
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
        .then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: Duration = Duration::from_secs(10);
    const SPEC: SpecVersion = SpecVersion::Draft07;

    fn code_and_root(v: serde_json::Value) -> (String, String, &'static str) {
        match generate(&v, SPEC, T) {
            GenOutcome::Code { code, root, rule } => (code, root, rule),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn object_root_is_named_root() {
        let (code, root, rule) =
            code_and_root(serde_json::json!({"type": "object", "properties": {"a": {"type": "string"}}}));
        assert_eq!((root.as_str(), rule), ("Root", "position"));
        assert!(code.contains("pub struct Root"));
    }

    /// The restriction the 0.0.6 row hit: a root that isn't an object with
    /// properties now gets a type.
    #[test]
    fn scalar_root_is_an_alias() {
        let (code, root, rule) = code_and_root(serde_json::json!({"type": "integer"}));
        assert_eq!((root.as_str(), rule), ("Root", "alias"));
        assert!(code.contains("pub type Root = "), "{code}");
        let (_, root, rule) = code_and_root(serde_json::json!({"minLength": 2}));
        assert_eq!((root.as_str(), rule), ("Root", "alias"));
    }

    #[test]
    fn title_renames_the_root() {
        let (_, root, rule) = code_and_root(
            serde_json::json!({"title": "my thing", "type": "object", "properties": {"a": {}}}),
        );
        assert_eq!((root.as_str(), rule), ("MyThing", "position"));
    }

    #[test]
    fn ref_root_is_named_for_the_def() {
        let (_, root, _) = code_and_root(serde_json::json!({
            "$ref": "#/definitions/foo",
            "definitions": {"foo": {"type": "object", "properties": {"a": {"type": "string"}}}}
        }));
        assert_eq!(root, "Foo");
    }

    #[test]
    fn root_any_of_is_an_enum() {
        let (_, root, _) = code_and_root(serde_json::json!({"anyOf": [
            {"type": "object", "properties": {"a": {"type": "string"}}},
            {"type": "integer"}
        ]}));
        assert_eq!(root, "RootAnyOf");
    }

    /// Children come before the root, and an unreached definition after it.
    #[test]
    fn the_root_closes_the_first_run() {
        let (code, root, _) = code_and_root(serde_json::json!({
            "type": "object",
            "properties": {"a": {"type": "object", "properties": {"x": {"type": "string"}}}},
            "definitions": {"later": {"type": "object", "properties": {"y": {"type": "string"}}}}
        }));
        assert_eq!(root, "Root");
        let (a, r, l) = (
            code.find("pub struct A").unwrap(),
            code.find("pub struct Root").unwrap(),
            code.find("pub struct Later").unwrap(),
        );
        assert!(a < r && r < l, "{code}");
    }

    /// A clashing name gets a numeric suffix; position still finds the root.
    #[test]
    fn a_suffixed_root_is_found_by_position() {
        let (code, root, rule) = code_and_root(serde_json::json!({
            "type": "object",
            "properties": {"root": {"type": "object", "properties": {"x": {"type": "string"}}}},
            "definitions": {"later": {"type": "object", "properties": {"y": {"type": "string"}}}}
        }));
        assert_eq!(rule, "position", "{code}");
        assert!(code.contains(&format!("pub root: Option<")), "{code}");
        assert!(code.contains(&format!("pub struct {root} {{\n    #[serde(skip_serializing_if = \"Option::is_none\")]\n    pub root:")), "{code}");
    }

    /// A leaf named `Root` (a property called `root`) before a root named
    /// for its `$ref` target: position, not the name, decides.
    #[test]
    fn a_leaf_named_root_does_not_win() {
        let (code, root, _) = code_and_root(serde_json::json!({
            "$id": "http://example.com/r.json",
            "$ref": "t.json",
            "definitions": {"t": {
                "$id": "t.json",
                "type": "object",
                "properties": {
                    "root": {"type": "object", "properties": {"x": {"type": "string"}}},
                    "n": {"type": "integer"}
                },
                "required": ["n"]
            }}
        }));
        assert_eq!(root, "T", "{code}");
    }

    /// A definition the root doesn't reach that refers back to the root
    /// comes after the root and qualifies too; it is passed over.
    #[test]
    fn a_definition_referring_back_is_passed_over() {
        let (code, root, rule) = code_and_root(serde_json::json!({
            "title": "Foo",
            "type": "object",
            "properties": {"a": {"type": "string"}},
            "definitions": {"root": {"type": "object", "properties": {"back": {"$ref": "#"}}}}
        }));
        assert_eq!((root.as_str(), rule), ("Foo", "position-past-def"), "{code}");
    }

    /// Enum variant names that match a later item are not references to it.
    #[test]
    fn variant_names_are_not_references() {
        let (code, root, _) = code_and_root(serde_json::json!({
            "enum": ["Bar", "baz"],
            "definitions": {"bar": {"type": "object", "properties": {"x": {"type": "string"}}}}
        }));
        assert_eq!(root, "Root", "{code}");
        let (code, root, _) = code_and_root(serde_json::json!({
            "type": "object",
            "properties": {"a": {"enum": ["Later"]}},
            "definitions": {"later": {"type": "object", "properties": {"q": {"type": "string"}}}}
        }));
        assert_eq!(root, "Root", "{code}");
    }

    /// Recursion used to hang 0.0.6; the type graph boxes the back edge.
    #[test]
    fn a_self_reference_terminates() {
        let (code, root, _) =
            code_and_root(serde_json::json!({"type": "object", "properties": {"x": {"$ref": "#"}}}));
        assert_eq!(root, "Root");
        assert!(code.contains("Option<Box<Root>>"), "{code}");
    }

    #[test]
    fn a_boolean_document_is_an_error() {
        assert!(matches!(generate(&serde_json::json!(true), SPEC, T), GenOutcome::Error(_)));
        assert!(matches!(generate(&serde_json::json!(false), SPEC, T), GenOutcome::Error(_)));
    }

    /// Every suite directory has a `SpecVersion`, and nothing else does.
    #[test]
    fn every_suite_draft_maps_and_nothing_else_does() {
        for (dir, spec) in [
            ("draft4", SpecVersion::Draft04),
            ("draft6", SpecVersion::Draft06),
            ("draft7", SpecVersion::Draft07),
            ("draft2019-09", SpecVersion::Draft201909),
            ("draft2020-12", SpecVersion::Draft202012),
        ] {
            assert_eq!(spec_version(Some(dir)), Ok(spec), "{dir}");
        }
        assert!(spec_version(Some("draft3")).is_err());
        assert!(spec_version(Some("")).is_err());
        assert!(spec_version(None).is_err());
    }

    /// The dialect reaches the parse: a draft-4 boolean `exclusiveMaximum`
    /// is normalised to the numeric bound under draft 4 only.
    #[test]
    fn the_dialect_is_live() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {"a": {"type": "integer", "minimum": 0, "maximum": 300, "exclusiveMaximum": true}}
        });
        assert_ne!(
            generate(&schema, SpecVersion::Draft04, T),
            generate(&schema, SpecVersion::Draft07, T)
        );
    }

    #[test]
    fn batch_deps_name_the_pinned_commit() {
        let sha = env!("JSR_GIT_SHA");
        assert_eq!(BATCH_DEPS.matches(&format!("rev = \"{sha}\"")).count(), 2, "{BATCH_DEPS}");
        assert!(VERSION.ends_with(&format!("+{}", &sha[..7])), "{VERSION}");
    }

    #[test]
    fn idents_skip_strings_and_comments() {
        let src = "/// Foo doc\n#[json_schema(title = \"Foo\")]\npub x: Option<Bar>, r#\"Baz\"#";
        assert_eq!(idents(src).collect::<Vec<_>>(), ["json_schema", "title", "pub", "x", "Option", "Bar"]);
    }

    #[test]
    fn decl_name_reads_only_item_declarations() {
        assert_eq!(decl_name("pub struct Foo {", "pub struct "), Some("Foo"));
        assert_eq!(decl_name("pub enum Foo {}", "pub enum "), Some("Foo"));
        assert_eq!(decl_name("pub type Root = i64;", "pub type "), Some("Root"));
        assert_eq!(decl_name("    pub struct Foo {", "pub struct "), None);
        assert_eq!(decl_name("pub enum Foo {", "pub struct "), None);
        assert_eq!(decl_name("pub struct {", "pub struct "), None);
    }
}
