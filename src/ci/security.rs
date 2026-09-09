use anyhow::{Context, Result};
use std::process::Command;

/// Runs `bandit` to scan for security vulnerabilities.
pub fn run_bandit() -> Result<()> {
    Command::new("bandit")
        .arg("-r")
        .arg(".")
        .arg("--format")
        .arg("json")
        .status()
        .context("Failed to run bandit")?;
    Ok(())
}