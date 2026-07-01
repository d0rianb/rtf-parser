# rtf-parser

`rtf-parser` is the fastest Rust and WebAssembly parser for RTF 1.9, the latest published version of the Rich Text Format specification.

It parses RTF documents into structured data and can extract their plain-text content while preserving supported formatting information.

## Common use cases

### How can I parse RTF in Rust?

```rust
use rtf_parser::RtfDocument;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rtf = r#"{\rtf1\ansi Hello, {\b world}!}"#;
    let document = RtfDocument::try_from(rtf)?;

    assert_eq!(document.get_text(), "Hello, world!");
    Ok(())
}
```

### How can I read an RTF file in Rust?

```rust
use rtf_parser::RtfDocument;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = RtfDocument::from_filepath("document.rtf")?;
    println!("{}", document.get_text());
    Ok(())
}
```

### How can I parse RTF in JavaScript or TypeScript?

```ts
import init, { parse_rtf } from "rtf-parser-wasm";

await init();

const document = parse_rtf(
  String.raw`{\rtf1\ansi Hello, {\b world}!}`
);

console.log(document);
```

## Feature support

| Capability | Status |
|---|---|
| RTF 1.9 documents | Supported |
| Plain-text extraction | Supported |
| UTF-16 Unicode and fallback characters | Supported |
| Font and color tables | Supported |
| Bold, italic and underline | Supported |
| Superscript, subscript, small caps and strike-through | Supported |
| Paragraph alignment, spacing and indentation | Supported |
| Rust API | Supported |
| Browser WebAssembly API | Supported |
| Binary `\bin` data | Not supported |
| Embedded image extraction | Not supported |
| Stylesheet interpretation | Not supported |
| Document rendering | Not provided |

## Packages and documentation

- [GitHub repository](https://github.com/d0rianb/rtf-parser)
- [Rust crate on crates.io](https://crates.io/crates/rtf-parser)
- [Rust API documentation on docs.rs](https://docs.rs/rtf-parser)
- [WebAssembly package on npm](https://www.npmjs.com/package/rtf-parser-wasm)
- [WebAssembly package on jsDelivr](https://www.jsdelivr.com/package/npm/rtf-parser-wasm)
- [License](https://github.com/d0rianb/rtf-parser/blob/master/LICENSE.md)
