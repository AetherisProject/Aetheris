//! Responsive UI components for Aetheris.
//! Provides component definitions for desktop, tablet, and mobile layouts.
use crate::design::tokens::{Shadow, Duration};
//! Includes serialization and export capabilities for cross-platform consistency.

use crate::design::tokens::{Breakpoint, BorderRadius, Spacing, Typography, ZIndex};
use serde::{Deserialize, Serialize};

/// Device type for responsive layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Device {
    /// Mobile phones in portrait (<= 768px)
    Mobile,
    /// Mobile phones in landscape (<= 1024px)
    MobileLandscape,
    /// Tablets in portrait (<= 1024px)
    Tablet,
    /// Tablets in landscape (<= 1280px)
    TabletLandscape,
    /// Desktop computers (>= 1280px)
    Desktop,
    /// Wide displays (>= 1920px)
    WideDesktop,
}

impl Device {
    /// Get device type based on viewport width
    pub fn for_width(width: u32) -> Self {
        match width {
            w if w <= 768 => Device::Mobile,
            w if w <= 1024 => {
                if w > 768 {
                    Device::MobileLandscape
                } else {
                    Device::Tablet
                }
            }
            w if w <= 1280 => Device::TabletLandscape,
            w if w <= 1920 => Device::Desktop,
            _ => Device::WideDesktop,
        }
    }

    /// Get device type for specific device form factor
    pub fn for_device(device: Device) -> Self {
        device
    }

    /// Get breakpoints for device type
    pub fn breakpoints(&self) -> Vec<Breakpoint> {
        match self {
            Device::Mobile | Device::MobileLandscape => vec![Breakpoint::MobilePortrait],
            Device::Tablet | Device::TabletLandscape => vec![Breakpoint::TabletPortrait],
            Device::Desktop => vec![Breakpoint::Desktop],
            Device::WideDesktop => vec![Breakpoint::WideDesktop],
        }
    }

    /// Get responsive breakpoints for layout
    pub fn responsive_breakpoints(&self) -> Vec<f32> {
        match self {
            Device::Mobile => vec![0.0, 375.0, 414.0, 768.0],
            Device::MobileLandscape => vec![0.0, 568.0, 667.0, 1024.0],
            Device::Tablet => vec![0.0, 768.0, 1024.0],
            Device::TabletLandscape => vec![0.0, 1024.0, 1366.0],
            Device::Desktop => vec![0.0, 1280.0, 1440.0, 1920.0],
            Device::WideDesktop => vec![0.0, 1920.0, 2560.0],
        }
    }
}

/// Layout type for responsive containers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layout {
    /// Horizontal flex layout
    Horizontal,
    /// Vertical flex layout
    Vertical,
    /// Grid layout
    Grid,
    /// Flexbox with wrap
    FlexWrap,
    /// CSS grid with explicit columns
    CSSGrid,
}

impl Layout {
    /// Convert to CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            Layout::Horizontal => "flex-row",
            Layout::Vertical => "flex-column",
            Layout::Grid => "grid",
            Layout::FlexWrap => "flex-wrap",
            Layout::CSSGrid => "css-grid",
        }
    }
}

/// Navigation bar configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navbar {
    /// Whether the navbar is fixed to viewport
    pub fixed: bool,
    /// Position of the navbar
    pub position: NavbarPosition,
    /// Height of the navbar
    pub height: Spacing,
    /// Background color (hex or CSS variable)
    pub background: String,
    /// Text color (hex or CSS variable)
    pub text_color: String,
    /// Border styling
    pub border: bool,
    /// Shadow level
    pub shadow: Shadow,
    /// Logo configuration
    pub logo: LogoConfig,
    /// Navigation items
    pub items: Vec<NavItem>,
    /// Navigation actions (search, user menu, etc.)
    pub actions: Vec<NavAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavbarPosition {
    /// Top of viewport
    Top,
    /// Left side of viewport
    Left,
    /// Bottom of viewport
    Bottom,
    /// Right side of viewport
    Right,
}

