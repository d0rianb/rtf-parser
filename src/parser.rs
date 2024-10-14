use std::collections::HashMap;
use std::{fmt, mem};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::document::RtfDocument;
use crate::header::{CharacterSet, Color, ColorRef, ColorTable, Font, FontFamily, FontRef, FontTable, RtfHeader, StyleSheet};
use crate::paragraph::{Alignment, Paragraph, SpaceBetweenLine};
use crate::tokens::{ControlWord, Property, Token};

// Use to specify control word in parse_header
macro_rules! header_control_word {
    ($cw:ident) => {
        &Token::ControlSymbol((ControlWord::$cw, _))
    };
    ($cw:ident, $prop:ident) => {
        &Token::ControlSymbol((ControlWord::$cw, Property::$prop))
    };
}

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct StyleBlock {
    pub painter: Painter,
    pub paragraph: Paragraph,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Hash, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct Painter {
    pub color_ref: ColorRef,
    pub font_ref: FontRef,
    pub font_size: u16,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub superscript: bool,
    pub subscript: bool,
    pub smallcaps: bool,
    pub strike: bool,
}

impl Default for Painter {
    fn default() -> Self {
        Self {
            color_ref: Default::default(),
            font_ref: Default::default(),
            font_size: 12,
            bold: Default::default(),
            italic: Default::default(),
            underline: Default::default(),
            superscript: Default::default(),
            subscript: Default::default(),
            smallcaps: Default::default(),
            strike: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    InvalidToken(String),
    IgnorableDestinationParsingError,
    MalformedPainterStack,
    InvalidFontIdentifier(Property),
    InvalidColorIdentifier(Property),
    NoMoreToken,
    ValueCastError(String),
    UnicodeParsingError(i32),
    ParseEmptyToken,
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "[RTF Parser] : ");
        return match self {
            ParserError::InvalidToken(msg) => write!(f, "{}", msg),
            ParserError::IgnorableDestinationParsingError => write!(f, "No ignorable destination should be left"),
            ParserError::MalformedPainterStack => write!(f, "Malformed painter stack : Unbalanced number of brackets"),
            ParserError::InvalidFontIdentifier(property) => write!(f, "Invalid font identifier : {:?}", property),
            ParserError::InvalidColorIdentifier(property) => write!(f, "Invalid color identifier : {:?}", property),
            ParserError::NoMoreToken => write!(f, "No more token to parse"),
            ParserError::ValueCastError(_type) => write!(f, "Unable to cast i32 to {_type}"),
            ParserError::UnicodeParsingError(value) => write!(f, "Unable to parse {value} value to unicode"),
            ParserError::ParseEmptyToken => write!(f, "Try to parse an empty token, this should never happen. If so, please open an issue in the github repository"),
        };
    }
}

// This state keeps track of each value that depends on the scope nesting
#[derive(Debug, Clone, PartialEq, Hash)]
struct ParserState {
    pub painter: Painter,
    pub paragraph: Paragraph,
    pub unicode_ignore_count: i32,
}

impl Default for ParserState {
    fn default() -> Self {
        Self {
            painter: Default::default(),
            paragraph: Default::default(),
            unicode_ignore_count: 1,
        }
    }
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    parsed_item: Vec<bool>,
    cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        return Self {
            parsed_item: vec![false; tokens.len()],
            tokens,
            cursor: 0,
        };
    }

    pub fn get_tokens(&self) -> Vec<&Token> {
        // It ignores the empty tokens, that replaced already parsed tokens istead of deleting them for performance reasons
        return self.tokens.iter().filter(|t| *t != &Token::Empty).collect();
    }

    fn check_document_validity(&self) -> Result<(), ParserError> {
        // Check the document boundaries
        if let Some(token) = self.tokens.first() {
            if token != &Token::OpeningBracket {
                return Err(ParserError::InvalidToken(format!("Invalid first token : {:?} not a '{{'", token)));
            }
        } else {
            return Err(ParserError::NoMoreToken);
        }
        if let Some(token) = self.tokens.last() {
            if token != &Token::ClosingBracket {
                return Err(ParserError::InvalidToken(format!("Invalid last token : {:?} not a '}}'", token)));
            }
        } else {
            return Err(ParserError::NoMoreToken);
        }
        return Ok(());
    }

    pub fn parse(&mut self) -> Result<RtfDocument, ParserError> {
        self.check_document_validity()?;
        let mut document = RtfDocument::default(); // Init empty document
                                                   // Traverse the document and consume the header groups (FontTable, StyleSheet, etc ...)
        document.header = self.parse_header()?;
        // Init the state of the docuement. the stack is used to keep track of the different scope changes.
        let mut state_stack: Vec<ParserState> = vec![ParserState::default()];
        // Parse the body
        let len = self.tokens.len();
        let mut i = 0;

        while i < len {
            if self.parsed_item[i] {
                // The item already has been parsed
                i += 1;
                continue;
            }
            let token = &self.tokens[i];

            match token {
                Token::OpeningBracket => {
                    if let Some(last_state) = state_stack.last() {
                        state_stack.push(last_state.clone()); // Inherit from the last state properties
                    } else {
                        state_stack.push(ParserState::default());
                    }
                }
                Token::ClosingBracket => {
                    let state = state_stack.pop();
                    if state.is_none() {
                        return Err(ParserError::MalformedPainterStack);
                    }
                }
                Token::ControlSymbol((control_word, property)) => {
                    let Some(current_state) = state_stack.last_mut() else {
                        return Err(ParserError::MalformedPainterStack);
                    };
                    let current_painter = &mut current_state.painter;
                    let paragraph = &mut current_state.paragraph;
                    #[rustfmt::skip]  // For now, rustfmt does not support this kind of alignement
                    match control_word {
                        ControlWord::ColorNumber        => current_painter.color_ref = property.get_value_as::<ColorRef>()?,
                        ControlWord::FontNumber         => current_painter.font_ref = property.get_value_as::<FontRef>()?,
                        ControlWord::FontSize           => current_painter.font_size = property.get_value_as::<u16>()?,
                        ControlWord::Bold               => current_painter.bold = property.as_bool(),
                        ControlWord::Italic             => current_painter.italic = property.as_bool(),
                        ControlWord::Underline          => current_painter.underline = property.as_bool(),
                        ControlWord::UnderlineNone      => current_painter.underline = false,
                        ControlWord::Superscript        => current_painter.superscript = property.as_bool(),
                        ControlWord::Subscript          => current_painter.subscript = property.as_bool(),
                        ControlWord::Smallcaps          => current_painter.smallcaps = property.as_bool(),
                        ControlWord::Strikethrough      => current_painter.strike = property.as_bool(),
                        // Paragraph
                        ControlWord::Pard               => *paragraph = Paragraph::default(), // Reset the par
                        ControlWord::Plain              => *current_painter = Painter::default(), // Reset the painter
                        ControlWord::ParDefTab          => paragraph.tab_width = property.get_value(),
                        ControlWord::LeftAligned
                            | ControlWord::RightAligned
                            | ControlWord::Center
                            | ControlWord::Justify      => paragraph.alignment = Alignment::from(control_word),
                        ControlWord::SpaceBefore        => paragraph.spacing.before = property.get_value(),
                        ControlWord::SpaceAfter         => paragraph.spacing.after = property.get_value(),
                        ControlWord::SpaceBetweenLine   => paragraph.spacing.between_line = SpaceBetweenLine::from(property.get_value()),
                        ControlWord::SpaceLineMul       => paragraph.spacing.line_multiplier = property.get_value(),
                        ControlWord::UnicodeIgnoreCount => current_state.unicode_ignore_count = property.get_value(),
                        ControlWord::Unicode            => {
                            let mut unicodes = Vec::with_capacity(current_state.unicode_ignore_count as usize + 1); // try to avoid realocation due to fallback unicodes
                            if let Ok(unicode) = property.get_unicode_value() {
                                unicodes.push(unicode);
                            }
                            // Get the following unicode in case of compounds characters
                            while i + 1 < len {
                                // We should not check if the tokens has already been parsed, because we are looking for the following token in the document
                                if let Token::ControlSymbol((ControlWord::Unicode, property)) = &self.tokens[i + 1] {
                                    if let Ok(unicode) = property.get_unicode_value() {
                                        unicodes.push(unicode);
                                    }
                                    i += 1;
                                } else {
                                    break;
                                }
                            }
                            if unicodes.len() > 0 {
                                // Handle the fallback unicode (\uc2 \u0000 'FA 'FB)
                                let mut ignore_mask = vec![true; unicodes.len()];
                                let mut ignore_counter = 0;
                                for i in 1..unicodes.len() {
                                    if unicodes[i] <= 255 && ignore_counter < current_state.unicode_ignore_count {
                                        ignore_counter += 1;
                                        ignore_mask[i] = false;
                                    } else {
                                        ignore_counter = 0;
                                    }
                                }
                                let mut ignore_mask_iter = ignore_mask.iter();
                                unicodes.retain(|_| *ignore_mask_iter.next().unwrap());
                                // Convert the unicode to string
                                let str = String::from_utf16(unicodes.as_slice()).unwrap();
                                Self::add_text_to_document(&str, &state_stack, &mut document)?;
                            }
                        }
                        // Others tokens
                        _ => {}
                    };
                }
                Token::PlainText(text) => Self::add_text_to_document(*text, &state_stack, &mut document)?,
                Token::CRLF => Self::add_text_to_document("\n", &state_stack, &mut document)?,
                Token::IgnorableDestination => {
                    return Err(ParserError::IgnorableDestinationParsingError);
                }
                Token::Empty => return Err(ParserError::ParseEmptyToken),
            };
            i += 1;
        }
        return Ok(document);
    }

    fn add_text_to_document(text: &str, state_stack: &Vec<ParserState>, document: &mut RtfDocument) -> Result<(), ParserError> {
        let Some(current_state) = state_stack.last() else {
            return Err(ParserError::MalformedPainterStack);
        };
        let current_painter = &current_state.painter;
        let paragraph = &current_state.paragraph;
        let last_style_group = document.body.last_mut();
        // If the painter is the same as the previous one, merge the two block.
        if let Some(group) = last_style_group {
            if group.painter.eq(current_painter) && group.paragraph.eq(&paragraph) {
                group.text.push_str(text);
                return Ok(());
            }
        }
        // Else, push another StyleBlock on the stack with its own painter
        document.body.push(StyleBlock {
            painter: current_painter.clone(),
            paragraph: paragraph.clone(),
            text: String::from(text),
        });
        return Ok(());
    }

    fn get_token_at(&'a self, index: usize) -> Option<&'a Token<'a>> {
        return self.tokens.get(index);
    }

    // Get a view of the next token after cursor
    fn get_next_token(&'a self) -> Option<&'a Token<'a>> {
        return self.get_token_at(self.cursor);
    }

    #[inline]
    fn consume_token_at(&mut self, index: usize) -> Option<Token<'a>> {
        if self.tokens.is_empty() || index >= self.tokens.len() {
            return None;
        }
        // PERF : vec.remove can require reallocation unlike this method
        self.cursor += 1;
        self.parsed_item[index] = true;
        return Some(mem::replace(&mut self.tokens[index], Token::Empty));
    }

    fn consume_next_token(&mut self) -> Option<Token<'a>> {
        return self.consume_token_at(self.cursor);
    }

    // Consume token from cursor to <reference-token>
    fn _consume_tokens_until(&mut self, reference_token: &Token<'a>) -> Vec<Token<'a>> {
        let mut ret = vec![];
        let token_type_id = mem::discriminant(reference_token);
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

    // Consume all the tokens inside a group ({ ... }) and returns the includes ones
    fn consume_group(&mut self) -> Vec<Token<'a>> {
        // TODO: check the the token at cursor is indeed an OpeningBracket
        self.consume_token_at(self.cursor); // Consume the opening bracket
        return self.consume_tokens_until_matching_bracket();
    }

    // Consume all tokens until the header is read
    fn parse_header(&mut self) -> Result<RtfHeader, ParserError> {
        self.cursor = 0; // Reset the cursor
        let mut header = RtfHeader::default();
        while let (Some(token), Some(mut next_token)) = (self.get_token_at(self.cursor), self.get_token_at(self.cursor + 1)) {
            // Manage the case where there is CRLF between { and control_word
            // {\n /*/ignoregroup }
            let mut i = 0;
            while *next_token == Token::CRLF {
                if let Some(next_token_not_crlf) = self.get_token_at(self.cursor + 1 + i) {
                    next_token = next_token_not_crlf;
                    i += 1;
                } else {
                    break;
                }
            }
            match (token, next_token) {
                (Token::OpeningBracket, Token::IgnorableDestination) => {
                    let ignore_group_tokens = self.consume_group();
                    Self::parse_ignore_groups(&ignore_group_tokens);
                }
                (Token::OpeningBracket, header_control_word!(FontTable, None)) => {
                    let font_table_tokens = self.consume_group();
                    header.font_table = Self::parse_font_table(&font_table_tokens)?;
                }
                (Token::OpeningBracket, header_control_word!(ColorTable, None)) => {
                    let color_table_tokens = self.consume_group();
                    header.color_table = Self::parse_color_table(&color_table_tokens)?;
                }
                (Token::OpeningBracket, header_control_word!(StyleSheet, None)) => {
                    let stylesheet_tokens = self.consume_group();
                    header.stylesheet = Self::parse_stylesheet(&stylesheet_tokens)?;
                }
                // Check and consume token
                (token, _) => {
                    if let Some(charset) = CharacterSet::from(token) {
                        header.character_set = charset;
                    }
                    self.cursor += 1;
                }
            }
        }
        return Ok(header);
    }

    fn parse_font_table(font_tables_tokens: &Vec<Token<'a>>) -> Result<FontTable, ParserError> {
        let Some(font_table_first_token) = font_tables_tokens.get(0) else {
            return Err(ParserError::NoMoreToken);
        };
        if font_table_first_token != header_control_word!(FontTable, None) {
            return Err(ParserError::InvalidToken(format!("{:?} is not a FontTable token", font_table_first_token)));
        }
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
                            return Err(ParserError::InvalidFontIdentifier(*property));
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
                    current_font.name = name.trim_end_matches(';').to_string();
                }
                Token::ClosingBracket => {
                    table.insert(current_key, current_font.clone());
                } // Insert previous font
                _ => {}
            }
        }
        return Ok(table);
    }

    fn parse_color_table(color_table_tokens: &Vec<Token<'a>>) -> Result<ColorTable, ParserError> {
        let Some(color_table_first_token) = color_table_tokens.get(0) else {
            return Err(ParserError::NoMoreToken);
        };
        if color_table_first_token != header_control_word!(ColorTable, None) {
            return Err(ParserError::InvalidToken(format!("ParserError: {:?} is not a ColorTable token", color_table_first_token)));
        }
        let mut table = HashMap::new();
        let mut current_key = 1;
        let mut current_color = Color::default();
        for token in color_table_tokens.iter() {
            match token {
                Token::ControlSymbol((control_word, property)) => match control_word {
                    ControlWord::ColorRed => current_color.red = property.get_value_as::<u8>()?,
                    ControlWord::ColorGreen => current_color.green = property.get_value_as::<u8>()?,
                    ControlWord::ColorBlue => {
                        current_color.blue = property.get_value_as::<u8>()?;
                        table.insert(current_key, current_color.clone());
                        current_key += 1;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        return Ok(table);
    }

    fn parse_stylesheet(_stylesheet_tokens: &Vec<Token<'a>>) -> Result<StyleSheet, ParserError> {
        // TODO : parse the stylesheet
        return Ok(StyleSheet::from([]));
    }

    fn parse_ignore_groups(_tokens: &Vec<Token<'a>>) {
        // Do nothing for now
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::header::CharacterSet::*;
    use crate::header::FontFamily::*;
    use crate::header::RtfHeader;
    use crate::include_test_file;
    use crate::lexer::Lexer;

    #[test]
    fn parser_header() {
        let tokens = Lexer::scan(r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#).unwrap();
        let doc = Parser::new(tokens).parse().unwrap();
        assert_eq!(
            doc.header,
            RtfHeader {
                character_set: Ansi,
                font_table: FontTable::from([(
                    0,
                    Font {
                        name: "Helvetica".into(),
                        character_set: 0,
                        font_family: Swiss
                    }
                )]),
                ..RtfHeader::default()
            }
        );
        assert_eq!(
            doc.body,
            [
                StyleBlock {
                    painter: Painter::default(),
                    paragraph: Default::default(),
                    text: "Voici du texte en ".into(),
                },
                StyleBlock {
                    painter: Painter { bold: true, ..Painter::default() },
                    paragraph: Default::default(),
                    text: "gras".into(),
                },
                StyleBlock {
                    painter: Painter::default(),
                    paragraph: Default::default(),
                    text: ".".into(),
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
        let tokens = Lexer::scan(document).unwrap();
        let _doc = Parser::new(tokens).parse().unwrap();
    }

    #[test]
    fn parse_entire_file_header() {
        let file_content = include_test_file!("test-file.rtf");
        let tokens = Lexer::scan(file_content).unwrap();
        let doc = Parser::new(tokens).parse().unwrap();
        assert_eq!(
            doc.header,
            RtfHeader {
                character_set: Ansi,
                font_table: FontTable::from([
                    (
                        0,
                        Font {
                            name: "Helvetica".into(),
                            character_set: 0,
                            font_family: Swiss,
                        }
                    ),
                    (
                        1,
                        Font {
                            name: "Helvetica-Bold".into(),
                            character_set: 0,
                            font_family: Swiss,
                        }
                    )
                ]),
                color_table: ColorTable::from([(1, Color { red: 255, green: 255, blue: 255 }),]),
                ..RtfHeader::default()
            }
        );
    }

    #[test]
    fn parse_ignore_group() {
        let rtf = r"{\*\expandedcolortbl;;}";
        let tokens = Lexer::scan(rtf).unwrap();
        let mut parser = Parser::new(tokens);
        let document = parser.parse().unwrap();
        assert_eq!(parser.get_tokens(), Vec::<&Token>::new()); // Should have consumed all the tokens
        assert_eq!(document.header, RtfHeader::default());
    }

    #[test]
    fn parse_ignore_group_with_crlf() {
        let rtf = r"{\
        \
        \*\expandedcolortbl;;}";
        let tokens = Lexer::scan(rtf).unwrap();
        let mut parser = Parser::new(tokens);
        let document = parser.parse().unwrap();
        assert_eq!(parser.get_tokens(), Vec::<&Token>::new()); // Should have consumed all the tokens
        assert_eq!(document.header, RtfHeader::default());
    }

    #[test]
    fn parse_whitespaces() {
        let file_content = include_test_file!("list-item.rtf");
        let tokens = Lexer::scan(file_content).unwrap();
        let mut parser = Parser::new(tokens);
        let document = parser.parse().unwrap();
        assert_eq!(
            document.body,
            vec![StyleBlock {
                painter: Painter { font_size: 24, ..Painter::default() },
                paragraph: Default::default(),
                text: "\nEmpty start\n\nList test : \n - item 1\n - item 2\n - item 3\n - item 4".into(),
            },]
        );
    }

    #[test]
    fn parse_image_data() {
        // Try to parse without error
        let rtf_content = include_test_file!("file-with-image.rtf");
        let tokens = Lexer::scan(rtf_content).unwrap();
        let _document = Parser::new(tokens).parse();
    }

    #[test]
    fn parse_header_and_body() {
        let rtf = r#"{\rtf1\ansi\ansicpg1252\cocoartf2639
\cocoatextscaling0\cocoaplatform0{\fonttbl\f0\froman\fcharset0 Times-Bold;\f1\froman\fcharset0 Times-Roman;\f2\froman\fcharset0 Times-Italic;
\f3\fswiss\fcharset0 Helvetica;}
{\colortbl;\red255\green255\blue255;\red0\green0\blue10;\red0\green0\blue1;\red191\green191\blue191;
}
\f0\b\fs21 \cf2 Lorem ipsum
\fs56 \
\pard\pardeftab709\sl288\slmult1\sa225\qj\partightenfactor0

\f1\b0\fs21 \cf0 \
\pard\pardeftab709\fi-432\ri-1\sb240\sa120\partightenfactor0
\ls1\ilvl0
\f0\b\fs36\cf2\plain Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc ac faucibus odio. \
\pard\pardeftab709\sl288\slmult1\sa225\qj\partightenfactor0
}"#;
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(document.body[0].text, "Lorem ipsum");
        assert_eq!(document.body[1].text, "\n");
        assert_eq!(document.body[2].text, "\n");
        assert_eq!(document.body[3].text, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc ac faucibus odio. \n");
    }

    #[test]
    fn parse_paragraph_aligment() {
        let rtf = r#"{\rtf1\ansi\deff0 {\fonttbl {\f0 Times;}}
        \fs34
        {\pard \qc \fs60 Annalium Romae\par}
        {\pard \qj
            Urbem Romam a principio reges habuere; libertatem et
            \par}
        {\pard \ql
            Non Cinnae, non Sullae longa dominatio; et Pompei Crassique potentia
            \par}"#;
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(document.body[0].paragraph.alignment, Alignment::Center);
        assert_eq!(document.body[1].paragraph.alignment, Alignment::Justify);
        assert_eq!(document.body[2].paragraph.alignment, Alignment::LeftAligned);
    }

    #[test]
    fn should_parse_escaped_char() {
        let rtf = r"{\rtf1\ansi\deff0 {\fonttbl {\f0 Times;}}je suis une b\'eate}";
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(document.body[0].text, "je suis une bête");
    }

    #[test]
    fn parse_plain_directive() {
        let rtf = r"{\rtf1{\fonttbl {\f0 Times;}}\f0\b\fs36\u\cf2\plain Plain text}";
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(document.body[0].painter, Painter::default());
    }

    #[test]
    fn parse_color_table() {
        // cf0 is unset color, cf1 is first color, cf2 is second color, etc ...
        let rtf = r#"{\rtf1\ansi\ansicpg936\cocoartf2761
            \cocoatextscaling0\cocoaplatform0{\fonttbl\f0\fswiss\fcharset0 Helvetica;\f1\fnil\fcharset134 PingFangSC-Regular;}
            {\colortbl;\red255\green255\blue255;\red251\green2\blue7;\red114\green44\blue253;}
            {\*\expandedcolortbl;;\cssrgb\c100000\c14913\c0;\cssrgb\c52799\c30710\c99498;}
            \f0\fs24 \cf2 A
            \f1 \cf3 B}"#;
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(document.header.color_table.get(&document.body[0].painter.color_ref).unwrap(), &Color { red: 251, green: 2, blue: 7 });
    }

    #[test]
    fn parse_underline() {
        // \\ul underline true
        // \\ulnone underline false
        let rtf = r#"{\rtf1\ansi\ansicpg936\cocoartf2761
            \cocoatextscaling0\cocoaplatform0{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
            {\colortbl;\red255\green255\blue255;}
            {\*\expandedcolortbl;;}
            \paperw11900\paperh16840\margl1440\margr1440\vieww11520\viewh8400\viewkind0
            \pard\tx720\tx1440\tx2160\tx2880\tx3600\tx4320\tx5040\tx5760\tx6480\tx7200\tx7920\tx8640\pardirnatural\partightenfactor0

            \f0\fs24 \cf0 \ul \ulc0 a\ulnone A}"#;
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(&document.body[0].painter.underline, &true);
        assert_eq!(&document.body[1].painter.underline, &false);
    }

    #[test]
    fn parse_unicode() {
        // start with \\uc0
        // \u21834 => 啊
        let rtf = r#"{\rtf1\ansi\ansicpg936\cocoartf2761
            \cocoatextscaling0\cocoaplatform0{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
            \f0\fs24 \cf0 \uc0\u21834  \u21834 }"#;
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(&document.body[0].text, "啊 啊");
    }

    #[test]
    fn parse_two_characters_compound_unicode() {
        let rtf = r#"{\rtf1\ansi
            \f0 a\u55357 \u56447 1 \u21834}"#;
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(&document.body[0].text, "a👿1 啊");
    }

    #[test]
    fn parse_unicode_with_fallback() {
        // Should only consider the first unicode, not the two fallback chars
        let rtf = r#"{\rtf1\ansi
            {\f0 \u-10179\'5f\u-9089\'5f}
            {\f1 \uc2\u32767\'c2\'52}
            {\f2 \uc2\u26789\'97\'73}
            {\f3 b\'eate}
            {\f4 \uc0 b\'ea\'eate}
           }"#;
        let tokens = Lexer::scan(rtf).unwrap();
        let document = Parser::new(tokens).parse().unwrap();
        assert_eq!(&document.body[0].text, "👿");
        assert_eq!(&document.body[1].text, "翿");
        assert_eq!(&document.body[2].text, "梥");
        assert_eq!(&document.body[3].text, "bête");
        assert_eq!(&document.body[4].text, "bêête");
    }

    #[test]
    fn body_starts_with_a_group() {
        let rtf = r"{\rtf1\ansi\deff0{\fonttbl {\f0\fnil\fcharset0 Calibri;}{\f1\fnil\fcharset2 Symbol;}}{\colortbl ;}{\pard \u21435  \sb70\par}}";
        let tokens = Lexer::scan(rtf).unwrap();
        let _document = Parser::new(tokens).parse().unwrap();
    }

    #[test]
    fn rtf_different_semantic() {
        let rtf1 = r"{\rtf1 \b bold \i Bold Italic \i0 Bold again}";
        let rtf2 = r"{\rtf1 \b bold {\i Bold Italic }Bold again}";
        let rtf3 = r"{\rtf1 \b bold \i Bold Italic \plain\b Bold again}";
        let doc1 = RtfDocument::try_from(rtf1).unwrap();
        let doc2 = RtfDocument::try_from(rtf2).unwrap();
        let doc3 = RtfDocument::try_from(rtf3).unwrap();
        assert_eq!(doc1.body, doc2.body);
        assert_eq!(doc3.body, doc2.body);
    }
}
