//! Implementation of the `run` subcommand.

use std::path::Path;
use std::process::ExitCode;

use crate::config;

/// Start the service using the configuration file at `path`.
///
/// The service itself is not implemented yet: the configuration file is
/// validated first, then a clear "not implemented" error is reported.
pub fn execute(path: &Path) -> ExitCode {
    if let Err(message) = config::load(path) {
        eprintln!("error: {message}");
        return ExitCode::FAILURE;
    }

    eprintln!("error: run: not implemented yet");
    ExitCode::FAILURE
}
