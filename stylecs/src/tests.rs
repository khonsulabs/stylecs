use crate::{
    style_sheet::{Classes, Rule, State},
    Style, StyleComponent,
};

// TODO test style store/retrieve/default
// TODO test fallback
// TODO test style merge
// TODO Test style evaluation order
// TODO test stylesheet merge

#[test]
fn classes_merge_test() {
    assert_eq!(
        Classes::from(vec!["a", "b", "c"]).merge(&Classes::from(vec!["c", "b", "a"])),
        Classes::from(vec!["a", "b", "c"])
    );
    assert_eq!(
        Classes::from(vec!["a", "c", "d", "e"]).merge(&Classes::from(vec!["b", "d", "f"])),
        Classes::from(vec!["a", "b", "c", "d", "e", "f"])
    );
}

#[test]
fn style_merge_test() {
    let original = Style::default().with(Classes::from("a"));
    let b_style = Style::default().with(Classes::from("b"));

    let merged = original.merge_with(&b_style, false);
    assert_eq!(
        merged.get::<Classes>().expect("no classes"),
        &Classes::from(vec!["a", "b"])
    );

    let merged = original.merge_with(&b_style, true);
    assert_eq!(
        merged.get::<Classes>().expect("no classes"),
        &Classes::from(vec!["a"])
    );
}

#[test]
fn rule_applies_tests() {
    let only_hovered = State {
        hovered: true,
        ..State::default()
    };
    let only_focused = State {
        focused: true,
        ..State::default()
    };
    let only_active = State {
        active: true,
        ..State::default()
    };

    assert!(Rule::for_id("a").when_hovered().applies(&only_hovered));
    assert!(!Rule::for_id("a").when_hovered().applies(&State::default()));
    assert!(Rule::for_id("a")
        .when_not_hovered()
        .applies(&State::default()));

    assert!(Rule::for_id("a").when_focused().applies(&only_focused));
    assert!(!Rule::for_id("a").when_focused().applies(&State::default()));
    assert!(Rule::for_id("a")
        .when_not_focused()
        .applies(&State::default()));

    assert!(Rule::for_id("a").when_active().applies(&only_active));
    assert!(!Rule::for_id("a").when_active().applies(&State::default()));
    assert!(Rule::for_id("a")
        .when_not_active()
        .applies(&State::default()));

    assert!(Rule::for_id("a").applies(&State::default()));
    assert!(Rule::for_id("a")
        .when_not_active()
        .applies(&State::default()));
    assert!(Rule::for_id("a").applies(&only_hovered));
}
