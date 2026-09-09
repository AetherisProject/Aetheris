#![feature(proc_macro)]

use anyhow::{Context, Result};
use std::env;
use std::process;

mod screenshot;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--screenshot".to_string()) {
        if args.contains(&"--web".to_string()) {
            screenshot::capture_web_screenshot().await?;
        } else {
            screenshot::capture_desktop_screenshot()?;
        }
        println!("Screenshot captured successfully!");
    }

    Ok(())
}