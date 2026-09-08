//! Design system module for Aetheris.
//! Provides responsive design tokens, themes, components, and icons for desktop, tablet, and mobile.
pub mod components;
pub mod icons;
pub mod tokens;
pub mod themes;

pub use components::{Device, Layout, ResponsiveDesign};
pub use icons::{Icon, IconSize};
pub use themes::{ColorPalette, Theme};
pub use tokens::{Breakpoint, BorderRadius, Duration, Shadow, Spacing, Typography, ZIndex};

/// Main design system entry point
pub struct DesignSystem {
    pub theme: Theme,
    pub responsive: ResponsiveDesign,
}

impl DesignSystem {
    pub fn new() -> Self {
        Self {
            theme: Theme::dark_cyber(),
            responsive: ResponsiveDesign::desktop(),
        }
    }
    
    pub fn with_theme(theme: Theme) -> Self {
        Self {
            theme,
            responsive: ResponsiveDesign::desktop(),
        }
    }
    
    pub fn with_device(device: Device) -> Self {
        Self {
            theme: Theme::dark_cyber(),
            responsive: ResponsiveDesign::for_device(device),
        }
    }
    
    pub fn with_width(width: u32) -> Self {
        Self {
            theme: Theme::dark_cyber(),
            responsive: ResponsiveDesign::for_width(width),
        }
    }
    
    pub fn load_theme(&mut self, name: &str) {
        self.theme = match name.to_lowercase().as_str() {
            "dark" | "dark-cyber" => Theme::dark_cyber(),
            "light" | "light-cyber" => Theme::light_cyber(),
            "high-contrast" | "high_contrast" => Theme::high_contrast(),
            _ => Theme::dark_cyber(),
        };
    }
    
    pub fn set_device(&mut self, device: Device) {
        self.responsive = ResponsiveDesign::for_device(device);
    }
    
    pub fn update_for_width(&mut self, width: u32) {
        self.responsive = ResponsiveDesign::for_width(width);
    }
}

impl Default for DesignSystem {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_design_system_new() {
        let ds = DesignSystem::new();
        assert_eq!(ds.theme.name, "Dark Cyber");
        assert_eq!(ds.responsive.device, Device::Desktop);
    }
    
    #[test]
    fn test_design_system_with_theme() {
        let ds = DesignSystem::with_theme(Theme::light_cyber());
        assert_eq!(ds.theme.name, "Light Cyber");
    }
    
    #[test]
    fn test_design_system_with_device() {
        let ds = DesignSystem::with_device(Device::Mobile);
        assert_eq!(ds.responsive.device, Device::Mobile);
    }
    
    #[test]
    fn test_load_theme() {
        let mut ds = DesignSystem::new();
        ds.load_theme("light");
        assert_eq!(ds.theme.name, "Light Cyber");
    }
}

