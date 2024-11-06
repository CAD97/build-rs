//! build-rs provides a strongly typed interface around the Cargo build script
//! protocol. Cargo provides inputs to the build script by environment variable
//! and accepts commands by printing to stdout.
#![cfg_attr(all(doc, feature = "unstable"), feature(doc_auto_cfg, doc_cfg))]

#[cfg(feature = "unstable")]
macro_rules! unstable {
    ($feature:ident, $issue:literal) => {
        concat!(
            r#"<div class="stab unstable">"#,
            r#"<span class="emoji">🔬</span>"#,
            r#"<span>This is a nightly-only experimental API. (<code>"#,
            stringify!($feature),
            r#"</code>&nbsp;<a href="https://github.com/rust-lang/rust/issues/"#,
            $issue,
            r#"">#"#,
            $issue,
            r#"</a>)</span>"#,
            r#"</div>"#
        )
    };
}

macro_rules! msrv {
    ($ver:literal) => {
        concat!("> MSRV: Respected as of ", $ver, ".")
    };
}

mod allow_use;
mod ident;

pub mod input;
pub mod output;
