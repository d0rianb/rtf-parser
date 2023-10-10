use crate::{ControlWord, Token};
use crate::utils::StrUtils;

pub struct Lexer<'a> {
    src: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    pub fn scan(&self) -> Vec<Token> {
        let mut it = self.src.chars();
        let mut tokens: Vec<Token> = vec![];
        let mut slice_start_index = 0;
        let mut current_index = 0;
        while let Some(c) = it.next() {
            match c {
                // End of slice chars
                '{' | '}' | '\n' | ';' | '.' | '\\' => {
                    if slice_start_index < current_index {
                        let slice = &self.src[slice_start_index .. current_index];
                        // Get the corresponding token(s)
                        let slice_tokens = Self::tokenize(slice);
                        for slice_token in slice_tokens {
                            tokens.push(slice_token);
                        }
                    }
                    slice_start_index = current_index;
                    current_index += 1;
                },
                // Regular chars
                'A'..='z' | '0'..='9' | ' ' => { current_index += 1; },
                _ => { panic!("[Lexer] Unknown char : \"{}\"", &c); }
            }
        }
        // Manage last token (should always be "}")
        if slice_start_index < current_index {
            let slice =  &self.src[slice_start_index .. current_index];
            assert_eq!(slice, "}", "[Lexer] Invalid last char, should be '}}'");
            tokens.push(Token::ClosingBracket);
        }
        return tokens;
    }

    fn tokenize(slice: &str) -> Vec<Token> {
        if slice.starts_with('\\') {
            // parse "\b Words in bold" -> (Token::ControlWord(ControlWord::Bold), Token::ControlWordArgumen("Words in bold")
            let (ident, tail) = slice.split_first_whitespace();
            let mut ret = vec![Token::ControlSymbol(ControlWord::from(ident))];
            if tail.len() > 0 {
                ret.push(Token::PlainText(tail));
            }
            return ret;
        }

        let single_token= match slice.trim() {
            ";" => Token::SemiColon,
            "{" => Token::OpeningBracket,
            "}" => Token::ClosingBracket,
            "\\n" => Token::CRLF,
            _ => Token::PlainText(slice)
        };
        return vec![single_token];
    }
}


#[cfg(test)]
pub(crate) mod tests {
    use crate::{ControlWord, Property};
    use crate::lexer::{Lexer, Token};

    #[test]
    fn simple_tokenize_test() {
        let tokens = Lexer::tokenize(r"\b Words in bold");
        assert_eq!(tokens, vec![
            Token::ControlSymbol((ControlWord::Bold, Property::None)),
            Token::PlainText("Words in bold")
        ]);
    }

    #[test]
    fn scan_entire_file_test() {
        use crate::ControlWord::{
            Unknown,
            Bold,
            FontNumber
        };
        use crate::Token::*;
        use crate::Property::*;

        let lexer = Lexer::new(r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#);
        assert_eq!(
            lexer.scan(),
            vec![
                OpeningBracket,
                ControlSymbol((Unknown("\\rtf"), Value(1))),
                ControlSymbol((Unknown("\\ansi"), None)),
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
            ]);
    }

}
