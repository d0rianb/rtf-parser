extern crate rtf_parser;
use rtf_parser::lexer::Lexer;
use rtf_parser::parser::Parser;

fn main() {
    let rtf_text = include_str!("../resources/tests/test-file.rtf");
    let tokens = Lexer::scan(rtf_text).expect("Invalid RTF content");
    let doc = Parser::new(tokens).parse();
}
