//! Feed a schema to json-schema-rs and capture code, error, or panic.

use json_schema_rs::json_schema::ref_resolver::{self, ParsedRef};
use json_schema_rs::sanitizers::sanitize_struct_name;
use json_schema_rs::{
    CodeGenSettings, DedupeMode, JsonSchema, JsonSchemaSettings, SpecVersion, generate_rust,
};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

/// Identity string: `json-schema-rs <pinned crate version>` (from build.rs).
pub const VERSION: &str = concat!("json-schema-rs ", env!("JSR_CRATE_VERSION"));

/// Lines appended to the batch crate's `[dependencies]` table (plus one extra
/// table) so the generated code's derive resolves.
///
/// Generated structs carry `#[derive(json_schema_rs_macro::ToJsonSchema)]` and
/// `#[json_schema(...)]` helper attributes; the derive expands to
/// `impl ::json_schema_rs::ToJsonSchema`, so both crates are needed. Both
/// are pinned to the same version as this integration's `json-schema-rs`:
/// the two crates are released together, and a mismatch would compile the
/// derive against a second copy of the library rather than the one these
/// results are named for. If a future version ships without a matching
/// macro release, the batch crate fails to resolve, loudly.
///
/// The `syn` line is a workaround, not a dependency of the generated code:
/// json-schema-rs-macro 0.0.6 names `syn::File` but depends on `syn` without
/// the `full` feature, so it only compiles when something else in the host
/// (proc-macro/build) dependency graph turns that feature on. Under cargo's
/// v2 feature resolver a *normal* dependency won't do it; a build-dependency
/// will, even with no build script present.
pub const BATCH_DEPS: &str = concat!(
    "json-schema-rs = \"=",
    env!("JSR_CRATE_VERSION"),
    "\"\njson-schema-rs-macro = \"=",
    env!("JSR_CRATE_VERSION"),
    r#""

# Not used by the generated code: json-schema-rs-macro refers to syn::File
# but doesn't enable syn's "full" feature, so it needs a host-graph syn that
# does. See generators/json-schema-rs-0.0/README.md.
[build-dependencies]
syn = { version = "2", features = ["full"] }
"#
);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GenOutcome {
    /// Generated Rust source. `root` is the type to deserialize into; it
    /// refers to an item in `code`, which is a complete module body.
    Code { code: String, root: String },
    Error(String),
    Panic(String),
}

/// The suite directory name as a `SpecVersion`, so the dialect comes from
/// where the schema lives rather than from its `$schema`.
///
/// Setting `spec_version` explicitly is what makes the suite directory win:
/// the crate consults `$schema` only when the setting is `None` (and falls
/// back to 2020-12 when `$schema` is absent, which most suite schemas are).
/// The directory is the suite's own claim about the dialect a group tests,
/// and it is never missing, so it is the one we follow — the same choice
/// `generators/jsonschema-0.58` makes.
///
/// All five suite directories map onto a variant, so the `Err` arm is for a
/// draft this integration has never seen: the group is recorded as an
/// `error` naming it rather than generated against a guessed dialect.
///
/// Note that `JsonSchemaSettings::spec_version` is inert in 0.0.6: nothing in
/// parsing, codegen or validation reads it back (`resolved_spec_version` is
/// public but called from nowhere but the crate's own tests). Wired up
/// anyway, because the generator contract says a tool that takes a dialect
/// should be given one, and so that it starts mattering by itself if a later
/// version makes the setting live.
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
/// recorded as a panic.
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
    match rx.recv_timeout(timeout) {
        Ok(outcome) => outcome,
        Err(_) => GenOutcome::Panic(format!("generation did not finish within {timeout:?}")),
    }
}

