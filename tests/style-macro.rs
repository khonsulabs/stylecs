use stylecs::{style, Name, StyleComponent};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct FontSize(u32);

impl StyleComponent for FontSize {
    fn name() -> Name {
        Name::private("font_size").unwrap()
    }

    fn inherited() -> bool {
        true
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotInheritable;

impl StyleComponent for NotInheritable {
    fn name() -> Name {
        Name::private("not_inheritable").unwrap()
    }
}

#[test]
fn style_macro() {
    assert!(style![].is_empty());
    for component in &style![FontSize(1)] {
        assert_eq!(component.get::<FontSize>(), Some(&FontSize(1)));
    }
    for component in style![FontSize(1), NotInheritable] {
        if let Some(FontSize(size)) = component.get() {
            assert_eq!(*size, 1);
        } else {
            let Some(NotInheritable) = component.get() else { unreachable!("no other component types") };
        }
    }
}
