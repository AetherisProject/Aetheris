//! Design tokens for Aetheris UI system.
//! Provides spacing, typography, colors, and breakpoints for responsive design.
//! Includes serialization and export capabilities for cross-platform consistency.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Spacing scale (8px base unit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Spacing {
    /// 0px (none)
    None = 0,
    /// 2px (hairline)
    Hairline = 2,
    /// 4px (compact)
    Compact = 4,
    /// 8px (base unit)
    Base = 8,
    /// 12px (small)
    Small = 12,
    /// 16px (medium)
    Medium = 16,
    /// 24px (large)
    Large = 24,
    /// 32px (extra large)
    XLarge = 32,
    /// 48px (2x large)
    XXLarge = 48,
    /// 64px (3x large)
    XXXLarge = 64,
}

impl Spacing {
    /// Convert to pixels
    pub fn as_px(self) -> u32 {
        self as u32
    }
}

/// Typography scale (font weights and sizes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Typography {
    /// Regular weight (400)
    Regular400,
    /// Medium weight (500)
    Medium500,
    /// Semibold weight (600)
    Semibold600,
    /// Bold weight (700)
    Bold700,
    /// ExtraBold weight (800)
    ExtraBold800,

    /// Font sizes (px)
    Caption10,
    Overline12,
    Body14,
    Body16,
    Subtitle18,
    Title20,
    Heading24,
    Display32,
    Hero48,
}

impl Typography {
    /// Get font weight
    pub fn weight(self) -> u16 {
        match self {
            Typography::Regular400 => 400,
            Typography::Medium500 => 500,
            Typography::Semibold600 => 600,
            Typography::Bold700 => 700,
            Typography::ExtraBold800 => 800,
            _ => 400,
        }
    }

    /// Get font size in pixels
    pub fn size_px(self) -> u32 {
        match self {
            Typography::Caption10 => 10,
            Typography::Overline12 => 12,
            Typography::Body14 => 14,
            Typography::Body16 => 16,
            Typography::Subtitle18 => 18,
            Typography::Title20 => 20,
            Typography::Heading24 => 24,
            Typography::Display32 => 32,
            Typography::Hero48 => 48,
        }
    }
}

/// Border radius scale
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderRadius {
    /// Sharp corners (0px)
    Sharp = 0,
    /// Slightly rounded (4px)
    Slight = 4,
    /// Rounded (8px)
    Rounded = 8,
    /// Pill shape (16px)
    Pill = 16,
    /// Circle (50% - calculated at runtime)
    Circle = 999,
}

impl BorderRadius {
    /// Get radius in pixels (999 for circle)
    pub fn as_px(self) -> u32 {
        self as u32
    }
}

/// Responsive breakpoints (viewport widths)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Breakpoint {
    /// Mobile portrait (320px)
    MobilePortrait = 320,
    /// Mobile landscape (480px)
    MobileLandscape = 480,
    /// Tablet portrait (768px)
    TabletPortrait = 768,
    /// Tablet landscape (1024px)
    TabletLandscape = 1024,
    /// Desktop (1280px)
    Desktop = 1280,
    /// Wide desktop (1920px)
    WideDesktop = 1920,
}

impl Breakpoint {
    /// Get breakpoint width in pixels
    pub fn width_px(self) -> u32 {
        self as u32
    }
}

/// Animation durations (ms)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Duration {
    /// Instant (0ms)
    Instant = 0,
    /// Fast (50ms)
    Fast = 50,
    /// Normal (100ms)
    Normal = 100,
    /// Slow (200ms)
    Slow = 200,
    /// Lingering (300ms)
    Lingering = 300,
    /// Extended (500ms)
    Extended = 500,
    /// Drawn out (1000ms)
    DrawnOut = 1000,
}

impl Duration {
    /// Get duration in milliseconds
    pub fn as_ms(self) -> u32 {
        self as u32
    }
}

