pub mod font;
pub mod renderer;
pub mod builder;
pub mod loader;

pub use font::{Font, ansi_compact_font, mini_font};
pub use renderer::render_text;
pub use builder::AsciiArtBuilder;

/// Re-export error types
pub use loader::LoadError as Error;

/// Render text using the default embedded font.
///
/// # Example
///
/// ```
/// use text2artfont::render;
///
/// let art = render("Hello");
/// println!("{}", art);
/// ```
pub fn render(text: &str) -> String {
    render_text(text, &font::default_font())
}

/// Render text using a custom font.
///
/// # Example
///
/// ```
/// use text2artfont::{render_with_font, Font};
///
/// let font = Font::default();
/// let art = render_with_font("Hello", &font);
/// println!("{}", art);
/// ```
pub fn render_with_font(text: &str, font: &Font) -> String {
    render_text(text, font)
}

/// Render text using the ANSI Compact font.
///
/// This is a more compact font style with smaller character dimensions.
///
/// # Example
///
/// ```
/// use text2artfont::{ansi_compact_font, render_with_font};
///
/// let font = ansi_compact_font();
/// let art = render_with_font("Hello", &font);
/// println!("{}", art);
/// ```
pub fn render_ansi_compact(text: &str) -> String {
    render_text(text, &ansi_compact_font())
}

/// Render text using the Mini font.
///
/// This is a very compact font style with minimal character dimensions (2x2).
///
/// # Example
///
/// ```
/// use text2artfont::{mini_font, render_with_font};
///
/// let font = mini_font();
/// let art = render_with_font("Hello", &font);
/// println!("{}", art);
/// ```
pub fn render_mini(text: &str) -> String {
    render_text(text, &mini_font())
}
