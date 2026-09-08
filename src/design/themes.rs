//! Theme system for Aetheris UI.
//! Provides color schemes matching the cyber-heraldic brand identity.

use serde::{Deserialize, Serialize};

/// Color palette for Aetheris cyber-heraldic theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    // Primary brand colors
    pub primary: String,
    pub primary_hover: String,
    pub primary_active: String,
    
    // Secondary colors
    pub secondary: String,
    pub secondary_hover: String,
    
    // Accent colors
    pub accent: String,
    pub accent_hover: String,
    
    // Success/Error/Warning
    pub success: String,
    pub error: String,
    pub warning: String,
    
    // Neutral colors
    pub background: String,
    pub surface: String,
    pub surface_hover: String,
    pub border: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_disabled: String,
    
    // Special colors
    pub cyber_glow: String,
    pub heraldic_gold: String,
    pub midnight_blue: String,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::dark_cyber()
    }
}

impl ColorPalette {
    /// Dark cyber-heraldic theme (default)
    pub fn dark_cyber() -> Self {
        Self {
            primary: "#6366f1".to_string(),           // Indigo-500
            primary_hover: "#4f46e5".to_string(),     // Indigo-600
            primary_active: "#4338ca".to_string(),    // Indigo-700
            
            secondary: "#8b5cf6".to_string(),         // Violet-500
            secondary_hover: "#7c3aed".to_string(),   // Violet-600
            
            accent: "#06b6d4".to_string(),            // Cyan-500
            accent_hover: "#0891b2".to_string(),      // Cyan-600
            
            success: "#10b981".to_string(),           // Emerald-500
            error: "#ef4444".to_string(),             // Red-500
            warning: "#f59e0b".to_string(),           // Amber-500
            
            background: "#0f172a".to_string(),         // Slate-900
            surface: "#1e293b".to_string(),           // Slate-800
            surface_hover: "#334155".to_string(),     // Slate-700
            border: "#475569".to_string(),            // Slate-600
            text_primary: "#f8fafc".to_string(),      // Slate-50
            text_secondary: "#cbd5e1".to_string(),   // Slate-300
            text_disabled: "#64748b".to_string(),     // Slate-500
            
            cyber_glow: "#00ffff".to_string(),       // Cyan glow
            heraldic_gold: "#ffd700".to_string(),     // Gold accent
            midnight_blue: "#191970".to_string(),     // Deep blue
        }
    }
    
    /// Light cyber-heraldic theme
    pub fn light_cyber() -> Self {
        Self {
            primary: "#6366f1".to_string(),
            primary_hover: "#4f46e5".to_string(),
            primary_active: "#4338ca".to_string(),
            
            secondary: "#8b5cf6".to_string(),
            secondary_hover: "#7c3aed".to_string(),
            
            accent: "#06b6d4".to_string(),
            accent_hover: "#0891b2".to_string(),
            
            success: "#10b981".to_string(),
            error: "#ef4444".to_string(),
            warning: "#f59e0b".to_string(),
            
            background: "#f8fafc".to_string(),        // Slate-50
            surface: "#ffffff".to_string(),           // White
            surface_hover: "#f1f5f9".to_string(),     // Slate-100
            border: "#e2e8f0".to_string(),            // Slate-200
            text_primary: "#0f172a".to_string(),      // Slate-900
            text_secondary: "#475569".to_string(),    // Slate-600
            text_disabled: "#94a3b8".to_string(),     // Slate-400
            
            cyber_glow: "#06b6d4".to_string(),
            heraldic_gold: "#d97706".to_string(),
            midnight_blue: "#1e3a8a".to_string(),
        }
    }
    
    /// High contrast accessibility theme
    pub fn high_contrast() -> Self {
        Self {
            primary: "#0000ff".to_string(),
            primary_hover: "#0000cc".to_string(),
            primary_active: "#000099".to_string(),
            
            secondary: "#800080".to_string(),
            secondary_hover: "#660066".to_string(),
            
            accent: "#008080".to_string(),
            accent_hover: "#006666".to_string(),
            
            success: "#008000".to_string(),
            error: "#ff0000".to_string(),
            warning: "#ff8000".to_string(),
            
            background: "#000000".to_string(),
            surface: "#1a1a1a".to_string(),
            surface_hover: "#333333".to_string(),
            border: "#ffffff".to_string(),
            text_primary: "#ffffff".to_string(),
            text_secondary: "#cccccc".to_string(),
            text_disabled: "#666666".to_string(),
            
            cyber_glow: "#00ffff".to_string(),
            heraldic_gold: "#ffff00".to_string(),
            midnight_blue: "#000080".to_string(),
        }
    }
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub palette: ColorPalette,
    pub is_dark: bool,
}

impl Theme {
    pub fn new(name: String, palette: ColorPalette, is_dark: bool) -> Self {
        Self { name, palette, is_dark }
    }
    
    pub fn dark_cyber() -> Self {
        Self::new("Dark Cyber".to_string(), ColorPalette::dark_cyber(), true)
    }
    
    pub fn light_cyber() -> Self {
        Self::new("Light Cyber".to_string(), ColorPalette::light_cyber(), false)
    }
    
    pub fn high_contrast() -> Self {
        Self::new("High Contrast".to_string(), ColorPalette::high_contrast(), true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_theme() {
        let theme = Theme::dark_cyber();
        assert_eq!(theme.name, "Dark Cyber");
        assert!(theme.is_dark);
    }
    
    #[test]
    fn test_color_palette() {
        let palette = ColorPalette::dark_cyber();
        assert_eq!(palette.primary, "#6366f1");
        assert_eq!(palette.background, "#0f172a");
    }
}