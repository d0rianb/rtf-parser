// RTF parser for Text Editor
// This library supports RTF version 1.9.1
// Specification is available here : https://dokumen.tips/documents/rtf-specification.html
// Explanations on specification here : https://www.oreilly.com/library/view/rtf-pocket-guide/9781449302047/ch01.html

#![allow(irrefutable_let_patterns)]

mod document;
mod header;
mod lexer;
mod paragraph;
mod parser;
mod tokens;
mod utils;

// Public API of the crate
pub use crate::document::RtfDocument;
pub use crate::header::{CharacterSet, FontFamily, RtfHeader};
pub use crate::lexer::{Lexer, LexerError};
pub use crate::paragraph::*;
pub use crate::parser::{Painter, Parser, ParserError, StyleBlock};
pub use crate::tokens::{ControlSymbol, ControlWord, Property, Token};