/// Shadow depth (elevation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shadow {
    /// No shadow
    None = 0,
    /// Subtle elevation (1px)
    Subtle = 1,
    /// Medium elevation (2px)
    Medium = 2,
    /// Raised (4px)
    Raised = 4,
    /// Highly elevated (8px)
    High = 8,
    /// Floating (12px)
    Floating = 12,
    /// Overlay (16px)
    Overlay = 16,
}

impl Shadow {
    /// Check if no shadow
    pub fn is_none(self) -> bool {
        matches!(self, Shadow::None)
    }
    /// Check if no shadow
    pub fn is_none(self) -> bool {
        matches!(self, Shadow::None)
    }
}

impl Shadow {
    /// Check if no shadow
    pub fn is_none(self) -> bool {
        matches!(self, Shadow::None)
    }
    /// Get shadow depth in pixels
    pub fn depth_px(self) -> u32 {
        self as u32
    }
}

/// Z-index scale (stacking order)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZIndex {
    /// Background (-1)
    Background = -1,
    /// Content (0)
    Content = 0,
    /// Docked (10)
    Docked = 10,
    /// Dragging (20)
    Dragging = 20,
    /// Modal (100)
    Modal = 100,
    /// Popover (200)
    Popover = 200,
    /// Tooltip (300)
    Tooltip = 300,
    /// Toast (400)
    Toast = 400,
    /// System overlay (500)
    SystemOverlay = 500,
    /// Fullscreen modal (1000)
    FullscreenModal = 1000,
}

impl ZIndex {
    /// Get z-index value
    pub fn value(self) -> i32 {
        self as i32
    }
}

/// Helper to generate all token values for export to JSON/TypeScript/Dart
pub fn generate_all_tokens() -> HashMap<String, serde_json::Value> {
    let mut map = HashMap::new();

    // Spacing tokens
    for variant in [
        Spacing::None,
        Spacing::Hairline,
        Spacing::Compact,
        Spacing::Base,
        Spacing::Small,
        Spacing::Medium,
        Spacing::Large,
        Spacing::XLarge,
        Spacing::XXLarge,
        Spacing::XXXLarge,
    ]
    .iter()
    {
        map.insert(
            format!("spacing.base"),
            serde_json::Value::Number(variant.as_px().into()),
        );
    }

    // Typography tokens (weights and sizes)
    for variant in [
        Typography::Regular400,
        Typography::Medium500,
        Typography::Semibold600,
        Typography::Bold700,
        Typography::ExtraBold800,
    ]
    .iter()
    {
        map.insert(
            format!("typography.weight.regular400"),
            serde_json::Value::Number(variant.weight().into()),
        );
    }
    for variant in [
        Typography::Caption10,
        Typography::Overline12,
        Typography::Body14,
        Typography::Body16,
        Typography::Subtitle18,
        Typography::Title20,
        Typography::Heading24,
        Typography::Display32,
        Typography::Hero48,
    ]
    .iter()
    {
        map.insert(
            format!("typography.size.body16"),
            serde_json::Value::Number(variant.size_px().into()),
        );
    }

    // Border radius tokens
    for variant in [
        BorderRadius::Sharp,
        BorderRadius::Slight,
        BorderRadius::Rounded,
        BorderRadius::Pill,
        BorderRadius::Circle,
    ]
    .iter()
    {
        map.insert(
            format!("border_radius.{}", variant),
            serde_json::Value::Number(variant.as_px().into()),
        );
    }

    // Breakpoint tokens
    for variant in [
        Breakpoint::MobilePortrait,
        Breakpoint::MobileLandscape,
        Breakpoint::TabletPortrait,
        Breakpoint::TabletLandscape,
        Breakpoint::Desktop,
        Breakpoint::WideDesktop,
    ]
    .iter()
    {
        map.insert(
            format!("breakpoint.{}", variant),
            serde_json::Value::Number(variant.width_px().into()),
        );
    }

    // Duration tokens
    for variant in [
        Duration::Instant,
        Duration::Fast,
        Duration::Normal,
        Duration::Slow,
        Duration::Lingering,
        Duration::Extended,
        Duration::DrawnOut,
    ]
    .iter()
    {
        map.insert(
            format!("duration.{}", variant),
            serde_json::Value::Number(variant.as_ms().into()),
        );
    }

    // Shadow tokens
    for variant in [
        Shadow::None,
        Shadow::Subtle,
        Shadow::Medium,
        Shadow::Raised,
        Shadow::High,
        Shadow::Floating,
        Shadow::Overlay,
    ]
    .iter()
    {
        map.insert(
            format!("shadow.{}", variant),
            serde_json::Value::Number(variant.depth_px().into()),
        );
    }

    // Z-index tokens
    for variant in [
        ZIndex::Background,
        ZIndex::Content,
        ZIndex::Docked,
        ZIndex::Dragging,
        ZIndex::Modal,
        ZIndex::Popover,
        ZIndex::Tooltip,
        ZIndex::Toast,
        ZIndex::SystemOverlay,
        ZIndex::FullscreenModal,
    ]
    .iter()
    {
        map.insert(
            format!("z_index.{}", variant),
            serde_json::Value::Number(variant.value().into()),
        );
    }

    map
}

