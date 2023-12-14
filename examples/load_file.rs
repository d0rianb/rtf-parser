extern crate rtf_parser;
use rtf_parser::{Lexer, Parser};

fn main() {
    let rtf_text = include_str!("/Users/dorian/Downloads/file-sample_1MB.rtf");
    let tokens = Lexer::scan(rtf_text);
    let doc = Parser::new(tokens).parse();
}
