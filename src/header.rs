use std::collections::HashMap;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

use crate::paragraph::Paragraph;
use crate::parser::Painter;
use crate::tokens::{ControlWord, Token};

pub type ColorRef = u16;
pub type ColorTable = HashMap<ColorRef, Color>;

pub type FontRef = u16;
pub type FontTable = HashMap<FontRef, Font>;

pub type StyleRef = u16;
pub type StyleSheet = HashMap<StyleRef, Style>;

/// Style for the StyleSheet
#[cfg(feature="serde_support")]
#[derive(Hash, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Style {
    painter: Painter,
    paragraph: Paragraph,
}
#[cfg(not(feature="serde_support"))]
#[derive(Hash, Default, Debug, Clone, PartialEq)]
pub struct Style {
    painter: Painter,
    paragraph: Paragraph,
}

/// Information about the document, including references to fonts & styles
#[cfg(feature="serde_support")]
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RtfHeader {
    pub character_set: CharacterSet,
    pub font_table: FontTable,
    pub color_table: ColorTable,
    pub stylesheet: StyleSheet,
}
#[cfg(not(feature="serde_support"))]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RtfHeader {
    pub character_set: CharacterSet,
    pub font_table: FontTable,
    pub color_table: ColorTable,
    pub stylesheet: StyleSheet,
}

#[cfg(feature="serde_support")]
#[derive(Hash, Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Font {
    pub name: String,
    pub character_set: u8,
    pub font_family: FontFamily,
}
#[cfg(not(feature="serde_support"))]
#[derive(Hash, Default, Clone, Debug, PartialEq)]
pub struct Font {
    pub name: String,
    pub character_set: u8,
    pub font_family: FontFamily,
}

#[cfg(feature="serde_support")]
#[derive(Hash, Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Color {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
#[cfg(not(feature="serde_support"))]
#[derive(Hash, Default, Clone, Debug, PartialEq)]
pub struct Color {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[cfg(feature="serde_support")]
#[allow(dead_code)]
#[derive(Debug, PartialEq, Default, Clone, Deserialize, Serialize)]
pub enum CharacterSet {
    #[default]
    Ansi,
    Mac,
    Pc,
    Pca,
    Ansicpg(u16),
}
#[cfg(not(feature="serde_support"))]
#[allow(dead_code)]
#[derive(Debug, PartialEq, Default, Clone)]
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

#[cfg(feature="serde_support")]
#[allow(dead_code)]
#[derive(Debug, PartialEq, Hash, Clone, Default, Deserialize, Serialize)]
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
#[cfg(not(feature="serde_support"))]
#[allow(dead_code)]
#[derive(Debug, PartialEq, Hash, Clone, Default)]
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
