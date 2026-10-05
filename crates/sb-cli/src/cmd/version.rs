//! Implementation of the `version` subcommand.

use std::process::ExitCode;

/// Print the version banner and report success.
pub fn execute() -> ExitCode {
    println!("{}", crate::version::banner());
    ExitCode::SUCCESS
}
