use std::{any::TypeId, fmt::Debug};

/// A style component. Implementors can be stored within
/// [`Style`](crate::Style).
pub trait StyleComponent: std::any::Any + Send + Sync + Debug + 'static {
    /// Returns whether the component should be inherited. Affects the behavior
    /// of [`Style::merge_with`](crate::Style::merge_with)
    fn should_be_inherited(&self) -> bool {
        true
    }

    /// Merges `self` with `other`, if it makes sense to do so for this type.
    /// The default implementation does not merge and just returns a clone of
    /// `self`.
    #[allow(unused_variables)]
    fn merge(&self, other: &Self) -> Self
    where
        Self: Clone,
    {
        self.clone()
    }
}

/// A style component that has a fallback. An example could be `TextColor` and
/// `ForegroundColor` components. `TextColor` could specify `ForegroundColor` as
/// a fallback. Used with
/// [`Style::get_with_fallback`](crate::Style::get_with_fallback).
pub trait FallbackComponent: StyleComponent {
    /// The style component to fall back to. If this is the root, use Self. The
    /// provided implementation of [`Self::has_fallback()`] will return false
    /// when `Self::Fallback` == `Self`.
    type Fallback: FallbackComponent<Value = Self::Value>;
    /// The contained value of this component.
    type Value;

    /// The contained value of the component.
    fn value(&self) -> Option<&'_ Self::Value>;

    /// Returns true if there is a fallback.
    #[must_use]
    fn has_fallback() -> bool {
        TypeId::of::<Self>() != TypeId::of::<Self::Fallback>()
    }
}
