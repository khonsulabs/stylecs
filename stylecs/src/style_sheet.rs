use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use crate::{Style, StyleComponent};

/// A set of style [`Rule`]s to apply to a program.
#[derive(Default, Debug)]
pub struct StyleSheet {
    rules: Vec<Rule>,

    rules_by_id: HashMap<String, Vec<usize>>,
    rules_by_class: HashMap<String, Vec<usize>>,
}

impl StyleSheet {
    /// Uses any [`Id`] and [`Classes`] components present in `style` to apply
    /// style rules. The result will prefer the components specified in `style`,
    /// but any components not specified will be provided by rules that match
    /// the id or classes provided.
    #[must_use]
    pub fn effective_style_for(&self, mut style: Style, state: &State) -> Style {
        let mut rules = HashSet::new();
        if let Some(id) = style.get::<Id>() {
            if let Some(id_rules) = self.rules_by_id.get(id.0.as_ref()) {
                rules.extend(id_rules.iter().cloned());
            }
        }
        if let Some(classes) = style.get::<Classes>() {
            for class in &classes.0 {
                if let Some(class_rules) = self.rules_by_class.get(class.as_ref()) {
                    rules.extend(class_rules.iter().cloned());
                }
            }
        }

        let mut rules = rules.into_iter().collect::<Vec<_>>();
        rules.sort_unstable();
        for rule in rules.into_iter().rev() {
            let rule = &self.rules[rule];
            if rule.applies(state) {
                style = style.merge_with(&rule.style, false);
            }
        }

        style
    }

    /// Pushes `rule` and returns self. Builder-style implementation of
    /// [`Self::push()`].
    #[must_use]
    pub fn with(mut self, rule: Rule) -> Self {
        self.push(rule);
        self
    }

    /// Pushes `rule` into the collection. Rules pushed later will have
    /// higher priority than rules that are pushed later.
    pub fn push(&mut self, rule: Rule) {
        let index = self.rules.len();
        match &rule.selector {
            Selector::Id(id) => {
                let rules = self.rules_by_id.entry(id.0.to_string()).or_default();
                rules.push(index);
            }
            Selector::Classes(classes) =>
                for class in &classes.0 {
                    let rules = self.rules_by_class.entry(class.to_string()).or_default();
                    rules.push(index);
                },
        }
        self.rules.push(rule);
    }

    /// Merges `self` with `other`, such that the rules in `self` are preferred
    /// to the ones in `other`.
    #[must_use]
    pub fn merge_with(&self, other: &Self) -> Self {
        let mut combined = Self {
            rules: Vec::with_capacity(self.rules.len() + other.rules.len()),
            rules_by_class: other.rules_by_class.clone(),
            rules_by_id: other.rules_by_id.clone(),
        };
        combined.rules.extend(other.rules.iter().cloned());
        let rule_offset = other.rules.len();
        for (key, index) in &self.rules_by_id {
            let id_rules = combined.rules_by_id.entry(key.clone()).or_default();
            id_rules.extend(index.iter().map(|&i| i + rule_offset));
        }
        for (key, index) in &self.rules_by_class {
            let class_rules = combined.rules_by_class.entry(key.clone()).or_default();
            class_rules.extend(index.iter().map(|&i| i + rule_offset));
        }

        combined
    }
}

/// A style rule.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Selecting styles by [`Id`] or [`Classes`].
    pub selector: Selector,
    /// If specified, only applies `style` if `hovered` matches
    /// [`State::hovered`].
    pub hovered: Option<bool>,
    /// If specified, only applies `style` if `focused` matches
    /// [`State::focused`].
    pub focused: Option<bool>,
    /// If specified, only applies `style` if `active` matches
    /// [`State::active`].
    pub active: Option<bool>,
    /// The style to apply if the criteria are met.
    pub style: Style,
}

impl Rule {
    /// Returns a default `Rule` with `selector` of [`Id`] `id`.
    #[must_use]
    pub fn for_id<I: Into<Id>>(id: I) -> Self {
        Self {
            selector: Selector::Id(id.into()),
            hovered: None,
            focused: None,
            active: None,
            style: Style::default(),
        }
    }

