extern crate rtf_parser;
use rtf_parser::{Lexer, Parser, StyleSheet};

fn main() {
    let rtf_text = include_str!("../resources/tests/file-sample_500kB.rtf");
    let tokens = Lexer::scan(rtf_text).expect("Invalid RTF content");
    let doc = Parser::new(tokens).parse();
    assert_eq!(doc.unwrap().header.stylesheet, StyleSheet::new());
}