impl NavbarPosition {
    /// Convert to CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            NavbarPosition::Top => "navbar-top",
            NavbarPosition::Left => "navbar-left",
            NavbarPosition::Bottom => "navbar-bottom",
            NavbarPosition::Right => "navbar-right",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoConfig {
    /// Logo URL or SVG content
    pub src: String,
    /// Alt text
    pub alt: String,
    /// Width in spacing units
    pub width: Spacing,
    /// Height in spacing units
    pub height: Spacing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavItem {
    /// Unique identifier
    pub id: String,
    /// Display label
    pub label: String,
    /// Icon name
    pub icon: Option<String>,
    /// Link URL
    pub href: String,
    /// Whether item is active
    pub active: bool,
    /// Sub-items (for dropdowns)
    pub children: Vec<NavItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavAction {
    /// Unique identifier
    pub id: String,
    /// Icon name
    pub icon: String,
    /// Action callback or URL
    pub action: NavActionType,
    /// Badge text (optional)
    pub badge: Option<String>,
    /// Tooltip text
    pub tooltip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavActionType {
    /// Link navigation
    Link(String),
    /// Button click
    Button(String),
    /// Toggle state
    Toggle(String),
    /// External link
    ExternalLink(String),
    /// Separator
    Separator,
}

impl Navbar {
    /// Create default navbar configuration
    pub fn default() -> Self {
        Self {
            fixed: true,
            position: NavbarPosition::Top,
            height: Spacing::Medium,
            background: "#1e293b".to_string(),
            text_color: "#f8fafc".to_string(),
            border: true,
            shadow: Shadow::Subtle,
            logo: LogoConfig {
                src: "assets/icons/logo.svg".to_string(),
                alt: "Aetheris".to_string(),
                width: Spacing::XLarge,
                height: Spacing::Large,
            },
            items: vec![],
            actions: vec![],
        }
    }

    /// Get CSS classes for navbar
    pub fn css_classes(&self) -> Vec<String> {
        let mut classes = vec![self.position.css_class()];
        if self.fixed {
            classes.push("fixed".to_string());
        }
        if self.border {
            classes.push("bordered".to_string());
        }
        if !self.shadow.is_none() {
            classes.push(format!("shadow-{}", self.shadow.depth_px()));
        }
        classes
    }
}

/// Card component configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Card background color
    pub background: String,
    /// Card border color
    pub border_color: String,
    /// Card border radius
    pub border_radius: BorderRadius,
    /// Card shadow
    pub shadow: Shadow,
    /// Card padding
    pub padding: Spacing,
    /// Card margin
    pub margin: Spacing,
    /// Whether card has hover effect
    pub hoverable: bool,
    /// Whether card is clickable
    pub clickable: bool,
    /// Card header configuration
    pub header: Option<CardHeader>,
    /// Card body content
    pub body: CardBody,
    /// Card footer configuration
    pub footer: Option<CardFooter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardHeader {
    /// Header title
    pub title: String,
    /// Header subtitle
    pub subtitle: Option<String>,
    /// Header actions (icons, buttons)
    pub actions: Vec<HeaderAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderAction {
    /// Action icon
    pub icon: String,
    /// Action label
    pub label: String,
    /// Action type
    pub action: HeaderActionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeaderActionType {
    /// Copy to clipboard
    Copy,
    /// Edit mode
    Edit,
    /// Delete
    Delete,
    /// External link
    External,
    /// Custom action
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardBody {
    /// Body content type
    pub content_type: ContentType,
    /// Text content (for text-based content)
    pub text: Option<String>,
    /// Image content (for image-based content)
    pub image: Option<ImageConfig>,
    /// Chart content (for data visualization)
    pub chart: Option<ChartConfig>,
    /// Custom content (for arbitrary content)
    pub custom: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    /// Text content
    Text,
    /// Image content
    Image,
    /// Chart content
    Chart,
    /// Empty content
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    /// Image URL
    pub src: String,
    /// Image alt text
    pub alt: String,
    /// Image fit behavior
    pub fit: ImageFit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFit {
    /// Cover the container
    Cover,
    /// Contain within container
    Contain,
    /// Fill the container
    Fill,
    /// Scale down if needed
    ScaleDown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    /// Chart type
    pub chart_type: ChartType,
    /// Chart data
    pub data: Vec<ChartDataPoint>,
    /// Chart colors
    pub colors: Vec<String>,
    /// Chart title
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    /// Line chart
    Line,
    /// Bar chart
    Bar,
    /// Pie chart
    Pie,
    /// Area chart
    Area,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartDataPoint {
    /// Data point label
    pub label: String,
    /// Data point value
    pub value: f64,
    /// Data point color (optional)
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardFooter {
    /// Footer actions
    pub actions: Vec<FooterAction>,
    /// Footer text
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterAction {
    /// Action label
    pub label: String,
    /// Action type
    pub action: FooterActionType,
    /// Action style
    pub style: ButtonStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FooterActionType {
    /// Primary action
    Primary,
    /// Secondary action
    Secondary,
    /// Destructive action
    Destructive,
    /// Outline action
    Outline,
}

impl Card {
    /// Create default card configuration
    pub fn default() -> Self {
        Self {
            background: "#1e293b".to_string(),
            border_color: "#334155".to_string(),
            border_radius: BorderRadius::Slight,
            shadow: Shadow::Subtle,
            padding: Spacing::Medium,
            margin: Spacing::Medium,
            hoverable: true,
            clickable: false,
            header: None,
            body: CardBody {
                content_type: ContentType::Empty,
                text: None,
                image: None,
                chart: None,
                custom: None,
            },
            footer: None,
        }
    }

    /// Get CSS classes for card
    pub fn css_classes(&self) -> Vec<String> {
        let mut classes = vec!["card".to_string()];
        if self.hoverable {
            classes.push("hoverable".to_string());
        }
        if self.clickable {
            classes.push("clickable".to_string());
        }
        classes.push(format!("radius-{}", self.border_radius as u32));
        classes.push(format!("shadow-{}", self.shadow.depth_px()));
        classes
    }
}

/// Button component configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    /// Button size
    pub size: ButtonSize,
    /// Button style
    pub style: ButtonStyle,
    /// Button type
    pub button_type: ButtonType,
    /// Button text
    pub text: String,
    /// Button icon (optional)
    pub icon: Option<String>,
    /// Whether button is disabled
    pub disabled: bool,
    /// Whether button is loading
    pub loading: bool,
    /// Button click handler
    pub on_click: Option<String>,
    /// Button tooltip
    pub tooltip: Option<String>,
    /// Button width
    pub width: Option<Spacing>,
    /// Button height
    pub height: Option<Spacing>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonSize {
    /// Extra small
    Xs,
    /// Small
    Sm,
    /// Medium
    Md,
    /// Large
    Lg,
    /// Extra large
    Xl,
}

impl ButtonSize {
    /// Get CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "btn-xs",
            ButtonSize::Sm => "btn-sm",
            ButtonSize::Md => "btn-md",
            ButtonSize::Lg => "btn-lg",
            ButtonSize::Xl => "btn-xl",
        }
    }

    /// Get padding
    pub fn padding(&self) -> Spacing {
        match self {
            ButtonSize::Xs => Spacing::Small,
            ButtonSize::Sm => Spacing::Small,
            ButtonSize::Md => Spacing::Medium,
            ButtonSize::Lg => Spacing::Large,
            ButtonSize::Xl => Spacing::XLarge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonStyle {
    /// Primary style (solid background)
    Primary,
    /// Secondary style (outline)
    Secondary,
    /// Success style (green)
    Success,
    /// Danger style (red)
    Danger,
    /// Warning style (yellow/orange)
    Warning,
    /// Info style (blue)
    Info,
    /// Outline style (transparent background)
    Outline,
    /// Ghost style (no background or border)
    Ghost,
    /// Link style (styled as text link)
    Link,
}

impl ButtonStyle {
    /// Get CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            ButtonStyle::Primary => "btn-primary",
            ButtonStyle::Secondary => "btn-secondary",
            ButtonStyle::Success => "btn-success",
            ButtonStyle::Danger => "btn-danger",
            ButtonStyle::Warning => "btn-warning",
            ButtonStyle::Info => "btn-info",
            ButtonStyle::Outline => "btn-outline",
            ButtonStyle::Ghost => "btn-ghost",
            ButtonStyle::Link => "btn-link",
        }
    }

    /// Get text color
    pub fn text_color(&self) -> &'static str {
        match self {
            ButtonStyle::Primary | ButtonStyle::Success | ButtonStyle::Danger | ButtonStyle::Info => "#ffffff",
            ButtonStyle::Secondary | ButtonStyle::Warning => "#0f172a",
            ButtonStyle::Outline | ButtonStyle::Ghost => "#06b6d4",
            ButtonStyle::Link => "#06b6d4",
        }
    }

    /// Get background color
    pub fn background_color(&self) -> &'static str {
        match self {
            ButtonStyle::Primary => "#06b6d4",
            ButtonStyle::Secondary => "#334155",
            ButtonStyle::Success => "#10b981",
            ButtonStyle::Danger => "#ef4444",
            ButtonStyle::Warning => "#f59e0b",
            ButtonStyle::Info => "#3b82f6",
            ButtonStyle::Outline => "transparent",
            ButtonStyle::Ghost => "transparent",
            ButtonStyle::Link => "transparent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonType {
    /// Button for form submission
    Submit,
    /// Button for reset
    Reset,
    /// Regular button
    Button,
}

impl Button {
    /// Create default button configuration
    pub fn default() -> Self {
        Self {
            size: ButtonSize::Md,
            style: ButtonStyle::Primary,
            button_type: ButtonType::Button,
            text: "Click".to_string(),
            icon: None,
            disabled: false,
            loading: false,
            on_click: None,
            tooltip: None,
            width: None,
            height: None,
        }
    }

    /// Get CSS classes for button
    pub fn css_classes(&self) -> Vec<String> {
        let mut classes = vec!["btn".to_string()];
        classes.push(self.size.css_class().to_string());
        classes.push(self.style.css_class().to_string());
        if self.disabled {
            classes.push("disabled".to_string());
        }
        if self.loading {
            classes.push("loading".to_string());
        }
        classes
    }
}

/// Input field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// Input type
    pub input_type: InputType,
    /// Input placeholder
    pub placeholder: Option<String>,
    /// Input value
    pub value: Option<String>,
    /// Input label
    pub label: Option<String>,
    /// Whether input is required
    pub required: bool,
    /// Whether input is disabled
    pub disabled: bool,
    /// Whether input is read-only
    pub readonly: bool,
    /// Input validation rules
    pub validation: ValidationRules,
    /// Input error message
    pub error_message: Option<String>,
    /// Input help text
    pub help_text: Option<String>,
    /// Input icon (optional)
    pub icon: Option<String>,
    /// Input size
    pub size: InputSize,
    /// Input style
    pub style: InputStyle,
    /// Input pattern (for regex validation)
    pub pattern: Option<String>,
    /// Input min value (for number inputs)
    pub min: Option<f64>,
    /// Input max value (for number inputs)
    pub max: Option<f64>,
    /// Input step value (for number inputs)
    pub step: Option<f64>,
    /// Input autocomplete
    pub autocomplete: Option<String>,
    /// Input spellcheck
    pub spellcheck: bool,
    /// Input autofocus
    pub autofocus: bool,
    /// Input onchange handler
    pub onchange: Option<String>,
    /// Input oninput handler
    pub oninput: Option<String>,
    /// Input onblur handler
    pub onblur: Option<String>,
    /// Input onfocus handler
    pub onfocus: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    /// Text input
    Text,
    /// Email input
    Email,
    /// Password input
    Password,
    /// Number input
    Number,
    /// Textarea
    Textarea,
    /// Select dropdown
    Select,
    /// Checkbox
    Checkbox,
    /// Radio button
    Radio,
    /// Date input
    Date,
    /// Time input
    Time,
    /// DateTime input
    DatetimeLocal,
    /// File input
    File,
    /// Hidden input
    Hidden,
}

impl InputType {
    /// Get HTML input type
    pub fn html_type(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Textarea => "textarea",
            InputType::Select => "select",
            InputType::Checkbox => "checkbox",
            InputType::Radio => "radio",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DatetimeLocal => "datetime-local",
            InputType::File => "file",
            InputType::Hidden => "hidden",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Required field validation
    pub required: bool,
    /// Minimum length validation
    pub min_length: Option<usize>,
    /// Maximum length validation
    pub max_length: Option<usize>,
    /// Minimum value validation (for numbers)
    pub min_value: Option<f64>,
    /// Maximum value validation (for numbers)
    pub max_value: Option<f64>,
    /// Pattern validation (regex)
    pub pattern: Option<String>,
    /// Custom validation rule
    pub custom: Option<String>,
    /// Validation error message
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputSize {
    /// Extra small
    Xs,
    /// Small
    Sm,
    /// Medium
    Md,
    /// Large
    Lg,
    /// Extra large
    Xl,
}

impl InputSize {
    /// Get CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            InputSize::Xs => "input-xs",
            InputSize::Sm => "input-sm",
            InputSize::Md => "input-md",
            InputSize::Lg => "input-lg",
            InputSize::Xl => "input-xl",
        }
    }

    /// Get padding
    pub fn padding(&self) -> Spacing {
        match self {
            InputSize::Xs => Spacing::Small,
            InputSize::Sm => Spacing::Small,
            InputSize::Md => Spacing::Medium,
            InputSize::Lg => Spacing::Large,
            InputSize::Xl => Spacing::XLarge,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputStyle {
    /// Default style
    Default,
    /// Filled style
    Filled,
    /// Flushed style
    Flushed,
    /// Underlined style
    Underlined,
    /// Rounded style
    Rounded,
    /// Sharp style
    Sharp,
}

impl InputStyle {
    /// Get CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            InputStyle::Default => "input-default",
            InputStyle::Filled => "input-filled",
            InputStyle::Flushed => "input-flushed",
            InputStyle::Underlined => "input-underlined",
            InputStyle::Rounded => "input-rounded",
            InputStyle::Sharp => "input-sharp",
        }
    }
}

impl Input {
    /// Create default input configuration
    pub fn default() -> Self {
        Self {
            input_type: InputType::Text,
            placeholder: None,
            value: None,
            label: None,
            required: false,
            disabled: false,
            readonly: false,
            validation: ValidationRules {
                required: false,
                min_length: None,
                max_length: None,
                min_value: None,
                max_value: None,
                pattern: None,
                custom: None,
                error_message: None,
            },
            error_message: None,
            help_text: None,
            icon: None,
            size: InputSize::Md,
            style: InputStyle::Default,
            pattern: None,
            min: None,
            max: None,
            step: None,
            autocomplete: None,
            spellcheck: false,
            autofocus: false,
            onchange: None,
            oninput: None,
            onblur: None,
            onfocus: None,
        }
    }

    /// Get CSS classes for input
    pub fn css_classes(&self) -> Vec<String> {
        let mut classes = vec!["input".to_string()];
        classes.push(format!("input-type-{}", self.input_type as u8));
        classes.push(self.size.css_class().to_string());
        classes.push(self.style.css_class().to_string());
        if self.disabled {
            classes.push("disabled".to_string());
        }
        if self.readonly {
            classes.push("readonly".to_string());
        }
        if self.validation.required {
            classes.push("required".to_string());
        }
        classes
    }
}

/// Modal/dialog configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modal {
    /// Modal title
    pub title: String,
    /// Modal subtitle
    pub subtitle: Option<String>,
    /// Modal content
    pub content: String,
    /// Modal size
    pub size: ModalSize,
    /// Whether modal is open
    pub open: bool,
    /// Whether modal has overlay
    pub with_overlay: bool,
    /// Whether modal can be closed by clicking outside
    pub close_on_outside_click: bool,
    /// Whether modal can be closed by pressing escape
    pub close_on_escape: bool,
    /// Modal position
    pub position: ModalPosition,
    /// Modal transition
    pub transition: ModalTransition,
    /// Modal backdrop blur
    pub backdrop_blur: bool,
    /// Modal z-index
    pub z_index: ZIndex,
    /// Modal padding
    pub padding: Spacing,
    /// Modal margin
    pub margin: Spacing,
    /// Modal header configuration
    pub header: ModalHeader,
    /// Modal body configuration
    pub body: ModalBody,
    /// Modal footer configuration
    pub footer: ModalFooter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalSize {
    /// Extra small
    Xs,
    /// Small
    Sm,
    /// Medium
    Md,
    /// Large
    Lg,
    /// Extra large
    Xl,
    /// Full screen
    Full,
    /// Content width
    Content,
}

impl ModalSize {
    /// Get CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            ModalSize::Xs => "modal-xs",
            ModalSize::Sm => "modal-sm",
            ModalSize::Md => "modal-md",
            ModalSize::Lg => "modal-lg",
            ModalSize::Xl => "modal-xl",
            ModalSize::Full => "modal-full",
            ModalSize::Content => "modal-content",
        }
    }

    /// Get max width in pixels
    pub fn max_width(&self) -> u32 {
        match self {
            ModalSize::Xs => 300,
            ModalSize::Sm => 500,
            ModalSize::Md => 800,
            ModalSize::Lg => 1000,
            ModalSize::Xl => 1200,
            ModalSize::Full => 0, // 100% width
            ModalSize::Content => 600,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalPosition {
    /// Top of viewport
    Top,
    /// Center of viewport
    Center,
    /// Bottom of viewport
    Bottom,
    /// Right side of viewport
    Right,
    /// Left side of viewport
    Left,
}

impl ModalPosition {
    /// Get CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            ModalPosition::Top => "modal-top",
            ModalPosition::Center => "modal-center",
            ModalPosition::Bottom => "modal-bottom",
            ModalPosition::Right => "modal-right",
            ModalPosition::Left => "modal-left",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalTransition {
    /// Fade in/out
    Fade,
    /// Slide in from top
    SlideTop,
    /// Slide in from bottom
    SlideBottom,
    /// Slide in from left
    SlideLeft,
    /// Slide in from right
    SlideRight,
    /// Scale in/out
    Scale,
    /// Bounce in/out
    Bounce,
}

impl ModalTransition {
    /// Get CSS class name
    pub fn css_class(&self) -> &'static str {
        match self {
            ModalTransition::Fade => "modal-fade",
            ModalTransition::SlideTop => "modal-slide-top",
            ModalTransition::SlideBottom => "modal-slide-bottom",
            ModalTransition::SlideLeft => "modal-slide-left",
            ModalTransition::SlideRight => "modal-slide-right",
            ModalTransition::Scale => "modal-scale",
            ModalTransition::Bounce => "modal-bounce",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalHeader {
    /// Header title
    pub title: String,
    /// Header subtitle
    pub subtitle: Option<String>,
    /// Header actions
    pub actions: Vec<HeaderAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalBody {
    /// Body content
    pub content: String,
    /// Body padding
    pub padding: Spacing,
    /// Body scrollable
    pub scrollable: bool,
    /// Body max height
    pub max_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalFooter {
    /// Footer actions
    pub actions: Vec<FooterAction>,
    /// Footer text
    pub text: Option<String>,
}

impl Modal {
    /// Create default modal configuration
    pub fn default() -> Self {
        Self {
            title: "Modal Title".to_string(),
            subtitle: None,
            content: "Modal content goes here.".to_string(),
            size: ModalSize::Md,
            open: false,
            with_overlay: true,
            close_on_outside_click: true,
            close_on_escape: true,
            position: ModalPosition::Center,
            transition: ModalTransition::Fade,
            backdrop_blur: true,
            z_index: ZIndex::Modal,
            padding: Spacing::Large,
            margin: Spacing::Medium,
            header: ModalHeader {
                title: "Modal Title".to_string(),
                subtitle: None,
                actions: vec![],
            },
            body: ModalBody {
                content: "Modal content goes here.".to_string(),
                padding: Spacing::Large,
                scrollable: true,
                max_height: None,
            },
            footer: ModalFooter {
                actions: vec![],
                text: None,
            },
        }
    }

    /// Get CSS classes for modal
    pub fn css_classes(&self) -> Vec<String> {
        let mut classes = vec!["modal".to_string()];
        classes.push(self.size.css_class().to_string());
        classes.push(self.position.css_class().to_string());
        classes.push(self.transition.css_class().to_string());
        if self.with_overlay {
            classes.push("with-overlay".to_string());
        }
        if self.backdrop_blur {
            classes.push("backdrop-blur".to_string());
        }
        classes.push(format!("z-{}", self.z_index.value()));
        classes
    }
}

/// Grid system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid {
    /// Grid columns
    pub columns: u32,
    /// Grid rows
    pub rows: u32,
    /// Grid gap
    pub gap: Spacing,
    /// Grid template columns
    pub template_columns: Option<String>,
    /// Grid template rows
    pub template_rows: Option<String>,
    /// Grid auto flow
    pub auto_flow: AutoFlow,
    /// Grid justify content
    pub justify_content: JustifyContent,
    /// Grid align content
    pub align_content: AlignContent,
    /// Grid justify items
    pub justify_items: JustifyItems,
    /// Grid align items
    pub align_items: AlignItems,
    /// Grid column gap
    pub column_gap: Spacing,
    /// Grid row gap
    pub row_gap: Spacing,
    /// Whether grid is responsive
    pub responsive: bool,
    /// Breakpoints for responsive grid
    pub breakpoints: Vec<GridBreakpoint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoFlow {
    /// Row auto flow
    Row,
    /// Column auto flow
    Column,
    /// Dense auto flow
    Dense,
    /// Row dense auto flow
    RowDense,
    /// Column dense auto flow
    ColumnDense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustifyContent {
    /// Start justification
    Start,
    /// Center justification
    Center,
    /// End justification
    End,
    /// Space between
    SpaceBetween,
    /// Space around
    SpaceAround,
    /// Space even
    SpaceEven,
    /// Stretch justification
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignContent {
    /// Start alignment
    Start,
    /// Center alignment
    Center,
    /// End alignment
    End,
    /// Space between
    SpaceBetween,
    /// Space around
    SpaceAround,
    /// Space even
    SpaceEven,
    /// Stretch alignment
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustifyItems {
    /// Start justification
    Start,
    /// Center justification
    Center,
    /// End justification
    End,
    /// Stretch justification
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignItems {
    /// Start alignment
    Start,
    /// Center alignment
    Center,
    /// End alignment
    End,
    /// Stretch alignment
    Stretch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridBreakpoint {
    /// Breakpoint width
    pub width: u32,
    /// Columns at this breakpoint
    pub columns: u32,
    /// Gap at this breakpoint
    pub gap: Spacing,
}

impl Grid {
    /// Create default grid configuration
    pub fn default() -> Self {
        Self {
            columns: 12,
            rows: 1,
            gap: Spacing::Medium,
            template_columns: None,
            template_rows: None,
            auto_flow: AutoFlow::Row,
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Start,
            justify_items: JustifyItems::Stretch,
            align_items: AlignItems::Stretch,
            column_gap: Spacing::Medium,
            row_gap: Spacing::Medium,
            responsive: true,
            breakpoints: vec![
                GridBreakpoint {
                    width: 768,
                    columns: 6,
                    gap: Spacing::Small,
                },
                GridBreakpoint {
                    width: 1024,
                    columns: 8,
                    gap: Spacing::Medium,
                },
                GridBreakpoint {
                    width: 1280,
                    columns: 12,
                    gap: Spacing::Large,
                },
            ],
        }
    }

    /// Get CSS classes for grid
    pub fn css_classes(&self) -> Vec<String> {
        let mut classes = vec!["grid".to_string()];
        classes.push(format!("grid-cols-{}", self.columns));
        classes.push(format!("grid-rows-{}", self.rows));
        classes.push(format!("gap-{}", self.gap.as_px()));
        match self.auto_flow {
            AutoFlow::Row => classes.push("grid-flow-row".to_string()),
            AutoFlow::Column => classes.push("grid-flow-column".to_string()),
            AutoFlow::Dense => classes.push("grid-flow-dense".to_string()),
            AutoFlow::RowDense => classes.push("grid-flow-row-dense".to_string()),
            AutoFlow::ColumnDense => classes.push("grid-flow-column-dense".to_string()),
        }
        classes.push(format!("justify-{}", self.justify_content as u8));
        classes.push(format!("align-{}", self.align_content as u8));
        classes.push(format!("justify-items-{}", self.justify_items as u8));
        classes.push(format!("align-items-{}", self.align_items as u8));
        classes
    }
}

/// Typography configuration for responsive text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveTypography {
    /// Desktop typography
    pub desktop: TypographyConfig,
    /// Tablet typography
    pub tablet: TypographyConfig,
    /// Mobile typography
    pub mobile: TypographyConfig,
    /// Typography scale factor
    pub scale_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyConfig {
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: Typography,
    /// Font weight
    pub font_weight: Typography,
    /// Line height
    pub line_height: f32,
    /// Letter spacing
    pub letter_spacing: f32,
    /// Text align
    pub text_align: TextAlign,
    /// Text transform
    pub text_transform: TextTransform,
    /// Text decoration
    pub text_decoration: TextDecoration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlign {
    /// Left align
    Left,
    /// Right align
    Right,
    /// Center align
    Center,
    /// Justify align
    Justify,
    /// Start align
    Start,
    /// End align
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextTransform {
    /// None transform
    None,
    /// Uppercase
    Uppercase,
    /// Lowercase
    Lowercase,
    /// Capitalize
    Capitalize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextDecoration {
    /// None decoration
    None,
    /// Underline
    Underline,
    /// Overline
    Overline,
    /// Line through
    LineThrough,
}

impl ResponsiveTypography {
    /// Create default responsive typography configuration
    pub fn default() -> Self {
        Self {
            desktop: TypographyConfig {
                font_family: "Plus Jakarta Sans, Inter, sans-serif".to_string(),
                font_size: Typography::Body16,
                font_weight: Typography::Regular400,
                line_height: 1.7,
                letter_spacing: 0.0,
                text_align: TextAlign::Left,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
            },
            tablet: TypographyConfig {
                font_family: "Plus Jakarta Sans, Inter, sans-serif".to_string(),
                font_size: Typography::Subtitle18,
                font_weight: Typography::Medium500,
                line_height: 1.6,
                letter_spacing: 0.0,
                text_align: TextAlign::Left,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
            },
            mobile: TypographyConfig {
                font_family: "Plus Jakarta Sans, Inter, sans-serif".to_string(),
                font_size: Typography::Body14,
                font_weight: Typography::Medium500,
                line_height: 1.5,
                letter_spacing: 0.0,
                text_align: TextAlign::Left,
                text_transform: TextTransform::None,
                text_decoration: TextDecoration::None,
            },
            scale_factor: 1.0,
        }
    }
}

/// Complete responsive design system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveDesign {
    /// Device type
    pub device: Device,
    /// Layout type
    pub layout: Layout,
    /// Grid configuration
    pub grid: Grid,
    /// Typography configuration
    pub typography: ResponsiveTypography,
    /// Component base configuration
    pub component_base: ComponentBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentBase {
    /// Border radius
    pub border_radius: BorderRadius,
    /// Shadow
    pub shadow: Shadow,
    /// Z-index
    pub z_index: ZIndex,
    /// Animation duration
    pub animation_duration: Duration,
    /// Spacing scale
    pub spacing: Spacing,
    /// Breakpoint
    pub breakpoint: Breakpoint,
}

impl ResponsiveDesign {
    /// Create default responsive design configuration
    pub fn default() -> Self {
        Self {
            device: Device::Desktop,
            layout: Layout::Vertical,
            grid: Grid::default(),
            typography: ResponsiveTypography::default(),
            component_base: ComponentBase {
                border_radius: BorderRadius::Slight,
                shadow: Shadow::Subtle,
                z_index: ZIndex::Content,
                animation_duration: Duration::Normal,
                spacing: Spacing::Medium,
                breakpoint: Breakpoint::Desktop,
            },
        }
    }

    /// Create responsive design for specific device
    pub fn for_device(device: Device) -> Self {
        let mut design = Self::default();
        design.device = device;
        design.typography = match device {
            Device::Mobile | Device::MobileLandscape => ResponsiveTypography {
                desktop: design.typography.desktop,
                tablet: design.typography.mobile,
                mobile: design.typography.mobile,
                scale_factor: 0.9,
            },
            Device::Tablet | Device::TabletLandscape => ResponsiveTypography {
                desktop: design.typography.desktop,
                tablet: design.typography.tablet,
                mobile: design.typography.mobile,
                scale_factor: 0.95,
            },
            Device::Desktop | Device::WideDesktop => design.typography,
        };
        design.grid = match device {
            Device::Mobile | Device::MobileLandscape => Grid {
                columns: 4,
                rows: 1,
                gap: Spacing::Small,
                template_columns: None,
                template_rows: None,
                auto_flow: AutoFlow::Row,
                justify_content: JustifyContent::Start,
                align_content: AlignContent::Start,
                justify_items: JustifyItems::Stretch,
                align_items: AlignItems::Stretch,
                column_gap: Spacing::Small,
                row_gap: Spacing::Small,
                responsive: false,
                breakpoints: vec![],
            },
            Device::Tablet | Device::TabletLandscape => Grid {
                columns: 8,
                rows: 1,
                gap: Spacing::Medium,
                template_columns: None,
                template_rows: None,
                auto_flow: AutoFlow::Row,
                justify_content: JustifyContent::Start,
                align_content: AlignContent::Start,
                justify_items: JustifyItems::Stretch,
                align_items: AlignItems::Stretch,
                column_gap: Spacing::Medium,
                row_gap: Spacing::Medium,
                responsive: false,
                breakpoints: vec![],
            },
            Device::Desktop | Device::WideDesktop => design.grid,
        };
        design
    }

    /// Create responsive design for specific width
    pub fn for_width(width: u32) -> Self {
        let device = Device::for_width(width);
        Self::for_device(device)
    }

    /// Get CSS classes for responsive design
    pub fn css_classes(&self) -> Vec<String> {
        let mut classes = vec!["responsive-design".to_string()];
        classes.push(format!("device-{}", self.device as u8));
        classes.push(format!("layout-{}", self.layout as u8));
        classes.extend(self.grid.css_classes());
        classes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_for_width() {
        assert_eq!(Device::for_width(320), Device::Mobile);
        assert_eq!(Device::for_width(480), Device::MobileLandscape);
        assert_eq!(Device::for_width(768), Device::Tablet);
        assert_eq!(Device::for_width(1024), Device::TabletLandscape);
        assert_eq!(Device::for_width(1280), Device::Desktop);
        assert_eq!(Device::for_width(1920), Device::WideDesktop);
    }

    #[test]
    fn test_layout_css_class() {
        assert_eq!(Layout::Horizontal.css_class(), "flex-row");
        assert_eq!(Layout::Vertical.css_class(), "flex-column");
        assert_eq!(Layout::Grid.css_class(), "grid");
        assert_eq!(Layout::FlexWrap.css_class(), "flex-wrap");
        assert_eq!(Layout::CSSGrid.css_class(), "css-grid");
    }

    #[test]
    fn test_navbar_default() {
        let navbar = Navbar::default();
        assert!(navbar.fixed);
        assert_eq!(navbar.position, NavbarPosition::Top);
        assert_eq!(navbar.height, Spacing::Medium);
    }

    #[test]
    fn test_card_default() {
        let card = Card::default();
        assert!(card.hoverable);
        assert!(!card.clickable);
        assert_eq!(card.border_radius, BorderRadius::Slight);
        assert_eq!(card.shadow, Shadow::Subtle);
    }

    #[test]
    fn test_button_default() {
        let button = Button::default();
        assert_eq!(button.size, ButtonSize::Md);
        assert_eq!(button.style, ButtonStyle::Primary);
        assert_eq!(button.button_type, ButtonType::Button);
        assert!(button.text == "Click");
    }

    #[test]
    fn test_input_default() {
        let input = Input::default();
        assert_eq!(input.input_type, InputType::Text);
        assert_eq!(input.size, InputSize::Md);
        assert_eq!(input.style, InputStyle::Default);
        assert!(!input.required);
    }

    #[test]
    fn test_modal_default() {
        let modal = Modal::default();
        assert_eq!(modal.size, ModalSize::Md);
        assert!(!modal.open);
        assert!(modal.with_overlay);
        assert!(modal.close_on_outside_click);
        assert!(modal.close_on_escape);
    }

    #[test]
    fn test_grid_default() {
        let grid = Grid::default();
        assert_eq!(grid.columns, 12);
        assert_eq!(grid.rows, 1);
        assert_eq!(grid.gap, Spacing::Medium);
        assert!(grid.responsive);
        assert_eq!(grid.breakpoints.len(), 3);
    }

    #[test]
    fn test_responsive_design_default() {
        let design = ResponsiveDesign::default();
        assert_eq!(design.device, Device::Desktop);
        assert_eq!(design.layout, Layout::Vertical);
        assert_eq!(design.typography.desktop.font_size, Typography::Body16);
    }

    #[test]
    fn test_responsive_design_for_device() {
        let mobile_design = ResponsiveDesign::for_device(Device::Mobile);
        assert_eq!(mobile_design.device, Device::Mobile);
        assert_eq!(mobile_design.typography.mobile.font_size, Typography::Body14);
        assert_eq!(mobile_design.grid.columns, 4);

        let desktop_design = ResponsiveDesign::for_device(Device::Desktop);
        assert_eq!(desktop_design.device, Device::Desktop);
        assert_eq!(desktop_design.typography.desktop.font_size, Typography::Body16);
        assert_eq!(desktop_design.grid.columns, 12);
    }

    #[test]
    fn test_responsive_design_for_width() {
        assert_eq!(ResponsiveDesign::for_width(320).device, Device::Mobile);
        assert_eq!(ResponsiveDesign::for_width(1024).device, Device::TabletLandscape);
        assert_eq!(ResponsiveDesign::for_width(1280).device, Device::Desktop);
    }
}