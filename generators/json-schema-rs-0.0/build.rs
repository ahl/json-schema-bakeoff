//! Expose the pinned json-schema-rs crate version to the harness so results
//! are keyed by the exact version that produced them.

use std::path::Path;

fn main() {
    let lock = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", lock.display());
    let text = std::fs::read_to_string(&lock).expect("read Cargo.lock");
    let mut version = None;
    let mut in_crate = false;
    for line in text.lines() {
        if line.starts_with("[[package]]") {
            in_crate = false;
        } else if line == "name = \"json-schema-rs\"" {
            in_crate = true;
        } else if let Some(v) = line.strip_prefix("version = \"").filter(|_| in_crate) {
            version = Some(v.trim_end_matches('"').to_string());
            break;
        }
    }
    let version = version.expect("json-schema-rs in Cargo.lock");
    println!("cargo:rustc-env=JSR_CRATE_VERSION={version}");
}
