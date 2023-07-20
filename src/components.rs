use std::any::Any;
use std::fmt::Debug;
use std::panic::{RefUnwindSafe, UnwindSafe};

use crate::{Identifier, Name};

/// A style component. Implementors can be stored within
/// [`Style`](crate::Style).
///
/// # Deriving this trait
///
/// This trait can be derived. It can be customized using the `style` attribute
/// with these parameters:
///
/// - `inherited`: A boolean value, `false` by default.
/// - `name`: An identifier. By default, the type's name converted to snake case
///   is used. For example, a type named `StyleComponent` would return
///   `Name::new(StyleComponent::authority(), "style_component")`.
/// - `authority`: An identifier. By default, this is [`Identifier::private()`].
/// - `merge`: An expression to evaluate when merging. `self` and `other` are
///   defined. By default, components do not merge.
pub trait StyleComponent: Any + RefUnwindSafe + UnwindSafe + Send + Sync + Debug + 'static {
    /// The unique name of this style component.
    ///
    /// This function returns a qualified name. The default implementation uses
    /// the type's name converted to snake case. E.g., `StyleCompoment` becomes
    /// `style_component`, and [`StyleComponent::authority()`] for the
    /// authority.
    #[must_use]
    fn name() -> Name {
        let type_name = std::any::type_name::<Self>();
        let Some((_, name)) = type_name.rsplit_once("::") else { unreachable!("Invalid type name") };
        Name::new(
            Self::authority(),
            stylecs_shared::pascal_case_to_snake_case(name.to_string())
                .expect("struct name contains invalid characters"),
        )
        .expect("already validated")
    }

    /// Returns the authority of this component. By default, this returns
    /// [`Identifier::private()`].
    #[must_use]
    fn authority() -> Identifier {
        Identifier::private()
    }

    /// Returns whether the component should be inherited. Affects the behavior
    /// of [`Style::inherited_from`](crate::Style::inherited_from)
    ///
    /// This provided imiplementation returns `false`.
    #[must_use]
    fn inherited() -> bool {
        false
    }

    /// Merges `self` with `other`, if it makes sense to do so for this type.
    /// The default implementation does nothing, preserving the `self` value.
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {}
}

/// A style component that can be powered by data contained in the structure.
///
/// This trait allows style components to be defined that didn't originate from
/// Rust code -- e.g., a scripting language.
pub trait DynamicComponent:
    Any + Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static
{
    /// The unique name of this style component.
    ///
    /// Each name component must be a valid identifier: `a-z`, `A-Z`, or `_`.
    ///
    /// The [`Name`] is namespaced. If this trait is implemented for a type that
    /// is distributed as part of a crate, the implementation should use a
    /// unique [authority](Name::authority) based on the crate it comes from.
    fn name(&self) -> Name;

    /// Returns whether the component should be inherited. Affects the behavior
    /// of [`Style::inherited_from`](crate::Style::inherited_from)
    #[must_use]
    fn inherited(&self) -> bool {
        false
    }

    /// Merges `self` with `other`, if it makes sense to do so for this type.
    /// The default implementation does not do anything, preserving the value in
    /// self.
    #[allow(unused_variables)]
    fn merge(&mut self, other: &Self) {}
}

impl<T> DynamicComponent for T
where
    T: StyleComponent,
{
    fn name(&self) -> Name {
        T::name()
    }

    fn inherited(&self) -> bool {
        T::inherited()
    }

    fn merge(&mut self, other: &Self) {
        <T as StyleComponent>::merge(self, other);
    }
}
