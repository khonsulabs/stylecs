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
#![cfg_attr(doc, deny(rustdoc))]

use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use euclid::Scale;

mod any;
mod colors;
mod dimension;
mod surround;
pub use self::{
    any::AnyStyleComponent,
    colors::{ColorPair, SystemTheme},
    dimension::Dimension,
    surround::Surround,
};

pub use palette;

/// A unit representing physical pixels on a display.
#[derive(Debug, Clone, Copy, Default)]
pub struct Pixels;

/// A unit representing [Desktop publishing points/PostScript points](https://en.wikipedia.org/wiki/Point_(typography)#Desktop_publishing_point). Measurements in this scale are equal to 1/72 of an [imperial inch](https://en.wikipedia.org/wiki/Inch).
///
#[derive(Debug, Clone, Copy, Default)]
pub struct Points;

/// A set of style components.
#[derive(Debug)]
pub struct Style<Unit: 'static> {
    components: HashMap<TypeId, Box<dyn AnyStyleComponent<Unit>>>,
}

impl<Unit: Send + Sync> Clone for Style<Unit> {
    fn clone(&self) -> Self {
        let mut new_map = HashMap::<TypeId, Box<dyn AnyStyleComponent<Unit>>>::new();

        for (type_id, value) in &self.components {
            new_map.insert(*type_id, value.clone_to_style_component());
        }

        Self {
            components: new_map,
        }
    }
}

impl<Unit> Default for Style<Unit> {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}

impl<Unit: Send + Sync + 'static> Style<Unit> {
    /// Returns a new style with no components.
    #[must_use]
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// Adds a component to this style. Any existing values of the same type will be replaced.
    pub fn push<T: StyleComponent<Unit> + Clone>(&mut self, component: T) {
        self.components
            .insert(component.type_id(), Box::new(component));
    }

    /// Adds a component to the style and returns it. Any existing values of the same type will be replaced.
    pub fn with<T: StyleComponent<Unit> + Clone>(mut self, component: T) -> Self {
        self.push(component);
        self
    }

    /// Returns the style component of type `T`, if present.
    #[must_use]
    pub fn get<T: StyleComponent<Unit>>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        self.components.get(&type_id).and_then(|w| {
            let component_as_any = w.as_any();
            component_as_any.downcast_ref::<T>()
        })
    }

    /// Returns the style component of type `T`. If not present, `T::default()`
    /// will be returned.
    #[must_use]
    pub fn get_or_default<T: StyleComponent<Unit> + Default + Clone>(&self) -> T {
        self.get::<T>().cloned().unwrap_or_default()
    }
}

/// A style component. Implementors can be stored within [`Style`].
pub trait StyleComponent<Unit>: std::any::Any + Send + Sync + Debug + 'static {
    /// Scale the component to `Pixels`, storing the result in `destination`.
    fn scale(&self, scale: Scale<f32, Unit, Pixels>, destination: &mut Style<Pixels>);

    /// Returns whether the component should be inherited. Affects the behavior of [`Style::merge_with`]
    fn should_be_inherited(&self) -> bool {
        true
    }
}

/// A style component that has no measurements. When implementing a new style
/// component type, if you have no measurements, you can `impl
/// UnscaledStyleComponent for YourType {}` to avoid needing to implement
/// [`StyleComponent`] for both [`Points`] and [`Pixels`].
pub trait UnscaledStyleComponent:
    AnyStyleComponent<Points> + Clone + Send + Sync + Debug + 'static
{
    /// Returns whether the component should be inherited. Affects the behavior of [`Style::merge_with`]
    fn should_be_inherited(&self) -> bool {
        true
    }
}

impl<T> StyleComponent<Points> for T
where
    T: UnscaledStyleComponent,
{
    fn scale(&self, _scale: Scale<f32, Points, Pixels>, destination: &mut Style<Pixels>) {
        destination.push(self.clone());
    }

    fn should_be_inherited(&self) -> bool {
        <Self as UnscaledStyleComponent>::should_be_inherited(self)
    }
}

impl<T> StyleComponent<Pixels> for T
where
    T: StyleComponent<Points> + Clone,
{
    fn scale(&self, _scale: Scale<f32, Pixels, Pixels>, destination: &mut Style<Pixels>) {
        destination.push(self.clone());
    }
}

impl<Unit: Send + Sync + 'static> Style<Unit> {
    /// Returns a new [`Style`] merging the components of `self` with `other`. If both `self` and `other` contain a value of the same type, the value in `self` will be used.
    ///
    /// When `is_inheritence` is `true`, values from `other` will not be used if [`StyleComponent::should_be_inherited`] return false.
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // The only calls to unwrap() are in situations that cannot fail.
    pub fn merge_with(&self, other: &Self, is_inheritance: bool) -> Self {
        let mut merged_components = HashMap::<TypeId, Box<dyn AnyStyleComponent<Unit>>>::new();
        let self_types = self.components.keys().cloned().collect::<HashSet<_>>();
        let parent_types = other.components.keys().cloned().collect::<HashSet<_>>();

        for type_id in self_types.union(&parent_types) {
            let value = if self_types.contains(type_id) {
                self.components.get(type_id).unwrap()
            } else {
                let value = other.components.get(type_id).unwrap();
                if is_inheritance && !value.should_be_inherited() {
                    continue;
                }
                value
            };
            merged_components.insert(*type_id, value.clone_to_style_component());
        }
        Self {
            components: merged_components,
        }
    }
}

impl Style<Points> {
    /// Convert this style from resolution-indendent measurements to [`Pixels`] using `scale`.
    #[must_use]
    pub fn to_screen_scale(&self, scale: Scale<f32, Points, Pixels>) -> Style<Pixels> {
        let mut style = Style::new();

        for component in self.components.values() {
            component.scale(scale, &mut style);
        }

        style
    }
}

// TODO re-evalute stylesheets.
// #[derive(Default, Clone, Debug)]
// pub struct StyleSheet {
//     pub normal: Style<Points>,
//     pub hover: Style<Points>,
//     pub focus: Style<Points>,
//     pub active: Style<Points>,
// }

// impl From<Style<Points>> for StyleSheet {
//     fn from(style: Style<Points>) -> Self {
//         Self {
//             normal: style.clone(),
//             active: style.clone(),
//             hover: style.clone(),
//             focus: style,
//         }
//     }
// }

// impl StyleSheet {
//     pub fn merge_with(&self, other: &StyleSheet, is_inheritance: bool) -> Self {
//         Self {
//             normal: self.normal.merge_with(&other.normal, is_inheritance),
//             active: self.active.merge_with(&other.active, is_inheritance),
//             hover: self.hover.merge_with(&other.hover, is_inheritance),
//             focus: self.focus.merge_with(&other.focus, is_inheritance),
//         }
//     }

//     pub fn map_each<F: Fn(&Style<Points>) -> Style<Points>>(&self, map: F) -> Self {
//         Self {
//             normal: map(&self.normal),
//             active: map(&self.active),
//             hover: map(&self.hover),
//             focus: map(&self.focus),
//         }
//     }
// }
