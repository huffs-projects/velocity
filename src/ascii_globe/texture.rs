use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn load_texture(filename: impl AsRef<Path>) -> Result<Vec<Vec<char>>> {
    let content = fs::read_to_string(filename.as_ref())
        .with_context(|| format!("Error loading texture: {:?}", filename.as_ref()))?;
    
    Ok(content
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}
