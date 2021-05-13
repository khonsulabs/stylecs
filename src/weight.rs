use crate::{Points, UnscaledStyleComponent};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Weight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    Other(u16),
}

impl Default for Weight {
    fn default() -> Self {
        Self::Normal
    }
}

impl UnscaledStyleComponent<Points> for Weight {}

impl Weight {
    pub fn to_number(self) -> u16 {
        match self {
            Self::Thin => 100,
            Self::ExtraLight => 200,
            Self::Light => 300,
            Self::Normal => 400,
            Self::Medium => 500,
            Self::SemiBold => 600,
            Self::Bold => 700,
            Self::ExtraBold => 800,
            Self::Black => 900,
            Self::Other(n) => n,
        }
    }
}

// impl From<ttf_parser::Weight> for Weight {
//     fn from(weight: ttf_parser::Weight) -> Self {
//         match weight {
//             ttf_parser::Weight::Thin => Weight::Thin,
//             ttf_parser::Weight::ExtraLight => Weight::ExtraLight,
//             ttf_parser::Weight::Light => Weight::Light,
//             ttf_parser::Weight::Normal => Weight::Normal,
//             ttf_parser::Weight::Medium => Weight::Medium,
//             ttf_parser::Weight::SemiBold => Weight::SemiBold,
//             ttf_parser::Weight::Bold => Weight::Bold,
//             ttf_parser::Weight::ExtraBold => Weight::ExtraBold,
//             ttf_parser::Weight::Black => Weight::Black,
//             ttf_parser::Weight::Other(value) => Weight::Other(value),
//         }
//     }
// }

// impl From<Weight> for ttf_parser::Weight {
//     fn from(weight: Weight) -> Self {
//         match weight {
//             Weight::Thin => Self::Thin,
//             Weight::ExtraLight => Self::ExtraLight,
//             Weight::Light => Self::Light,
//             Weight::Normal => Self::Normal,
//             Weight::Medium => Self::Medium,
//             Weight::SemiBold => Self::SemiBold,
//             Weight::Bold => Self::Bold,
//             Weight::ExtraBold => Self::ExtraBold,
//             Weight::Black => Self::Black,
//             Weight::Other(value) => Self::Other(value),
//         }
//     }
// }
