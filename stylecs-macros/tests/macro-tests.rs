use stylecs::{Identifier, StyleComponent};

#[derive(StyleComponent, Debug, Clone)]
struct Inheritable;

#[derive(StyleComponent, Debug, Clone)]
#[style(inherited = false)]
struct NotInheritable;

#[derive(StyleComponent, Debug, Eq, PartialEq, Clone)]
#[style(name = additive, authority = gooey, inherited = false, merge = Self(self.0 + other.0))]
struct AdditiveMerge(u32);

#[test]
fn defined_correctly() {
    assert_eq!(Inheritable::name().name, "inheritable");
    assert_eq!(Inheritable::name().authority, Identifier::private());
    assert!(Inheritable::inherited());
    assert!(!NotInheritable::inherited());
    assert_eq!(NotInheritable::name().name, "not_inheritable");
    let mut mergable = AdditiveMerge(1);
    mergable.merge(&AdditiveMerge(2));
    assert_eq!(mergable, AdditiveMerge(3));
    assert_eq!(AdditiveMerge::name().name, "additive");
    assert_eq!(AdditiveMerge::name().authority, "gooey");
}
