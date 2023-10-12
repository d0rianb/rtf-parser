use crate::Token;
use std::collections::HashMap;

type FontRef<'a> = &'a str;

#[derive(Hash)]
pub struct Font<'a> {
    character_set: u8,
    name: &'a str,
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
    fn default() -> Self {
        CharacterSet::Ansi
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum FontFamily {
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
    fn default() -> Self {
        FontFamily::Nil
    }
}

pub struct RTFHeader<'a> {
    character_set: CharacterSet,
    font_table: HashMap<FontRef<'a>, Font<'a>>,
    font_family: FontFamily,
}

pub struct Painter<'a> {
    font_ref: FontRef<'a>,
}

pub struct Parser;

impl<'a> Parser {
    pub fn check_document_validity(tokens: &Vec<Token>) {
        // Check the document boundaries
        assert_eq!(
            tokens.first().expect("Unable to retrieve first token"),
            &Token::OpeningBracket,
            "Invalid first token : not a {{"
        );
        assert_eq!(
            tokens.last().expect("Unable to retrieve last token"),
            &Token::ClosingBracket,
            "Invalid last token : not a }}"
        );
    }

    pub fn parse(tokens: &Vec<Token>) {
        let mut painter_stack: Vec<Painter> = vec![];
        let mut it = tokens.iter();
        while let Some(token) = it.next() {
            match token {
                Token::OpeningBracket => {}
                Token::ClosingBracket => {
                    let _ = painter_stack.pop();
                }
                _ => {}
            }
        }
    }

    pub fn parse_header(tokens: &Vec<Token>) {}

    pub fn to_text(_tokens: &Vec<Token>) -> &'a str {
        ""
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::ControlWord::{Ansi, Bold, FontNumber, Rtf, Unknown};
    use crate::Property::*;
    use crate::Token::*;

    #[test]
    fn parser_simple_test() {
        let tokens = Lexer::scan(
            r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#,
        );
        let parser = Parser::parse(&tokens);

        // assert_eq!(
        //     tokens,
        //     vec![
        //         OpeningBracket,
        //         ControlSymbol((Rtf, Value(1))),
        //         ControlSymbol((Ansi, None)),
        //         OpeningBracket,
        //         ControlSymbol((Unknown("\\fonttbl"), None)),
        //         ControlSymbol((FontNumber, Value(0))),
        //         ControlSymbol((Unknown("\\fswiss"), None)),
        //         PlainText("Helvetica;"),
        //         ClosingBracket,
        //         ControlSymbol((FontNumber, Value(0))),
        //         ControlSymbol((Unknown("\\pard"), None)),
        //         PlainText("Voici du texte en "),
        //         OpeningBracket,
        //         ControlSymbol((Bold, None)),
        //         PlainText("gras"),
        //         ClosingBracket,
        //         PlainText("."),
        //         ControlSymbol((Unknown("\\par"), None)),
        //         ClosingBracket
        //     ]
        // );
    }

    // #[test]
    fn rtf_to_text() {
        let rtf = r#"{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard
             Voici du texte en {\b gras}.\par
             }"#;
        let tokens = Lexer::scan(rtf);
        let text = Parser::to_text(&tokens);
        assert_eq!(text, "Voici du texte en gras.")
    }
}
