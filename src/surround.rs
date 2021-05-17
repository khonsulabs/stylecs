use euclid::{Length, Size2D};

use crate::Dimension;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Surround<Unit> {
    pub left: Dimension<Unit>,
    pub top: Dimension<Unit>,
    pub right: Dimension<Unit>,
    pub bottom: Dimension<Unit>,
}

impl<Unit> Surround<Unit> {
    pub fn minimum_width(&self) -> Length<f32, Unit> {
        self.left.length().unwrap_or_default() + self.right.length().unwrap_or_default()
    }

    pub fn minimum_height(&self) -> Length<f32, Unit> {
        self.top.length().unwrap_or_default() + self.bottom.length().unwrap_or_default()
    }

    pub fn minimum_size(&self) -> Size2D<f32, Unit> {
        Size2D::from_lengths(self.minimum_width(), self.minimum_height())
    }
}
