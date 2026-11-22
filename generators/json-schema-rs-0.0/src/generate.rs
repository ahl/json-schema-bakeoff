//! Feed a schema to json-schema-rs and capture code, error, or panic.

use json_schema_rs::{
    CodeGenInput, CodeGenSettings, JsonSchemaSettings, Schema, SpecVersion, generate_rust,
};
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
/// `#[json_schema(...)]` helper attributes, structs derive
/// `json_schema_rs_macro::Deserialize`, and the output calls the library's
/// `constrained_newtype!` / `self_validating!` / `deserialize_union!`
/// macros and names `json_schema_rs::TypeValidators`, so both crates are
/// needed. Both are pinned
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
    /// Generated Rust source. `root` is the type to deserialize into, as the
    /// tool reports it (`GenerateRustOutput::root_types`); it names an item
    /// or alias in `code`, which is a complete module body.
    Code { code: String, root: String },
    Error(String),
    Panic(String),
}

/// The harness's dialect name as a `SpecVersion`.
///
/// It goes in as `JsonSchemaSettings::default_spec_version`: the draft for a
/// document whose root has no `$schema`. That is the setting the crate
/// documents for exactly this use (its own suite and Bowtie harnesses pass
/// the suite directory's draft there), and it is what the jsonschema row
/// does too (`with_draft` sets the root's draft and lets embedded resources
/// follow their own `$schema`). The unified suite's schemas carry no
/// `$schema` (six custom metaschema cases aside), so the dialect field
/// decides every root. The other knob,
/// `forced_spec_version`, would also override an embedded resource's
/// `$schema`, which the spec says it must not.
///
/// All five released dialects map onto a variant, so the `Err` arm is for a
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
        other => Err(format!(
            "no json-schema-rs SpecVersion for dialect {other:?}"
        )),
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
        Ok(Ok((code, root))) => GenOutcome::Code { code, root },
        Ok(Err(outcome)) => outcome,
        Err(RecvTimeoutError::Timeout) => {
            GenOutcome::Panic(format!("generation did not finish within {timeout:?}"))
        }
        // generate_inner catches every unwind, so this is a crash of the
        // thread itself (e.g. a panic while panicking).
        Err(RecvTimeoutError::Disconnected) => {
            GenOutcome::Panic("generation thread exited without a result".to_string())
        }
    }
}

/// The generated module body and its root type's name, or the outcome to
/// record instead.
fn generate_inner(
    schema_value: serde_json::Value,
    spec: SpecVersion,
) -> Result<(String, String), GenOutcome> {
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
        let parsed = Schema::new_from_serde_value(&schema_value, &settings).map_err(|e| e.to_string())?;
        // The codegen input carries the same settings: they are what any
        // document a `$ref` retrieves is parsed with (the crate's CLI passes
        // its parse settings there too). The retriever is the default
        // `OfflineRetriever`, so a remote ref fails, as policy wants.
        let input = CodeGenInput::new().schema(&parsed).settings(settings);
        // `CodeGenSettings` defaults throughout, `DedupeMode::Full`
        // included. Earlier pins used `Disabled` to keep a merge from folding
        // the root into an identical `$defs` entry and fooling the glue's
        // own root detection; the tool now reports the root's name, so that
        // reason is gone. With one schema per call nothing is shared (only
        // items used by two or more schemas are), so a shared buffer would
        // be a surprise and is an error rather than silently dropped.
        let output = generate_rust(input, &CodeGenSettings::default()).map_err(|e| e.to_string())?;
        if output.shared.is_some() {
            return Err("generate_rust produced a shared buffer for a single schema".to_string());
        }
        let [code] = output.per_schema.as_slice() else {
            return Err(format!("generate_rust returned {} buffers for one schema", output.per_schema.len()));
        };
        let [root] = output.root_types.as_slice() else {
            return Err(format!("generate_rust returned {} root types for one schema", output.root_types.len()));
        };
        Ok::<_, String>((String::from_utf8_lossy(code).into_owned(), root.clone()))
    }));
    IN_GENERATION.fetch_sub(1, Ordering::SeqCst);

    match result {
        Ok(Ok(out)) => Ok(out),
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
        assert!(code.contains("pub struct Root"), "{code}");
    }

    /// The restriction the 0.0.6 row hit: a root that isn't an object with
    /// properties gets a type, and the reported root names it.
    #[test]
    fn scalar_root_is_an_alias() {
        let (code, root) = code_and_root(serde_json::json!({"type": "integer"}));
        assert_eq!(root, "Root");
        assert!(code.contains("pub type Root = "), "{code}");
        // A constrained scalar is a newtype (constrained newtypes are on by
        // default).
        let (code, root) = code_and_root(serde_json::json!({"minLength": 2}));
        assert_eq!(root, "Root");
        assert!(code.contains("pub struct Root(String);"), "{code}");
    }

    /// The reported root is declared in the buffer: a titled root, a `$ref`
    /// root, a root identical to a definition (which the default dedupe can
    /// merge), and a union root.
    #[test]
    fn the_reported_root_is_declared() {
        for v in [
            serde_json::json!({"title": "my thing", "type": "object", "properties": {"a": {}}}),
            serde_json::json!({
                "$ref": "#/definitions/foo",
                "definitions": {"foo": {"type": "object", "properties": {"a": {"type": "string"}}}}
            }),
            serde_json::json!({
                "type": "object",
                "properties": {"a": {"type": "string"}},
                "definitions": {"same": {"type": "object", "properties": {"a": {"type": "string"}}}}
            }),
            serde_json::json!({"anyOf": [
                {"type": "object", "properties": {"a": {"type": "string"}}},
                {"type": "integer"}
            ]}),
        ] {
            let (code, root) = code_and_root(v);
            assert!(
                ["struct", "enum", "type"].iter().any(|k| code.contains(&format!("pub {k} {root}"))),
                "{root}: {code}"
            );
        }
    }

    /// Recursion used to hang 0.0.6; the type graph boxes the back edge.
    #[test]
    fn a_self_reference_terminates() {
        let (code, root) =
            code_and_root(serde_json::json!({"type": "object", "properties": {"x": {"$ref": "#"}}}));
        assert_eq!(root, "Root");
        assert!(code.contains("Option<Box<Root>>"), "{code}");
    }

    /// Boolean documents get a type since PR #75: `true` is a `Value` alias
    /// and `false` an uninhabited enum.
    #[test]
    fn boolean_documents_generate() {
        let (code, root) = code_and_root(serde_json::json!(true));
        assert_eq!(root, "Root");
        assert!(code.contains("pub type Root = serde_json::Value;"), "{code}");
        let (code, root) = code_and_root(serde_json::json!(false));
        assert_eq!(root, "Root");
        assert!(code.contains("pub enum Root {"), "{code}");
    }

    /// Every released dialect has a `SpecVersion`, and nothing else does.
    #[test]
    fn every_released_dialect_maps_and_nothing_else_does() {
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
}
