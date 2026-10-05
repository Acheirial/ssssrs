//! Records the Rust toolchain version used for this build.

use std::env;
use std::process::Command;

fn main() {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));

    let (version, channel, date) = match Command::new(&rustc).arg("-Vv").output() {
        Ok(output) if output.status.success() => {
            let text = String::from_utf8_lossy(&output.stdout);
            (
                field(&text, "release"),
                field(&text, "channel"),
                field(&text, "commit-date"),
            )
        }
        _ => (
            String::from("unknown"),
            String::from("unknown"),
            String::from("unknown"),
        ),
    };

    println!("cargo:rustc-env=SSSSRS_RUSTC_VERSION={version}");
    println!("cargo:rustc-env=SSSSRS_RUSTC_CHANNEL={channel}");
    println!("cargo:rustc-env=SSSSRS_RUSTC_DATE={date}");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RUSTC");
}

/// Extract the value of a `key: value` line from `rustc -Vv` output.
fn field(text: &str, key: &str) -> String {
    let prefix = format!("{key}: ");
    text.lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| String::from("unknown"))
}
