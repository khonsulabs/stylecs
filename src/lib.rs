use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use euclid::Scale;

mod alignment;
mod any;
mod colors;
mod font_family;
mod font_size;
mod font_style;
mod weight;
pub use self::{
    alignment::{Alignment, VerticalAlignment},
    any::AnyStyleComponent,
    colors::{BackgroundColor, ColorPair, ForegroundColor, SystemTheme},
    font_family::FontFamily,
    font_size::FontSize,
    font_style::FontStyle,
    weight::Weight,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Pixels;

#[derive(Debug, Clone, Copy, Default)]
pub struct Points;

#[derive(Debug)]
pub struct Style<Unit: 'static> {
    components: HashMap<TypeId, Box<dyn AnyStyleComponent<Unit>>>,
}

impl<Unit: Send + Sync> Clone for Style<Unit> {
    fn clone(&self) -> Self {
        let mut new_map = HashMap::<TypeId, Box<dyn AnyStyleComponent<Unit>>>::new();

        for (type_id, value) in self.components.iter() {
            new_map.insert(*type_id, value.clone_to_style_component());
        }

        Self {
            components: new_map,
        }
    }
}

impl<Unit> Default for Style<Unit> {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}

impl<Unit: Send + Sync + 'static> Style<Unit> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn push<T: StyleComponent<Unit> + Clone>(&mut self, component: T) {
        self.components
            .insert(component.type_id(), Box::new(component));
    }

    pub fn with<T: StyleComponent<Unit> + Clone>(mut self, component: T) -> Self {
        self.push(component);
        self
    }

    pub fn get<T: StyleComponent<Unit>>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        self.components
            .get(&type_id)
            .map(|w| {
                let component_as_any = w.as_any();
                component_as_any.downcast_ref::<T>()
            })
            .flatten()
    }

    pub fn get_or_default<T: StyleComponent<Unit> + Default + Clone>(&self) -> T {
        self.get::<T>().cloned().unwrap_or_default()
    }
}

pub struct ComponentCollection<Unit: 'static> {
    map: HashMap<TypeId, Box<dyn StyleComponent<Unit>>>,
}

impl<Unit> ComponentCollection<Unit> {
    pub fn push<T: StyleComponent<Unit>>(&mut self, component: T) {
        self.map.insert(component.type_id(), Box::new(component));
    }
}

pub trait StyleComponent<Unit>: std::any::Any + Send + Sync + Debug + 'static {
    fn scale(&self, scale: Scale<f32, Unit, Pixels>, destination: &mut Style<Pixels>);

    fn should_be_inherited(&self) -> bool {
        true
    }
}

pub trait UnscaledStyleComponent<Unit>:
    AnyStyleComponent<Unit> + Clone + Send + Sync + Debug + 'static
{
    fn unscaled_should_be_inherited(&self) -> bool {
        true
    }
}

impl<T> StyleComponent<Points> for T
where
    T: UnscaledStyleComponent<Points>,
{
    fn scale(&self, _scale: Scale<f32, Points, Pixels>, destination: &mut Style<Pixels>) {
        destination.push(self.clone());
    }

    fn should_be_inherited(&self) -> bool {
        self.unscaled_should_be_inherited()
    }
}

impl<T> StyleComponent<Pixels> for T
where
    T: StyleComponent<Points> + Clone,
{
    fn scale(&self, _scale: Scale<f32, Pixels, Pixels>, destination: &mut Style<Pixels>) {
        destination.push(self.clone());
    }
}

impl<Unit: Send + Sync + 'static> Style<Unit> {
    pub fn merge_with(&self, other: &Style<Unit>, is_inheritance: bool) -> Self {
        let mut merged_components = HashMap::<TypeId, Box<dyn AnyStyleComponent<Unit>>>::new();
        let self_types = self.components.keys().cloned().collect::<HashSet<_>>();
        let parent_types = other.components.keys().cloned().collect::<HashSet<_>>();

        for type_id in self_types.union(&parent_types) {
            let value = if self_types.contains(type_id) {
                self.components.get(type_id).unwrap()
            } else {
                let value = other.components.get(type_id).unwrap();
                if is_inheritance && !value.should_be_inherited() {
                    continue;
                }
                value
            };
            merged_components.insert(*type_id, value.clone_to_style_component());
        }
        Self {
            components: merged_components,
        }
    }
}

impl Style<Points> {
    pub fn to_screen_scale(&self, scale: Scale<f32, Points, Pixels>) -> Style<Pixels> {
        let mut style = Style::new();

        for component in self.components.values() {
            component.scale(scale, &mut style);
        }

        style
    }
}

#[derive(Default, Clone, Debug)]
pub struct StyleSheet {
    pub normal: Style<Points>,
    pub hover: Style<Points>,
    pub focus: Style<Points>,
    pub active: Style<Points>,
}

impl From<Style<Points>> for StyleSheet {
    fn from(style: Style<Points>) -> Self {
        Self {
            normal: style.clone(),
            active: style.clone(),
            hover: style.clone(),
            focus: style,
        }
    }
}

impl StyleSheet {
    pub fn merge_with(&self, other: &StyleSheet, is_inheritance: bool) -> Self {
        Self {
            normal: self.normal.merge_with(&other.normal, is_inheritance),
            active: self.active.merge_with(&other.active, is_inheritance),
            hover: self.hover.merge_with(&other.hover, is_inheritance),
            focus: self.focus.merge_with(&other.focus, is_inheritance),
        }
    }

    pub fn map_each<F: Fn(&Style<Points>) -> Style<Points>>(&self, map: F) -> Self {
        Self {
            normal: map(&self.normal),
            active: map(&self.active),
            hover: map(&self.hover),
            focus: map(&self.focus),
        }
    }
}

pub enum GenericStyle<'a> {
    Points(&'a Style<Points>),
    Pixels(&'a Style<Pixels>),
}

impl<'a> GenericStyle<'a> {
    pub fn get<T: StyleComponent<Pixels> + StyleComponent<Points>>(&'a self) -> Option<&'a T> {
        match self {
            Self::Points(style) => style.get::<T>(),
            Self::Pixels(style) => style.get::<T>(),
        }
    }
}
