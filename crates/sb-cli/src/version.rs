//! Version and toolchain information baked in at compile time.

/// Cargo package version of the `ssssrs` binary.
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Release version of the Rust compiler used for this build.
pub const RUSTC_VERSION: &str = env!("SSSSRS_RUSTC_VERSION");

/// Release channel of the Rust compiler used for this build.
pub const RUSTC_CHANNEL: &str = env!("SSSSRS_RUSTC_CHANNEL");

/// Commit date of the Rust compiler used for this build.
pub const RUSTC_DATE: &str = env!("SSSSRS_RUSTC_DATE");

/// Multi-line banner printed by the `version` subcommand.
pub fn banner() -> String {
    format!(
        "ssssrs version {PACKAGE_VERSION}\nrustc {RUSTC_VERSION} ({RUSTC_CHANNEL} {RUSTC_DATE})"
    )
}
