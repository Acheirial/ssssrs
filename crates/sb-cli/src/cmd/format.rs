//! Implementation of the `format` subcommand.

use std::path::Path;
use std::process::ExitCode;

use crate::config;

/// Validate the configuration file at `path`.
///
/// Re-serialising the configuration is not implemented yet, so the file is only
/// checked for existence and JSON syntax and left untouched.
pub fn execute(path: &Path) -> ExitCode {
    match config::load(path) {
        Ok(_) => {
            println!("config file is valid: {}", path.display());
            eprintln!("warning: format: not implemented yet, no changes written");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
