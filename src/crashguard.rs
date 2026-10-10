//! See `crashguard.inc.rs`, which is both compiled here and embedded as
//! `SOURCE` in the generated runner binary (see `compile.rs`).

include!("crashguard.inc.rs");

/// The module source, for embedding in the generated runner.
pub const SOURCE: &str = include_str!("crashguard.inc.rs");
