use euclid::{Length, Scale};

use crate::{Pixels, Points, Style, StyleComponent};

#[derive(Debug, Copy)]
pub struct FontSize<Unit>(pub Length<f32, Unit>);

impl<Unit> Clone for FontSize<Unit> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Default for FontSize<Points> {
    fn default() -> Self {
        Self::new(14.)
    }
}

impl<Unit> FontSize<Unit> {
    pub fn new(value: f32) -> Self {
        Self(Length::new(value))
    }

    pub fn get(&self) -> f32 {
        self.0.get()
    }

    pub fn length(&self) -> Length<f32, Unit> {
        self.0
    }
}

impl StyleComponent<Points> for FontSize<Points> {
    fn scale(&self, scale: Scale<f32, Points, Pixels>, map: &mut Style<Pixels>) {
        let value = self.0 * scale;
        map.push(FontSize(value));
    }
}

impl StyleComponent<Pixels> for FontSize<Pixels> {
    fn scale(&self, _scale: Scale<f32, Pixels, Pixels>, map: &mut Style<Pixels>) {
        map.push(FontSize(self.0));
    }
}
