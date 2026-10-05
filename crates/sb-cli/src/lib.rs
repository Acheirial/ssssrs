//! Command line interface for the `ssssrs` binary.
//!
//! The crate is split into small modules so that the individual subcommands can
//! be filled in independently later on:
//!
//! * [`cli`] holds the clap definitions and dispatches to a subcommand.
//! * [`cmd`] holds one module per subcommand.
//! * [`config`] holds the configuration file loading shared by subcommands.
//! * [`version`] holds the version and toolchain information.

pub mod cli;
pub mod cmd;
pub mod config;
pub mod version;

use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;

/// Parse the process arguments and run the selected subcommand.
pub fn run() -> ExitCode {
    cli::dispatch(Cli::parse())
}
