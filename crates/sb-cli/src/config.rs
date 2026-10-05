//! Configuration file loading shared by the subcommands.

use std::fs;
use std::path::Path;

/// A configuration file that has been read and parsed as JSON.
#[derive(Debug)]
pub struct LoadedConfig {
    /// Raw text of the configuration file.
    pub source: String,
    /// Parsed JSON value of the configuration file.
    pub value: serde_json::Value,
}

/// Read and JSON-parse the configuration file at `path`.
///
/// Returns a human readable error message when the file does not exist, cannot
/// be read, or does not contain valid JSON.
pub fn load(path: &Path) -> Result<LoadedConfig, String> {
    if !path.exists() {
        return Err(format!("config file not found: {}", path.display()));
    }

    let source = fs::read_to_string(path)
        .map_err(|error| format!("failed to read config file {}: {error}", path.display()))?;

    let value: serde_json::Value = serde_json::from_str(&source)
        .map_err(|error| format!("invalid JSON in config file {}: {error}", path.display()))?;

    Ok(LoadedConfig { source, value })
}