fn generate_inner(schema_value: serde_json::Value, spec: SpecVersion) -> GenOutcome {
    // The schema goes in as-is, with the suite's draft as the dialect, which
    // is what keeps the crate from reading the dialect out of `$schema`.
    // `disallow_unknown_fields` stays at its default of false, so keywords
    // the crate doesn't model are ignored rather than rejected.
    let settings = JsonSchemaSettings {
        spec_version: Some(spec),
        ..JsonSchemaSettings::default()
    };
    let parsed = match JsonSchema::new_from_serde_value(&schema_value, &settings) {
        Ok(s) => s,
        Err(e) => return GenOutcome::Error(e.to_string()),
    };

    // `DedupeMode::Disabled` is the one-schema-at-a-time path: the output is
    // a single buffer with no shared buffer, so nothing needs splicing, and
    // it's the only path that reports `RootNotObject` and that emits root
    // `anyOf`/`oneOf` enums. (The default `Full` mode emits
    // `pub use crate::Name;` lines for its shared buffer, which can't be
    // right inside a non-root module, and silently drops a non-object root.)
    let code_settings = CodeGenSettings::builder()
        .dedupe_mode(DedupeMode::Disabled)
        .build();

    // Capture the panic message without letting the default hook spew to
    // stderr. The hook is process-global and installed once, writing into a
    // global slot that we clear before each attempt: installing and removing
    // it per generation would need a lock held across the whole generation,
    // and json-schema-rs loops forever on a self-recursive `$ref`, so one
    // abandoned thread would wedge every group after it. `IN_GENERATION`
    // keeps the installed hook from swallowing a panic in this program's own
    // code, which would otherwise leave the harness with a dead generator
    // and no message at all.
    install_hook();
    *CAPTURED.lock().unwrap_or_else(|e| e.into_inner()) = None;
    IN_GENERATION.fetch_add(1, Ordering::SeqCst);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let output = generate_rust(std::slice::from_ref(&parsed), &code_settings)?;
        let code = String::from_utf8_lossy(&output.per_schema[0]).into_owned();
        Ok::<_, json_schema_rs::CodeGenError>(code)
    }));
    IN_GENERATION.fetch_sub(1, Ordering::SeqCst);

    match result {
        Ok(Ok(code)) => match root_type_name(&parsed, &code) {
            Some(root) => GenOutcome::Code { code, root },
            // Shouldn't happen: every path that returns Ok emits at least
            // the root item. Recorded as an error rather than guessed at.
            None => GenOutcome::Error(
                "generated code declares no struct or enum to use as the root type".to_string(),
            ),
        },
        Ok(Err(e)) => GenOutcome::Error(e.to_string()),
        Err(_) => {
            let msg = CAPTURED
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take()
                .unwrap_or_else(|| "unknown panic".to_string());
            GenOutcome::Panic(msg)
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
/// generation's panic message instead of printing it. Panics from anywhere
/// else still go to the default hook: a silent crash in this program would
/// reach the harness as "generate printed something other than a status
/// array" and nothing more.
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

/// Name of the type the root instance deserializes into.
///
/// json-schema-rs has no "call the root type `Root`" setting: the root item is
/// named from `title` (`ModelNameSource::TitleFirst`, the default), else from
/// the `$defs`/`definitions` key when the root is a `$ref`, else `Root` — and
/// a root `anyOf`/`oneOf` becomes an enum with an `AnyOf`/`OneOf` suffix. We
/// predict that name and confirm it was actually declared; if the prediction
/// misses (a root `allOf` is merged by private code whose naming we don't
/// replicate), we fall back to the last item declared, since structs come out
/// after every enum and in post-order, with the root last.
///
/// The fallback is reached by 21 of the 109 suite groups that generate code,
/// every one of them a root `allOf`, and each one lands on `Root`, which is
/// right. Its ordering assumption holds for a struct root only; see the
/// generator's README for why no suite group is in the other position.
fn root_type_name(schema: &JsonSchema, code: &str) -> Option<String> {
    if let Some(name) = predicted_root_name(schema)
        && (declares(code, "pub struct ", &name) || declares(code, "pub enum ", &name))
    {
        return Some(name);
    }
    last_declared(code, "pub struct ").or_else(|| last_declared(code, "pub enum "))
}

fn predicted_root_name(schema: &JsonSchema) -> Option<String> {
    // A non-empty root `allOf` is merged before naming, by a private
    // function; don't guess what the merge produced. (An empty one is left
    // alone by the crate, so the ordinary naming applies.)
    if schema.all_of.as_ref().is_some_and(|v| !v.is_empty()) {
        return None;
    }
    // A root `$ref` into `$defs`/`definitions` contributes the def key as the
    // naming fallback, and the *resolved* schema supplies the title.
    let from_key = match schema.ref_.as_deref() {
        Some(r) => match ref_resolver::parse_ref(r) {
            Ok(ParsedRef::Defs(name) | ParsedRef::Definitions(name)) => Some(name),
            _ => None,
        },
        None => None,
    };
    let effective = match schema.ref_.as_deref() {
        Some(_) => ref_resolver::resolve_schema_ref_transitive(schema, schema).ok()?,
        None => schema,
    };
    let title = effective
        .title
        .as_deref()
        .map(str::trim)
        .filter(|t| !t.is_empty());

    let nonempty = |v: &Option<Vec<JsonSchema>>| v.as_ref().is_some_and(|b| !b.is_empty());
    if let Some(suffix) = if nonempty(&effective.any_of) {
        Some("AnyOf")
    } else if nonempty(&effective.one_of) {
        Some("OneOf")
    } else {
        None
    } {
        // The variant enum prefers the def key over the title.
        return Some(match (&from_key, title) {
            (Some(k), _) => sanitize_struct_name(k) + suffix,
            (None, Some(t)) => sanitize_struct_name(t) + suffix,
            (None, None) => format!("Root{suffix}"),
        });
    }

    // `ModelNameSource::TitleFirst`: title, then the def key, then `Root`.
    Some(match (title, &from_key) {
        (Some(t), _) => sanitize_struct_name(t),
        (None, Some(k)) => sanitize_struct_name(k),
        (None, None) => "Root".to_string(),
    })
}

/// Is `name` declared with this item keyword anywhere in `code`?
fn declares(code: &str, keyword: &str, name: &str) -> bool {
    code.lines().any(|l| decl_name(l, keyword) == Some(name))
}

/// Name in the last `<keyword><Name>` declaration in `code`.
fn last_declared(code: &str, keyword: &str) -> Option<String> {
    code.lines()
        .filter_map(|l| decl_name(l, keyword))
        .next_back()
        .map(str::to_string)
}

/// The identifier in a one-line item declaration, e.g. `pub struct Foo {`.
fn decl_name<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(keyword)?;
    let name = rest
        .split([' ', '{', '(', '<', ';'])
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

    fn code_and_root(v: serde_json::Value) -> (String, String) {
        match generate(&v, SPEC, T) {
            GenOutcome::Code { code, root } => (code, root),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn object_root_is_named_root() {
        let (code, root) =
            code_and_root(serde_json::json!({"type": "object", "properties": {"a": {"type": "string"}}}));
        assert_eq!(root, "Root");
        assert!(code.contains("pub struct Root"));
    }

    #[test]
    fn title_renames_the_root() {
        let (_, root) = code_and_root(
            serde_json::json!({"title": "my thing", "type": "object", "properties": {"a": {}}}),
        );
        assert_eq!(root, "MyThing");
    }

    #[test]
    fn ref_root_is_named_for_the_def() {
        let (_, root) = code_and_root(serde_json::json!({
            "$ref": "#/definitions/foo",
            "definitions": {"foo": {"type": "object", "properties": {"a": {"type": "string"}}}}
        }));
        assert_eq!(root, "Foo");
    }

    #[test]
    fn root_any_of_is_an_enum() {
        let (_, root) = code_and_root(serde_json::json!({"anyOf": [
            {"type": "object", "properties": {"a": {"type": "string"}}},
            {"type": "integer"}
        ]}));
        assert_eq!(root, "RootAnyOf");
    }

    #[test]
    fn nested_objects_leave_the_root_last() {
        let (code, root) = code_and_root(serde_json::json!({
            "type": "object",
            "properties": {"a": {"type": "object", "properties": {"x": {"type": "string"}}}}
        }));
        assert_eq!(root, "Root");
        assert!(code.find("pub struct A").unwrap() < code.find("pub struct Root").unwrap());
    }

    /// The `last_declared` fallback only runs when the prediction misses,
    /// and a root `allOf` is the case that misses: the crate merges it with
    /// a private function and names the merged schema itself. The merge
    /// result is a struct, and structs are emitted after every enum and in
    /// post-order, so the last one is the root.
    #[test]
    fn root_all_of_falls_back_to_the_last_struct() {
        let (code, root) = code_and_root(serde_json::json!({"allOf": [
            {"type": "object", "properties": {"a": {"type": "string"}}},
            {"type": "object", "properties": {"b": {"type": "object", "properties": {"x": {}}}}}
        ]}));
        assert_eq!(root, "Root");
        assert!(code.contains("pub a: Option<String>"), "{code}");
        assert!(code.contains("pub b: Option<B>"), "{code}");
        assert!(code.find("pub struct B").unwrap() < code.find("pub struct Root").unwrap());
    }

    #[test]
    fn non_object_root_is_an_error() {
        // The suite is mostly this: no `type: "object"` with properties.
        assert!(matches!(
            generate(&serde_json::json!({"type": "integer"}), SPEC, T),
            GenOutcome::Error(_)
        ));
        assert!(matches!(
            generate(&serde_json::json!({"properties": {"a": {}}}), SPEC, T),
            GenOutcome::Error(_)
        ));
        // A boolean schema doesn't even parse into the crate's model.
        assert!(matches!(generate(&serde_json::json!(true), SPEC, T), GenOutcome::Error(_)));
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

    /// `spec_version` is inert in this version: the setting changes nothing
    /// about what comes out. Pinned so that a version where it *does* start
    /// to matter shows up here as a failure rather than as a silent change
    /// of results — the generated code is the measurement.
    #[test]
    fn the_dialect_is_inert_in_this_version() {
        // A draft-4 schema whose `exclusiveMaximum` is a boolean, an `id`
        // rather than `$id`, and an array `items`: all things whose meaning
        // depends on the dialect.
        let schema = serde_json::json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "id": "http://example.com/x",
            "type": "object",
            "properties": {
                "a": {"type": "integer", "maximum": 3, "exclusiveMaximum": true},
                "b": {"type": "array", "items": {"type": "string"}}
            }
        });
        let baseline = generate(&schema, SpecVersion::Draft04, T);
        for spec in [
            SpecVersion::Draft06,
            SpecVersion::Draft07,
            SpecVersion::Draft201909,
            SpecVersion::Draft202012,
        ] {
            assert_eq!(generate(&schema, spec, T), baseline, "{spec:?} differed");
        }
        assert!(matches!(baseline, GenOutcome::Code { .. }));
    }

    /// The batch crate's extra dependencies name the version this
    /// integration is pinned to, so bumping the pin can't leave the
    /// generated code compiling against a different json-schema-rs.
    #[test]
    fn batch_deps_name_the_pinned_version() {
        let pinned = VERSION.rsplit(' ').next().unwrap();
        assert!(BATCH_DEPS.contains(&format!("json-schema-rs = \"={pinned}\"")), "{BATCH_DEPS}");
        assert!(
            BATCH_DEPS.contains(&format!("json-schema-rs-macro = \"={pinned}\"")),
            "{BATCH_DEPS}"
        );
        // Lines for `[dependencies]` come first; the one extra table is last.
        assert!(BATCH_DEPS.find("json-schema-rs").unwrap() < BATCH_DEPS.find("[build-").unwrap());
    }

    #[test]
    fn decl_name_reads_only_item_declarations() {
        assert_eq!(decl_name("pub struct Foo {", "pub struct "), Some("Foo"));
        assert_eq!(decl_name("pub struct Foo;", "pub struct "), Some("Foo"));
        assert_eq!(decl_name("pub enum Foo<T> {", "pub enum "), Some("Foo"));
        // Not a declaration of this kind, or not at the start of the line.
        assert_eq!(decl_name("    pub struct Foo {", "pub struct "), None);
        assert_eq!(decl_name("pub enum Foo {", "pub struct "), None);
        assert_eq!(decl_name("pub struct {", "pub struct "), None);
        assert_eq!(decl_name("pub struct Fo-o {", "pub struct "), None);
    }
}
