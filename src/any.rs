use std::fmt::Debug;
use std::option::Option;
use std::panic::{RefUnwindSafe, UnwindSafe};

use crate::components::DynamicComponent;
use crate::{Name, StyleComponent};

/// A [`DynamicComponent`]/[`StyleComponent`] that can be boxed for storage and
/// cloned.
#[allow(clippy::module_name_repetitions)]
pub(crate) trait AnyStyleComponent:
    RefUnwindSafe + UnwindSafe + Send + Sync + Debug + 'static
{
    /// Returns the style component as `Any`.
    #[must_use]
    fn as_any(&self) -> &'_ dyn std::any::Any;
    /// Returns the mutable style component as `Any`.
    #[must_use]
    fn as_mut_any(&mut self) -> &'_ mut dyn std::any::Any;

    /// Returns boxed clone of the style component.
    #[must_use]
    fn clone_any(&self) -> AnyComponent;

    fn merge_with(&mut self, other: &dyn AnyStyleComponent);

    fn inherited(&self) -> bool;

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn name(&self) -> Name;
}

impl<T> AnyStyleComponent for Option<T>
where
    T: DynamicComponent + Clone + RefUnwindSafe + UnwindSafe,
{
    fn as_any(&self) -> &'_ dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &'_ mut dyn std::any::Any {
        self
    }

    fn clone_any(&self) -> AnyComponent {
        AnyComponent::new(self.clone().expect("component unboxed"))
    }

    fn merge_with(&mut self, other: &dyn AnyStyleComponent) {
        let myself = self
            .as_mut_any()
            .downcast_mut::<Self>()
            .expect("incorrect type")
            .as_mut()
            .expect("style unboxed");
        let other = other
            .as_any()
            .downcast_ref::<Self>()
            .expect("incorrect type")
            .as_ref()
            .expect("style unboxed");

        myself.merge(other);
    }

    fn inherited(&self) -> bool {
        self.as_ref().expect("style unboxed").inherited()
    }

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().expect("style unboxed").fmt(f)
    }

    fn name(&self) -> Name {
        self.as_ref().expect("style unboxed").name()
    }
}

/// A boxed [`StyleComponent`].
pub struct AnyComponent(Box<dyn AnyStyleComponent>);

impl AnyComponent {
    /// Returns a new instance wrapping `component`.
    pub fn new<C: DynamicComponent + Clone>(component: C) -> Self {
        Self(Box::new(Some(component)))
    }

    /// Returns the contained style component. Returns `None` if `T` is not the
    /// same type that was wrapped.
    #[must_use]
    pub fn get<T: StyleComponent>(&self) -> Option<&T> {
        self.0
            .as_any()
            .downcast_ref::<Option<T>>()
            .and_then(Option::as_ref)
    }

    /// Returns the contained style component. Returns `None` if `T` is not the
    /// same type that was wrapped.
    #[must_use]
    pub fn get_mut<T: StyleComponent>(&mut self) -> Option<&mut T> {
        self.0
            .as_mut_any()
            .downcast_mut::<Option<T>>()
            .and_then(Option::as_mut)
    }

    /// Returns the result of [`DynamicComponent::inherited`].
    #[must_use]
    pub fn inherited(&self) -> bool {
        self.0.inherited()
    }

    /// Calls [`DynamicComponent::merge`] to merge `self` with `other`.
    ///
    /// # Panics
    ///
    /// This function panics if `other` does not wrap the same type as `self` is
    /// wrapping.
    pub fn merge_with(&mut self, other: &Self) {
        self.0.merge_with(other.0.as_ref());
    }

    /// Calls [`DynamicComponent::merge`] and returns the updated value.
    ///
    /// # Panics
    ///
    /// This function panics if `other` does not wrap the same type as `self` is
    /// wrapping.
    #[must_use]
    pub fn merged_with(mut self, other: &Self) -> Self {
        self.merge_with(other);
        self
    }

    /// Returns the name of the component.
    #[must_use]
    pub fn name(&self) -> Name {
        self.0.name()
    }
}

impl Clone for AnyComponent {
    fn clone(&self) -> Self {
        self.0.clone_any()
    }
}

impl Debug for AnyComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.debug(f)
    }
}
