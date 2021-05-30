//! A style component system to aide in building themable apps.

#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms
)]
#![cfg_attr(doc, deny(rustdoc::all))]

mod any;
mod colors;
mod components;
mod style;
/// Types for defining sets of rules.
pub mod style_sheet;
mod surround;
pub use palette;

pub use self::{
    any::AnyStyleComponent,
    colors::{ColorPair, SystemTheme},
    components::{FallbackComponent, StyleComponent},
    style::Style,
    surround::Surround,
};

#[cfg(test)]
mod tests;
