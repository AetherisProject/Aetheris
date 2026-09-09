#!/usr/bin/env rust

// Subagent implementations for CI/CD orchestration

use std::process;
use std::io;
use anyhow::{Context, Result};
use tracing::{info, error};

// Subagent traits and types
pub trait Subagent: Send + Sync {
    fn name(&self) -> String;
    fn execute(&self) -> Result<String>;
}

// CodeFormat subagent
pub struct CodeFormatSubagent {
    command: String,
}

impl CodeFormatSubagent {
    pub fn new() -> Self {
        CodeFormatSubagent {
            command: "cargo fmt -- --check".to_string(),
        }
    }
    
    pub fn execute(&self) -> Result<String> {
        let output = process::Command::cargo_exe()
            .arg("run")
            .arg("bash")
            .arg("-c")
            .arg(&self.command)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!(format!("Code format failed: {}", error_msg)))
        }
    }
    
    fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
        "CodeFormat".to_string()
    }
}

// Lint subagent
pub struct LintSubagent {
    command: String,
}

impl LintSubagent {
    pub fn new() -> Self {
        LintSubagent {
            command: "cargo clippy --all-targets --all-features -- -D warnings".to_string(),
        }
    }
    
    pub fn execute(&self) -> Result<String> {
        let output = process::Command::cargo_exe()
            .arg("run")
            .arg("bash")
            .arg("-c")
            .arg(&self.command)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!(format!("Lint failed: {}", error_msg)))
        }
    }
    
    fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
        "Lint".to_string()
    }
}

// Test subagent
pub struct TestSubagent {
    command: String,
}

impl TestSubagent {
    pub fn new() -> Self {
        TestSubagent {
            command: "cargo test --all-features".to_string(),
        }
    }
    
    pub fn execute(&self) -> Result<String> {
        let output = process::Command::cargo_exe()
            .arg("run")
            .arg("bash")
            .arg("-c")
            .arg(&self.command)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!(format!("Test failed: {}", error_msg)))
        }
    }
    
    fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
        "Test".to_string()
    }
}

// SecurityScan subagent
pub struct SecurityScanSubagent {
    command: String,
}

impl SecurityScanSubagent {
    pub fn new() -> Self {
        SecurityScanSubagent {
            command: "cargo audit".to_string(),
        }
    }
    
    pub fn execute(&self) -> Result<String> {
        let output = process::Command::cargo_exe()
            .arg("run")
            .arg("bash")
            .arg("-c")
            .arg(&self.command)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!(format!("Security scan failed: {}", error_msg)))
        }
    }
    
    fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
        "SecurityScan".to_string()
    }
    pub struct ArtifactGenerationSubagent {
        command: String,
    }

    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }
        
        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }
        
        fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
            "ArtifactGeneration".to_string()
        }
    }
        command: String,
    }

    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }
        
        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }
        
        fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
            "ArtifactGeneration".to_string()
        }
    }

    pub struct ArtifactGenerationSubagent {
        command: String,
    }

    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }
        
        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }
        
        fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
            "ArtifactGeneration".to_string()
        }
    }
        command: String,
    }

    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }
        
        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }
        
        fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
            "ArtifactGeneration".to_string()
        }
    }
    pub struct ArtifactGenerationSubagent {
        command: String,
    }

    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }
        
        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }
        
        fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
            "ArtifactGeneration".to_string()
        }
    }
// ArtifactGeneration subagent
    pub struct ArtifactGenerationSubagent {
        command: String,
    }

    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }
        
        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }
        
        fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
            "ArtifactGeneration".to_string()
        }
    }
// ArtifactGeneration subagent
    pub struct ArtifactGenerationSubagent {
        command: String,
    }

    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }
        
        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }
        
        fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
            "ArtifactGeneration".to_string()
        }
    }
// ArtifactGeneration subagent
    pub struct ArtifactGenerationSubagent {
        command: String,
    }

impl ArtifactGenerationSubagent {
    pub fn new() -> Self {
        ArtifactGenerationSubagent {
            command: "cargo build --release".to_string(),
        }
    }
    
    pub fn execute(&self) -> Result<String> {
        let output = process::Command::cargo_exe()
            .arg("run")
            .arg("bash")
            .arg("-c")
            .arg(&self.command)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string()
        }
        else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
        }
    }
    
    fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
        "ArtifactGeneration".to_string()
    }
}
// ArtifactGeneration subagent
pub struct ArtifactGenerationSubagent {
    command: String,
}

impl ArtifactGenerationSubagent {
    pub fn new() -> Self {
        ArtifactGenerationSubagent {
            command: "cargo build --release".to_string(),
        }
    }
    
    pub fn execute(&self) -> Result<String> {
        let output = process::Command::cargo_exe()
            .arg("run")
            .arg("bash")
            .arg("-c")
            .arg(&self.command)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
        }
    }
    
    fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
        "ArtifactGeneration".to_string()
    }
}
// ArtifactGeneration subagent
pub struct ArtifactGenerationSubagent {
    command: String,
}

impl ArtifactGenerationSubagent {
    pub fn new() -> Self {
        ArtifactGenerationSubagent {
            command: "cargo build --release".to_string(),
        }
    }
    
    pub fn execute(&self) -> Result<String> {
        let output = process::Command::cargo_exe()
            .arg("run")
            .arg("bash")
            .arg("-c")
            .arg(&self.command)
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
        }
    }
    
    fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
        "ArtifactGeneration".to_string()
    }
}
pub struct PlatformSpecificSubagent {
    command: String,
}

impl PlatformSpecificSubagent {
    pub fn new(platform: &str) -> Self {
        PlatformSpecificSubagent {
            command: format!("echo 'Running platform-specific task for {}"", platform),
        }
    }
    
    pub fn execute(&self) -> Result<String> {
        let output = process::Command::cargo_exe()
            .arg("run")
            .arg("bash")
            .arg("-c")
            .arg(&self.command)
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    fn name(&self) -> String {

    pub struct ArtifactGenerationSubagent {
        command: String,
    }\n\n    impl ArtifactGenerationSubagent {
        pub fn new() -> Self {
            ArtifactGenerationSubagent {
                command: "cargo build --release".to_string(),
            }
        }\n        \n        pub fn execute(&self) -> Result<String> {
            let output = process::Command::cargo_exe()
                .arg("run")
                .arg("bash")
                .arg("-c")
                .arg(&self.command)
                .output()?;
            \n            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string()
            }
            else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(anyhow::anyhow!(format!("Artifact generation failed: {}", error_msg)))
            }
        }\n        \n        fn name(&self) -> String {
            "ArtifactGeneration".to_string()
        }
    }
        "PlatformSpecific".to_string()
    }
}

// Helper function to execute a subagent
pub fn execute_subagent(subagent: &dyn Subagent) -> Result<String> {
    let output = subagent.execute()?;
    info!(subagent_name = %subagent.name(), "Subagent executed successfully")
    Ok(output)
}