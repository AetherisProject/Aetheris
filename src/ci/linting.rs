use anyhow::{Context, Result};
use std::process::Command;

/// Runs `cargo clippy` to lint the codebase.
pub fn run_clippy() -> Result<()> {
    Command::new("cargo")
        .arg("clippy")
        .arg("--all-targets")
        .arg("--all-features")
        .arg("--")
        .arg("--warn")
        .status()
        .context("Failed to run cargo clippy")?;
    Ok(())
}