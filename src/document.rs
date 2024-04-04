use crate::header::RtfHeader;
use crate::parser::StyleBlock;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

#[cfg(feature="serde_support")]
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct RtfDocument {
    pub header: RtfHeader,
    pub body: Vec<StyleBlock>,
}

#[cfg(not(feature="serde_support"))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RtfDocument {
    pub header: RtfHeader,
    pub body: Vec<StyleBlock>,
}

// impl<'a> From<String> for RtfDocument<'a> {
//     // Create a RTF document from file content
//     fn from(file_content: String) -> Self {
//         let tokens = Lexer::scan(&file_content);
//         let document = Parser::new(tokens).parse();
//         return document;
//     }
// }
//
// impl<'a> From<&'a str> for RtfDocument<'a> {
//     // Create a RTF document from file content
//     fn from(file_content: &str) -> Self {
//         let tokens = Lexer::scan(file_content);
//         let document =  Parser::new(tokens).parse();
//         return document;
//     }
// }

impl RtfDocument {
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
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_get_text() {
        let rtf = r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#;
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(document.get_text(), "Voici du texte en gras.")
    }
}
