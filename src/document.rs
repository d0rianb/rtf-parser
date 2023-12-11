use crate::header::RtfHeader;
use crate::StyleBlock;

#[derive(Debug, Default)]
pub struct RtfDocument<'a> {
    pub header: RtfHeader<'a>,
    pub body: Vec<StyleBlock<'a>>,
}

impl<'a> RtfDocument<'a> {
    pub fn get_text(&self) -> String {
        let mut result = String::new();
        for style_block in &self.body {
            result.push_str(style_block.text);
        }
        return result;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{Lexer, Parser};

    #[test]
    fn test_get_text() {
        let rtf = r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#;
        let tokens = Lexer::scan(rtf);
        let document = Parser::new(tokens).parse();
        assert_eq!(
            document.get_text(),
            "Voici du texte en gras."
        )
    }

}