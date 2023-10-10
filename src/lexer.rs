use crate::utils::StrUtils;
use crate::{ControlWord, Token};

pub struct Lexer<'a> {
    src: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn scan(src: &str) -> Vec<Token> {
        let mut it =src.chars();
        let mut tokens: Vec<Token> = vec![];
        let mut slice_start_index = 0;
        let mut current_index = 0;
        let mut previous_char = ' ';
        while let Some(c) = it.next() {
            match c {
                // Handle Escaped chars
                // TODO: Handle char over code 127 for escaped chars
                '{' | '}' | '\\' if previous_char == '\\' => {}
                // End of slice chars
                '{' | '}' | ';' | '.' | '\\' => {
                    if slice_start_index < current_index
                        && !(previous_char == '\\' && ['{', '}', '\\'].contains(&c))
                    {
                        // Close slice
                        let slice = &src[slice_start_index..current_index];
                        // Get the corresponding token(s)
                        let slice_tokens = Self::tokenize(slice);
                        for slice_token in slice_tokens {
                            tokens.push(slice_token);
                        }
                        slice_start_index = current_index;
                    }
                }
                // Others chars
                _ => {}
            }
            current_index += 1;
            previous_char = c;
        }
        // Manage last token (should always be "}")
        if slice_start_index < current_index {
            let slice = &src[slice_start_index..current_index];
            assert_eq!(slice, "}", "[Lexer] Invalid last char, should be '}}'");
            tokens.push(Token::ClosingBracket);
        }
        return tokens;
    }

    fn tokenize(slice: &str) -> Vec<Token> {
        if slice.starts_with('\\') {
            // Handle escaped chars
            if slice[1..].starts_with(['{', '}', '\\']) {
                return vec![Token::PlainText(&slice[1..])];
            }
            // parse "\b Words in bold" -> (Token::ControlWord(ControlWord::Bold), Token::ControlWordArgumen("Words in bold")
            let (ident, tail) = slice.split_first_whitespace();
            let mut ret = vec![Token::ControlSymbol(ControlWord::from(ident))];
            if tail.len() > 0 {
                ret.push(Token::PlainText(tail));
            }
            return ret;
        }

        let single_token = match slice.trim() {
            ";" => Token::SemiColon,
            "{" => Token::OpeningBracket,
            "}" => Token::ClosingBracket,
            "\\n" => Token::CRLF,
            _ => Token::PlainText(slice),
        };
        return vec![single_token];
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::lexer::Lexer;
    use crate::ControlWord::{Ansi, Bold, FontNumber, FontSize, Rtf, Unknown};
    use crate::Property::*;
    use crate::Token::*;

    #[test]
    fn simple_tokenize_test() {
        let tokens = Lexer::tokenize(r"\b Words in bold");
        assert_eq!(
            tokens,
            vec![ControlSymbol((Bold, None)), PlainText("Words in bold"),]
        );
    }

    #[test]
    fn scan_entire_file_test() {
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
                ClosingBracket,
            ]
        );
    }

    #[test]
    fn scan_escaped_text() {
        let tokens = Lexer::scan(
            r#"\f0\fs24 \cf0 test de code \
if (a == b) \{\
    test();\
\} else \{\
    return;\
\}}"#,
        );
        assert_eq!(
            tokens,
            vec![
                ControlSymbol((FontNumber, Value(0))),
                ControlSymbol((FontSize, Value(24))),
                ControlSymbol((Unknown("\\cf"), Value(0))),
                PlainText("test de code "),
                ControlSymbol((Unknown("\\"), None)),
                PlainText("if (a == b) "),
                PlainText("{"),
                ControlSymbol((Unknown("\\"), None)),
                PlainText("    test()"),
                SemiColon,
                ControlSymbol((Unknown("\\"), None)),
                PlainText("} else "),
                PlainText("{"),
                ControlSymbol((Unknown("\\"), None)),
                PlainText("    return"),
                SemiColon,
                ControlSymbol((Unknown("\\"), None)),
                PlainText("}"),
                ClosingBracket
            ],
        );
    }
}
