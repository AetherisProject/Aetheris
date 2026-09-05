//! Design system module for Aetheris.
pub mod components;
pub mod icons;
pub mod tokens;
pub mod themes;

pub struct DesignSystem;

impl DesignSystem {
    pub fn new() -> Self { Self }
    pub fn load_theme(&self, _name: &str) { unimplemented!("load_theme") }
}

impl Default for DesignSystem {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_design_system_new() { let _d = DesignSystem::new(); }
}
