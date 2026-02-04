# text2artfont

A Rust crate for converting text into ASCII art using custom fonts.

## Features

- Simple API for quick text rendering
- Builder pattern for advanced configuration
- Embedded default font
- Load custom fonts from JSON files
- Support for alignment, spacing, and line spacing

## Quick Start

```rust
use text2artfont::render;

let art = render("Hello");
println!("{}", art);
```

### Using ANSI Compact Font

```rust
use text2artfont::{ansi_compact_font, render_with_font};

let font = ansi_compact_font();
let art = render_with_font("Hello", &font);
println!("{}", art);
```

Or use the convenience function:

```rust
use text2artfont::render_ansi_compact;

let art = render_ansi_compact("Hello");
println!("{}", art);
```

## Advanced Usage

### Using the Builder Pattern

```rust
use text2artfont::AsciiArtBuilder;

let art = AsciiArtBuilder::new()
    .text("Hello World")
    .spacing(2)
    .align_center()
    .line_spacing(1)
    .build();
```

### Loading Custom Fonts

```rust
use text2artfont::{render_with_font, loader::load_from_json};
use std::path::Path;

let font = load_from_json(Path::new("my_font.json"))?;
let art = render_with_font("Hello", &font);
```

## Font JSON Format

Fonts are stored as JSON files with the following structure:

```json
{
  "width": 7,
  "height": 7,
  "spacing": 1,
  "glyphs": {
    "A": [
      "  ▄▄  ",
      "  ██  ",
      " ████▄ ",
      " ██ ██ ",
      " ██ ██ ",
      "      ",
      "      "
    ]
  }
}
```
