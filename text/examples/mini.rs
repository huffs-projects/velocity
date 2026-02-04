use text2artfont::render_mini;

fn main() {
    println!("Mini font test:");
    println!("{}", render_mini("Hello"));
    println!("\n---\n");
    println!("{}", render_mini("ABC"));
    println!("\n---\n");
    println!("{}", render_mini("123"));
}
