use std::error::Error;
use std::fs;
use std::io::Read;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::header::RtfHeader;
use crate::lexer::Lexer;
use crate::parser::{Parser, StyleBlock};

// Interface to WASM to be used in JS
#[wasm_bindgen]
pub fn parse_rtf(rtf: String) -> RtfDocument {
    return RtfDocument::try_from(rtf).unwrap()
}

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[wasm_bindgen(getter_with_clone)]
pub struct RtfDocument {
    pub header: RtfHeader,
    pub body: Vec<StyleBlock>,
}

// Create a RTF document from a String content
impl TryFrom<String> for RtfDocument {
    type Error = Box<dyn Error>;
    fn try_from(file_content: String) -> Result<Self, Self::Error> {
        let tokens = Lexer::scan(file_content.as_str())?;
        let document = Parser::new(tokens).parse()?;
        return Ok(document);
    }
}

// Create a RTF document from file content
impl TryFrom<&str> for RtfDocument {
    type Error = Box<dyn Error>;
    fn try_from(file_content: &str) -> Result<Self, Self::Error> {
        let tokens = Lexer::scan(file_content)?;
        let document = Parser::new(tokens).parse()?;
        return Ok(document);
    }
}

// Create an RTF document from a file
impl TryFrom<&mut fs::File> for RtfDocument {
    type Error = Box<dyn Error>;
    fn try_from(file: &mut fs::File) -> Result<Self, Self::Error> {
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        return Self::try_from(file_content);
    }
}

impl RtfDocument {
    /// Create an `RtfDocument` from a rtf file path
    pub fn from_filepath(filename: &str) -> Result<RtfDocument, Box<dyn Error>> {
        let file_content = fs::read_to_string(filename)?;
        return Self::try_from(file_content);
    }

    /// Get the raw text of an RTF document
    pub fn get_text(&self) -> String {
        let mut result = String::new();
        for style_block in &self.body {
            result.push_str(&style_block.text);
        }
        return result;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::document::RtfDocument;

    #[test]
    fn get_text_from_document() {
        let rtf = r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#;
        let document = RtfDocument::try_from(rtf).unwrap();
        assert_eq!(document.get_text(), "Voici du texte en gras.")
    }

    #[test]
    fn create_document_from_file() {
        let mut file = fs::File::open("./resources/tests/test-file.rtf").unwrap();
        let document = RtfDocument::try_from(&mut file).unwrap();
        assert_eq!(document.header.font_table.get(&0).unwrap().name, String::from("Helvetica"));
    }

    #[test]
    fn create_document_from_filepath() {
        let filename = "./resources/tests/test-file.rtf";
        let document = RtfDocument::from_filepath(filename).unwrap();
        assert_eq!(document.header.font_table.get(&0).unwrap().name, String::from("Helvetica"));
    }
}
