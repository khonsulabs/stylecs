use crate::style_sheet::{Rule, State};

// TODO test style store/retrieve/default
// TODO test fallback
// TODO test style merge
// TODO test classes merge
// TODO Test style evaluation order
// TODO test stylesheet merge

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
