// RTF parser for Text Editor
// Support RTF version 1.9.1
// specification is available here : https://dokumen.tips/documents/rtf-specification.html

#![allow(irrefutable_let_patterns)]

mod document;
mod tokens;
mod lexer;
mod parser;
mod header;
mod utils;

// Public API of the crate
pub use crate::header::{CharacterSet, RtfHeader};
pub use crate::lexer::Lexer;
pub use crate::parser::{Painter, Parser, StyleBlock};
pub use crate::document::RtfDocument;
pub use crate::tokens::Token;