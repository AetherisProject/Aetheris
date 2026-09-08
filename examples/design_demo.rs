//! Design system demo for Aetheris.
//! Demonstrates responsive design tokens, themes, and components for desktop, tablet, and mobile.

use aetheris::design::{
    BorderRadius, Breakpoint, DesignSystem, Device, Icon, IconSize, Layout, ResponsiveDesign,
    Shadow, Spacing, Theme, Typography, ZIndex,
};

fn main() {
    println!("=== Aetheris Design System Demo ===\n");

    // Theme demonstration
    println!("--- Themes ---");
    demo_themes();

    // Responsive design demonstration
    println!("\n--- Responsive Design ---");
    demo_responsive_design();

    // Design tokens demonstration
    println!("\n--- Design Tokens ---");
    demo_design_tokens();

    // Icons demonstration
    println!("\n--- Icons ---");
    demo_icons();

    // Component configurations
    println!("\n--- Component Configurations ---");
    demo_components();

    // Design system API demonstration
    demo_design_system_api();
}

fn demo_themes() {
    let dark_theme = Theme::dark_cyber();
    println!("Dark Cyber Theme:");
    println!("  Primary: {}", dark_theme.palette.primary);
    println!("  Background: {}", dark_theme.palette.background);
    println!("  Text: {}", dark_theme.palette.text_primary);
    println!("  Cyber Glow: {}", dark_theme.palette.cyber_glow);

    let light_theme = Theme::light_cyber();
    println!("\nLight Cyber Theme:");
    println!("  Primary: {}", light_theme.palette.primary);
    println!("  Background: {}", light_theme.palette.background);
    println!("  Text: {}", light_theme.palette.text_primary);

    let high_contrast = Theme::high_contrast();
    println!("\nHigh Contrast Theme:");
    println!("  Primary: {}", high_contrast.palette.primary);
    println!("  Background: {}", high_contrast.palette.background);
    println!("  Text: {}", high_contrast.palette.text_primary);
}

fn demo_responsive_design() {
    // Mobile design
    let mobile = ResponsiveDesign::mobile();
    println!("Mobile Design (< 768px):");
    println!("  Layout: {:?}", mobile.layout);
    println!("  Navbar Position: {:?}", mobile.navbar.position);
    println!("  Navbar Height: {}px", mobile.navbar.height);
    println!("  Grid Columns: {}", mobile.grid.columns);
    println!("  Card Padding: {}px", mobile.card.padding);

    // Tablet design
    let tablet = ResponsiveDesign::tablet();
    println!("\nTablet Design (768px - 992px):");
    println!("  Layout: {:?}", tablet.layout);
    println!("  Navbar Position: {:?}", tablet.navbar.position);
    println!("  Navbar Height: {}px", tablet.navbar.height);
    println!("  Grid Columns: {}", tablet.grid.columns);
    println!("  Card Padding: {}px", tablet.card.padding);

    // Desktop design
    let desktop = ResponsiveDesign::desktop();
    println!("\nDesktop Design (>= 992px):");
    println!("  Layout: {:?}", desktop.layout);
    println!("  Navbar Position: {:?}", desktop.navbar.position);
    println!("  Navbar Height: {}px", desktop.navbar.height);
    println!("  Grid Columns: {}", desktop.grid.columns);
    println!("  Card Padding: {}px", desktop.card.padding);

    // Width-based responsive design
    let width_600 = ResponsiveDesign::for_width(600);
    println!("\nWidth 600px -> {:?}", width_600.device);
    let width_800 = ResponsiveDesign::for_width(800);
    println!("Width 800px -> {:?}", width_800.device);
    let width_1200 = ResponsiveDesign::for_width(1200);
    println!("Width 1200px -> {:?}", width_1200.device);
}

fn demo_design_tokens() {
    println!("Spacing Scale:");
    println!("  Xxs: {}px", Spacing::Xxs.as_px());
    println!("  Xs: {}px", Spacing::Xs.as_px());
    println!("  Sm: {}px", Spacing::Sm.as_px());
    println!("  Md: {}px", Spacing::Md.as_px());
    println!("  Lg: {}px", Spacing::Lg.as_px());
    println!("  Xl: {}px", Spacing::Xl.as_px());
    println!("  Xxl: {}px", Spacing::Xxl.as_px());

    println!("\nTypography Scale:");
    println!(
        "  Display XL: {}px (line-height: {:.1})",
        Typography::DisplayXl.font_size(),
        Typography::DisplayXl.line_height()
    );
    println!(
        "  H1: {}px (line-height: {:.1})",
        Typography::H1.font_size(),
        Typography::H1.line_height()
    );
    println!(
        "  H2: {}px (line-height: {:.1})",
        Typography::H2.font_size(),
        Typography::H2.line_height()
    );
    println!(
        "  Body: {}px (line-height: {:.1})",
        Typography::Body.font_size(),
        Typography::Body.line_height()
    );

    println!("\nBorder Radius:");
    println!("  Sm: {}px", BorderRadius::Sm.as_px());
    println!("  Md: {}px", BorderRadius::Md.as_px());
    println!("  Lg: {}px", BorderRadius::Lg.as_px());
    println!("  Xl: {}px", BorderRadius::Xl.as_px());

    println!("\nBreakpoints:");
    println!("  Xs: {}px", Breakpoint::Xs.as_px());
    println!("  Sm: {}px", Breakpoint::Sm.as_px());
    println!("  Md: {}px", Breakpoint::Md.as_px());
    println!("  Lg: {}px", Breakpoint::Lg.as_px());
    println!("  Xl: {}px", Breakpoint::Xl.as_px());
    println!("  Xxl: {}px", Breakpoint::Xxl.as_px());

    println!("\nZ-Index Scale:");
    println!("  Dropdown: {}", ZIndex::Dropdown.as_i32());
    println!("  Modal: {}", ZIndex::Modal.as_i32());
    println!("  Tooltip: {}", ZIndex::Tooltip.as_i32());
}

