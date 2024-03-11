use std::fmt;

use crate::LexerError;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone)]
pub enum Token<'a> {
    PlainText(&'a str),
    OpeningBracket,
    ClosingBracket,
    CRLF,                 // Line-return \n
    IgnorableDestination, // \*\ <destination-name>
    ControlSymbol(ControlSymbol<'a>),
}

#[allow(dead_code)]
impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::PlainText(text) => write!(f, r"PlainText : {:?}", text),
            Token::OpeningBracket => write!(f, "OpeningBracket"),
            Token::ClosingBracket => write!(f, "ClosingBracket"),
            Token::CRLF => write!(f, "CRLF"),
            Token::IgnorableDestination => write!(f, "IgnorableDestination"),
            Token::ControlSymbol(symbol) => write!(f, "ControlSymbol : {:?}", symbol),
        }
    }
}

// A control symbol is a pair (control_word, property)
// In the RTF specification, it refers to 'control word entity'
pub type ControlSymbol<'a> = (ControlWord<'a>, Property);

// Parameters for a control word
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Property {
    On,         // 1
    Off,        // 0
    Value(i32), // Specified as i32 in the specification
    None,       // No parameter
}

impl Property {
    pub fn as_bool(&self) -> bool {
        match self {
            Property::On => true,
            Property::Off => false,
            Property::None => true,
            Property::Value(val) => *val == 1,
        }
    }

    pub fn get_value(&self) -> i32 {
        if let Property::Value(value) = &self {
            return *value;
        }
        return 0;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ControlWord<'a> {
    Rtf,
    Ansi,

    FontTable,
    FontCharset,
    FontNumber,
    FontSize,

    ColorTable,
    FileTable,

    Italic,
    Bold,
    Underline,

    Par,
    Pard,
    Sectd,
    Plain,

    Unknown(&'a str),
}

impl<'a> ControlWord<'a> {
    pub fn from(input: &str) -> Result<ControlSymbol, LexerError> {
        // Loop backward the string to get the number
        let mut it = input.chars().rev();
        let mut suffix_index = 0;
        while let Some(c) = it.next() {
            match c {
                '0'..='9' | '-' => {
                    suffix_index += 1;
                }
                _ => break,
            }
        }

        // f0 -> prefix: f, suffix: 0
        let index = input.len() - suffix_index;
        let prefix = &input[..index];
        let suffix = &input[index..];

        let property = if suffix == "" {
            Property::None
        } else {
            let Ok(value) = suffix.parse::<i32>() else {
                return Err(LexerError::Error(format!("[Lexer] Unable to parse {} as integer", &suffix)));
            };
            Property::Value(value)
        };

        let control_word = match prefix {
            r"\rtf" => ControlWord::Rtf,
            r"\ansi" => ControlWord::Ansi,
            r"\fonttbl" => ControlWord::FontTable,
            r"\colortabl" => ControlWord::ColorTable,
            r"\filetbl" => ControlWord::FileTable,
            r"\fcharset" => ControlWord::FontCharset,
            r"\f" => ControlWord::FontNumber,
            r"\fs" => ControlWord::FontSize,
            r"\i" => ControlWord::Italic,
            r"\b" => ControlWord::Bold,
            r"\u" => ControlWord::Underline,
            r"\par" => ControlWord::Par,
            r"\pard" => ControlWord::Pard,
            r"\sectd" => ControlWord::Sectd,
            r"\plain" => ControlWord::Plain,
            _ => ControlWord::Unknown(prefix),
        };
        return Ok((control_word, property));
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{ControlWord, Property};

    #[test]
    fn control_word_from_input_test() {
        let input = r"\rtf1";
        assert_eq!(ControlWord::from(input).unwrap(), (ControlWord::Rtf, Property::Value(1)))
    }

    #[test]
    fn control_word_with_negative_parameter() {
        let input = r"\rtf-1";
        assert_eq!(ControlWord::from(input).unwrap(), (ControlWord::Rtf, Property::Value(-1)))
    }
}
