use std::collections::hash_map::RandomState;
use std::collections::{hash_map, HashMap, HashSet};
use std::hash::BuildHasher;

use crate::any::AnyComponent;
use crate::components::DynamicComponent;
use crate::{Name, StyleComponent};

/// A set of style components.
#[derive(Default)]
pub struct Style<S = RandomState> {
    components: HashMap<Name, AnyComponent, S>,
}

impl<S> Clone for Style<S>
where
    S: BuildHasher + Clone,
{
    fn clone(&self) -> Self {
        let mut new_map = HashMap::with_capacity_and_hasher(
            self.components.len(),
            self.components.hasher().clone(),
        );

        for (name, value) in &self.components {
            new_map.insert(name.clone(), value.clone());
        }

        Self {
            components: new_map,
        }
    }
}

impl<S> std::fmt::Debug for Style<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut t = f.debug_tuple("Style");
        for component in self.components.values() {
            t.field(component);
        }
        t.finish()
    }
}

impl Style {
    /// Returns a new style with no components.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new style with enough capacity to hold `capacity` components
    /// without reallocting.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            components: HashMap::with_capacity(capacity),
        }
    }
}

impl<S> Style<S>
where
    S: BuildHasher,
{
    /// Returns a new style using the provided `hasher`.
    pub fn with_hasher(hasher: S) -> Self {
        Self {
            components: HashMap::with_hasher(hasher),
        }
    }

    /// Returns a new style with that:
    ///
    /// - has enough storage to hold `capacity` elements before reallocating
    /// - uses `hasher` for the internal [`HashMap`]
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            components: HashMap::with_capacity_and_hasher(capacity, hasher),
        }
    }

    /// Adds a component to this style. Any existing values of the same type
    /// will be replaced.
    pub fn push<T: DynamicComponent + Clone>(&mut self, component: T) {
        let c = AnyComponent::new(component);
        self.components.insert(c.name(), c);
    }

    /// Adds a component to the style and returns it. Any existing values of the
    /// same type will be replaced.
    #[must_use]
    pub fn with<T: DynamicComponent + Clone>(mut self, component: T) -> Self {
        self.push(component);
        self
    }

    /// Returns the style component of type `T`, if present.
    #[must_use]
    pub fn get<T: StyleComponent>(&self) -> Option<&T> {
        self.components.get(&T::name()).and_then(AnyComponent::get)
    }

    /// Returns the style component of type `T`, if present.
    #[must_use]
    pub fn get_by_name(&self, name: &Name) -> Option<&AnyComponent> {
        self.components.get(name)
    }

    /// Returns the style component of type `T`. If not present, `T::default()`
    /// will be returned.
    #[must_use]
    pub fn get_or_default<T: StyleComponent + Default + Clone>(&self) -> T {
        self.get::<T>().cloned().unwrap_or_default()
    }

    /// Returns a new [`Style`], merging the components of `self` with `other`.
    /// If both `self` and `other` contain a value of the same type, the value
    /// in `self` will be used.
    #[must_use]
    pub fn merged_with(mut self, other: &Self) -> Self
    where
        S: Clone,
    {
        self.merge_with_filter(other, |_| true);
        self
    }

    /// Returns a new [`Style`], merging the components of `self` with `other`
    /// only when the component is [`inherited`](StyleComponent::inherited).
    #[must_use]
    pub fn inherited_from(mut self, parent: &Self) -> Self
    where
        S: Clone,
    {
        self.merge_with_filter(parent, AnyComponent::inherited);
        self
    }

    fn merge_with_filter(&mut self, other: &Self, mut filter: impl FnMut(&AnyComponent) -> bool)
    where
        S: Clone,
    {
        let self_types = self.components.keys().cloned().collect::<HashSet<_>>();
        let parent_types = other.components.keys().cloned().collect::<HashSet<_>>();

        for type_id in self_types.union(&parent_types) {
            match (
                self.components.get_mut(type_id),
                other.components.get(type_id),
            ) {
                (Some(self_component), Some(other_component)) => {
                    if filter(other_component) {
                        self_component.merge_with(other_component);
                    }
                }
                (Some(_), None) => {}
                (None, Some(component)) => {
                    if !filter(component) {
                        continue;
                    }
                    self.components.insert(type_id.clone(), component.clone());
                }
                (None, None) => unreachable!(),
            };
        }
    }

    /// Returns the number of components in this style.
    #[must_use]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns true if this style has no components.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns an iterator over the elements in this style.
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }
}

impl<'a, S> IntoIterator for &'a Style<S> {
    type IntoIter = Iter<'a>;
    type Item = &'a AnyComponent;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.components.values())
    }
}

impl<S> IntoIterator for Style<S> {
    type IntoIter = IntoIter;
    type Item = AnyComponent;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.components.into_values())
    }
}

/// An iterator over the components contained in a [`Style`].
pub struct Iter<'a>(hash_map::Values<'a, Name, AnyComponent>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a AnyComponent;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct IntoIter(hash_map::IntoValues<Name, AnyComponent>);

impl Iterator for IntoIter {
    type Item = AnyComponent;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
