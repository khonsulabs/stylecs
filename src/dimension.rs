#![allow(missing_docs)] // TODO not sure if this file is even going to be used.

use euclid::Length;

use crate::Points;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension<Unit = Points> {
    Auto,
    /// In situations where applicable, attempt to shrink to fit the "content"
    /// this dimension is restricting. In all other situations, equivalent to
    /// Auto.
    Minimal,
    /// Scale-corrected to the users preference of DPI
    Length(Length<f32, Unit>),
}

impl<Unit> Dimension<Unit> {
    #[must_use]
    pub const fn from_f32(value: f32) -> Self {
        Self::Length(Length::new(value))
    }

    #[must_use]
    pub fn from_length<V: Into<Length<f32, Unit>>>(value: V) -> Self {
        Self::Length(value.into())
    }

    #[must_use]
    pub const fn is_auto(&self) -> bool {
        match self {
            Dimension::Minimal | Dimension::Auto => true,
            Dimension::Length(_) => false,
        }
    }

    #[must_use]
    pub const fn is_length(&self) -> bool {
        !self.is_auto()
    }

    #[must_use]
    pub const fn length(&self) -> Option<Length<f32, Unit>> {
        if let Dimension::Length(points) = &self {
            Some(*points)
        } else {
            None
        }
    }
}

impl<Unit> Default for Dimension<Unit> {
    fn default() -> Self {
        Self::Auto
    }
}

impl<Unit> From<Length<f32, Unit>> for Dimension<Unit> {
    fn from(value: Length<f32, Unit>) -> Self {
        Self::from_length(value)
    }
}
