# rtf-parser
A Rust RTF parser &amp; lexer library designed for speed and memory efficiency, with no external dependencies.

The library is split into 2 main components:
1. The lexer
2. The parser

The lexer scan the document and return a `Vec<Token>` which represent the RTF file in a code-understandable manner.
To use it : 
```rust
use rtf_parser::{Lexer, Parser, Token};

let tokens: Vec<Token> = Lexer::scan("<rtf>")?;
```

These tokens can then be passed to the parser to transcript it to a real document : `RtfDocument`.
```rust
let parser = Parser::new(tokens)?;
let doc: RtfDocument = parser.parse()?;
```

An `RtfDocument` is composed with : 
- the **header**, containing among others the font table and the encoding.
- the **body**, which is a `Vec<StyledBlock>`

A `StyledBlock` contains all the information about the formatting of a specific block of text.  
It contains a `Painter` and the text (`String`).
The `Painter` is defined below, and the rendering implementation depends on the user.
```rust
pub struct Painter {
    pub font_ref: FontRef,
    pub font_size: u16,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub superscript: bool,
    pub subscript: bool,
    pub smallcaps: bool,
    pub strike: bool,
}
```

The layout information are exposed in the `paragraph` property :
```rust
pub struct Paragraph {
    pub alignment: Alignment,
    pub spacing: Spacing,
    pub indent: Indentation,
    pub tab_width: i32,
}
```
It defined the way a blovk is aligned, what spacing it uses, etc...

You also can extract the text without any formatting information, with the `to_text()` method of the `RtfDocument` struct.

```rust
let rtf = r#"{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par}"#;
let tokens = Lexer::scan(rtf)?;
let document = Parser::new(tokens)?;
let text = document.to_text();
assert_eq!(text, "Voici du texte en gras.");
```

## Examples 
A complete example of rtf parsing is presented below : 
```rust
use rtf_parser::{Lexer, Parser};

let rtf_text = r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#;
let tokens = Lexer::scan(rtf_text)?;
let doc = Parser::new(tokens).parse()?;

assert_eq!(
    doc.header,
    RtfHeader {
        character_set: Ansi,
        font_table: FontTable::from([
            (0, Font { name: "Helvetica", character_set: 0, font_family: Swiss })
        ])
    }
);
assert_eq!(
    doc.body,
    [
        StyleBlock {
            painter: Painter { font_ref: 0, font_size: 0, bold: false, italic: false, underline: false },
            paragraph: Paragraph { 
                alignment: LeftAligned, 
                spacing: Spacing { before: 0, after: 0, between_line: Auto, line_multiplier: 0, },
                indent: Indentation { left: 0, right: 0, first_line: 0, },
                tab_width: 0,
            },            
            text: "Voici du texte en ",
        },
        StyleBlock {
            painter: Painter { font_ref: 0, font_size: 0, bold: true, italic: false, underline: false },
            paragraph: Paragraph {
                alignment: LeftAligned,
                spacing: Spacing { before: 0, after: 0, between_line: Auto, line_multiplier: 0, },
                indent: Indentation { left: 0, right: 0, first_line: 0, },
                tab_width: 0,
            },
            text: "gras",
        },
        StyleBlock {
            painter: Painter { font_ref: 0, font_size: 0, bold: false, italic: false, underline: false },
            paragraph: Paragraph {
                alignment: LeftAligned,
                spacing: Spacing { before: 0, after: 0, between_line: Auto, line_multiplier: 0, },
                indent: Indentation { left: 0, right: 0, first_line: 0, },
                tab_width: 0,
            },
            text: ".",
        },
    ]
);
```

