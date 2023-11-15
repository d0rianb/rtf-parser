# rtf-parser
A Rust RTF parser &amp; lexer library designed for speed and memory efficiency.

The library is split into 2 main components:
1. The lexer
2. The parser

The lexer scan the document and return a `Vec<Token>` which represent the RTF file in a code-understandable manner.
To use it : 
```rust
use rtf_parser::{Lexer, Parser, Token};

let tokens: Vec<Token> = Lexer::scan("<rtf>");
```

These tokens can then be passed to the parser to transcript it to a real document : `RtfDocument`.
```rust
let parser = Parser::new(tokens);
let doc: RtfDocument = parser.parse();
```

An `RtfDocument` is composed with : 
- the **header**, containing among others the font table and the encoding.
- the **body**, which is a `Vec<StyledBlock>`

A `StyledBlock` contains all the information about the formatting of a specific block of text.  
It contains a `Painter` and the text (`&str`).
The `Painter` is defined below, and the rendering implementation depends on the user. For now, it only supports font, bold, italic and underline.
```rust
struct Painter {
    font_ref: FontRef,
    font_size: u16,
    bold: bool,
    italic: bool,
    underline: bool,
}
```

The parser could also return the text without any formatting information, with the `to_text()` method.

```rust
let rtf = r#"{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par}"#;
let tokens = Lexer::scan(rtf);
let text = Parser::new(tokens).to_text();
assert_eq!(text, "Voici du texte en gras.");
```

## Examples 
A complete example of rtf parsing is presented below : 
```rust
use rtf_parser::{Lexer, Parser};

let rtf_text = r#"{ \rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard Voici du texte en {\b gras}.\par }"#;
let tokens = Lexer::scan(rtf_text);
let doc = Parser::new(tokens).parse();

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
            text: "Voici du texte en ",
        },
        StyleBlock {
            painter: Painter { font_ref: 0, font_size: 0, bold: true, italic: false, underline: false },
            text: "gras",
        },
        StyleBlock {
            painter: Painter { font_ref: 0, font_size: 0, bold: false, italic: false, underline: false },
            text: ".",
        },
    ]
);
```