/// Export design system to JSON string for cross-platform consumption
pub fn export_to_json() -> String {
    let tokens = generate_all_tokens();
    serde_json::to_string_pretty(&tokens).unwrap_or_else(|_| "{}".to_string())
}

/// Export design system to TypeScript interface definition
pub fn export_to_typescript() -> String {
    let mut ts = String::new();
    ts.push_str("// Auto-generated from Aetheris Rust design system\n");
    ts.push_str("export interface AetherisDesignTokens {\n");

    // Group tokens by category
    let mut spacing = String::new();
    let mut typography_weight = String::new();
    let mut typography_size = String::new();
    let mut border_radius = String::new();
    let mut breakpoint = String::new();
    let mut duration = String::new();
    let mut shadow = String::new();
    let mut z_index = String::new();

    let tokens = generate_all_tokens();
    for (key, value) in tokens.iter() {
        let line = format!("  {}: number;\n", key.replace(".", "_"));
        if key.starts_with("spacing.") {
            spacing.push_str(&line);
        } else if key.starts_with("typography.weight.") {
            typography_weight.push_str(&line);
        } else if key.starts_with("typography.size.") {
            typography_size.push_str(&line);
        } else if key.starts_with("border_radius.") {
            border_radius.push_str(&line);
        } else if key.starts_with("breakpoint.") {
            breakpoint.push_str(&line);
        } else if key.starts_with("duration.") {
            duration.push_str(&line);
        } else if key.starts_with("shadow.") {
            shadow.push_str(&line);
        } else if key.starts_with("z_index.") {
            z_index.push_str(&line);
        }
        // Ignore value - only interface shape needed
        let _ = value;
    }

    if !spacing.is_empty() {
        ts.push_str("  // Spacing tokens (px)\n");
        ts.push_str(&spacing);
    }
    if !typography_weight.is_empty() {
        ts.push_str("\n  // Typography weights\n");
        ts.push_str(&typography_weight);
    }
    if !typography_size.is_empty() {
        ts.push_str("\n  // Typography sizes (px)\n");
        ts.push_str(&typography_size);
    }
    if !border_radius.is_empty() {
        ts.push_str("\n  // Border radius (px)\n");
        ts.push_str(&border_radius);
    }
    if !breakpoint.is_empty() {
        ts.push_str("\n  // Breakpoints (px)\n");
        ts.push_str(&breakpoint);
    }
    if !duration.is_empty() {
        ts.push_str("\n  // Animation durations (ms)\n");
        ts.push_str(&duration);
    }
    if !shadow.is_empty() {
        ts.push_str("\n  // Shadow depths (px)\n");
        ts.push_str(&shadow);
    }
    if !z_index.is_empty() {
        ts.push_str("\n  // Z-index values\n");
        ts.push_str(&z_index);
    }

    ts.push_str("}\n\n");
    ts.push_str("// Usage: import { AetherisDesignTokens } from './design-tokens';\n");
    ts.push_str("// const tokens: AetherisDesignTokens = {/* ... */};\n");

    ts
}

