use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::str::FromStr;

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
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
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

impl<'a> From<Name> for Cow<'a, Name> {
    fn from(value: Name) -> Self {
        Cow::Owned(value)
    }
}

impl<'a> From<&'a Name> for Cow<'a, Name> {
    fn from(value: &'a Name) -> Self {
        Cow::Borrowed(value)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if PRIVATE != self.authority.0 {
            f.write_str(&self.authority)?;
            f.write_str("::")?;
        }
        f.write_str(&self.name)
    }
}

impl FromStr for Name {
    type Err = InvalidIdentifier;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((authority, name)) = s.split_once("::") {
            let authority = Identifier::new(authority)?;
            let name = Identifier::new(name)?;
            Ok(Self { authority, name })
        } else {
            let name = Identifier::new(s)?;
            Ok(Self {
                authority: Identifier::private(),
                name,
            })
        }
    }
}

#[test]
fn name_strings() {
    let private = Name::private("private").unwrap();
    let private_string = private.to_string();
    assert_eq!(private_string, "private");
    let parsed: Name = private_string.parse().unwrap();
    assert_eq!(parsed, private);

    let qualified = Name::new("authority", "name").unwrap();
    let qualified_string = qualified.to_string();
    assert_eq!(qualified_string, "authority::name");
    let parsed: Name = qualified_string.parse().unwrap();
    assert_eq!(parsed, qualified);
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

/// A [`Name`] type used for efficient lookups in ordered collections.
///
/// This type's [`Ord`] implementation provides a stable ordering that is
/// efficient and does not rely on string comparisons. However, it does not sort
/// ascending, while `Name`'s [`Ord`] implementation sorts ascending.
///
/// There is no benefit to using this type in a hash-based collection, so this
/// type does not implement [`Hash`].
#[derive(Clone)]
pub enum NameKey<'a> {
    /// A borrowed name.
    Borrowed(&'a Name),
    /// An owned name.
    Owned(Name),
}

impl<'a> From<NameKey<'a>> for Name {
    fn from(name: NameKey<'a>) -> Self {
        match name {
            NameKey::Borrowed(name) => name.clone(),
            NameKey::Owned(name) => name,
        }
    }
}

impl<'a> Deref for NameKey<'a> {
    type Target = Name;

    fn deref(&self) -> &Self::Target {
        match self {
            NameKey::Borrowed(name) => name,
            NameKey::Owned(name) => name,
        }
    }
}

impl<'a> Eq for NameKey<'a> {}

impl<'a, 'b> PartialEq<NameKey<'b>> for NameKey<'a> {
    fn eq(&self, other: &NameKey<'b>) -> bool {
        **self == **other
    }
}

impl<'a> Ord for NameKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prioritize comparing the name, as in general the component names
        // shouldn't conflict.
        let a = &**self;
        let b = &**other;
        match a.name.as_ptr().cmp(&b.name.as_ptr()) {
            order @ (Ordering::Greater | Ordering::Less) => order,
            Ordering::Equal => a.authority.as_ptr().cmp(&b.authority.as_ptr()),
        }
    }
}

impl<'a, 'b> PartialOrd<NameKey<'b>> for NameKey<'a> {
    fn partial_cmp(&self, other: &NameKey<'b>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> From<&'a Name> for NameKey<'a> {
    fn from(value: &'a Name) -> Self {
        Self::Borrowed(value)
    }
}

impl From<Name> for NameKey<'_> {
    fn from(value: Name) -> Self {
        Self::Owned(value)
    }
}

impl Debug for NameKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl Display for NameKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&**self, f)
    }
}
