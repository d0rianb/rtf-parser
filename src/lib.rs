// RTF parser for Text Editor
// Support RTF version 1.9.1
// specification is available here : https://dokumen.tips/documents/rtf-specification.html

#![allow(irrefutable_let_patterns)]

mod tokens;
mod lexer;
mod parser;
mod header;
mod utils;

// expose the lexer and the parser
pub use crate::lexer::Lexer;
pub use crate::parser::{Parser, Painter};
pub use crate::tokens::Token;
