use std::fmt::Debug;

use crate::StyleComponent;

/// A [`StyleComponent`] that can be boxed for storage and cloned.
#[allow(clippy::module_name_repetitions)]
pub trait AnyStyleComponent: StyleComponent + Send + Sync + Debug + 'static {
    /// Returns the style component as `Any`.
    #[must_use]
    fn as_any(&self) -> &'_ dyn std::any::Any;

    /// Returns boxed clone of the style component.
    #[must_use]
    fn clone_to_style_component(&self) -> Box<dyn AnyStyleComponent>;
}

impl<T: StyleComponent + Clone> AnyStyleComponent for T {
    fn as_any(&self) -> &'_ dyn std::any::Any {
        self
    }

    fn clone_to_style_component(&self) -> Box<dyn AnyStyleComponent> {
        Box::new(self.clone())
    }
}
