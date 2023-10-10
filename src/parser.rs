use std::marker::PhantomData;
use crate::Token;

pub struct Parser<'a> {
    _phantom: PhantomData<&'a str>
}

impl<'a> Parser<'a> {
    pub fn parse(tokens: Vec<Token>) -> Vec<Token> {
        vec![]
    }

    pub fn to_text(_tokens: &Vec<Token>) -> &'a str {
        ""
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn parser_simple_test() {
        let lexer = Lexer::new(r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#);
        let tokens = lexer.scan();

    }

    // #[test]
    fn rtf_to_text() {
        let rtf = r#"{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard
             Voici du texte en {\b gras}.\par
             }"#;
        let lexer = Lexer::new(rtf);
        let tokens = lexer.scan();
        let text = Parser::to_text(&tokens);
        assert_eq!(text, "Voici du texte en gras.")
    }
}