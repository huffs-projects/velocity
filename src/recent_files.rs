use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config::Config;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RecentFileEntry {
    path: PathBuf,
    opened_at: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct RecentFilesData {
    files: Vec<RecentFileEntry>,
}

pub struct RecentFiles {
    max_files: usize,
    data_path: PathBuf,
}

impl RecentFiles {
    pub fn new() -> Result<Self> {
        let config_dir = Config::config_dir()?;
        let data_path = config_dir.join("recent_files.json");
        Ok(Self {
            max_files: 20,
            data_path,
        })
    }

    fn load_data(&self) -> Result<RecentFilesData> {
        if !self.data_path.exists() {
            return Ok(RecentFilesData { files: Vec::new() });
        }

        let content = fs::read_to_string(&self.data_path)
            .with_context(|| format!("Failed to read recent files from {:?}", self.data_path))?;
        
        let data: RecentFilesData = serde_json::from_str(&content)
            .with_context(|| "Failed to parse recent files JSON")?;
        
        Ok(data)
    }

    fn save_data(&self, data: &RecentFilesData) -> Result<()> {
        let config_dir = self.data_path.parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid data path"))?;
        fs::create_dir_all(config_dir)
            .with_context(|| format!("Failed to create config directory {:?}", config_dir))?;
        
        let content = serde_json::to_string_pretty(data)
            .context("Failed to serialize recent files")?;
        
        fs::write(&self.data_path, content)
            .with_context(|| format!("Failed to write recent files to {:?}", self.data_path))?;
        
        Ok(())
    }

    fn is_text_file(path: &Path) -> bool {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        matches!(ext.as_str(), 
            "txt" | "md" | "rs" | "py" | "js" | "ts" | "json" | "toml" | "yaml" | "yml" | 
            "xml" | "html" | "css" | "sh" | "zsh" | "bash" | "c" | "cpp" | "h" | "hpp" |
            "java" | "go" | "rb" | "php" | "swift" | "kt" | "scala" | "clj" | "lua" |
            "vim" | "conf" | "config" | "ini" | "log" | "csv" | "tsv"
        )
    }

    pub fn add_file(&self, path: PathBuf) -> Result<()> {
        if !Self::is_text_file(&path) {
            return Ok(()); // Silently skip non-text files
        }

        let mut data = self.load_data()?;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Remove existing entry if present
        data.files.retain(|e| e.path != path);
        
        // Add new entry at the beginning
        data.files.insert(0, RecentFileEntry {
            path,
            opened_at: timestamp,
        });
        
        // Keep only the most recent files
        if data.files.len() > self.max_files {
            data.files.truncate(self.max_files);
        }
        
        self.save_data(&data)?;
        Ok(())
    }

    pub fn get_files(&self) -> Result<Vec<PathBuf>> {
        let data = self.load_data()?;
        Ok(data.files.iter().map(|e| e.path.clone()).collect())
    }

}
