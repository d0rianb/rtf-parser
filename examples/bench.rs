use std::time::Instant;
use rtf_parser::header::StyleSheet;
use rtf_parser::lexer::Lexer;
use rtf_parser::parser::Parser;

fn main() {
    let start = Instant::now();
    let doc;
    {
        let rtf_text = include_str!("../resources/tests/file-sample_500kB.rtf");
        let tokens = Lexer::scan(rtf_text).expect("Invalid RTF content");
        doc = Parser::new(tokens).parse();
    }
    let elapsed = start.elapsed();
    assert_eq!(doc.unwrap().header.stylesheet, StyleSheet::new()); // in order to not get optimized out
    println!("Elapsed: {:.2?}", elapsed);
}