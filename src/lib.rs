//! A specialized component system aimed at helping build a styling foundation
//! for Rust apps.
//!
//! This crate makes it easy to annotate types as [`StyleComponent`]s that can
//! be used within a [`Style`]. This crate provides no [`StyleComponent`]
//! implementors.
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms
)]
#![allow(
    clippy::multiple_crate_versions, // syn 1 + 2, not currently avoidable
    clippy::module_name_repetitions,
)]
#![cfg_attr(doc, warn(rustdoc::all))]

mod any;
mod components;
mod names;
mod object;
mod style;

#[doc(hidden)]
pub use names::IDENTIFIERS;
pub use names::{Identifier, Name, StaticName};
#[cfg(feature = "derive")]
pub use stylecs_macros::StyleComponentAttribute as StyleComponent;
pub use stylecs_shared::InvalidIdentifier;

pub use self::components::{DynamicComponent, StyleComponent};
pub use self::style::{Iter, Style};

#[doc(hidden)]
#[macro_export]
macro_rules! __count {
    () => (0_usize);
    ( $x:tt $($xs:tt)* ) => (1_usize + $crate::__count!($($xs)*));
}

/// A shorthand for creating a [`Style`] type from a compile-time list of
/// [`StyleComponent`] implementors.
///
/// # Example Usage
///
/// ```rust
/// use stylecs::{style, StyleComponent};
///
/// #[derive(StyleComponent, Debug, Clone, Eq, PartialEq)]
/// struct SomeComponent(u32);
/// #[derive(StyleComponent, Debug, Clone)]
/// struct AnotherComponent;
///
/// let style = style![SomeComponent(42), AnotherComponent];
/// assert_eq!(style.len(), 2);
/// assert_eq!(style.get::<SomeComponent>(), Some(&SomeComponent(42)));
/// ```
#[macro_export]
macro_rules! style {
    () => {
        $crate::Style::new()
    };
    ($($component:expr),+ $(,)?) => {{
        let mut s = $crate::Style::with_capacity($crate::__count!($($component)+));
        $(s.push($component);)+
        s
    }};
}

#[cfg(test)]
mod tests;
