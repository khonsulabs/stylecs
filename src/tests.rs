use crate::{Name, Style, StyleComponent};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct FontSize(u32);

impl StyleComponent for FontSize {
    fn inherited() -> bool {
        true
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotInheritable;

impl StyleComponent for NotInheritable {}

#[test]
fn basics() {
    let a = Style::new().with(FontSize(1)).with(NotInheritable);
    let b = Style::new().with(FontSize(2));
    let ab = a.clone().merged_with(&b);
    assert_eq!(ab.get::<FontSize>(), Some(&FontSize(1)));
    assert_eq!(ab.get::<NotInheritable>(), Some(&NotInheritable));
    let ba = b.merged_with(&a);
    assert_eq!(ba.get::<FontSize>(), Some(&FontSize(2)));
    let inherited = Style::new().inherited_from(&a);
    assert_eq!(inherited.get::<FontSize>(), Some(&FontSize(1)));
    assert!(inherited.get::<NotInheritable>().is_none());

    assert_eq!(
        Style::new().get_or_default::<FontSize>(),
        FontSize::default()
    );
}

#[test]
fn debug() {
    let debugged = format!("{:?}", Style::new().with(FontSize(1)));
    assert_eq!(debugged, "Style(FontSize(1))");
}

#[test]
fn names() {
    assert_eq!(FontSize::name(), Name::private("font_size").unwrap());
    assert_eq!(
        NotInheritable::name(),
        Name::private("not_inheritable").unwrap()
    );
}
