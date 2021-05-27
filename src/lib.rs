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

use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

mod any;
mod colors;
mod surround;
pub use palette;

pub use self::{
    any::AnyStyleComponent,
    colors::{ColorPair, SystemTheme},
    surround::Surround,
};

/// A set of style components.
#[derive(Debug)]
pub struct Style {
    components: HashMap<TypeId, Box<dyn AnyStyleComponent>>,
}

impl Clone for Style {
    fn clone(&self) -> Self {
        let mut new_map = HashMap::<TypeId, Box<dyn AnyStyleComponent>>::new();

        for (type_id, value) in &self.components {
            new_map.insert(*type_id, value.clone_to_style_component());
        }

        Self {
            components: new_map,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}

impl Style {
    /// Returns a new style with no components.
    #[must_use]
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// Adds a component to this style. Any existing values of the same type
    /// will be replaced.
    pub fn push<T: StyleComponent + Clone>(&mut self, component: T) {
        self.components
            .insert(component.type_id(), Box::new(component));
    }

    /// Adds a component to the style and returns it. Any existing values of the
    /// same type will be replaced.
    pub fn with<T: StyleComponent + Clone>(mut self, component: T) -> Self {
        self.push(component);
        self
    }

    /// Returns the style component of type `T`, if present.
    #[must_use]
    pub fn get<T: StyleComponent>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        self.components.get(&type_id).and_then(|w| {
            let component_as_any = w.as_any();
            component_as_any.downcast_ref::<T>()
        })
    }

    /// Returns the style component of type `T`, if present.
    #[must_use]
    pub fn get_with_fallback<T: FallbackComponent<V>, V>(&self) -> Option<&V> {
        if let Some(component) = self.get::<T>() {
            component.value()
        } else if T::has_fallback() {
            self.get_with_fallback::<T::Fallback, V>()
        } else {
            None
        }
    }

    /// Returns the style component of type `T`. If not present, `T::default()`
    /// will be returned.
    #[must_use]
    pub fn get_or_default<T: StyleComponent + Default + Clone>(&self) -> T {
        self.get::<T>().cloned().unwrap_or_default()
    }
}

/// A style component. Implementors can be stored within [`Style`].
pub trait StyleComponent: std::any::Any + Send + Sync + Debug + 'static {
    /// Returns whether the component should be inherited. Affects the behavior
    /// of [`Style::merge_with`]
    fn should_be_inherited(&self) -> bool {
        true
    }
}

impl Style {
    /// Returns a new [`Style`] merging the components of `self` with `other`.
    /// If both `self` and `other` contain a value of the same type, the value
    /// in `self` will be used.
    ///
    /// When `is_inheritence` is `true`, values from `other` will not be used if
    /// [`StyleComponent::should_be_inherited`] return false.
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // The only calls to unwrap() are in situations that cannot fail.
    pub fn merge_with(&self, other: &Self, is_inheritance: bool) -> Self {
        let mut merged_components = HashMap::<TypeId, Box<dyn AnyStyleComponent>>::new();
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

/// A style component that has a fallback. An example could be `TextColor` and
/// `ForegroundColor` components. `TextColor` could specify `ForegroundColor` as
/// a fallback. Used with [`Style::get_with_fallback`].
pub trait FallbackComponent<Value>: StyleComponent {
    /// The style component to fall back to.
    type Fallback: FallbackComponent<Value>;

    /// The contained value of the component.
    fn value(&self) -> Option<&'_ Value>;

    /// Returns true if there is a fallback. Typically, you should never
    /// implement this, and use [`ComponentRoot`] if you're defining a root
    /// fallback component.
    #[must_use]
    fn has_fallback() -> bool {
        true
    }
}

/// A type used to signal the root of a fallback chain.
#[derive(Debug)]
pub struct ComponentRoot;

impl StyleComponent for ComponentRoot {}

impl<T> FallbackComponent<T> for ComponentRoot {
    type Fallback = Self;

    fn value(&self) -> Option<&'_ T> {
        None
    }

    fn has_fallback() -> bool {
        false
    }
}
