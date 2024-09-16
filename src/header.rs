use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::paragraph::Paragraph;
use crate::parser::Painter;
use crate::tokens::{ControlWord, Token};

/// The ColorRef represent the index of the color in the ColorTable
/// It's use in the document's body to reference a specific color with the \cfN or \cbN control words
pub type ColorRef = u16;
pub type ColorTable = HashMap<ColorRef, Color>;

/// The FontRef represent the index of the color in the FontTable
/// It's use in the document's body to reference a specific font with the \fN control word
pub type FontRef = u16;
pub type FontTable = HashMap<FontRef, Font>;

/// The StyleRef represent the index of the style in the StyleSheet
/// It's use in the document's body to reference a specific style with the \sN control word
pub type StyleRef = u16;
pub type StyleSheet = HashMap<StyleRef, Style>;

/// Style for the StyleSheet
#[derive(Hash, Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Style {
    /// The style attributes
    painter: Painter,
    /// The layout attributes
    paragraph: Paragraph,
}

/// Information about the document, including references to fonts & styles
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize, Tsify))]
#[cfg_attr(all(feature = "serde"), tsify(into_wasm_abi, from_wasm_abi))]
pub struct RtfHeader {
    pub character_set: CharacterSet,
    pub font_table: FontTable,
    pub color_table: ColorTable,
    pub stylesheet: StyleSheet,
}

#[derive(Hash, Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[wasm_bindgen(getter_with_clone)]
pub struct Font {
    pub name: String,
    pub character_set: u8,
    pub font_family: FontFamily,
}

#[derive(Hash, Default, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[wasm_bindgen]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Default, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize, Tsify))]
#[cfg_attr(all(feature = "serde", target_arch = "wasm32"), tsify(into_wasm_abi, from_wasm_abi))]
pub enum CharacterSet {
    #[default]
    Ansi,
    Mac,
    Pc,
    Pca,
    Ansicpg(u16),
}

impl CharacterSet {
    pub fn from(token: &Token) -> Option<Self> {
        match token {
            Token::ControlSymbol((ControlWord::Ansi, _)) => Some(Self::Ansi),
            // TODO: implement the rest
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Hash, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize, Tsify))]
#[cfg_attr(all(feature = "serde"), tsify(into_wasm_abi, from_wasm_abi))]
pub enum FontFamily {
    #[default]
    Nil,
    Roman,
    Swiss,
    Modern,
    Script,
    Decor,
    Tech,
    Bidi,
}

impl FontFamily {
    pub fn from(string: &str) -> Option<Self> {
        match string {
            r"\fnil" => Some(Self::Nil),
            r"\froman" => Some(Self::Roman),
            r"\fswiss" => Some(Self::Swiss),
            r"\fmodern" => Some(Self::Modern),
            r"\fscript" => Some(Self::Script),
            r"\fdecor" => Some(Self::Decor),
            r"\ftech" => Some(Self::Tech),
            r"\fbidi" => Some(Self::Bidi),
            _ => None,
        }
    }
}
