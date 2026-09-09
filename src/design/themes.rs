#! Theme management for Aetheris.

use serde::{Serialize, Deserialize};

// Define theme preferences
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemePreferences {
    dark_mode: bool,
    // Add other theme preferences as needed
}

// Default theme preferences
pub const DEFAULT_THEME: ThemePreferences = ThemePreferences {
    dark_mode: false,
};