    /// Returns a default `Rule` with `selector` of [`Classes`] `classes`.
    #[must_use]
    pub fn for_classes<C: Into<Classes>>(classes: C) -> Self {
        Self {
            selector: Selector::Classes(classes.into()),
            hovered: None,
            focused: None,
            active: None,
            style: Style::default(),
        }
    }

    /// Builder-style function that sets [`Self::hovered`] to `Some(true)`.
    #[must_use]
    pub const fn when_hovered(mut self) -> Self {
        self.hovered = Some(true);
        self
    }

    /// Builder-style function that sets [`Self::hovered`] to `Some(false)`.
    #[must_use]
    pub const fn when_not_hovered(mut self) -> Self {
        self.hovered = Some(false);
        self
    }

    /// Builder-style function that sets [`Self::focused`] to `Some(true)`.
    #[must_use]
    pub const fn when_focused(mut self) -> Self {
        self.focused = Some(true);
        self
    }

    /// Builder-style function that sets [`Self::focused`] to `Some(false)`.
    #[must_use]
    pub const fn when_not_focused(mut self) -> Self {
        self.focused = Some(false);
        self
    }

    /// Builder-style function that sets [`Self::active`] to `Some(true)`.
    #[must_use]
    pub const fn when_active(mut self) -> Self {
        self.active = Some(true);
        self
    }

    /// Builder-style function that sets [`Self::active`] to `Some(false)`.
    #[must_use]
    pub const fn when_not_active(mut self) -> Self {
        self.active = Some(false);
        self
    }

    /// Builder-style function that passes the current value of [`Self::style`]
    /// into `initializer` and stores the result back into [`Self::style`].
    #[must_use]
    pub fn with_styles<F: FnOnce(Style) -> Style>(mut self, initializer: F) -> Self {
        self.style = initializer(self.style);
        self
    }

    /// Returns true if the rule should apply based on `state`.
    #[must_use]
    pub fn applies(&self, state: &State) -> bool {
        check_one_state(self.hovered, state.hovered)
            .or_else(|| check_one_state(self.focused, state.focused))
            .or_else(|| check_one_state(self.active, state.active))
            .unwrap_or(true)
    }
}

fn check_one_state(condition: Option<bool>, state: bool) -> Option<bool> {
    condition.map(|condition| condition == state)
}

/// A filter for a [`Rule`].
#[derive(Debug, Clone)]
pub enum Selector {
    /// Matches when a [`Style`] has an [`Id`] component that equals the
    /// contained value.
    Id(Id),

    /// Matches when a [`Style`] has a [`Classes`] component that contains all
    /// of the classes in the contianed value.
    Classes(Classes),
}

/// A unique Id. Not inherited when merging styles.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Id(pub Cow<'static, str>);

impl StyleComponent for Id {
    fn should_be_inherited(&self) -> bool {
        false
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&'static str> for Id {
    fn from(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

/// A list of class names. Not inherited when merging styles.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Classes(pub Vec<Cow<'static, str>>);

impl StyleComponent for Classes {
    fn should_be_inherited(&self) -> bool {
        false
    }

    fn merge(&self, other: &Self) -> Self
    where
        Self: Clone,
    {
        let mut merged = self.0.iter().cloned().collect::<HashSet<_>>();
        let missing = other
            .0
            .iter()
            .filter(|other| !merged.contains(other.as_ref()))
            .cloned()
            .collect::<Vec<_>>();
        merged.extend(missing);
        Self(merged.into_iter().collect())
    }
}

impl From<String> for Classes {
    fn from(s: String) -> Self {
        Self(vec![Cow::Owned(s)])
    }
}

impl From<&'static str> for Classes {
    fn from(s: &'static str) -> Self {
        Self(vec![Cow::Borrowed(s)])
    }
}

impl From<Vec<String>> for Classes {
    fn from(s: Vec<String>) -> Self {
        Self(s.into_iter().map(Cow::Owned).collect())
    }
}

impl From<Vec<&'static str>> for Classes {
    fn from(s: Vec<&'static str>) -> Self {
        Self(s.into_iter().map(|s| Cow::Borrowed(s)).collect())
    }
}

/// An element state.
#[derive(Default, Debug)]
pub struct State {
    /// Whether the element is hovered or not.
    pub hovered: bool,
    /// Whether the element is focused or not.
    pub focused: bool,
    /// Whether the element is active or not. For example, a push button
    /// actively being depressed.
    pub active: bool,
}
