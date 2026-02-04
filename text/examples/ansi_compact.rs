use text2artfont::{ansi_compact_font, render_with_font};

fn main() {
    let font = ansi_compact_font();
    
    println!("ANSI Compact Font Demo:\n");
    println!("{}", render_with_font("Hello", &font));
    println!("\n---\n");
    println!("{}", render_with_font("ABC", &font));
    println!("\n---\n");
    println!("{}", render_with_font("123", &font));
}
