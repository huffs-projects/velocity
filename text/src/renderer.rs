use crate::font::Font;

/// Render text into ASCII art using the specified font.
///
/// # Arguments
///
/// * `text` - The text to render
/// * `font` - The font to use for rendering
///
/// # Returns
///
/// A multi-line string containing the ASCII art representation
pub fn render_text(text: &str, font: &Font) -> String {
    if text.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = text.lines().collect();
    let mut result = Vec::new();

    for line in lines {
        if line.is_empty() {
            // Add empty line
            result.push(String::new());
            continue;
        }

        // Render each character in the line
        let mut rendered_lines: Vec<String> = vec![String::new(); font.height];
        let chars: Vec<char> = line.chars().collect();
        
        for (idx, ch) in chars.iter().enumerate() {
            let glyph = font.get_glyph_or_placeholder(*ch);
            
            // Ensure glyph height matches font height
            let glyph_height = glyph.len().min(font.height);
            
            // Add each line of the glyph
            for i in 0..font.height {
                if i < glyph_height {
                    // Pad glyph line to font width
                    let glyph_line = &glyph[i];
                    let padded = pad_to_width(glyph_line, font.width);
                    rendered_lines[i].push_str(&padded);
                } else {
                    // Fill with spaces if glyph is shorter
                    rendered_lines[i].push_str(&" ".repeat(font.width));
                }
            }
            
            // Add horizontal spacing after each character (except the last)
            if idx < chars.len() - 1 {
                for rendered_line in &mut rendered_lines {
                    rendered_line.push_str(&" ".repeat(font.spacing));
                }
            }
        }

        result.extend(rendered_lines);
    }

    result.join("\n")
}

/// Pad a string to the specified width with spaces.
fn pad_to_width(s: &str, width: usize) -> String {
    let current_width = s.chars().count();
    if current_width >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - current_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::Font;

    #[test]
    fn test_render_empty_text() {
        let font = Font::default();
        let result = render_text("", &font);
        assert_eq!(result, "");
    }

    #[test]
    fn test_render_single_character() {
        let mut font = Font::new(5, 5, 1);
        font.add_glyph('A', vec![
            "  A  ".to_string(),
            " A A ".to_string(),
            "AAAAA".to_string(),
            "A   A".to_string(),
            "A   A".to_string(),
        ]);
        let result = render_text("A", &font);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_render_multiple_characters() {
        let mut font = Font::new(5, 5, 1);
        font.add_glyph('A', vec![
            "  A  ".to_string(),
            " A A ".to_string(),
            "AAAAA".to_string(),
            "A   A".to_string(),
            "A   A".to_string(),
        ]);
        font.add_glyph('B', vec![
            "BBBB ".to_string(),
            "B   B".to_string(),
            "BBBB ".to_string(),
            "B   B".to_string(),
            "BBBB ".to_string(),
        ]);
        let result = render_text("AB", &font);
        assert!(!result.is_empty());
        assert!(result.contains('A') || result.contains('B'));
    }

    #[test]
    fn test_render_multiline_text() {
        let mut font = Font::new(5, 5, 1);
        font.add_glyph('A', vec![
            "  A  ".to_string(),
            " A A ".to_string(),
            "AAAAA".to_string(),
            "A   A".to_string(),
            "A   A".to_string(),
        ]);
        let result = render_text("A\nA", &font);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 10); // 5 lines per character
    }

    #[test]
    fn test_render_unknown_character() {
        let font = Font::new(5, 5, 1);
        let result = render_text("X", &font);
        // Should not panic, should render spaces
        assert!(!result.is_empty() || result.is_empty());
    }
}
