use std::collections::HashMap;

/// Represents a single character glyph as a 2D grid of characters.
pub type Glyph = Vec<String>;

/// A font containing glyphs for rendering ASCII art.
#[derive(Debug, Clone)]
pub struct Font {
    /// Width of each character in the font (in characters)
    pub width: usize,
    /// Height of each character in the font (in lines)
    pub height: usize,
    /// Horizontal spacing between characters
    pub spacing: usize,
    /// Map of characters to their glyph representations
    pub glyphs: HashMap<char, Glyph>,
}

impl Font {
    /// Create a new font with the specified dimensions and glyphs.
    pub fn new(width: usize, height: usize, spacing: usize) -> Self {
        Self {
            width,
            height,
            spacing,
            glyphs: HashMap::new(),
        }
    }

    /// Add a glyph to the font.
    pub fn add_glyph(&mut self, ch: char, glyph: Glyph) {
        self.glyphs.insert(ch, glyph);
    }

    /// Get a glyph for a character, returning None if not found.
    pub fn get_glyph(&self, ch: char) -> Option<&Glyph> {
        self.glyphs.get(&ch)
    }

    /// Get a glyph for a character, or return a default placeholder glyph.
    pub fn get_glyph_or_placeholder(&self, ch: char) -> Glyph {
        self.glyphs.get(&ch).cloned().unwrap_or_else(|| {
            // Return a placeholder glyph (empty space with correct height)
            vec![" ".repeat(self.width); self.height]
        })
    }
}

impl Default for Font {
    fn default() -> Self {
        default_font()
    }
}

/// Returns the default embedded font matching the example style.
pub fn default_font() -> Font {
    let mut font = Font::new(7, 7, 1);
    
    // Extract glyphs from the example - I'll need to parse the user's example carefully
    // For now, I'll create a structure that can be populated
    
    // Lowercase letters
    add_lowercase_glyphs(&mut font);
    
    // Uppercase letters  
    add_uppercase_glyphs(&mut font);
    
    // Numbers
    add_number_glyphs(&mut font);
    
    // Special characters
    add_special_glyphs(&mut font);
    
    font
}

