use std::any::TypeId;
use std::fmt::Debug;
use std::option::Option;

use crate::components::DynamicComponent;
use crate::{Name, StyleComponent};

/// A [`DynamicComponent`]/[`StyleComponent`] that can be boxed for storage and
/// cloned.
#[allow(clippy::module_name_repetitions)]
pub(crate) trait AnyStyleComponent: Send + Sync + Debug + 'static {
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

impl<T: DynamicComponent + Clone> AnyStyleComponent for Option<T> {
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

pub struct AnyComponent(Box<dyn AnyStyleComponent>);

impl AnyComponent {
    pub fn new<C: DynamicComponent + Clone>(component: C) -> Self {
        Self(Box::new(Some(component)))
    }

    #[must_use]
    pub fn component_type_id(&self) -> TypeId {
        self.0.as_any().type_id()
    }

    #[must_use]
    pub fn get<T: StyleComponent>(&self) -> Option<&T> {
        self.0
            .as_any()
            .downcast_ref::<Option<T>>()
            .and_then(Option::as_ref)
    }

    pub fn inherited(&self) -> bool {
        self.0.inherited()
    }

    pub fn merge_with(&mut self, other: &Self) {
        self.0.merge_with(other.0.as_ref());
    }

    pub fn merged_with(mut self, other: &Self) -> Self {
        self.merge_with(other);
        self
    }

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
