use std::fmt::Debug;

use palette::Srgba;

use crate::{Points, UnscaledStyleComponent};

#[derive(Debug, Clone)]
pub enum SystemTheme {
    Light,
    Dark,
}

impl UnscaledStyleComponent<Points> for SystemTheme {
    fn unscaled_should_be_inherited(&self) -> bool {
        true
    }
}

impl Default for SystemTheme {
    fn default() -> Self {
        // So tempted to make this dark.
        SystemTheme::Light
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct ColorPair {
    pub light_color: Srgba,
    pub dark_color: Srgba,
}

impl ColorPair {
    pub fn with_alpha(mut self, alpha: f32) -> Self {
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
    pub fn themed_color(&self, system_theme: &SystemTheme) -> Srgba {
        match system_theme {
            SystemTheme::Light => self.light_color,
            SystemTheme::Dark => self.dark_color,
        }
    }
}
