use std::marker::PhantomData;

use euclid::{Length, Size2D};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Surround<Unit> {
    pub left: Option<f32>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    _phantom: PhantomData<Unit>,
}

impl<Unit> Surround<Unit> {
    pub fn minimum_width(&self) -> Length<f32, Unit> {
        self.left().unwrap_or_default() + self.right().unwrap_or_default()
    }

    pub fn minimum_height(&self) -> Length<f32, Unit> {
        self.top().unwrap_or_default() + self.bottom().unwrap_or_default()
    }

    pub fn minimum_size(&self) -> Size2D<f32, Unit> {
        Size2D::from_lengths(self.minimum_width(), self.minimum_height())
    }

    pub fn left(&self) -> Option<Length<f32, Unit>> {
        self.left.map(Length::new)
    }

    pub fn right(&self) -> Option<Length<f32, Unit>> {
        self.right.map(Length::new)
    }

    pub fn bottom(&self) -> Option<Length<f32, Unit>> {
        self.bottom.map(Length::new)
    }

    pub fn top(&self) -> Option<Length<f32, Unit>> {
        self.top.map(Length::new)
    }
}
