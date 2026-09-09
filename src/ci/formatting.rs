use anyhow::{Context, Result};
use std::process::Command;

/// Runs `cargo fmt` to format the codebase.
pub fn run_rustfmt() -> Result<()> {
    Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .arg("--emit-errors")
        .status()
        .context("Failed to run cargo fmt")?;
    Ok(())
}