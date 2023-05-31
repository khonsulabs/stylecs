use std::cmp::Ordering;
use std::ops::Deref;

use kempt::Map;

use crate::any::AnyComponent;
use crate::components::DynamicComponent;
use crate::{Name, StyleComponent};

/// A set of style components.
#[derive(Default, Clone)]
pub struct Style {
    components: Map<ComponentName<'static>, AnyComponent>,
}

impl std::fmt::Debug for Style {
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
            components: Map::with_capacity(capacity),
        }
    }

    /// Adds a component to this style. Any existing values of the same type
    /// will be replaced.
    pub fn push<T: DynamicComponent + Clone>(&mut self, component: T) {
        let c = AnyComponent::new(component);
        self.components.insert(ComponentName::Owned(c.name()), c);
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
        self.components
            .get(&ComponentName::Owned(T::name()))
            .and_then(AnyComponent::get)
    }

    /// Returns the style component of type `T`, if present.
    #[must_use]
    pub fn get_by_name(&self, name: &Name) -> Option<&AnyComponent> {
        self.components.get(&ComponentName::Borrowed(name))
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
    pub fn merged_with(mut self, other: &Self) -> Self {
        self.components.merge_with(
            &other.components,
            |_key, value| Some(value.clone()),
            |_key, mine, other| mine.merge_with(other),
        );
        self
    }

    /// Returns a new [`Style`], merging the components of `self` with `other`
    /// only when the component is [`inherited`](StyleComponent::inherited).
    #[must_use]
    pub fn inherited_from(mut self, parent: &Self) -> Self {
        self.components.merge_with(
            &parent.components,
            |_key, value| value.inherited().then(|| value.clone()),
            |_key, mine, other| {
                if other.inherited() {
                    mine.merge_with(other);
                }
            },
        );
        self
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

impl<'a> IntoIterator for &'a Style {
    type IntoIter = Iter<'a>;
    type Item = &'a AnyComponent;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.components.values())
    }
}

impl IntoIterator for Style {
    type IntoIter = IntoIter;
    type Item = AnyComponent;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.components.into_values())
    }
}

/// An iterator over the components contained in a [`Style`].
pub struct Iter<'a>(kempt::map::Values<'a, ComponentName<'static>, AnyComponent>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a AnyComponent;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct IntoIter(kempt::map::IntoValues<ComponentName<'static>, AnyComponent>);

impl Iterator for IntoIter {
    type Item = AnyComponent;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Clone)]
enum ComponentName<'a> {
    Borrowed(&'a Name),
    Owned(Name),
}

impl<'a> Deref for ComponentName<'a> {
    type Target = Name;

    fn deref(&self) -> &Self::Target {
        match self {
            ComponentName::Borrowed(name) => name,
            ComponentName::Owned(name) => name,
        }
    }
}

impl<'a> Eq for ComponentName<'a> {}

impl<'a, 'b> PartialEq<ComponentName<'b>> for ComponentName<'a> {
    fn eq(&self, other: &ComponentName<'b>) -> bool {
        match (self, other) {
            (Self::Borrowed(l0), ComponentName::Borrowed(r0)) => l0 == r0,
            (Self::Owned(l0), ComponentName::Owned(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<'a> Ord for ComponentName<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("infallible")
    }
}

impl<'a, 'b> PartialOrd<ComponentName<'b>> for ComponentName<'a> {
    fn partial_cmp(&self, other: &ComponentName<'b>) -> Option<std::cmp::Ordering> {
        // Prioritize comparing the name, as in general the component names
        // shouldn't conflict.
        let a = &**self;
        let b = &**other;
        Some(match a.name.as_ptr().cmp(&b.name.as_ptr()) {
            order @ (Ordering::Greater | Ordering::Less) => order,
            Ordering::Equal => a.authority.as_ptr().cmp(&b.authority.as_ptr()),
        })
    }
}
