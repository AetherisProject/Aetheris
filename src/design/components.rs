//! Responsive UI components for Aetheris.
use crate::design::tokens::{
    BorderRadius, Breakpoint, Duration, Shadow, Spacing, Typography, ZIndex,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Device {
    Mobile,
    Tablet,
    Desktop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layout {
    Stacked,
    SideBySide,
    Grid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navbar {
    pub title: String,
    pub position: String,
}
pub fn apply_theme() -> () {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_apply_theme() {
        assert!(true);
    }
}
