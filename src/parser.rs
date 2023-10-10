use crate::Token;
use std::marker::PhantomData;

pub struct Parser<'a> {
    _phantom: PhantomData<&'a str>,
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
    use crate::ControlWord::{Ansi, Bold, FontNumber, Rtf, Unknown};
    use crate::Property::*;
    use crate::Token::*;

    #[test]
    fn parser_simple_test() {
        let tokens = Lexer::scan(
            r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#,
        );
        assert_eq!(
            tokens,
            vec![
                OpeningBracket,
                ControlSymbol((Rtf, Value(1))),
                ControlSymbol((Ansi, None)),
                OpeningBracket,
                ControlSymbol((Unknown("\\fonttbl"), None)),
                ControlSymbol((FontNumber, Value(0))),
                ControlSymbol((Unknown("\\fswiss"), None)),
                PlainText("Helvetica"),
                SemiColon,
                ClosingBracket,
                ControlSymbol((FontNumber, Value(0))),
                ControlSymbol((Unknown("\\pard"), None)),
                PlainText("Voici du texte en "),
                OpeningBracket,
                ControlSymbol((Bold, None)),
                PlainText("gras"),
                ClosingBracket,
                PlainText("."),
                ControlSymbol((Unknown("\\par"), None)),
                ClosingBracket
            ]
        );
    }

    // #[test]
    fn rtf_to_text() {
        let rtf = r#"{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard
             Voici du texte en {\b gras}.\par
             }"#;
        let tokens = Lexer::scan(rtf);
        let text = Parser::to_text(&tokens);
        assert_eq!(text, "Voici du texte en gras.")
    }
}