fn add_lowercase_glyphs(font: &mut Font) {
    // Extracted from the example output
    // a
    font.add_glyph('a', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▀▀█▄ ".to_string(),
        "▄█▀██ ".to_string(),
        "▀█▄██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // b
    font.add_glyph('b', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄████ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // c
    font.add_glyph('c', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄████ ".to_string(),
        " ██    ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // d
    font.add_glyph('d', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄█▀█▄ ".to_string(),
        " ██▄█▀ ".to_string(),
        " ▀█▄▄▄ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // e
    font.add_glyph('e', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄███▄ ".to_string(),
        " ▀████ ".to_string(),
        " ▀███▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // f
    font.add_glyph('f', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄███▄ ".to_string(),
        " ████▄ ".to_string(),
        " ████▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // g
    font.add_glyph('g', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄█▀█▄ ".to_string(),
        " ██▄█▀ ".to_string(),
        " ▀█▄▄▄ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // h
    font.add_glyph('h', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // i
    font.add_glyph('i', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ".to_string(),
        " ██ ".to_string(),
        " ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // j
    font.add_glyph('j', vec![
        "      ".to_string(),
        "      ".to_string(),
        "   ██ ".to_string(),
        "   ██ ".to_string(),
        " ▀██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // k
    font.add_glyph('k', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ██ ".to_string(),
        " ████ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // l
    font.add_glyph('l', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ".to_string(),
        " ██ ".to_string(),
        " ▀██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // m
    font.add_glyph('m', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ████▄ ".to_string(),
        " ██▄██ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // n
    font.add_glyph('n', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // o
    font.add_glyph('o', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄████ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // p
    font.add_glyph('p', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ████▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // q
    font.add_glyph('q', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄█▀█▄ ".to_string(),
        " ██▄█▀ ".to_string(),
        " ▀█▄▄▄ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // r
    font.add_glyph('r', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ████▄ ".to_string(),
        " ██    ".to_string(),
        " ██    ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // s
    font.add_glyph('s', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ▄███▄ ".to_string(),
        " ▀████ ".to_string(),
        " ▀███▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // t
    font.add_glyph('t', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ████▄ ".to_string(),
        "   ██ ".to_string(),
        "   ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // u
    font.add_glyph('u', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // v
    font.add_glyph('v', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "  ▀██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // w
    font.add_glyph('w', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ██ ".to_string(),
        " ██▄██ ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // x - missing from example, creating placeholder
    font.add_glyph('x', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ██ ".to_string(),
        "  ██  ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // y
    font.add_glyph('y', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "  ▀██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // z
    font.add_glyph('z', vec![
        "      ".to_string(),
        "      ".to_string(),
        " █████ ".to_string(),
        "  ██  ".to_string(),
        " █████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
}

fn add_uppercase_glyphs(font: &mut Font) {
    // Extracted from the example output
    // A
    font.add_glyph('A', vec![
        " ▄▄ ".to_string(),
        " ██ ".to_string(),
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // B
    font.add_glyph('B', vec![
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ████▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // C
    font.add_glyph('C', vec![
        " ▄████ ".to_string(),
        " ██    ".to_string(),
        " ██    ".to_string(),
        " ██    ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // D
    font.add_glyph('D', vec![
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ████▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // E
    font.add_glyph('E', vec![
        " █████ ".to_string(),
        " ██    ".to_string(),
        " ████  ".to_string(),
        " ██    ".to_string(),
        " █████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // F
    font.add_glyph('F', vec![
        " █████ ".to_string(),
        " ██    ".to_string(),
        " ████  ".to_string(),
        " ██    ".to_string(),
        " ██    ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // G
    font.add_glyph('G', vec![
        " ▄████ ".to_string(),
        " ██    ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // H
    font.add_glyph('H', vec![
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " █████ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // I
    font.add_glyph('I', vec![
        " ███ ".to_string(),
        "  ██ ".to_string(),
        "  ██ ".to_string(),
        "  ██ ".to_string(),
        " ███ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // J
    font.add_glyph('J', vec![
        "   ██ ".to_string(),
        "   ██ ".to_string(),
        "   ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀███ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // K
    font.add_glyph('K', vec![
        " ██ ██ ".to_string(),
        " ████ ".to_string(),
        " ███  ".to_string(),
        " ████ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // L
    font.add_glyph('L', vec![
        " ██    ".to_string(),
        " ██    ".to_string(),
        " ██    ".to_string(),
        " ██    ".to_string(),
        " █████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // M
    font.add_glyph('M', vec![
        " ██ ██ ".to_string(),
        " ████▄ ".to_string(),
        " ██▄██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // N
    font.add_glyph('N', vec![
        " ██ ██ ".to_string(),
        " ████ ".to_string(),
        " ██▄██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // O
    font.add_glyph('O', vec![
        " ▄████ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // P
    font.add_glyph('P', vec![
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ████▄ ".to_string(),
        " ██    ".to_string(),
        " ██    ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // Q
    font.add_glyph('Q', vec![
        " ▄████ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██▄██ ".to_string(),
        " ▀████▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // R
    font.add_glyph('R', vec![
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ████▄ ".to_string(),
        " ████ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // S
    font.add_glyph('S', vec![
        " ▄███▄ ".to_string(),
        " ██    ".to_string(),
        " ▀███▄ ".to_string(),
        "    ██ ".to_string(),
        " ▀███▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // T
    font.add_glyph('T', vec![
        " █████ ".to_string(),
        "  ██  ".to_string(),
        "  ██  ".to_string(),
        "  ██  ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // U
    font.add_glyph('U', vec![
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // V
    font.add_glyph('V', vec![
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀███ ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // W
    font.add_glyph('W', vec![
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ██▄██ ".to_string(),
        " ████▄ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // X - missing from example, creating placeholder
    font.add_glyph('X', vec![
        " ██ ██ ".to_string(),
        " ▀███ ".to_string(),
        "  ██  ".to_string(),
        " ████ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // Y
    font.add_glyph('Y', vec![
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀███ ".to_string(),
        "  ██  ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // Z
    font.add_glyph('Z', vec![
        " █████ ".to_string(),
        "   ██ ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        " █████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
}

fn add_number_glyphs(font: &mut Font) {
    // Extracted from the example output
    // 1
    font.add_glyph('1', vec![
        "  ▄▄ ".to_string(),
        " ▄███ ".to_string(),
        "   ██ ".to_string(),
        "   ██ ".to_string(),
        " ▄████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 2
    font.add_glyph('2', vec![
        " ▄▄▄▄ ".to_string(),
        "▄██▀▀██▄ ".to_string(),
        "   ███ ".to_string(),
        " ▄███▄ ".to_string(),
        " ▀████▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 3
    font.add_glyph('3', vec![
        " ▄▄▄▄ ".to_string(),
        "▄██▀▀██▄ ".to_string(),
        "   ▄██▀ ".to_string(),
        "▄██▀▀██ ".to_string(),
        " ▀▀▀▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 4
    font.add_glyph('4', vec![
        " ▄▄  ▄▄ ".to_string(),
        "▄███ ███▄ ".to_string(),
        " ▀██████▀ ".to_string(),
        "    ██ ".to_string(),
        "    ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 5
    font.add_glyph('5', vec![
        " ▄▄▄▄▄ ".to_string(),
        "▄██▀▀▀▀ ".to_string(),
        " ▀▀▀▀██▄ ".to_string(),
        "▄██▀▀██ ".to_string(),
        " ▀▀▀▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 6
    font.add_glyph('6', vec![
        "  ▄▄▄ ".to_string(),
        " ▄██▀▀ ".to_string(),
        "▄██▀▀▀ ".to_string(),
        "▄██▀▀██ ".to_string(),
        " ▀▀▀▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 7
    font.add_glyph('7', vec![
        "▄▄▄▄▄▄▄ ".to_string(),
        "▀▀▀▀▀███ ".to_string(),
        "    ▄██ ".to_string(),
        "   ███ ".to_string(),
        "  ███ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 8
    font.add_glyph('8', vec![
        " ▄▄▄ ".to_string(),
        "▄██▀▀██▄ ".to_string(),
        " ▀▀▀▀ ".to_string(),
        "▄██▀▀██▄ ".to_string(),
        " ▀▀▀▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 9
    font.add_glyph('9', vec![
        " ▄▄▄ ".to_string(),
        "▄██▀▀██ ".to_string(),
        " ▀▀▀▀██ ".to_string(),
        "   ▄██▀ ".to_string(),
        " ▀▀▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // 0
    font.add_glyph('0', vec![
        " ▄▄▄ ".to_string(),
        "▄██▀▀██▄ ".to_string(),
        "▄██  ███▄ ".to_string(),
        "▄██▀▀██▄ ".to_string(),
        " ▀▀▀▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
}

fn add_special_glyphs(font: &mut Font) {
    // Extracted from the example output
    // =
    font.add_glyph('=', vec![
        "      ".to_string(),
        "      ".to_string(),
        " █████ ".to_string(),
        "      ".to_string(),
        " █████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // +
    font.add_glyph('+', vec![
        "      ".to_string(),
        "  ██  ".to_string(),
        " █████ ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // <
    font.add_glyph('<', vec![
        "      ".to_string(),
        "   ██ ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        "  ██  ".to_string(),
        "   ██ ".to_string(),
        "      ".to_string(),
    ]);
    
    // >
    font.add_glyph('>', vec![
        "      ".to_string(),
        " ██   ".to_string(),
        "  ██  ".to_string(),
        "   ██ ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        "      ".to_string(),
    ]);
    
    // ,
    font.add_glyph(',', vec![
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        "      ".to_string(),
    ]);
    
    // .
    font.add_glyph('.', vec![
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // /
    font.add_glyph('/', vec![
        "      ".to_string(),
        "    ██ ".to_string(),
        "   ██ ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // ?
    font.add_glyph('?', vec![
        " ▄███▄ ".to_string(),
        " ██ ██ ".to_string(),
        "   ██ ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
    ]);
    
    // ;
    font.add_glyph(';', vec![
        "      ".to_string(),
        "      ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        "      ".to_string(),
    ]);
    
    // "
    font.add_glyph('"', vec![
        " ██ ██ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // :
    font.add_glyph(':', vec![
        "      ".to_string(),
        "      ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // `
    font.add_glyph('`', vec![
        " ██   ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // ~
    font.add_glyph('~', vec![
        "      ".to_string(),
        "      ".to_string(),
        " ██ ██ ".to_string(),
        " ██▄██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // !
    font.add_glyph('!', vec![
        "  ██  ".to_string(),
        "  ██  ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // @
    font.add_glyph('@', vec![
        " ▄███▄ ".to_string(),
        " ██ ██ ".to_string(),
        " ██▄██ ".to_string(),
        " ██    ".to_string(),
        " ▀███▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // #
    font.add_glyph('#', vec![
        " ██ ██ ".to_string(),
        " █████ ".to_string(),
        " ██ ██ ".to_string(),
        " █████ ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // $
    font.add_glyph('$', vec![
        "  ██  ".to_string(),
        " ▄███▄ ".to_string(),
        " ██    ".to_string(),
        " ▀███▄ ".to_string(),
        " ▄███▀ ".to_string(),
        "  ██  ".to_string(),
        "      ".to_string(),
    ]);
    
    // %
    font.add_glyph('%', vec![
        " ██   ██ ".to_string(),
        "   ██ ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        " ██   ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // ^
    font.add_glyph('^', vec![
        "  ██  ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // &
    font.add_glyph('&', vec![
        " ▄███ ".to_string(),
        " ██   ".to_string(),
        " ▄███ ".to_string(),
        " ██ ██ ".to_string(),
        " ▀███▀ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // *
    font.add_glyph('*', vec![
        " ██ ██ ".to_string(),
        "  ██  ".to_string(),
        " █████ ".to_string(),
        "  ██  ".to_string(),
        " ██ ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // (
    font.add_glyph('(', vec![
        "   ██ ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        "  ██  ".to_string(),
        "   ██ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // )
    font.add_glyph(')', vec![
        " ██   ".to_string(),
        "  ██  ".to_string(),
        "   ██ ".to_string(),
        "  ██  ".to_string(),
        " ██   ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // -
    font.add_glyph('-', vec![
        "      ".to_string(),
        "      ".to_string(),
        " █████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
    
    // _
    font.add_glyph('_', vec![
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
        " █████ ".to_string(),
        "      ".to_string(),
        "      ".to_string(),
    ]);
}

/// Returns the ANSI Compact font.
///
/// This is a more compact font style with smaller character dimensions.
pub fn ansi_compact_font() -> Font {
    let mut font = Font::new(6, 4, 0);
    
    add_ansi_compact_lowercase(&mut font);
    add_ansi_compact_uppercase(&mut font);
    add_ansi_compact_numbers(&mut font);
    add_ansi_compact_special(&mut font);
    
    font
}

fn add_ansi_compact_lowercase(font: &mut Font) {
    // Extracted from the example output - ANSI Compact font
    // Analyzing the rendered output to extract glyphs
    
    // a
    font.add_glyph('a', vec![
        " ▄▄▄ ".to_string(),
        "██▀██".to_string(),
        "██▀██".to_string(),
        "     ".to_string(),
    ]);
    
    // b
    font.add_glyph('b', vec![
        "██▄  ".to_string(),
        "██▀██".to_string(),
        "██▄▄ ".to_string(),
        "     ".to_string(),
    ]);
    
    // c
    font.add_glyph('c', vec![
        " ▄▄▄ ".to_string(),
        "██   ".to_string(),
        " ▀▀▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // d
    font.add_glyph('d', vec![
        "  ▄██".to_string(),
        " ██▀██".to_string(),
        " ▀▀▀██".to_string(),
        "     ".to_string(),
    ]);
    
    // e
    font.add_glyph('e', vec![
        " ▄▄▄ ".to_string(),
        "██▄▄ ".to_string(),
        " ▀▀▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // f
    font.add_glyph('f', vec![
        " ▄▄▄ ".to_string(),
        "██▄  ".to_string(),
        "██   ".to_string(),
        "     ".to_string(),
    ]);
    
    // g
    font.add_glyph('g', vec![
        " ▄▄▄ ".to_string(),
        "██   ".to_string(),
        "██▄▄ ".to_string(),
        " ▀▀▀ ".to_string(),
    ]);
    
    // h
    font.add_glyph('h', vec![
        "██▄  ".to_string(),
        "██▀██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // i
    font.add_glyph('i', vec![
        "██ ".to_string(),
        "   ".to_string(),
        "██ ".to_string(),
        "   ".to_string(),
    ]);
    
    // j
    font.add_glyph('j', vec![
        "  ██".to_string(),
        "    ".to_string(),
        "██▄ ".to_string(),
        " ▀▀ ".to_string(),
    ]);
    
    // k
    font.add_glyph('k', vec![
        "██ ██".to_string(),
        "██▄  ".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // l
    font.add_glyph('l', vec![
        "██  ".to_string(),
        "██  ".to_string(),
        " ▀▀ ".to_string(),
        "    ".to_string(),
    ]);
    
    // m
    font.add_glyph('m', vec![
        "██▄██".to_string(),
        "██▀██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // n
    font.add_glyph('n', vec![
        "██▄  ".to_string(),
        "██▀██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // o
    font.add_glyph('o', vec![
        " ▄▄▄ ".to_string(),
        "██ ██".to_string(),
        " ▀▀▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // p
    font.add_glyph('p', vec![
        "██▄  ".to_string(),
        "██▀██".to_string(),
        "██   ".to_string(),
        "     ".to_string(),
    ]);
    
    // q
    font.add_glyph('q', vec![
        " ▄▄▄ ".to_string(),
        "██ ██".to_string(),
        " ▀▀██".to_string(),
        "     ".to_string(),
    ]);
    
    // r
    font.add_glyph('r', vec![
        "██▄  ".to_string(),
        "██   ".to_string(),
        "██   ".to_string(),
        "     ".to_string(),
    ]);
    
    // s
    font.add_glyph('s', vec![
        " ▄▄▄ ".to_string(),
        " ▀▀▄ ".to_string(),
        "▄▄▄▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // t
    font.add_glyph('t', vec![
        "██▄ ".to_string(),
        " ██ ".to_string(),
        " ██ ".to_string(),
        "    ".to_string(),
    ]);
    
    // u
    font.add_glyph('u', vec![
        "██ ██".to_string(),
        "██ ██".to_string(),
        " ▀▀▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // v
    font.add_glyph('v', vec![
        "██ ██".to_string(),
        "██ ██".to_string(),
        " ▀██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // w
    font.add_glyph('w', vec![
        "██ ██".to_string(),
        "██▀██".to_string(),
        "██▄██".to_string(),
        "     ".to_string(),
    ]);
    
    // x
    font.add_glyph('x', vec![
        "██ ██".to_string(),
        " ▀██ ".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // y
    font.add_glyph('y', vec![
        "██ ██".to_string(),
        "██ ██".to_string(),
        " ▀▀▀ ".to_string(),
        "  ██ ".to_string(),
    ]);
    
    // z
    font.add_glyph('z', vec![
        "▄▄▄▄ ".to_string(),
        "  ██ ".to_string(),
        "▄▄▄▄ ".to_string(),
        "     ".to_string(),
    ]);
}

fn add_ansi_compact_uppercase(font: &mut Font) {
    // A
    font.add_glyph('A', vec![
        " ▄▄▄ ".to_string(),
        "██▀██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // B
    font.add_glyph('B', vec![
        "██▄  ".to_string(),
        "██▀██".to_string(),
        "██▄▄ ".to_string(),
        "     ".to_string(),
    ]);
    
    // C
    font.add_glyph('C', vec![
        " ▄▄▄ ".to_string(),
        "██   ".to_string(),
        " ▀▀▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // D
    font.add_glyph('D', vec![
        "██▄  ".to_string(),
        "██ ██".to_string(),
        "██▄▄ ".to_string(),
        "     ".to_string(),
    ]);
    
    // E
    font.add_glyph('E', vec![
        "▄▄▄▄ ".to_string(),
        "██▄  ".to_string(),
        "▄▄▄▄ ".to_string(),
        "     ".to_string(),
    ]);
    
    // F
    font.add_glyph('F', vec![
        "▄▄▄▄ ".to_string(),
        "██▄  ".to_string(),
        "██   ".to_string(),
        "     ".to_string(),
    ]);
    
    // G
    font.add_glyph('G', vec![
        " ▄▄▄ ".to_string(),
        "██   ".to_string(),
        "██▄▄ ".to_string(),
        " ▀▀▀ ".to_string(),
    ]);
    
    // H
    font.add_glyph('H', vec![
        "██ ██".to_string(),
        "██▄██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // I
    font.add_glyph('I', vec![
        "▄▄▄ ".to_string(),
        " ██ ".to_string(),
        "▄▄▄ ".to_string(),
        "     ".to_string(),
    ]);
    
    // J
    font.add_glyph('J', vec![
        "   ██".to_string(),
        "   ██".to_string(),
        "██▄ ".to_string(),
        " ▀▀ ".to_string(),
    ]);
    
    // K
    font.add_glyph('K', vec![
        "██ ██".to_string(),
        "██▄  ".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // L
    font.add_glyph('L', vec![
        "██   ".to_string(),
        "██   ".to_string(),
        "▄▄▄▄ ".to_string(),
        "     ".to_string(),
    ]);
    
    // M
    font.add_glyph('M', vec![
        "██▄██".to_string(),
        "██▀██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // N
    font.add_glyph('N', vec![
        "██▄██".to_string(),
        "██▀██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // O
    font.add_glyph('O', vec![
        " ▄▄▄ ".to_string(),
        "██ ██".to_string(),
        " ▀▀▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // P
    font.add_glyph('P', vec![
        "██▄  ".to_string(),
        "██▀██".to_string(),
        "██   ".to_string(),
        "     ".to_string(),
    ]);
    
    // Q
    font.add_glyph('Q', vec![
        " ▄▄▄ ".to_string(),
        "██ ██".to_string(),
        " ▀▀██".to_string(),
        "     ".to_string(),
    ]);
    
    // R
    font.add_glyph('R', vec![
        "██▄  ".to_string(),
        "██▀██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // S
    font.add_glyph('S', vec![
        " ▄▄▄ ".to_string(),
        " ▀▀▄ ".to_string(),
        "▄▄▄▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // T
    font.add_glyph('T', vec![
        "▄▄▄▄ ".to_string(),
        "  ██ ".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // U
    font.add_glyph('U', vec![
        "██ ██".to_string(),
        "██ ██".to_string(),
        " ▀▀▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // V
    font.add_glyph('V', vec![
        "██ ██".to_string(),
        "██ ██".to_string(),
        " ▀██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // W
    font.add_glyph('W', vec![
        "██ ██".to_string(),
        "██▀██".to_string(),
        "██▄██".to_string(),
        "     ".to_string(),
    ]);
    
    // X
    font.add_glyph('X', vec![
        "██ ██".to_string(),
        " ▀██ ".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
    ]);
    
    // Y
    font.add_glyph('Y', vec![
        "██ ██".to_string(),
        "██ ██".to_string(),
        " ▀██ ".to_string(),
        "  ██ ".to_string(),
    ]);
    
    // Z
    font.add_glyph('Z', vec![
        "▄▄▄▄ ".to_string(),
        "  ██ ".to_string(),
        "▄▄▄▄ ".to_string(),
        "     ".to_string(),
    ]);
}

fn add_ansi_compact_numbers(font: &mut Font) {
    // 1
    font.add_glyph('1', vec![
        " ▄▄ ".to_string(),
        "▄██ ".to_string(),
        " ██ ".to_string(),
        "▄▄▄ ".to_string(),
    ]);
    
    // 2
    font.add_glyph('2', vec![
        " ▄▄▄ ".to_string(),
        "▄██▀ ".to_string(),
        " ██ ".to_string(),
        "▄▄▄▄ ".to_string(),
    ]);
    
    // 3
    font.add_glyph('3', vec![
        " ▄▄▄ ".to_string(),
        "▄██▀ ".to_string(),
        " ▄██▀".to_string(),
        " ▀▀▀ ".to_string(),
    ]);
    
    // 4
    font.add_glyph('4', vec![
        " ▄  ▄".to_string(),
        "▄███ ".to_string(),
        " ▀▀██".to_string(),
        "    █".to_string(),
    ]);
    
    // 5
    font.add_glyph('5', vec![
        "▄▄▄▄ ".to_string(),
        "██▄  ".to_string(),
        " ▀▀██".to_string(),
        " ▀▀▀ ".to_string(),
    ]);
    
    // 6
    font.add_glyph('6', vec![
        "  ▄▄ ".to_string(),
        " ▄██ ".to_string(),
        "██▀██".to_string(),
        " ▀▀▀ ".to_string(),
    ]);
    
    // 7
    font.add_glyph('7', vec![
        "▄▄▄▄ ".to_string(),
        "▀▀██ ".to_string(),
        "  ██ ".to_string(),
        "  ██ ".to_string(),
    ]);
    
    // 8
    font.add_glyph('8', vec![
        " ▄▄ ".to_string(),
        "██▀██".to_string(),
        " ▀▀ ".to_string(),
        "██▄██".to_string(),
    ]);
    
    // 9
    font.add_glyph('9', vec![
        " ▄▄ ".to_string(),
        "██▀██".to_string(),
        " ▀▀██".to_string(),
        "  ▄██".to_string(),
    ]);
    
    // 0
    font.add_glyph('0', vec![
        " ▄▄ ".to_string(),
        "██ ██".to_string(),
        "██ ██".to_string(),
        " ▀▀ ".to_string(),
    ]);
}

fn add_ansi_compact_special(font: &mut Font) {
    // =
    font.add_glyph('=', vec![
        "     ".to_string(),
        "▄▄▄▄ ".to_string(),
        "▀▀▀▀ ".to_string(),
        "     ".to_string(),
    ]);
    
    // +
    font.add_glyph('+', vec![
        "  ██ ".to_string(),
        "▄▄▄▄ ".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // <
    font.add_glyph('<', vec![
        "  ██ ".to_string(),
        " ██  ".to_string(),
        "██   ".to_string(),
        "     ".to_string(),
    ]);
    
    // >
    font.add_glyph('>', vec![
        "██   ".to_string(),
        " ██  ".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // ,
    font.add_glyph(',', vec![
        "     ".to_string(),
        "     ".to_string(),
        "  ██ ".to_string(),
        " ██  ".to_string(),
    ]);
    
    // .
    font.add_glyph('.', vec![
        "     ".to_string(),
        "     ".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // /
    font.add_glyph('/', vec![
        "   ██".to_string(),
        "  ██ ".to_string(),
        " ██  ".to_string(),
        "██   ".to_string(),
    ]);
    
    // ?
    font.add_glyph('?', vec![
        " ▄▄▄ ".to_string(),
        "██ ██".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // ;
    font.add_glyph(';', vec![
        "     ".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
        "  ██ ".to_string(),
    ]);
    
    // "
    font.add_glyph('"', vec![
        "██ ██".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
        "     ".to_string(),
    ]);
    
    // :
    font.add_glyph(':', vec![
        "     ".to_string(),
        "  ██ ".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // `
    font.add_glyph('`', vec![
        "██   ".to_string(),
        " ██  ".to_string(),
        "     ".to_string(),
        "     ".to_string(),
    ]);
    
    // ~
    font.add_glyph('~', vec![
        "     ".to_string(),
        "██ ██".to_string(),
        "██▄██".to_string(),
        "     ".to_string(),
    ]);
    
    // !
    font.add_glyph('!', vec![
        "  ██ ".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
        "  ██ ".to_string(),
    ]);
    
    // @
    font.add_glyph('@', vec![
        " ▄▄▄ ".to_string(),
        "██ ██".to_string(),
        "██▄██".to_string(),
        " ▀▀▀ ".to_string(),
    ]);
    
    // #
    font.add_glyph('#', vec![
        "██ ██".to_string(),
        "▄▄▄▄ ".to_string(),
        "██ ██".to_string(),
        "▄▄▄▄ ".to_string(),
    ]);
    
    // $
    font.add_glyph('$', vec![
        "  ██ ".to_string(),
        " ▄▄▄ ".to_string(),
        "██   ".to_string(),
        "▄▄▄▀ ".to_string(),
    ]);
    
    // %
    font.add_glyph('%', vec![
        "██  ██".to_string(),
        "  ██ ".to_string(),
        " ██  ".to_string(),
        "██  ██".to_string(),
    ]);
    
    // ^
    font.add_glyph('^', vec![
        "  ██ ".to_string(),
        "██ ██".to_string(),
        "     ".to_string(),
        "     ".to_string(),
    ]);
    
    // &
    font.add_glyph('&', vec![
        " ▄██ ".to_string(),
        "██   ".to_string(),
        " ▄██ ".to_string(),
        "██ ██".to_string(),
    ]);
    
    // *
    font.add_glyph('*', vec![
        "██ ██".to_string(),
        "  ██ ".to_string(),
        "▄▄▄▄ ".to_string(),
        "  ██ ".to_string(),
    ]);
    
    // (
    font.add_glyph('(', vec![
        "  ██ ".to_string(),
        " ██  ".to_string(),
        "██   ".to_string(),
        "     ".to_string(),
    ]);
    
    // )
    font.add_glyph(')', vec![
        "██   ".to_string(),
        " ██  ".to_string(),
        "  ██ ".to_string(),
        "     ".to_string(),
    ]);
    
    // -
    font.add_glyph('-', vec![
        "     ".to_string(),
        "▄▄▄▄ ".to_string(),
        "     ".to_string(),
        "     ".to_string(),
    ]);
    
    // _
    font.add_glyph('_', vec![
        "     ".to_string(),
        "     ".to_string(),
        "     ".to_string(),
        "▀▀▀▀ ".to_string(),
    ]);
}

/// Returns the Mini font.
///
/// This is a very compact font style with minimal character dimensions.
pub fn mini_font() -> Font {
    let mut font = Font::new(2, 2, 0);
    
    add_mini_lowercase(&mut font);
    add_mini_uppercase(&mut font);
    add_mini_numbers(&mut font);
    add_mini_special(&mut font);
    
    font
}

fn add_mini_lowercase(font: &mut Font) {
    // a
    font.add_glyph('a', vec![
        "▀▌".to_string(),
        "█▌".to_string(),
    ]);
    
    // b
    font.add_glyph('b', vec![
        "▛▌".to_string(),
        "▙▌".to_string(),
    ]);
    
    // c
    font.add_glyph('c', vec![
        "▛▘".to_string(),
        "▙▖".to_string(),
    ]);
    
    // d
    font.add_glyph('d', vec![
        "▛▌".to_string(),
        "▙▌".to_string(),
    ]);
    
    // e
    font.add_glyph('e', vec![
        "█▌".to_string(),
        "▙▖".to_string(),
    ]);
    
    // f
    font.add_glyph('f', vec![
        "▜▘".to_string(),
        "▐ ".to_string(),
    ]);
    
    // g
    font.add_glyph('g', vec![
        "▛▌".to_string(),
        "▙▌".to_string(),
    ]);
    
    // h
    font.add_glyph('h', vec![
        "▛▌".to_string(),
        "▌▌".to_string(),
    ]);
    
    // i
    font.add_glyph('i', vec![
        "▌ ".to_string(),
        "▌ ".to_string(),
    ]);
    
    // j
    font.add_glyph('j', vec![
        "▙▘".to_string(),
        "▐▖".to_string(),
    ]);
    
    // k
    font.add_glyph('k', vec![
        "▐ ".to_string(),
        "▙▌".to_string(),
    ]);
    
    // l
    font.add_glyph('l', vec![
        " ▛".to_string(),
        "▖▌".to_string(),
    ]);
    
    // m
    font.add_glyph('m', vec![
        "▛▌".to_string(),
        "▌▌".to_string(),
    ]);
    
    // n
    font.add_glyph('n', vec![
        "▛▌".to_string(),
        "▌▌".to_string(),
    ]);
    
    // o
    font.add_glyph('o', vec![
        "▛▌".to_string(),
        "▙▌".to_string(),
    ]);
    
    // p
    font.add_glyph('p', vec![
        "▛▌".to_string(),
        "▙▌".to_string(),
    ]);
    
    // q
    font.add_glyph('q', vec![
        "▛▌".to_string(),
        "▙▌".to_string(),
    ]);
    
    // r
    font.add_glyph('r', vec![
        "▛▘".to_string(),
        "▌ ".to_string(),
    ]);
    
    // s
    font.add_glyph('s', vec![
        "▛▘".to_string(),
        "▄▌".to_string(),
    ]);
    
    // t
    font.add_glyph('t', vec![
        "▜▘".to_string(),
        "▐▖".to_string(),
    ]);
    
    // u
    font.add_glyph('u', vec![
        "▌▌".to_string(),
        "▙▌".to_string(),
    ]);
    
    // v
    font.add_glyph('v', vec![
        "▌▌".to_string(),
        "▚▘".to_string(),
    ]);
    
    // w
    font.add_glyph('w', vec![
        "▌▌".to_string(),
        "▚▚".to_string(),
    ]);
    
    // s (appears again in wsyz - treating as duplicate)
    // y
    font.add_glyph('y', vec![
        "▘▌".to_string(),
        "▌▙".to_string(),
    ]);
    
    // z
    font.add_glyph('z', vec![
        "▌▀".to_string(),
        "▌▙".to_string(),
    ]);
}

fn add_mini_uppercase(font: &mut Font) {
    // A
    font.add_glyph('A', vec![
        "▄▖".to_string(),
        "▌▌".to_string(),
    ]);
    
    // B
    font.add_glyph('B', vec![
        "▄ ".to_string(),
        "▙▘".to_string(),
    ]);
    
    // C
    font.add_glyph('C', vec![
        "▄▖".to_string(),
        "▌ ".to_string(),
    ]);
    
    // D
    font.add_glyph('D', vec![
        "▄ ".to_string(),
        "▌▌".to_string(),
    ]);
    
    // E
    font.add_glyph('E', vec![
        "▄▖".to_string(),
        "▙▖".to_string(),
    ]);
    
    // F
    font.add_glyph('F', vec![
        "▄▖".to_string(),
        "▙▖".to_string(),
    ]);
    
    // G
    font.add_glyph('G', vec![
        "▄▖".to_string(),
        "▌ ".to_string(),
    ]);
    
    // H
    font.add_glyph('H', vec![
        "▖▖".to_string(),
        "▙▌".to_string(),
    ]);
    
    // I
    font.add_glyph('I', vec![
        "▄▖".to_string(),
        "▐ ".to_string(),
    ]);
    
    // J
    font.add_glyph('J', vec![
        " ▖".to_string(),
        " ▌".to_string(),
    ]);
    
    // K
    font.add_glyph('K', vec![
        "▖▖".to_string(),
        "▙▘".to_string(),
    ]);
    
    // L
    font.add_glyph('L', vec![
        "▖ ".to_string(),
        "▌ ".to_string(),
    ]);
    
    // M
    font.add_glyph('M', vec![
        "▖ ".to_string(),
        "▛▖".to_string(),
    ]);
    
    // N
    font.add_glyph('N', vec![
        " ▖".to_string(),
        "▞▌".to_string(),
    ]);
    
    // O
    font.add_glyph('O', vec![
        "▖ ".to_string(),
        "▛▖".to_string(),
    ]);
    
    // P
    font.add_glyph('P', vec![
        "▖▄".to_string(),
        "▌▌".to_string(),
    ]);
    
    // Q
    font.add_glyph('Q', vec![
        "▖▄".to_string(),
        "▌▙".to_string(),
    ]);
    
    // R
    font.add_glyph('R', vec![
        "▖▄".to_string(),
        "▌▌".to_string(),
    ]);
    
    // S
    font.add_glyph('S', vec![
        "▖▄".to_string(),
        "▌▙".to_string(),
    ]);
    
    // T
    font.add_glyph('T', vec![
        "▖▄".to_string(),
        "▘▚".to_string(),
    ]);
    
    // U
    font.add_glyph('U', vec![
        "▖▄".to_string(),
        " ▐".to_string(),
    ]);
    
    // V
    font.add_glyph('V', vec![
        "▖▖".to_string(),
        " ▌".to_string(),
    ]);
    
    // W
    font.add_glyph('W', vec![
        "▖▖".to_string(),
        "▌▌".to_string(),
    ]);
    
    // Y
    font.add_glyph('Y', vec![
        "▖▖".to_string(),
        "▌▌".to_string(),
    ]);
    
    // Z
    font.add_glyph('Z', vec![
        "  ".to_string(),
        "▞▖".to_string(),
    ]);
}

fn add_mini_numbers(font: &mut Font) {
    // 1
    font.add_glyph('1', vec![
        "▗ ".to_string(),
        "▄▖".to_string(),
    ]);
    
    // 2
    font.add_glyph('2', vec![
        "▄▖".to_string(),
        "▄▌".to_string(),
    ]);
    
    // 3
    font.add_glyph('3', vec![
        "▖▖".to_string(),
        "▄▌".to_string(),
    ]);
    
    // 4
    font.add_glyph('4', vec![
        "▄▖".to_string(),
        "▙▖".to_string(),
    ]);
    
    // 5
    font.add_glyph('5', vec![
        "▄▖".to_string(),
        "▄▖".to_string(),
    ]);
    
    // 6
    font.add_glyph('6', vec![
        "▄▖".to_string(),
        "▙▖".to_string(),
    ]);
    
    // 7
    font.add_glyph('7', vec![
        "▄▖".to_string(),
        "▄▌".to_string(),
    ]);
    
    // 8
    font.add_glyph('8', vec![
        "▄▖".to_string(),
        "▄▖".to_string(),
    ]);
    
    // 9
    font.add_glyph('9', vec![
        "▄▖".to_string(),
        "▄▖".to_string(),
    ]);
    
    // 0
    font.add_glyph('0', vec![
        "▄▖".to_string(),
        "▙▖".to_string(),
    ]);
}

fn add_mini_special(font: &mut Font) {
    // =
    font.add_glyph('=', vec![
        "▄▌".to_string(),
        "▄▌".to_string(),
    ]);
    
    // +
    font.add_glyph('+', vec![
        "▀▘".to_string(),
        "▟▖".to_string(),
    ]);
    
    // <
    font.add_glyph('<', vec![
        "▖ ".to_string(),
        "▖ ".to_string(),
    ]);
    
    // >
    font.add_glyph('>', vec![
        " ▌".to_string(),
        " ▌".to_string(),
    ]);
    
    // ,
    font.add_glyph(',', vec![
        "  ".to_string(),
        "▘ ".to_string(),
    ]);
    
    // .
    font.add_glyph('.', vec![
        "  ".to_string(),
        "▘ ".to_string(),
    ]);
    
    // /
    font.add_glyph('/', vec![
        "▞ ".to_string(),
        "▝▖".to_string(),
    ]);
    
    // ?
    font.add_glyph('?', vec![
        "▜▜".to_string(),
        "▘ ".to_string(),
    ]);
    
    // ;
    font.add_glyph(';', vec![
        "  ".to_string(),
        "▘ ".to_string(),
    ]);
    
    // "
    font.add_glyph('"', vec![
        "█▘".to_string(),
        "  ".to_string(),
    ]);
    
    // :
    font.add_glyph(':', vec![
        "▗▘".to_string(),
        "  ".to_string(),
    ]);
    
    // `
    font.add_glyph('`', vec![
        "▀ ".to_string(),
        "  ".to_string(),
    ]);
    
    // ~
    font.add_glyph('~', vec![
        "  ".to_string(),
        "  ".to_string(),
    ]);
    
    // !
    font.add_glyph('!', vec![
        "▚▘".to_string(),
        "▟▖".to_string(),
    ]);
    
    // @
    font.add_glyph('@', vec![
        "▄▖".to_string(),
        "▙▖".to_string(),
    ]);
    
    // #
    font.add_glyph('#', vec![
        "▄▌".to_string(),
        "▄▌".to_string(),
    ]);
    
    // $
    font.add_glyph('$', vec![
        "▄▖".to_string(),
        "▄▖".to_string(),
    ]);
    
    // %
    font.add_glyph('%', vec![
        "▝▘".to_string(),
        "▘ ".to_string(),
    ]);
    
    // ^
    font.add_glyph('^', vec![
        "▚▌".to_string(),
        "  ".to_string(),
    ]);
    
    // &
    font.add_glyph('&', vec![
        "▞▖".to_string(),
        "▌ ".to_string(),
    ]);
    
    // *
    font.add_glyph('*', vec![
        "▄▖".to_string(),
        "▄▖".to_string(),
    ]);
    
    // (
    font.add_glyph('(', vec![
        "▗ ".to_string(),
        "▝▖".to_string(),
    ]);
    
    // )
    font.add_glyph(')', vec![
        "▝ ".to_string(),
        "▗▘".to_string(),
    ]);
    
    // -
    font.add_glyph('-', vec![
        "  ".to_string(),
        "▀ ".to_string(),
    ]);
    
    // _
    font.add_glyph('_', vec![
        "  ".to_string(),
        "▀ ".to_string(),
    ]);
}
