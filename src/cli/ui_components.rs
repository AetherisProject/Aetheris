//! CLI TUI component primitives for Aetheris using ratatui.
//! Built from Rust design system (design/tokens.rs, design/components.rs).

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::design::tokens::{Spacing, Typography, BorderRadius, Shadow, Breakpoint, Duration, ZIndex};

/// Aetheris design colors for CLI (dark cyber-heraldic theme)
pub struct AetherisColors;

impl AetherisColors {
    /// Deep void background
    pub const BG: Color = Color::Rgb(9, 13, 22);
    /// Slate surface
    pub const SURFACE: Color = Color::Rgb(30, 41, 59);
    /// Primary brand (indigo)
    pub const PRIMARY: Color = Color::Rgb(99, 102, 241);
    /// Secondary (violet)
    pub const SECONDARY: Color = Color::Rgb(168, 85, 247);
    /// Accent (cyan)
    pub const ACCENT: Color = Color::Rgb(6, 182, 212);
    /// Success (emerald)
    pub const SUCCESS: Color = Color::Rgb(16, 185, 129);
    /// Warning
    pub const WARNING: Color = Color::Rgb(245, 158, 11);
    /// Error
    pub const ERROR: Color = Color::Rgb(239, 68, 68);
    /// Text primary
    pub const TEXT_PRIMARY: Color = Color::Rgb(248, 250, 252);
    /// Text secondary
    pub const TEXT_SECONDARY: Color = Color::Rgb(203, 213, 225);
    /// Text disabled
    pub const TEXT_DISABLED: Color = Color::Rgb(100, 116, 139);
    /// Border
    pub const BORDER: Color = Color::Rgb(51, 65, 85);
}

/// CLI Card widget using design tokens
pub fn render_card(
    f: &mut Frame,
    area: ratatui::prelude::Rect,
    title: &str,
    content: &str,
) {
    let block = Block::default()
        .title(Line::from(format!(" {} ", title)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(AetherisColors::BORDER))
        .style(Style::default().bg(AetherisColors::SURFACE));
    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default().fg(AetherisColors::TEXT_PRIMARY));
    f.render_widget(paragraph, area);
}

/// CLI Button widget using design tokens
pub fn render_button(
    f: &mut Frame,
    area: ratatui::prelude::Rect,
    label: &str,
    is_selected: bool,
) {
    let style = if is_selected {
        Style::default()
            .fg(AetherisColors::BG)
            .bg(AetherisColors::PRIMARY)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(AetherisColors::TEXT_PRIMARY)
            .bg(AetherisColors::SURFACE)
    };
    let block = Block::default().style(style);
    let paragraph = Paragraph::new(label)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

/// CLI Input/Field widget using design tokens
pub fn render_input(
    f: &mut Frame,
    area: ratatui::prelude::Rect,
    label: &str,
    value: &str,
) {
    let block = Block::default()
        .title(Line::from(format!(" {} ", label)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(AetherisColors::BORDER));
    let paragraph = Paragraph::new(value)
        .block(block)
        .style(Style::default().fg(AetherisColors::TEXT_PRIMARY));
    f.render_widget(paragraph, area);
}

/// CLI Modal/Popup widget using design tokens
pub fn render_modal(
    f: &mut Frame,
    area: ratatui::prelude::Rect,
    title: &str,
    content: &str,
) {
    // Clear background area
    f.render_widget(Clear, area);
    let block = Block::default()
        .title(Line::from(format!(" {} ", title)).style(Style::default().fg(AetherisColors::PRIMARY).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(AetherisColors::PRIMARY).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(AetherisColors::SURFACE));
    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default().fg(AetherisColors::TEXT_PRIMARY));
    f.render_widget(paragraph, area);
}

/// CLI Grid layout for dashboard using design tokens
pub fn render_dashboard_grid(f: &mut Frame, area: ratatui::prelude::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    // Vault card (top left)
    render_card(f, chunks[0], "Vault", "12 items | 3 secure categories");
    // SSH card (bottom left) 
    render_card(f, chunks[1], "SSH", "3 active sessions | 1 tunnel");
}
