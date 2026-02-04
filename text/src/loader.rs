use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::font::Font;

/// Error types for font loading operations.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid font data: {0}")]
    InvalidFont(String),
}

/// JSON representation of a font file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontJson {
    /// Width of each character
    pub width: usize,
    /// Height of each character
    pub height: usize,
    /// Horizontal spacing between characters
    #[serde(default = "default_spacing")]
    pub spacing: usize,
    /// Map of character strings to their glyph representations
    pub glyphs: HashMap<String, Vec<String>>,
}

fn default_spacing() -> usize {
    1
}

/// Load a font from a JSON file.
///
/// # Arguments
///
/// * `path` - Path to the JSON font file
///
/// # Returns
///
/// A `Result` containing the loaded `Font` or a `LoadError`
///
/// # Example JSON Format
///
/// ```json
/// {
///   "width": 7,
///   "height": 7,
///   "spacing": 1,
///   "glyphs": {
///     "A": [
///       "  ▄▄  ",
///       "  ██  ",
///       " ████▄ ",
///       " ██ ██ ",
///       " ██ ██ ",
///       "      ",
///       "      "
///     ]
///   }
/// }
/// ```
///
/// # Example
///
/// ```no_run
/// use text2artfont::loader::load_from_json;
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let font = load_from_json(Path::new("font.json"))?;
/// # Ok(())
/// # }
/// ```
pub fn load_from_json(path: &Path) -> Result<Font, LoadError> {
    let content = fs::read_to_string(path)?;
    let font_json: FontJson = serde_json::from_str(&content)?;

    validate_font_json(&font_json)?;

    let mut font = Font::new(font_json.width, font_json.height, font_json.spacing);

    for (char_str, glyph_lines) in font_json.glyphs {
        // Parse the character string - handle single characters and escape sequences
        let ch = parse_character(&char_str)?;
        
        // Validate glyph dimensions
        if glyph_lines.len() != font_json.height {
            return Err(LoadError::InvalidFont(format!(
                "Glyph for '{}' has {} lines, expected {}",
                char_str,
                glyph_lines.len(),
                font_json.height
            )));
        }

        // Validate each line width
        for (line_idx, line) in glyph_lines.iter().enumerate() {
            let line_width = line.chars().count();
            if line_width > font_json.width {
                return Err(LoadError::InvalidFont(format!(
                    "Glyph for '{}' line {} has {} characters, maximum is {}",
                    char_str, line_idx, line_width, font_json.width
                )));
            }
        }

        font.add_glyph(ch, glyph_lines);
    }

    Ok(font)
}

/// Parse a character string, handling escape sequences.
fn parse_character(s: &str) -> Result<char, LoadError> {
    if s.len() == 1 {
        Ok(s.chars().next().unwrap())
    } else if s.starts_with('\\') {
        // Handle escape sequences
        match s {
            "\\n" => Ok('\n'),
            "\\t" => Ok('\t'),
            "\\r" => Ok('\r'),
            "\\0" => Ok('\0'),
            "\\'" => Ok('\''),
            "\\\"" => Ok('"'),
            "\\\\" => Ok('\\'),
            _ => {
                // Try to parse as unicode escape \u{...}
                if s.starts_with("\\u{") && s.ends_with('}') {
                    let hex = &s[3..s.len() - 1];
                    let code = u32::from_str_radix(hex, 16)
                        .map_err(|_| LoadError::InvalidFont(format!("Invalid unicode escape: {}", s)))?;
                    char::from_u32(code)
                        .ok_or_else(|| LoadError::InvalidFont(format!("Invalid unicode character: {}", s)))
                } else {
                    Err(LoadError::InvalidFont(format!("Unknown escape sequence: {}", s)))
                }
            }
        }
    } else {
        // Multi-character string - take first character
        s.chars()
            .next()
            .ok_or_else(|| LoadError::InvalidFont(format!("Empty character string: {}", s)))
    }
}

/// Validate font JSON structure.
fn validate_font_json(font_json: &FontJson) -> Result<(), LoadError> {
    if font_json.width == 0 {
        return Err(LoadError::InvalidFont("Font width must be greater than 0".to_string()));
    }
    if font_json.height == 0 {
        return Err(LoadError::InvalidFont("Font height must be greater than 0".to_string()));
    }
    Ok(())
}

/// Save a font to a JSON file.
///
/// # Arguments
///
/// * `font` - The font to save
/// * `path` - Path where the JSON file should be written
///
/// # Example
///
/// ```no_run
/// use text2artfont::{Font, loader::save_to_json};
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let font = Font::default();
/// save_to_json(&font, Path::new("font.json"))?;
/// # Ok(())
/// # }
/// ```
pub fn save_to_json(font: &Font, path: &Path) -> Result<(), LoadError> {
    let mut glyphs_map = HashMap::new();
    
    for (ch, glyph) in &font.glyphs {
        let char_str = if ch.is_control() || *ch == '\\' || *ch == '"' {
            // Escape special characters
            format!("\\u{{{:04x}}}", *ch as u32)
        } else {
            ch.to_string()
        };
        glyphs_map.insert(char_str, glyph.clone());
    }

    let font_json = FontJson {
        width: font.width,
        height: font.height,
        spacing: font.spacing,
        glyphs: glyphs_map,
    };

    let json = serde_json::to_string_pretty(&font_json)?;
    fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_font() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"{{
  "width": 5,
  "height": 5,
  "spacing": 1,
  "glyphs": {{
    "A": [
      "  A  ",
      " A A ",
      "AAAAA",
      "A   A",
      "A   A"
    ]
  }}
}}"#
        )
        .unwrap();

        let font = load_from_json(file.path()).unwrap();
        assert_eq!(font.width, 5);
        assert_eq!(font.height, 5);
        assert_eq!(font.spacing, 1);
        assert!(font.get_glyph('A').is_some());
    }

    #[test]
    fn test_load_invalid_dimensions() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"{{
  "width": 0,
  "height": 5,
  "spacing": 1,
  "glyphs": {{}}
}}"#
        )
        .unwrap();

        let result = load_from_json(file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_character_escape() {
        assert_eq!(parse_character("\\n").unwrap(), '\n');
        assert_eq!(parse_character("\\t").unwrap(), '\t');
        assert_eq!(parse_character("A").unwrap(), 'A');
    }
}
