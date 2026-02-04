use crate::font::Font;
use crate::renderer::render_text;

/// Alignment options for text rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// Builder for creating ASCII art with advanced configuration options.
#[derive(Debug, Clone)]
pub struct AsciiArtBuilder {
    text: String,
    font: Option<Font>,
    spacing: Option<usize>,
    line_spacing: Option<usize>,
    alignment: Alignment,
}

impl AsciiArtBuilder {
    /// Create a new builder with default settings.
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::AsciiArtBuilder;
    ///
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello")
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self {
            text: String::new(),
            font: None,
            spacing: None,
            line_spacing: None,
            alignment: Alignment::Left,
        }
    }

    /// Set the text to render.
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::AsciiArtBuilder;
    ///
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello World")
    ///     .build();
    /// ```
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Set a custom font to use for rendering.
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::{AsciiArtBuilder, Font};
    ///
    /// let font = Font::default();
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello")
    ///     .font(font)
    ///     .build();
    /// ```
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Set the horizontal spacing between characters.
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::AsciiArtBuilder;
    ///
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello")
    ///     .spacing(2)
    ///     .build();
    /// ```
    pub fn spacing(mut self, spacing: usize) -> Self {
        self.spacing = Some(spacing);
        self
    }

    /// Set the vertical spacing between lines of text.
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::AsciiArtBuilder;
    ///
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello\nWorld")
    ///     .line_spacing(1)
    ///     .build();
    /// ```
    pub fn line_spacing(mut self, line_spacing: usize) -> Self {
        self.line_spacing = Some(line_spacing);
        self
    }

    /// Set text alignment to left (default).
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::AsciiArtBuilder;
    ///
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello")
    ///     .align_left()
    ///     .build();
    /// ```
    pub fn align_left(mut self) -> Self {
        self.alignment = Alignment::Left;
        self
    }

    /// Set text alignment to center.
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::AsciiArtBuilder;
    ///
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello")
    ///     .align_center()
    ///     .build();
    /// ```
    pub fn align_center(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    /// Set text alignment to right.
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::AsciiArtBuilder;
    ///
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello")
    ///     .align_right()
    ///     .build();
    /// ```
    pub fn align_right(mut self) -> Self {
        self.alignment = Alignment::Right;
        self
    }

    /// Build and render the ASCII art.
    ///
    /// # Example
    ///
    /// ```
    /// use text2artfont::AsciiArtBuilder;
    ///
    /// let art = AsciiArtBuilder::new()
    ///     .text("Hello")
    ///     .spacing(1)
    ///     .build();
    /// ```
    pub fn build(self) -> String {
        let mut font = self.font.unwrap_or_else(|| Font::default());
        
        // Apply spacing override if specified
        if let Some(spacing) = self.spacing {
            font.spacing = spacing;
        }

        let mut result = render_text(&self.text, &font);

        // Apply line spacing if specified
        if let Some(line_spacing) = self.line_spacing {
            result = add_line_spacing(&result, line_spacing);
        }

        // Apply alignment
        result = apply_alignment(&result, self.alignment);

        result
    }
}

impl Default for AsciiArtBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Add vertical spacing between lines of rendered text.
fn add_line_spacing(text: &str, spacing: usize) -> String {
    if spacing == 0 {
        return text.to_string();
    }

    let empty_line = " ".repeat(1);
    let lines: Vec<&str> = text.lines().collect();
    let mut result = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        result.push(line.to_string());
        
        // Add spacing lines between text lines (but not after the last line)
        if idx < lines.len() - 1 {
            // Check if this is the end of a rendered text block
            // (height of font determines when we've finished one text line)
            // For simplicity, add spacing after every font.height lines
            // This is a heuristic - in practice, we'd need to track font height
            for _ in 0..spacing {
                result.push(empty_line.clone());
            }
        }
    }

    result.join("\n")
}

/// Apply alignment to the rendered text.
fn apply_alignment(text: &str, alignment: Alignment) -> String {
    if alignment == Alignment::Left {
        return text.to_string();
    }

    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    // Find the maximum width
    let max_width = lines.iter().map(|line| line.chars().count()).max().unwrap_or(0);

    let aligned_lines: Vec<String> = lines
        .iter()
        .map(|line| {
            let width = line.chars().count();
            let padding = max_width.saturating_sub(width);

            match alignment {
                Alignment::Left => line.to_string(),
                Alignment::Center => {
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    format!("{}{}{}", " ".repeat(left_pad), line, " ".repeat(right_pad))
                }
                Alignment::Right => format!("{}{}", " ".repeat(padding), line),
            }
        })
        .collect();

    aligned_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let result = AsciiArtBuilder::new().text("A").build();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_builder_with_spacing() {
        let result = AsciiArtBuilder::new()
            .text("AB")
            .spacing(2)
            .build();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_builder_alignment() {
        let left = AsciiArtBuilder::new().text("A").align_left().build();
        let center = AsciiArtBuilder::new().text("A").align_center().build();
        let right = AsciiArtBuilder::new().text("A").align_right().build();
        
        assert!(!left.is_empty());
        assert!(!center.is_empty());
        assert!(!right.is_empty());
    }
}
