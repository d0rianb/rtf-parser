// RTF parser for Text Editor
// Support RTF version 1.9.1
// specification is available here : https://dokumen.tips/documents/rtf-specification.html

mod lexer;
mod parser;
mod header;
mod utils;

// expose the lexer and the parser
pub use crate::lexer::Lexer as Lexer;
pub use crate::parser::Parser as Parser;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    PlainText(&'a str),
    OpeningBracket,
    ClosingBracket,
    CRLF, // Line-return \n
    IgnorableDestination, // \*\ <destination-name>
    ControlSymbol(ControlSymbol<'a>),
}


// A control symbol is a pair (control_word, property)
// In the RTF specifiaction, it refer to 'control word entity'
type ControlSymbol<'a> = (ControlWord<'a>, Property);

// Parameters for a control word
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Property {
    On,  // 1
    Off, // 0
    Value(i32),
    None, // No parameter
}

impl Property {
    fn as_bool(&self) -> bool {
        match self {
            Property::On => true,
            Property::Off => false,
            Property::None => true,
            Property::Value(val) => if *val == 1 { true } else { false },
        }
    }

    fn get_value(&self) -> i32 {
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

    Unknown(&'a str),
}

impl<'a> ControlWord<'a> {
    pub fn from(input: &str) -> ControlSymbol {
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
            Property::Value(suffix.parse::<i32>().expect(&format!("[Lexer] Unable to parse {} as integer", &suffix)))
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
            _ => ControlWord::Unknown(prefix),
        };
        return (control_word, property);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ControlWord, Property};

    #[test]
    fn control_word_from_input_test() {
        let input = r"\rtf1";
        assert_eq!(ControlWord::from(input), (ControlWord::Rtf, Property::Value(1)))
    }

    #[test]
    fn control_word_with_negative_parameter() {
        let input = r"\rtf-1";
        assert_eq!(ControlWord::from(input), (ControlWord::Rtf, Property::Value(-1)))
    }
}
