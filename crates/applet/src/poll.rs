//! checkupdates poll.

use std::process::Command;

/// Checker binary path.
pub const CHECKUPDATES_BIN: &str = "checkupdates";

/// Checker env override.
pub const CHECKUPDATES_ENV: &str = "COSMIC_WICARA_CHECKUPDATES";

/// One pending update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingUpdate {
    /// Package name.
    pub package: String,
    /// Full update line.
    pub line: String,
}

impl PendingUpdate {
    /// Create update record.
    #[must_use]
    pub fn new(line: String) -> Self {
        let package = line.split_whitespace().next().unwrap_or("?").to_owned();
        Self { package, line }
    }
}

/// Parse update lines.
#[must_use]
pub fn parse_checkupdates_output(output: &str) -> Vec<PendingUpdate> {
    output
        .lines()
        .filter_map(|raw| {
            let line = raw.trim();
            if line.is_empty() {
                None
            } else {
                Some(PendingUpdate::new(line.to_owned()))
            }
        })
        .collect()
}

/// Pending update count.
#[must_use]
pub fn count_pending(output: &str) -> usize {
    parse_checkupdates_output(output).len()
}

/// Checker command.
#[must_use]
pub fn checker_command() -> String {
    std::env::var(CHECKUPDATES_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| CHECKUPDATES_BIN.to_owned())
}

/// Run one poll.
pub fn run_checkupdates() -> Result<Vec<PendingUpdate>, String> {
    let command = checker_command();
    let output = Command::new(&command)
        .output()
        .map_err(|err| format!("{command}: {err}"))?;
    if !output.status.success() {
        return Err(format!("{command}: exited {}", output.status));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_checkupdates_output(&stdout))
}