/// Export design system to Dart class definition
pub fn export_to_dart() -> String {
    let mut dart = String::new();
    dart.push_str("// Auto-generated from Aetheris Rust design system\n");
    dart.push_str("class AetherisDesignTokens {\n");

    let tokens = generate_all_tokens();
    for (key, value) in tokens.iter() {
        let dart_key = key.replace(".", "_");
        let value_str = match value {
            serde_json::Value::Number(n) => n.as_u64().unwrap_or(0).to_string(),
            serde_json::Value::String(s) => format!("\"{}\"", s),
            serde_json::Value::Bool(b) => b.to_string(),
            _ => "0".to_string(),
        };
        dart.push_str(&format!(
            "  static const int {} = {};\n",
            dart_key, value_str
        ));
    }

    dart.push_str("}\n\n");
    dart.push_str("// Usage: final int spacing_base = AetherisDesignTokens.spacing_base;\n");

    dart
}

/// Validate design tokens match design-system.md specification
pub fn validate_against_docs() -> bool {
    // Check that all required tokens from design-system.md are present
    let required_tokens = [
        "spacing.base",
        "spacing.small",
        "spacing.medium",
        "spacing.large",
        "spacing.xlarge",
        "typography.weight.regular400",
        "typography.weight.medium500",
        "typography.weight.semibold600",
        "typography.weight.bold700",
        "typography.size.body16",
        "typography.size.subtitle18",
        "typography.size.title20",
        "border_radius.slight",
        "border_radius.rounded",
        "breakpoint.mobileportrait",
        "breakpoint.tabletportrait",
        "breakpoint.desktop",
        "duration.normal",
        "duration.extended",
        "shadow.subtle",
        "shadow.raised",
        "z_index.content",
        "z_index.modal",
        "z_index.popover",
    ];

    let tokens = generate_all_tokens();
    for token in required_tokens.iter() {
        if !tokens.contains_key(token) {
            return false;
        }
    }
    true
}

