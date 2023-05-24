use std::borrow::Cow;
use std::fmt::{Debug, Display};
use std::ops::Deref;

use interner::global::{GlobalString, StaticPooledString, StringPool};
use stylecs_shared::InvalidIdentifier;

/// A name that contains only `a-z`, `A-Z`, `0-9`, or `_` characters.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Identifier(GlobalString);

#[doc(hidden)]
pub static IDENTIFIERS: StringPool = StringPool::new();
static PRIVATE: StaticPooledString = IDENTIFIERS.get_static("_");

impl Identifier {
    /// Returns tne identifier used to designate a private authority.
    pub fn private() -> Self {
        Self(PRIVATE.clone())
    }

    /// Validates `name` and returns an `Identifier` if name does not contain
    /// any disallowed characters.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidIdentifier`] if `name` contains any character that is
    /// not one of:
    ///
    /// - `a-z`
    /// - `A-Z`
    /// - `0-9`
    /// - `_`
    pub fn new<'a>(name: impl Into<Cow<'a, str>>) -> Result<Self, InvalidIdentifier> {
        let name = name.into();
        stylecs_shared::validate_identifier(&name).map(|_| Self(IDENTIFIERS.get(name)))
    }

    /// Validates `name` and returns an error if any invalid characters are
    /// encountered.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidIdentifier`] if `name` contains any character that is
    /// not one of:
    ///
    /// - `a-z`
    /// - `A-Z`
    /// - `0-9`
    /// - `_`
    pub const fn validate(name: &str) -> Result<(), InvalidIdentifier> {
        stylecs_shared::validate_identifier(name)
    }
}

impl<'a> TryFrom<&'a str> for Identifier {
    type Error = InvalidIdentifier;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl<'a> PartialEq<&'a str> for Identifier {
    fn eq(&self, other: &&'a str) -> bool {
        self == *other
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A globally unique name.
///
/// This structure has an [`authority`](Self::authority) and a [`name`](Self::name).
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Name {
    /// The unique name of the source of this name. For example, this could be
    /// the name of the crate it was defined within.
    ///
    /// [`Identifier::private()`] is used as the authority when
    /// [`Name::private()`] is called.
    pub authority: Identifier,
    /// The locally unique name.
    ///
    /// This name only needs to be unique within its `authority`. For example,
    /// two authorities can define their own `color` components without
    /// conflicts.
    pub name: Identifier,
}

impl Name {
    /// Returns a new [`Name`] with `_` used as the authority.
    ///
    /// This is equivalent to calling `Name::new("_", name)`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidIdentifier`] if any invalid [`Identifier`] characters
    /// are encountered.
    pub fn private(name: impl Identifiable) -> Result<Self, InvalidIdentifier> {
        Self::new(Identifier::private(), name)
    }

    /// Returns a new [`Name`] using `authority` and `name`.
    ///
    /// Each `name` should be unique within the `authority` namespace.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidIdentifier`] if any invalid [`Identifier`] characters
    /// are encountered.
    pub fn new(
        authority: impl Identifiable,
        name: impl Identifiable,
    ) -> Result<Self, InvalidIdentifier> {
        Ok(Self {
            authority: authority.into_identifier()?,
            name: name.into_identifier()?,
        })
    }
}

pub trait Identifiable {
    fn into_identifier(self) -> Result<Identifier, InvalidIdentifier>;
}

impl<'a> Identifiable for &'a str {
    fn into_identifier(self) -> Result<Identifier, InvalidIdentifier> {
        Identifier::new(self)
    }
}

impl Identifiable for String {
    fn into_identifier(self) -> Result<Identifier, InvalidIdentifier> {
        Identifier::new(self)
    }
}

impl Identifiable for Identifier {
    fn into_identifier(self) -> Result<Identifier, InvalidIdentifier> {
        Ok(self)
    }
}

/// Returns a [`StaticName`], which allows for a name to be defined statically.
///
/// This allows a minor optimization such that [`Identifier`]s created always
/// exist.
#[macro_export]
macro_rules! static_name {
    ($private_name:expr) => {
        $crate::static_name!("_", $private_name)
    };
    ($authority:expr, $name:expr) => {
        $crate::StaticName::new(
            match $crate::Identifier::validate($authority) {
                Ok(_) => $crate::IDENTIFIERS.get_static($authority),
                Err(_) => panic!("invalid character in authority"),
            },
            match $crate::Identifier::validate($name) {
                Ok(_) => $crate::IDENTIFIERS.get_static($name),
                Err(_) => panic!("invalid character in name"),
            },
        )
    };
}

/// A statically defined [`Name`].
///
/// # Creating a static private name
///
/// ```rust
/// use stylecs::{static_name, Name, StaticName};
///
/// static PRIVATE: StaticName = static_name!("private");
/// assert_eq!(PRIVATE.to_name(), Name::private("private").unwrap());
/// ```
///
/// # Why use [`StaticName`]?
///
/// This type enables a minor optimization. Each [`Identifier`] guarantees that
/// only one copy of the string it points to exists. This allows for
/// optimizations when using names as keys in a hash map. To support this, each
/// time an [`Identifier`] is created, the global list must be first checked and
/// a copy of the existing value returned if it already exists.
///
/// This type allows performing this initialization once upon first access of
/// the [`StaticName`]. This type can be converted to [`Name`] using
/// `Into`/`From`.
pub struct StaticName {
    authority: StaticPooledString,
    name: StaticPooledString,
}

impl StaticName {
    #[doc(hidden)]
    pub const fn new(authority: StaticPooledString, name: StaticPooledString) -> Self {
        Self { authority, name }
    }

    /// Returns this static instance as a regular [`Name`].
    pub fn to_name(&self) -> Name {
        Name::from(self)
    }
}

impl<'a> From<&'a StaticName> for Name {
    fn from(value: &'a StaticName) -> Self {
        Self {
            authority: Identifier(value.authority.get().clone()),
            name: Identifier(value.name.get().clone()),
        }
    }
}
