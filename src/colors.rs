use std::fmt::Debug;

use palette::Srgba;

use crate::UnscaledStyleComponent;

/// The theme variant for the system.
#[derive(Debug, Clone)]
pub enum SystemTheme {
    /// A light theme.
    Light,
    /// A dark theme.
    Dark,
}

impl UnscaledStyleComponent for SystemTheme {
    fn should_be_inherited(&self) -> bool {
        true
    }
}

impl Default for SystemTheme {
    fn default() -> Self {
        // So tempted to make this dark.
        Self::Light
    }
}

/// A pair of colors, one for each [`SystemTheme`] variant.
#[derive(Debug, Clone, Default, Copy)]
pub struct ColorPair {
    /// The color used when the current system theme is [`SystemTheme::Light`].
    pub light_color: Srgba,
    /// The color used when the current system theme is [`SystemTheme::Dark`].
    pub dark_color: Srgba,
}

impl ColorPair {
    /// Returns a copy of the color pair, replacing each colors alpha channel
    /// with the value provided (0.0-1.0 range).
    #[must_use]
    pub const fn with_alpha(mut self, alpha: f32) -> Self {
        self.light_color.alpha = alpha;
        self.dark_color.alpha = alpha;
        self
    }
}

impl From<Srgba> for ColorPair {
    fn from(color: Srgba) -> Self {
        Self {
            light_color: color,
            dark_color: color,
        }
    }
}

impl ColorPair {
    /// Returns color corresponding to `system_theme`.
    #[must_use]
    pub const fn themed_color(&self, system_theme: &SystemTheme) -> Srgba {
        match system_theme {
            SystemTheme::Light => self.light_color,
            SystemTheme::Dark => self.dark_color,
        }
    }
}
