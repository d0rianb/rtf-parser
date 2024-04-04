use std::fmt;

use crate::lexer::LexerError;

/// Parser representation of an RTF token
#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone)]
pub enum Token<'a> {
    PlainText(&'a str),
    EscapedChar(char),
    OpeningBracket,
    ClosingBracket,
    CRLF,                 // Line-return \n
    IgnorableDestination, // \*\ <destination-name>
    ControlSymbol(ControlSymbol<'a>),
}

#[allow(dead_code)]
impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        return match self {
            Token::PlainText(text)        => write!(f, r"PlainText : {:?}", *text),
            Token::EscapedChar(ch)        => write!(f, r"EscapedChar : {ch}"),
            Token::OpeningBracket         => write!(f, "OpeningBracket"),
            Token::ClosingBracket         => write!(f, "ClosingBracket"),
            Token::CRLF                   => write!(f, "CRLF"),
            Token::IgnorableDestination   => write!(f, "IgnorableDestination"),
            Token::ControlSymbol(symbol)  => write!(f, "ControlSymbol : {:?}", symbol),
        };
    }
}

/// A control symbol is a pair (control_word, property)
/// In the RTF specification, it refers to 'control word entity'
pub type ControlSymbol<'a> = (ControlWord<'a>, Property);

/// Parameters for a control word
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

    Unicode,

    FontTable,
    FontCharset,
    FontNumber,
    FontSize, // Expressed in half point
    ColorNumber,

    ColorTable,
    FileTable,
    StyleSheet,

    Italic,
    Bold,
    Underline,
    UnderlineNone,
    Superscript, // 5th
    Subscript,   // H20
    Smallcaps,
    Strikethrough,

    Par,  // New paragraph
    Pard, // Resets to default paragraph properties
    Sectd,
    Plain,
    ParStyle,  // Designates paragraph style. If a paragraph style is specified, style properties must be specified with the paragraph. N references an entry in the stylesheet.
    ParDefTab, // Tab width
    // Paragraph indent
    FirstLineIdent,
    LeftIndent,
    RightIndent,
    // Paragraph alignment
    LeftAligned,
    RightAligned,
    Center,
    Justify,
    // Paragraph spacing
    SpaceBefore,
    SpaceAfter,
    SpaceBetweenLine,
    SpaceLineMul, // Line spacing multiple. Indicates that the current line spacing is a multiple of "Single" line spacing. This control word can follow only the \sl control word and works in conjunction with it.

    Unknown(&'a str),
}

impl<'a> ControlWord<'a> {
    // https://www.biblioscape.com/rtf15_spec.htm
    // version 1.5 should be compatible with 1.9
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

        #[rustfmt::skip]
        let control_word = match prefix {
            r"\rtf"           => ControlWord::Rtf,
            r"\ansi"          => ControlWord::Ansi,
            // Header
            r"\fonttbl"       => ControlWord::FontTable,
            r"\colortbl"      => ControlWord::ColorTable,
            r"\filetbl"       => ControlWord::FileTable,
            r"\stylesheet"    => ControlWord::StyleSheet,
            // Font
            r"\fcharset"      => ControlWord::FontCharset,
            r"\f"             => ControlWord::FontNumber,
            r"\fs"            => ControlWord::FontSize,
            r"\cf"            => ControlWord::ColorNumber,
            // Format
            r"\i"             => ControlWord::Italic,
            r"\b"             => ControlWord::Bold,
            r"\u"             => ControlWord::Unicode,
            r"\ul"            => ControlWord::Underline,
            r"\ulnone"        => ControlWord::UnderlineNone,
            r"\super"         => ControlWord::Superscript,
            r"\sub"           => ControlWord::Subscript,
            r"\scaps"         => ControlWord::Smallcaps,
            r"\strike"        => ControlWord::Strikethrough,
            // Paragraph
            r"\par"           => ControlWord::Par,
            r"\pard"          => ControlWord::Pard,
            r"\sectd"         => ControlWord::Sectd,
            r"\plain"         => ControlWord::Plain,
            r"\s"             => ControlWord::ParStyle,
            r"\pardeftab"     => ControlWord::ParDefTab,
            // Paragraph alignment
            r"\ql"            => ControlWord::LeftAligned,
            r"\qr"            => ControlWord::RightAligned,
            r"\qj"            => ControlWord::Justify,
            r"\qc"            => ControlWord::Center,
            // Paragraph indent
            r"\fi"             => ControlWord::FirstLineIdent,
            r"\ri"            => ControlWord::RightIndent,
            r"\li"            => ControlWord::LeftIndent,
            // Paragraph Spacing
            r"\sb"            => ControlWord::SpaceBefore,
            r"\sa"            => ControlWord::SpaceAfter,
            r"\sl"            => ControlWord::SpaceBetweenLine,
            r"\slmul"         => ControlWord::SpaceLineMul,
            // Unknown
            _                 => ControlWord::Unknown(prefix),
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
