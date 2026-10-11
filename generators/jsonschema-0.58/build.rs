//! Expose the pinned jsonschema crate version to the harness so results are
//! keyed by the exact version that produced them, and to the generator so
//! the `batch-deps.toml` it writes pins the same one.

use std::path::Path;

fn main() {
    let lock = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", lock.display());
    let text = std::fs::read_to_string(&lock).expect("read Cargo.lock");
    let mut version = None;
    let mut in_jsonschema = false;
    for line in text.lines() {
        if line.starts_with("[[package]]") {
            in_jsonschema = false;
        } else if line == "name = \"jsonschema\"" {
            in_jsonschema = true;
        } else if let Some(v) = line.strip_prefix("version = \"").filter(|_| in_jsonschema) {
            version = Some(v.trim_end_matches('"').to_string());
            break;
        }
    }
    let version = version.expect("jsonschema in Cargo.lock");
    println!("cargo:rustc-env=JSONSCHEMA_CRATE_VERSION={version}");
}
