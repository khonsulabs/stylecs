use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

use crate::{AnyStyleComponent, FallbackComponent, StyleComponent};

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
    pub fn get_with_fallback<T: FallbackComponent>(&self) -> Option<&T::Value> {
        if let Some(component) = self.get::<T>() {
            component.value()
        } else if T::has_fallback() {
            self.get_with_fallback::<T::Fallback>()
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
            let value = match (self.components.get(type_id), other.components.get(type_id)) {
                (Some(self_component), Some(other_component)) =>
                    if is_inheritance {
                        self_component.clone_to_style_component()
                    } else {
                        self_component.merge_with(other_component.as_ref())
                    },
                (Some(component), None) => component.clone_to_style_component(),
                (None, Some(component)) => {
                    if is_inheritance && !component.should_be_inherited() {
                        continue;
                    }
                    component.clone_to_style_component()
                }
                (None, None) => unreachable!(),
            };
            merged_components.insert(*type_id, value);
        }
        Self {
            components: merged_components,
        }
    }
}
