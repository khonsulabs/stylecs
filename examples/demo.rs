use stylecs::{style, Style, StyleComponent};

/// FontSize is a [`StyleComponent`] that is inheritable when doing a merge
/// operation.
#[derive(StyleComponent, Debug, Clone, Copy, Eq, PartialEq)]
#[style(inherited = true)]
pub struct FontSize(u32);

#[derive(StyleComponent, Debug, Clone, Copy, Eq, PartialEq)]
#[style(inherited = true)]
pub struct TextColor(u32);

/// Padding is not inheritable, but it has custom merge behavior.
#[derive(StyleComponent, Default, Debug, Clone, Copy, Eq, PartialEq)]
#[style(name = padding, authority = gooey, merge = self.merge(other))]
pub struct Padding {
    pub left: Option<u32>,
    pub right: Option<u32>,
    pub top: Option<u32>,
    pub bottom: Option<u32>,
}

fn main() {
    // Let's start with the style that might be applied to a body tag.
    let body = style!(FontSize(12), Padding::uniform(5), TextColor(0xFF0000));

    // Let's make a style that we will apply to all heading tags.
    let heading = style!(Padding::vertical(10));

    // Now, we'll construct the style for a heading that is inside of the body.
    // For demonstration reasons, we're assigning some horizontal padding at the
    // base style. This also shows how the builder-style pattern for `Style` can
    // be used instead of the `style!` macro.
    let h1 = Style::new()
        .with(FontSize(18))
        .with(Padding::horizontal(10));
    println!("Base h1: {h1:?}");
    // Next, we'll apply the heading styles.
    let h1 = h1.merged_with(&heading);
    // The padding component is now merged together:
    println!("Merged with heading: {h1:?}");
    assert_eq!(h1.get::<Padding>(), Some(&Padding::uniform(10)));
    // Next, we'll inherit any inheritable values from the body tag.
    let h1 = h1.inherited_from(&body);
    println!("Inherited from body: {h1:?}");
    // Despite being inheritable, the FontSize value from the body is ignored.
    assert_eq!(h1.get::<FontSize>(), Some(&FontSize(18)));
    // However, the heading now has a TextColor component inherited from the
    // body.
    assert_eq!(h1.get::<TextColor>(), Some(&TextColor(0xFF0000)))
}

impl Padding {
    fn uniform(measurement: u32) -> Self {
        Self {
            left: Some(measurement),
            right: Some(measurement),
            top: Some(measurement),
            bottom: Some(measurement),
        }
    }

    fn vertical(measurement: u32) -> Self {
        Self {
            top: Some(measurement),
            bottom: Some(measurement),
            ..Self::default()
        }
    }

    fn horizontal(measurement: u32) -> Self {
        Self {
            left: Some(measurement),
            right: Some(measurement),
            ..Self::default()
        }
    }

    fn merge(&mut self, other: &Self) {
        self.left = self.left.or(other.left);
        self.right = self.right.or(other.right);
        self.top = self.top.or(other.top);
        self.bottom = self.bottom.or(other.bottom);
    }
}

#[test]
fn runs() {
    main();
}
