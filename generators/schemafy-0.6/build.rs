//! Expose the pinned schemafy_lib crate version to the harness so results
//! are keyed by the exact version that produced them.

use std::path::Path;

fn main() {
    let lock = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", lock.display());
    let text = std::fs::read_to_string(&lock).expect("read Cargo.lock");
    let mut version = None;
    let mut in_schemafy = false;
    for line in text.lines() {
        if line.starts_with("[[package]]") {
            in_schemafy = false;
        } else if line == "name = \"schemafy_lib\"" {
            in_schemafy = true;
        } else if let Some(v) = line.strip_prefix("version = \"").filter(|_| in_schemafy) {
            version = Some(v.trim_end_matches('"').to_string());
            break;
        }
    }
    let version = version.expect("schemafy_lib in Cargo.lock");
    println!("cargo:rustc-env=SCHEMAFY_CRATE_VERSION={version}");
}
