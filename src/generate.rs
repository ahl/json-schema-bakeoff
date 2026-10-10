//! Step 1: feed a group's schema to typify and capture code, error, or panic.

use crate::suite::Group;
use schemars::schema::{RootSchema, Schema};
use std::sync::Mutex;
use std::time::Duration;
use typify::{TypeSpace, TypeSpaceSettings};

/// The version string reported in results for the `typify1` label.
pub const TYPIFY1_VERSION: &str = concat!("typify ", "0.10.0-alpha.2");

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GenOutcome {
    /// Generated Rust source. `root` is the type path to deserialize into
    /// (e.g. `Root`, `i64`, `::std::vec::Vec<i64>`); it refers to items in
    /// `code`, which is a complete module body (items only).
    Code { code: String, root: String },
    Error(String),
    Panic(String),
}

/// Run typify on the group's schema with a fixed root name of `Root`.
///
/// Runs on a dedicated thread with a large stack (typify recurses on deep
/// schemas) and catches panics. A generation that exceeds `timeout` is
/// abandoned (the thread is leaked) and recorded as a panic.
pub fn generate(group: &Group, timeout: Duration) -> GenOutcome {
    let schema_value = group.schema.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name(format!("gen {}", group.id()))
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let _ = tx.send(generate_inner(schema_value));
        })
        .expect("spawn generation thread");
    match rx.recv_timeout(timeout) {
        Ok(outcome) => outcome,
        Err(_) => GenOutcome::Panic(format!("generation did not finish within {timeout:?}")),
    }
}

fn generate_inner(schema_value: serde_json::Value) -> GenOutcome {
    // A boolean schema has no definitions. An object schema is parsed as a
    // RootSchema so its `definitions` can be registered first (that is how
    // `#/definitions/x` refs resolve); the root itself goes in with the
    // suggested name `Root`. We don't use `add_root_schema`: it names the
    // root only from a title, and injecting one trips typify into
    // `get_type_name().unwrap()` panics on merged schemas, which would
    // mask whether the construct itself is supported. The cost is that a
    // `$ref` to `#` (the root) is unresolvable and shows up as a panic.
    enum Input {
        Bool(bool),
        Root(Box<RootSchema>),
    }
    let input = match &schema_value {
        serde_json::Value::Bool(b) => Input::Bool(*b),
        _ => match serde_json::from_value::<RootSchema>(schema_value) {
            Ok(root) => Input::Root(Box::new(root)),
            Err(e) => {
                return GenOutcome::Error(format!(
                    "schema did not parse as schemars::schema::RootSchema: {e}"
                ))
            }
        },
    };

    // Capture the panic message without letting the default hook spew to
    // stderr. The hook is process-global, so serialize generation across
    // threads while it is installed.
    static HOOK_LOCK: Mutex<()> = Mutex::new(());
    let _guard = HOOK_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let captured: std::sync::Arc<Mutex<Option<String>>> = Default::default();
    let prev = std::panic::take_hook();
    {
        let captured = captured.clone();
        std::panic::set_hook(Box::new(move |info| {
            let msg = info
                .payload()
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| info.payload().downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "non-string panic payload".to_string());
            let loc = info.location().map(|l| format!(" at {}:{}", l.file(), l.line())).unwrap_or_default();
            *captured.lock().unwrap() = Some(format!("{msg}{loc}"));
        }));
    }
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut ts = TypeSpace::new(&TypeSpaceSettings::default());
        let root_name = Some("Root".to_string());
        let id = match input {
            Input::Bool(b) => ts.add_type_with_name(&Schema::Bool(b), root_name)?,
            Input::Root(root) => {
                ts.add_ref_types(root.definitions)?;
                ts.add_type_with_name(&Schema::Object(root.schema), root_name)?
            }
        };
        let tsp = ts.to_typespace()?;
        let root = tsp.get_type(&id).ident().to_string();
        let stream = ts.to_stream()?;
        Ok::<_, typify::Error>((stream, root))
    }));
    std::panic::set_hook(prev);

    match result {
        Ok(Ok((stream, root))) => {
            let code = match syn::parse2::<syn::File>(stream.clone()) {
                Ok(file) => prettyplease::unparse(&file),
                // Unparseable output still gets handed to rustc so the
                // failure lands in compile_fail with a real diagnostic.
                Err(_) => stream.to_string(),
            };
            GenOutcome::Code { code, root }
        }
        Ok(Err(e)) => GenOutcome::Error(e.to_string()),
        Err(_) => {
            let msg = captured.lock().unwrap().take().unwrap_or_else(|| "unknown panic".to_string());
            GenOutcome::Panic(msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// Every draft7 group generates (code, error, or panic) promptly.
    #[test]
    fn draft7_generates_without_hanging() {
        let groups = crate::suite::load_draft("draft7", false).unwrap();
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        let start = std::time::Instant::now();
        for g in &groups {
            let o = generate(g, Duration::from_secs(30));
            let k = match &o {
                GenOutcome::Code { .. } => "code",
                GenOutcome::Error(_) => "error",
                GenOutcome::Panic(_) => "panic",
            };
            *counts.entry(k).or_default() += 1;
            if let GenOutcome::Panic(m) | GenOutcome::Error(m) = &o {
                eprintln!("{k:5} {}: {}", g.id(), m.lines().next().unwrap_or(""));
            }
        }
        eprintln!("{counts:?} in {:.1?}", start.elapsed());
        assert_eq!(counts.values().sum::<usize>(), groups.len());
    }
}