/// Get all registered token names for introspection
pub fn all_token_names() -> Vec<String> {
    let mut names = Vec::new();
    let tokens = generate_all_tokens();
    for key in tokens.keys() {
        names.push(key.clone());
    }
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_values() {
        assert_eq!(Spacing::Base.as_px(), 8);
        assert_eq!(Spacing::Small.as_px(), 12);
        assert_eq!(Spacing::Medium.as_px(), 16);
        assert_eq!(Spacing::Large.as_px(), 24);
    }

    #[test]
    fn test_typography() {
        assert_eq!(Typography::Regular400.weight(), 400);
        assert_eq!(Typography::Bold700.weight(), 700);
        assert_eq!(Typography::Body16.size_px(), 16);
        assert_eq!(Typography::Hero48.size_px(), 48);
    }

    #[test]
    fn test_breakpoints() {
        assert_eq!(Breakpoint::MobilePortrait.width_px(), 320);
        assert_eq!(Breakpoint::TabletPortrait.width_px(), 768);
        assert_eq!(Breakpoint::Desktop.width_px(), 1280);
    }

    #[test]
    fn test_durations() {
        assert_eq!(Duration::Normal.as_ms(), 100);
        assert_eq!(Duration::Extended.as_ms(), 500);
    }

    #[test]
    fn test_shadows() {
        assert_eq!(Shadow::Subtle.depth_px(), 1);
        assert_eq!(Shadow::Raised.depth_px(), 4);
    }

    #[test]
    fn test_z_index() {
        assert_eq!(ZIndex::Content.value(), 0);
        assert_eq!(ZIndex::Modal.value(), 100);
        assert_eq!(ZIndex::Popover.value(), 200);
    }

    #[test]
    fn test_export_json() {
        let json = export_to_json();
        assert!(!json.is_empty());
        assert!(json.contains("spacing.Base"));
        assert!(json.contains("typography.weight.Regular400"));
    }

    #[test]
    fn test_export_typescript() {
        let ts = export_to_typescript();
        assert!(!ts.is_empty());
        assert!(ts.contains("interface AetherisDesignTokens"));
        assert!(ts.contains("spacing_Base: number;"));
    }

    #[test]
    fn test_export_dart() {
        let dart = export_to_dart();
        assert!(!dart.is_empty());
        assert!(dart.contains("class AetherisDesignTokens"));
        assert!(dart.contains("spacing_Base = 8;"));
    }

    #[test]
    fn test_validation() {
        assert!(validate_against_docs());
    }

    #[test]
    fn test_all_token_names() {
        let names = all_token_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"spacing.Base".to_string()));
        assert!(names.contains(&"typography.weight.Regular400".to_string()));
    }
}
/// Export design tokens to CSS-in-JS format (styled-components / emotion)
pub fn export_to_css_in_js() -> String {
    let mut css = String::new();
    css.push_str("// Auto-generated CSS-in-JS design tokens\n");
    css.push_str("export const aetherisDesignTokens = {\n");
    css.push_str("  colors: {\n");
    css.push_str("    primary: '#6366F1',\n");
    css.push_str("    secondary: '#A855F7',\n");
    css.push_str("    accent: '#06B6D4',\n");
    css.push_str("    success: '#10B981',\n");
    css.push_str("    error: '#EF4444',\n");
    css.push_str("    warning: '#F59E0B',\n");
    css.push_str("    background: '#090D16',\n");
    css.push_str("    surface: '#1E293B',\n");
    css.push_str("    textPrimary: '#F8FAFC',\n");
    css.push_str("    textSecondary: '#CBD5E1',\n");
    css.push_str("    textDisabled: '#64748B',\n");
    css.push_str("    cyberGlow: '#6366F1',\n");
    css.push_str("    heraldicGold: '#F59E0B',\n");
    css.push_str("    midnightBlue: '#090D16',\n");
    css.push_str("  },\n");
    css.push_str("  spacing: {\n");
    css.push_str("    base: '8px',\n");
    css.push_str("    small: '12px',\n");
    css.push_str("    medium: '16px',\n");
    css.push_str("    large: '24px',\n");
    css.push_str("    xlarge: '32px',\n");
    css.push_str("    xxlarge: '48px',\n");
    css.push_str("  },\n");
    css.push_str("  typography: {\n");
    css.push_str("    fontFamily: \"'Plus Jakarta Sans', 'Inter', sans-serif\",\n");
    css.push_str("    fontFamilyMono: \"'JetBrains Mono', monospace\",\n");
    css.push_str(
        "    weights: { regular: 400, medium: 500, semibold: 600, bold: 700, extraBold: 800 },\n",
    );
    css.push_str("  },\n");
    css.push_str("  borderRadius: {\n");
    css.push_str("    sharp: '0px',\n");
    css.push_str("    slight: '4px',\n");
    css.push_str("    rounded: '8px',\n");
    css.push_str("    pill: '16px',\n");
    css.push_str("  },\n");
    css.push_str("  shadow: {\n");
    css.push_str("    none: 'none',\n");
    css.push_str("    subtle: '0 1px 2px rgba(0,0,0,0.05)',\n");
    css.push_str("    medium: '0 2px 4px rgba(0,0,0,0.1)',\n");
    css.push_str("    raised: '0 4px 8px rgba(0,0,0,0.15)',\n");
    css.push_str("    high: '0 8px 16px rgba(0,0,0,0.2)',\n");
    css.push_str("  },\n");
    css.push_str("  breakpoints: {\n");
    css.push_str("    mobile: '320px',\n");
    css.push_str("    tablet: '768px',\n");
    css.push_str("    desktop: '1280px',\n");
    css.push_str("  },\n");
    css.push_str("  zIndex: {\n");
    css.push_str("    background: -1,\n");
    css.push_str("    content: 0,\n");
    css.push_str("    modal: 100,\n");
    css.push_str("    popover: 200,\n");
    css.push_str("    tooltip: 300,\n");
    css.push_str("  },\n");
    css.push_str("};\n");
    css.push_str("export default aetherisDesignTokens;\n");
    css
}
