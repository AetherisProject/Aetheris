// src/screenshot.rs
use std::fs::File;
use std::io::Write;
use std::process::Command;
use anyhow::{Context, Result};
use playwright::Browser;
use playwright::error::Error;
use image::DynamicImage;

pub async fn capture_web_screenshot() -> Result<()> {
    let project = playwright::project::Project::new()?;
    let browser = project.launch()?;
    let page = browser.new_page()?;
    page.navigate("http://localhost:8080")?;
    page.screenshot(&mut playwright::ScreenshotOptions::default().path("screenshots/web_screenshot.png"))?;
    browser.close()?;
    Ok(())
}

pub fn capture_desktop_screenshot() -> Result<()> {
    use std::env;

    // Use Xvfb to simulate a display
    let _ = Command::new("Xvfb")
        .arg(":99")
        .arg("-screen")
        .arg("0")
        .arg("1024x768x16")
        .status()
        .context("Failed to start Xvfb")?;

    env!("DISPLAY") = ":99".to_string();

    // Run your application in the virtual framebuffer
    let status = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--bin")
        .arg("main")
        .arg("--screenshot")
        .status()
        .context("Failed to run application for screenshot")?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to run application for screenshot"));
    }

    // Capture screenshot using `scrot`
    let _ = Command::new("scrot")
        .arg("screenshots/desktop_screenshot.png")
        .status()
        .context("Failed to capture desktop screenshot")?;

    Ok(())
}