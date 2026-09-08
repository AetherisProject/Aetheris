//! Theme switching for CLI TUI using Aetheris design system.

use ratatui::style::{Color, Modifier, Style};
use crate::cli::ui_components::AetherisColors;

/// Aetheris CLI theme configurations
pub enum Theme {
    /// Default cyber-heraldic dark theme
    Dark,
    /// High-contrast dark theme
    HighContrast,
    /// Light theme (alternate)
    Light,
}

impl Theme {
    /// Get background color for theme
    pub fn bg(&self) -> Color {
        match self {
            Theme::Dark => AetherisColors::BG,
            Theme::HighContrast => Color::Rgb(0, 0, 0),
            Theme::Light => Color::Rgb(248, 250, 252),
        }
    }

    /// Get surface color for theme
    pub fn surface(&self) -> Color {
        match self {
            Theme::Dark => AetherisColors::SURFACE,
            Theme::HighContrast => Color::Rgb(30, 30, 30),
            Theme::Light => Color::Rgb(255, 255, 255),
        }
    }

    /// Get primary color for theme
    pub fn primary(&self) -> Color {
        match self {
            Theme::Dark => AetherisColors::PRIMARY,
            Theme::HighContrast => Color::Rgb(255, 255, 255),
            Theme::Light => AetherisColors::PRIMARY,
        }
    }

    /// Get text primary for theme
    pub fn text_primary(&self) -> Color {
        match self {
            Theme::Dark => AetherisColors::TEXT_PRIMARY,
            Theme::HighContrast => Color::Rgb(255, 255, 255),
            Theme::Light => AetherisColors::BG,
        }
    }

    /// Get text secondary for theme
    pub fn text_secondary(&self) -> Color {
        match self {
            Theme::Dark => AetherisColors::TEXT_SECONDARY,
            Theme::HighContrast => Color::Rgb(200, 200, 200),
            Theme::Light => Color::Rgb(30, 41, 59),
        }
    }

    /// Get base style for theme
    pub fn base_style(&self) -> Style {
        Style::default()
            .fg(self.text_primary())
            .bg(self.bg())
    }

    /// Get card style for theme
    pub fn card_style(&self) -> Style {
        Style::default()
            .fg(self.text_primary())
            .bg(self.surface())
    }
}

/// Apply theme to a widget block
pub fn apply_theme(block: &mut ratatui::widgets::Block, theme: Theme) {
    block.set_style(Style::default()
        .bg(theme.surface())
        .fg(theme.text_primary()));
}
Theme toggle
