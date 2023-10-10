use crate::{CONTROL_WORD_HASMAP, Token};
use crate::utils::StrUtils;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlWord<'a> {
    Italic,
    Bold,
    Underline,
    Unknown(&'a str)
}

impl<'a> ControlWord<'a> {
    pub fn from(input: &str) -> ControlWord {
        *CONTROL_WORD_HASMAP
            .get(input)
            .unwrap_or(&ControlWord::Unknown(input))
    }
}

struct Lexer<'a> {
    src: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    pub fn scan(&self) -> Vec<Token> {
        let mut it = self.src.chars();
        let mut slices: Vec<Token> = vec![];
        let mut slice_start_index = 0;
        let mut current_index = 0;
        while let Some(c) = it.next() {
            match c {
                '{' | '}' | '\n' | ';' | '.' | '\\' => {
                    if slice_start_index < current_index {
                        let slice = &self.src[slice_start_index .. current_index];
                        // Get the corresponding token(s)
                        let tokens = Self::tokenize(slice);
                        for token in tokens {
                            slices.push(token);
                        }
                    }
                    slice_start_index = current_index;
                    current_index += 1;
                },
                'A'..='z' | '0'..='9' | ' ' => { current_index += 1; },
                _ => { dbg!(&c); }
            }
        }
        slices
    }

    fn tokenize(slice: &str) -> Vec<Token> {
        if slice.starts_with('\\') {
            // parse "\b Words in bold" -> (Token::ControlWord(ControlWord::Bold), Token::ControlWordArgumen("Words in bold")
            let (ident, tail) = slice.split_first_whitespace();
            let mut ret = vec![Token::ControlWord(ControlWord::from(ident))];
            if tail.len() > 0 {
                ret.push(Token::ControlWordArgument(tail));
            }
            return ret;
        }

        let single_token= match slice.trim() {
            ";" => Token::SemiColon,
            "{" => Token::OpeningBracket,
            "}" => Token::ClosingBracket,
            _ => Token::PlainText(slice)
        };
        return vec![single_token];
    }

    pub fn parse(tokens: Vec<Token>) -> Vec<Token> {
        vec![]
    }

    pub fn to_text(&self) -> &str {
        ""
    }
}


#[cfg(test)]
pub mod tests {
    use crate::lexer::{ControlWord, Lexer, Token};

    #[test]
    fn simple_tokenize_test() {
        let tokens = Lexer::tokenize(r"\b Words in bold");
        assert_eq!(tokens, vec![Token::ControlWord(ControlWord::Bold), Token::ControlWordArgument("Words in bold")]);
    }

    #[test]
    fn scan_simple_text() {
        use crate::lexer::ControlWord::Unknown;
        use crate::lexer::ControlWord::Bold;
        use crate::Token::*;
        let lexer = Lexer::new(r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#);
        assert_eq!(
            lexer.scan(),
            vec![
                OpeningBracket,
                ControlWord(Unknown("\\rtf1")),
                ControlWord(Unknown("\\ansi")),
                OpeningBracket,
                ControlWord(Unknown("\\fonttbl")),
                ControlWord(Unknown("\\f0")),
                ControlWord(Unknown("\\fswiss")),
                ControlWordArgument("Helvetica"),
                SemiColon,
                ClosingBracket,
                ControlWord(Unknown("\\f0")),
                ControlWord(Unknown("\\pard")),
                ControlWordArgument("Voici du texte en "),
                OpeningBracket,
                ControlWord(Bold),
                ControlWordArgument("gras"),
                ClosingBracket,
                PlainText("."),
                ControlWord(Unknown("\\par")),
        ]);
    }

    // #[test]
    fn rtf_to_text() {
        let rtf = r#"{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard
             Voici du texte en {\b gras}.\par
             }"#;
        let lexer = Lexer::new(rtf);
        let text = lexer.to_text();
        assert_eq!(text, "Voici du texte en gras.")
    }
}
