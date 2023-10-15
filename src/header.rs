use std::collections::HashMap;

pub type FontRef = u16;
pub type FontTable<'a> = HashMap<FontRef, Font<'a>>;

#[derive(Default)]
pub struct RTFHeader<'a> {
    character_set: CharacterSet,
    font_table: FontTable<'a>,
}

#[derive(Hash, Default, Clone, Debug)]
pub struct Font<'a> {
    pub name: &'a str,
    pub character_set: u8,
    pub font_family: FontFamily,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum CharacterSet {
    Ansi,
    Mac,
    Pc,
    Pca,
    Ansicpg(u16),
}

impl Default for CharacterSet {
    fn default() -> Self { CharacterSet::Ansi }
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
            "fnil" => Some(Self::Nil),
            "froman" => Some(Self::Roman),
            "fswiss" => Some(Self::Swiss),
            // TODO: implement the rest
            _ => None
        }
    }
}

