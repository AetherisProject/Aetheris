//! Responsive layout system for CLI TUI using Aetheris design tokens.

use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use crate::design::tokens::{Breakpoint, Device};

/// Responsive layout for CLI based on viewport size
pub struct ResponsiveLayout {
    /// Current device type (derived from terminal width)
    pub device: Device,
    /// Terminal width
    pub width: u16,
    /// Terminal height
    pub height: u16,
}

impl ResponsiveLayout {
    /// Create new responsive layout from terminal size
    pub fn new(width: u16, height: u16) -> Self {
        let device = if width < 80 {
            Device::Mobile
        } else if width < 120 {
            Device::Tablet
        } else {
            Device::Desktop
        };
        Self { device, width, height }
    }

    /// Get layout direction based on device
    pub fn direction(&self) -> Direction {
        match self.device {
            Device::Mobile => Direction::Vertical,
            Device::Desktop | Device::Tablet => Direction::Horizontal,
            _ => Direction::Vertical,
        }
    }

    /// Get main layout split
    pub fn main_layout(&self) -> Vec<Rect> {
        let main = Layout::default()
            .direction(self.direction())
            .constraints(match self.device {
                Device::Mobile => vec![Constraint::Percentage(100)],
                Device::Tablet => vec![Constraint::Percentage(50), Constraint::Percentage(50)],
                Device::Desktop => vec![Constraint::Percentage(30), Constraint::Percentage(70)],
                _ => vec![Constraint::Length(self.width), Constraint::Length(self.height)],
            })
            .split(Rect::new(0, 0, self.width, self.height));
        main
    }

    /// Get grid columns for dashboard
    pub fn dashboard_grid(&self) -> Vec<Rect> {
        let area = Rect::new(0, 0, self.width, self.height);
        let cols = match self.device {
            Device::Mobile => 1,
            Device::Tablet => 2,
            Device::Desktop => 3,
            _ => 3,
        };
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(area.height / 3)])
            .split(area)
    }
}
