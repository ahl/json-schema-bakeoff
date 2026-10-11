//! Expose the pinned typify crate version to the harness so results are
//! keyed by the exact version that produced them.

use std::path::Path;

fn main() {
    let lock = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", lock.display());
    let text = std::fs::read_to_string(&lock).expect("read Cargo.lock");
    let mut version = None;
    let mut in_typify = false;
    for line in text.lines() {
        if line.starts_with("[[package]]") {
            in_typify = false;
        } else if line == "name = \"typify\"" {
            in_typify = true;
        } else if let Some(v) = line.strip_prefix("version = \"").filter(|_| in_typify) {
            version = Some(v.trim_end_matches('"').to_string());
            break;
        }
    }
    let version = version.expect("typify in Cargo.lock");
    println!("cargo:rustc-env=TYPIFY_CRATE_VERSION={version}");
}