fn demo_icons() {
    println!("Icon Sizes:");
    println!("  Xs: {}px", IconSize::Xs.as_px());
    println!("  Sm: {}px", IconSize::Sm.as_px());
    println!("  Md: {}px", IconSize::Md.as_px());
    println!("  Lg: {}px", IconSize::Lg.as_px());
    println!("  Xl: {}px", IconSize::Xl.as_px());

    println!("\nCore Icons:");
    println!(
        "  Shield: {} (path: {})",
        Icon::Shield.name(),
        Icon::Shield.svg_path()
    );
    println!(
        "  Key: {} (path: {})",
        Icon::Key.name(),
        Icon::Key.svg_path()
    );
    println!(
        "  Lock: {} (path: {})",
        Icon::Lock.name(),
        Icon::Lock.svg_path()
    );
    println!(
        "  Vault: {} (path: {})",
        Icon::Vault.name(),
        Icon::Vault.svg_path()
    );

    println!("\nSSH Icons:");
    println!(
        "  Terminal: {} (path: {})",
        Icon::Terminal.name(),
        Icon::Terminal.svg_path()
    );
    println!(
        "  Server: {} (path: {})",
        Icon::Server.name(),
        Icon::Server.svg_path()
    );
    println!(
        "  Network: {} (path: {})",
        Icon::Network.name(),
        Icon::Network.svg_path()
    );

    println!("\nAPI Key Icons:");
    println!(
        "  Api: {} (path: {})",
        Icon::Api.name(),
        Icon::Api.svg_path()
    );
    println!(
        "  Cloud: {} (path: {})",
        Icon::Cloud.name(),
        Icon::Cloud.svg_path()
    );
    println!(
        "  Sync: {} (path: {})",
        Icon::Sync.name(),
        Icon::Sync.svg_path()
    );
}

fn demo_components() {
    let mobile = ResponsiveDesign::mobile();
    println!("Mobile Components:");
    println!(
        "  Button: {}x{} padding, {}px radius, {}px font",
        mobile.button.padding_x,
        mobile.button.padding_y,
        mobile.button.border_radius,
        mobile.button.font_size
    );
    println!(
        "  Input: {}x{} padding, {}px radius, {}px height",
        mobile.input.padding_x,
        mobile.input.padding_y,
        mobile.input.border_radius,
        mobile.input.height
    );
    println!(
        "  Modal: max-width {}px, padding {}px",
        mobile.modal.max_width, mobile.modal.padding
    );

    let desktop = ResponsiveDesign::desktop();
    println!("\nDesktop Components:");
    println!(
        "  Button: {}x{} padding, {}px radius, {}px font",
        desktop.button.padding_x,
        desktop.button.padding_y,
        desktop.button.border_radius,
        desktop.button.font_size
    );
    println!(
        "  Input: {}x{} padding, {}px radius, {}px height",
        desktop.input.padding_x,
        desktop.input.padding_y,
        desktop.input.border_radius,
        desktop.input.height
    );
    println!(
        "  Modal: max-width {}px, padding {}px",
        desktop.modal.max_width, desktop.modal.padding
    );
}

fn demo_design_system_api() {
    println!("\n--- Design System API ---");

    // Create design system with defaults
    let mut ds = DesignSystem::new();
    println!("Default Design System:");
    println!("  Theme: {}", ds.theme.name);
    println!("  Device: {:?}", ds.responsive.device);

    // Change theme
    ds.load_theme("light");
    println!("\nAfter loading light theme:");
    println!("  Theme: {}", ds.theme.name);

    // Change device
    ds.set_device(Device::Tablet);
    println!("\nAfter setting to tablet:");
    println!("  Device: {:?}", ds.responsive.device);

    // Update for specific width
    ds.update_for_width(600);
    println!("\nAfter updating for 600px width:");
    println!("  Device: {:?}", ds.responsive.device);

    // Create with specific configuration
    let custom_ds = DesignSystem::with_theme(Theme::high_contrast());
    println!("\nCustom Design System (High Contrast):");
    println!("  Theme: {}", custom_ds.theme.name);
    println!("  Device: {:?}", custom_ds.responsive.device);

    let mobile_ds = DesignSystem::with_device(Device::Mobile);
    println!("\nMobile Design System:");
    println!("  Theme: {}", mobile_ds.theme.name);
    println!("  Device: {:?}", mobile_ds.responsive.device);

    let width_ds = DesignSystem::with_width(1200);
    println!("\nWidth-based Design System (1200px):");
    println!("  Theme: {}", width_ds.theme.name);
    println!("  Device: {:?}", width_ds.responsive.device);
}
