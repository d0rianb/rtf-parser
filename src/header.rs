use std::collections::HashMap;
use crate::{ControlWord, Token};

pub type FontRef = u16;
pub type FontTable<'a> = HashMap<FontRef, Font<'a>>;

#[derive(Default, Debug, PartialEq)]
pub struct RtfHeader<'a> {
    pub character_set: CharacterSet,
    pub font_table: FontTable<'a>,
}

#[derive(Hash, Default, Clone, Debug, PartialEq)]
pub struct Font<'a> {
    pub name: &'a str,
    pub character_set: u8,
    pub font_family: FontFamily,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum CharacterSet {
    Ansi,
    Mac,
    Pc,
    Pca,
    Ansicpg(u16),
}

impl Default for CharacterSet {
    fn default() -> Self { CharacterSet::Ansi }
}

impl CharacterSet {
    pub fn from(token: &Token) -> Option<Self> {
        match token {
            Token::ControlSymbol((ControlWord::Ansi, _)) => Some(Self::Ansi),
            // TODO: implement the rest
            _ => None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Hash, Clone)]
pub enum FontFamily {
    Nil,
    Roman,
    Swiss,
    Modern,
    Script,
    Decor,
    Tech,
    Bidi,
}

impl Default for FontFamily {
    fn default() -> Self { FontFamily::Nil }
}

impl FontFamily {
    pub fn from(string: &str) -> Option<Self> {
        match string {
            r"\fnil" => Some(Self::Nil),
            r"\froman" => Some(Self::Roman),
            r"\fswiss" => Some(Self::Swiss),
            // TODO: implement the rest
            _ => None
        }
    }
}

