use std::collections::HashMap;
use std::mem;

use crate::header::{CharacterSet, Font, FontFamily, FontRef, FontTable, RtfHeader};
use crate::tokens::{ControlWord, Property, Token};

const CONTROL_TABLE_TOKEN: Token<'static> = Token::ControlSymbol((ControlWord::FontTable, Property::None));

#[derive(Debug, Default)]
pub struct RtfDocument<'a> {
    pub header: RtfHeader<'a>,
    pub body: Vec<StyleBlock<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct StyleBlock<'a> {
    pub painter: Painter,
    pub text: &'a str,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Painter {
    font_ref: FontRef,
    font_size: u16,
    bold: bool,
    italic: bool,
    underline: bool,
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn check_document_validity(&self) {
        // Check the document boundaries
        assert_eq!(self.tokens.first().expect("Unable to retrieve first token"), &Token::OpeningBracket, "Invalid first token : not a {{");
        assert_eq!(self.tokens.last().expect("Unable to retrieve last token"), &Token::ClosingBracket, "Invalid last token : not a }}");
    }

    pub fn parse(&mut self) -> RtfDocument<'a> {
        self.check_document_validity();
        let mut document = RtfDocument::default();
        self.parse_ignore_groups();
        document.header = self.parse_header();
        // Parse Body
        let mut painter_stack: Vec<Painter> = vec![Painter::default()];
        let mut it = self.tokens.iter();
        while let Some(token) = it.next() {
            match token {
                Token::OpeningBracket => {
                    painter_stack.push(Painter::default());
                }
                Token::ClosingBracket => {
                    let _ = painter_stack.pop().expect("[Parser] : Empty painter stack : Too many closing brackets");
                }
                Token::ControlSymbol((control_word, property)) => {
                    let mut current_painter = painter_stack.last_mut().expect("[Parser] : Malformed painter stack");
                    match control_word {
                        ControlWord::FontNumber => current_painter.font_ref = property.get_value() as u16,
                        ControlWord::Bold => {
                            current_painter.bold = property.as_bool();
                        }
                        ControlWord::Italic => {
                            current_painter.italic = property.as_bool();
                        }
                        ControlWord::Underline => {
                            current_painter.underline = property.as_bool();
                        }
                        _ => {}
                    }
                }
                Token::PlainText(text) => {
                    let current_painter = painter_stack.last().expect("[Parser] : Empty painter stack : Too many closing brackets");
                    document.body.push(StyleBlock {
                        painter: current_painter.clone(),
                        text: *text,
                    })
                }
                Token::CRLF => {}
                Token::IgnorableDestination => {
                    panic!("[Parser] : No ignorable destination should be left");
                }
            }
        }
        return document;
    }

    fn get_token_at(&'a self, index: usize) -> Option<&'a Token<'a>> {
        return self.tokens.get(index);
    }

    // get a reference of the next token after cursor
    fn get_next_token(&'a self) -> Option<&'a Token<'a>> {
        return self.get_token_at(self.cursor);
    }

    fn consume_token_at(&mut self, index: usize) -> Option<Token<'a>> {
        if self.tokens.is_empty() {
            return None;
        }
        Some(self.tokens.remove(index))
    }

    fn consume_next_token(&mut self) -> Option<Token<'a>> {
        return self.consume_token_at(self.cursor);
    }

    // Consume token from cursor to <reference-token>
    fn _consume_tokens_until(&mut self, reference_token: Token<'a>) -> Vec<Token<'a>> {
        let mut ret = vec![];
        let token_type_id = mem::discriminant(&reference_token);
        while let Some(token) = self.consume_next_token() {
            let type_id = mem::discriminant(&token);
            ret.push(token);
            if type_id == token_type_id {
                break;
            }
        }
        return ret;
    }

    // The opening bracket should already be consumed
    fn consume_tokens_until_matching_bracket(&mut self) -> Vec<Token<'a>> {
        let mut ret = vec![];
        let mut count = 0;
        while let Some(token) = self.consume_next_token() {
            match token {
                Token::OpeningBracket => count += 1,
                Token::ClosingBracket => count -= 1,
                _ => {}
            }
            ret.push(token);
            if count < 0 {
                break;
            }
        }
        return ret;
    }

    // Consume all tokens until the header is read
    fn parse_header(&mut self) -> RtfHeader<'a> {
        self.cursor = 0; // Reset the cursor
        let mut header = RtfHeader::default();
        while let (token, next_token) = (self.consume_next_token(), self.get_next_token()) {
            match (token, next_token) {
                (Some(Token::OpeningBracket), Some(&CONTROL_TABLE_TOKEN)) => {
                    let font_table_tokens = self.consume_tokens_until_matching_bracket();
                    header.font_table = Self::parse_font_table(&font_table_tokens);
                    break; // HACK: header is not finished after the character table but I still haven't figure out a proper way to determine it
                }
                (Some(ref token), _) => {
                    if let Some(charset) = CharacterSet::from(token) {
                        header.character_set = charset;
                    }
                }
                (None, None) => break,
                (_, _) => {}
            }
        }
        return header;
    }

    fn parse_font_table(font_tables_tokens: &Vec<Token<'a>>) -> FontTable<'a> {
        assert_eq!(font_tables_tokens.get(0), Some(&CONTROL_TABLE_TOKEN));
        let mut table = HashMap::new();
        let mut current_key = 0;
        let mut current_font = Font::default();
        for token in font_tables_tokens.iter() {
            match token {
                Token::ControlSymbol((control_word, property)) => match control_word {
                    ControlWord::FontNumber => {
                        // Insert previous font
                        table.insert(current_key, current_font.clone());
                        if let Property::Value(key) = property {
                            current_key = *key as FontRef;
                        } else {
                            panic!("[Parser] Invalid font identifier : {:?}", property)
                        }
                    }
                    ControlWord::Unknown(name) => {
                        if let Some(font_family) = FontFamily::from(name) {
                            current_font.font_family = font_family;
                        }
                    }
                    _ => {}
                },
                Token::PlainText(name) => {
                    current_font.name = name.trim_end_matches(';');
                }
                Token::ClosingBracket => {
                    table.insert(current_key, current_font.clone());
                } // Insert previous font
                _ => {}
            }
        }
        return table;
    }

    // Delete the ignore groups
    fn parse_ignore_groups(&mut self) {
        self.cursor = 0; // Reset the cursor
        while let (Some(token), Some(next_token)) = (self.get_token_at(self.cursor), self.get_token_at(self.cursor + 1)) {
            match (token, next_token) {
                (Token::OpeningBracket, Token::IgnorableDestination) => {
                    self.consume_token_at(self.cursor); // Consume the opening bracket
                    self.consume_tokens_until_matching_bracket();
                }
                _ => {}
            }
            self.cursor += 1;
        }
    }

    pub fn to_text(&mut self) -> String {
        let mut text = String::new();
        let doc = self.parse();
        for block in doc.body.iter() {
            text.push_str(block.text);
        }
        return text;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::header::{CharacterSet::*, FontFamily::*, RtfHeader};
    use crate::lexer::Lexer;

    #[test]
    fn parser_simple_test() {
        let tokens = Lexer::scan(r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#);
        let doc = Parser::new(tokens).parse();
        assert_eq!(
            doc.header,
            RtfHeader {
                character_set: Ansi,
                font_table: FontTable::from([(
                    0,
                    Font {
                        name: "Helvetica",
                        character_set: 0,
                        font_family: Swiss
                    }
                )])
            }
        );
        assert_eq!(
            doc.body,
            [
                StyleBlock {
                    painter: Painter {
                        font_ref: 0,
                        font_size: 0,
                        bold: false,
                        italic: false,
                        underline: false
                    },
                    text: "Voici du texte en ",
                },
                StyleBlock {
                    painter: Painter {
                        font_ref: 0,
                        font_size: 0,
                        bold: true,
                        italic: false,
                        underline: false
                    },
                    text: "gras",
                },
                StyleBlock {
                    painter: Painter {
                        font_ref: 0,
                        font_size: 0,
                        bold: false,
                        italic: false,
                        underline: false
                    },
                    text: ".",
                },
            ]
        );
    }

    #[test]
    fn parse_multiline_document() {
        let document = r"{\rtf1\ansi\deff0 {\fonttbl {\f0 Courier;}{\f1 ProFontWindows;}}
            {\colortbl;\red0\green0\blue0;\red255\green0\blue0;\red255\green255\blue0;}
            This line is font 0 which is courier\line
            \f1
            This line is font 1\line
            \f0
            This line is font 0 again\line
            This line has a \cf2 red \cf1 word\line
            \highlight3 while this line has a \cf2 red \cf1 word and is highlighted in yellow\highlight0\line
            Finally, back to the default color.\line
            }";
        let tokens = Lexer::scan(document);
        let doc = Parser::new(tokens).parse();
        dbg!(doc);
    }

    #[test]
    fn parse_entire_file_header() {
        let file_content = include_str!("../test-file.rtf");
        let tokens = Lexer::scan(file_content);
        dbg!(&tokens);
        let doc = Parser::new(tokens).parse();
        assert_eq!(
            doc.header,
            RtfHeader {
                character_set: Ansi,
                font_table: FontTable::from([
                    (
                        0,
                        Font {
                            name: "Helvetica",
                            character_set: 0,
                            font_family: Swiss,
                        }
                    ),
                    (
                        1,
                        Font {
                            name: "Helvetica-Bold",
                            character_set: 0,
                            font_family: Swiss,
                        }
                    )
                ]),
            }
        );
        dbg!(doc.body);
    }

    #[test]
    fn parse_ignore_group_test() {
        let rtf = r"{\*\expandedcolortbl;;}";
        let tokens = Lexer::scan(rtf);
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(parser.tokens, vec![]);
    }

    #[test]
    fn rtf_to_text() {
        let rtf = r#"{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par}"#;
        let tokens = Lexer::scan(rtf);
        let text = Parser::new(tokens).to_text();
        assert_eq!(text, "Voici du texte en gras.")
    }
}
