//! Expose the pinned json-schema-rs version and git revision to the harness
//! so results are keyed by exactly what produced them.
//!
//! Both come out of Cargo.lock: the package's `version` (the workspace
//! version at that commit) and the `#<sha>` at the end of its `source`.

fn main() {
    let lock = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", lock.display());
    let text = std::fs::read_to_string(&lock).expect("read Cargo.lock");
    let (mut version, mut sha) = (None, None);
    let mut in_crate = false;
    for line in text.lines() {
        if line.starts_with("[[package]]") {
            in_crate = false;
        } else if line == "name = \"json-schema-rs\"" {
            in_crate = true;
        } else if !in_crate {
            continue;
        } else if let Some(v) = line.strip_prefix("version = \"") {
            version = Some(v.trim_end_matches('"').to_string());
        } else if let Some(s) = line.strip_prefix("source = \"git+") {
            let full = s.trim_end_matches('"').rsplit('#').next().expect("rsplit yields one item");
            assert!(full.len() == 40 && full.chars().all(|c| c.is_ascii_hexdigit()), "no sha in {s}");
            sha = Some(full.to_string());
        }
    }
    let version = version.expect("json-schema-rs version in Cargo.lock");
    let sha = sha.expect("json-schema-rs is a git dependency in Cargo.lock");
    println!("cargo:rustc-env=JSR_CRATE_VERSION={version}");
    println!("cargo:rustc-env=JSR_GIT_SHA={sha}");
    println!("cargo:rustc-env=JSR_GIT_SHORT={}", &sha[..7]);
}
