use std::collections::HashMap;
use std::mem;

use crate::header::{Font, FontFamily, FontRef, FontTable, RTFHeader};
use crate::{ControlWord, Property, Token};

const ControlTableToken: Token<'static> = Token::ControlSymbol((ControlWord::FontTable, Property::None));

pub struct Painter {
    font_ref: FontRef,
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> Parser<'a> {

    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens }
    }

    pub fn check_document_validity(&self) {
        // Check the document boundaries
        assert_eq!(self.tokens.first().expect("Unable to retrieve first token"), &Token::OpeningBracket, "Invalid first token : not a {{");
        assert_eq!(self.tokens.last().expect("Unable to retrieve last token"), &Token::ClosingBracket, "Invalid last token : not a }}");
    }

    pub fn parse(&mut self) {
        self.parse_header();
        let mut painter_stack: Vec<Painter> = vec![];
        let mut it = self.tokens.iter();
        // while let Some(token) = it.next() {
        //     match token {
        //         Token::OpeningBracket => {}
        //         Token::ClosingBracket => {
        //             let _ = painter_stack.pop();
        //         }
        //         _ => {}
        //     }
        // }
    }

    fn get_next_token(&'a self) -> Option<&'a Token<'a>> {
        self.tokens.get(0)
    }

    fn consume_token(&mut self) -> Option<Token<'a>> {
        if self.tokens.is_empty() { return None; }
        Some(self.tokens.remove(0))
    }

    fn consume_token_until(&mut self, reference_token: Token<'a>) -> Vec<Token<'a>> {
        let mut ret = vec![];
        let token_type_id = mem::discriminant(&reference_token);
        while let Some(token) = self.consume_token() {
            let type_id = mem::discriminant(&token);
            ret.push(token);
            if type_id == token_type_id { break; }
        }
        return ret;
    }

    fn parse_header(&mut self) {
        let mut header = RTFHeader::default();
        while let (token, next_token) = (self.consume_token(), self.get_next_token()) {
            match (token, next_token) {
                (Some(Token::OpeningBracket), Some(&ControlTableToken)) => {
                    let font_table_tokens = self.consume_token_until(Token::ClosingBracket);
                    let font_table = Self::parse_font_table(&font_table_tokens);
                    dbg!(&font_table);
                },
                (None, None) => break,
                (a, b) => {},
            }
        }
    }

    fn parse_font_table(font_tables_tokens: &Vec<Token<'a>>) -> FontTable<'a> {
        assert_eq!(font_tables_tokens.get(0), Some(&ControlTableToken));
        let mut table = HashMap::new();
        let mut current_key = 0;
        let mut current_font = Font::default();
        for (index, token) in font_tables_tokens.iter().enumerate() {
            match token {
                Token::ControlSymbol((control_word, property)) => match control_word {
                    ControlWord::FontNumber => {
                        // Insert previous font
                        table.insert(current_key, current_font.clone());
                        if let Property::Value(key) = property { current_key = *key as FontRef; }
                        else { panic!("[Parser] Invalid font indentifier : {:?}", property) }
                    },
                    ControlWord::Unknown(name) => if let Some(font_family) = FontFamily::from(name) {
                        current_font.font_family = font_family;
                    }
                    _ => {},
                },
                Token::PlainText(name) => { current_font.name = name; },
                Token::ClosingBracket => { table.insert(current_key, current_font.clone()); }, // Insert previous font
                _ => {}
            }
        }
        return table;
    }


    pub fn to_text(&mut self) -> &'a str { "" }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::Property::*;
    use crate::Token::*;

    #[test]
    fn parser_simple_test() {
        let tokens = Lexer::scan(r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#);
        let parser = Parser::new(tokens).parse();

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

    #[test]
    fn parse_multiple_documents() {
        let documents = vec![
            r"{\rtf1\ansi\deff0 {\fonttbl {\f0 Courier;}{\f1 ProFontWindows;}}
                This line is font 0 which is courier\line
                \f1
                This line is font 1\line
                \f0
                This line is font 0 again\line
            }",

            r"{\rtf1\ansi\deff0 {\fonttbl {\f0 Courier;}{\f1 ProFontWindows;}}
            {\colortbl;\red0\green0\blue0;\red255\green0\blue0;\red255\green255\blue0;}
            This line is font 0 which is courier\line
            \f1
            This line is font 1\line
            \f0
            This line is font 0 again\line
            This line has a \cf2 red \cf1 word\line
            \highlight3 while this line has a \cf2 red \cf1 word and is highlighted in yellow\highlight0\line
            Finally, back to the default color.\line
            }",

            r"{\rtf1\ansi\deff0 {\fonttbl {\f0 Courier;}}
                \tqr\tx4320\tab Right tab\par\pard
                \tqc\tx4320\tab Center tab\par\pard
                \tx4320\tab Left tab
            }"
        ];
        for document in documents.iter() {
            let tokens = Lexer::scan(*document);
            Parser::new(tokens);
        }
    }

    // #[test]
    fn rtf_to_text() {
        let rtf = r#"{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard
             Voici du texte en {\b gras}.\par
             }"#;
        let tokens = Lexer::scan(rtf);
        let text = Parser::new(tokens).to_text();
        assert_eq!(text, "Voici du texte en gras.")
    }
}
