//! Debug system module for Aetheris.
pub mod diagnostics;
pub mod metrics;
pub mod tracing;

pub struct DebugSystem;

impl DebugSystem {
    pub fn new() -> Self {
        Self
    }
    pub fn enable(&self) {
        unimplemented!("enable")
    }
    pub fn disable(&self) {
        unimplemented!("disable")
    }
}

impl Default for DebugSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_debug_system_new() {
        let _d = DebugSystem::new();
    }
}
