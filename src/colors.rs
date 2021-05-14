use std::fmt::Debug;

use palette::Srgba;

use crate::{Points, UnscaledStyleComponent};

pub enum SystemTheme {
    Light,
    Dark,
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

#[derive(Debug, Clone)]
pub struct ForegroundColor(pub ColorPair);
impl UnscaledStyleComponent<Points> for ForegroundColor {}

impl Default for ForegroundColor {
    fn default() -> Self {
        ForegroundColor(ColorPair {
            light_color: Srgba::new(0., 0., 0., 1.),
            dark_color: Srgba::new(1., 1., 1., 1.),
        })
    }
}

impl From<ForegroundColor> for ColorPair {
    fn from(color: ForegroundColor) -> Self {
        color.0
    }
}

#[derive(Debug, Clone)]
pub struct BackgroundColor(pub ColorPair);
impl UnscaledStyleComponent<Points> for BackgroundColor {
    fn unscaled_should_be_inherited(&self) -> bool {
        false
    }
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor(ColorPair {
            light_color: Srgba::new(1., 1., 1., 1.),
            dark_color: Srgba::new(0., 0., 0., 1.),
        })
    }
}

impl From<BackgroundColor> for ColorPair {
    fn from(color: BackgroundColor) -> Self {
        color.0
    }
}
