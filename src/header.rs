use std::collections::HashMap;

use crate::tokens::{ControlWord, Token};

pub type FontRef = u16;
pub type FontTable = HashMap<FontRef, Font>;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RtfHeader {
    pub character_set: CharacterSet,
    pub font_table: FontTable,
    pub stylesheet: StyleSheet,
}

#[derive(Hash, Default, Clone, Debug, PartialEq)]
pub struct Font {
    pub name: String,
    pub character_set: u8,
    pub font_family: FontFamily,
}

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
            // TODO: implement the rest
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StyleSheet;
