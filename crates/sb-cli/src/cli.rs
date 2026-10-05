//! Command line argument definitions and dispatch.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

use crate::cmd;

/// sssrs is a Rust rewrite of the sing-box universal proxy platform.
#[derive(Debug, Parser)]
#[command(name = "ssssrs", version, about, long_about = None)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Top level subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run the service using the given configuration file.
    Run(CommandArgs),
    /// Check the given configuration file for syntax errors.
    Check(CommandArgs),
    /// Format the given configuration file.
    Format(CommandArgs),
    /// Print version information.
    Version,
}

/// Arguments shared by the configuration driven subcommands.
#[derive(Debug, Args)]
pub struct CommandArgs {
    /// Path to the configuration file.
    #[arg(short, long, value_name = "PATH")]
    pub config: PathBuf,
}

/// Execute the subcommand selected on the command line.
pub fn dispatch(cli: Cli) -> ExitCode {
    match cli.command {
        Command::Run(args) => cmd::run::execute(&args.config),
        Command::Check(args) => cmd::check::execute(&args.config),
        Command::Format(args) => cmd::format::execute(&args.config),
        Command::Version => cmd::version::execute(),
    }
}
