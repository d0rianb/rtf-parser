// RTF parser for Text Editor
// This library supports RTF version 1.9.1
// Specification is available here : https://dokumen.tips/documents/rtf-specification.html
// Explanations on specification here : https://www.oreilly.com/library/view/rtf-pocket-guide/9781449302047/ch01.html

#![allow(irrefutable_let_patterns)]

// Public API of the crate
pub mod document;
pub mod header;
pub mod lexer;
pub mod paragraph;
pub mod parser;
pub mod tokens;
mod utils;
