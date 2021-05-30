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
mod components;
mod style;
/// Types for defining sets of rules.
pub mod style_sheet;

pub use self::{
    any::AnyStyleComponent,
    components::{FallbackComponent, StyleComponent},
    style::Style,
};

#[cfg(test)]
mod tests;
