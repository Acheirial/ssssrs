//! Implementation of the `check` subcommand.

use std::path::Path;
use std::process::ExitCode;

use crate::config;

/// Validate the configuration file at `path`.
pub fn execute(path: &Path) -> ExitCode {
    match config::load(path) {
        Ok(_) => {
            println!("config file is valid: {}", path.display());
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
