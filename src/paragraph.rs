/// Define the paragraph related structs and enums

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::tokens::ControlWord;

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[wasm_bindgen]
pub struct Paragraph {
    pub alignment: Alignment,
    pub spacing: Spacing,
    pub indent: Indentation,
    pub tab_width: i32,
}

/// Alignement of a paragraph (left, right, center, justify)
#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize, Tsify))]
#[cfg_attr(all(feature = "serde"), tsify(into_wasm_abi, from_wasm_abi))]
pub enum Alignment {
    #[default]
    LeftAligned, // \ql
    RightAligned, // \qr
    Center,       // \qc
    Justify,      // \qj
}

impl From<&ControlWord<'_>> for Alignment {
    fn from(cw: &ControlWord) -> Self {
        return match cw {
            ControlWord::LeftAligned  => Alignment::LeftAligned,
            ControlWord::RightAligned => Alignment::RightAligned,
            ControlWord::Center       => Alignment::Center,
            ControlWord::Justify      => Alignment::Justify,
            _  /* default */          => Alignment::LeftAligned,
        };
    }
}

/// The vertical margin before / after a block of text
#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[wasm_bindgen]
pub struct Spacing {
    pub before: i32,
    pub after: i32,
    pub between_line: SpaceBetweenLine,
    pub line_multiplier: i32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize, Tsify))]
#[cfg_attr(all(feature = "serde"), tsify(into_wasm_abi, from_wasm_abi))]
pub enum SpaceBetweenLine {
    Value(i32),
    #[default]
    Auto,
    Invalid,
}

/// Space between lines.
// If this control word is missing or if \sl1000 is used, the line spacing is automatically determined by the tallest character in the line;
// if N is a positive value, this size is used only if it is taller than the tallest character (otherwise, the tallest character is used);
// if N is a negative value, the absolute value of N is used, even if it is shorter than the tallest character.
impl From<i32> for SpaceBetweenLine {
    fn from(value: i32) -> Self {
        return match value {
            1000 => SpaceBetweenLine::Auto,
            val if val < 0 => SpaceBetweenLine::Value(val.abs()),
            val => SpaceBetweenLine::Value(val),
        };
    }
}

// This struct can not be an enum because left-indent and right-ident can both be defined at the same time
#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[wasm_bindgen]
pub struct Indentation {
    pub left: i32,
    pub right: i32,
    pub first_line: i32,
}
