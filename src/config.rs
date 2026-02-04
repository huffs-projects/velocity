use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub apps: Vec<AppEntry>,
    pub globe: GlobeConfig,
    pub ui: UiConfig,
    #[serde(default = "default_theme")]
    pub theme: ThemeConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GlobeConfig {
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default = "default_speed")]
    pub speed: f64,
    #[serde(default = "default_tilt")]
    pub tilt: f64,
    #[serde(default = "default_lighting")]
    pub lighting: bool,
    #[serde(default = "default_texture_path")]
    pub texture_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UiConfig {
    #[serde(default = "default_fps")]
    pub target_fps: u32,
    #[serde(default = "default_text_editor")]
    pub text_editor: String,
    #[serde(default = "default_text_dir")]
    pub default_text_dir: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ThemeConfig {
    #[serde(default = "default_text_primary")]
    pub text_primary: [u8; 3],
    #[serde(default = "default_text_secondary")]
    pub text_secondary: [u8; 3],
    #[serde(default = "default_text_selected")]
    pub text_selected: [u8; 3],
    #[serde(default = "default_text_accent")]
    pub text_accent: [u8; 3],
    
    #[serde(default = "default_star_dim")]
    pub star_dim: [u8; 3],
    #[serde(default = "default_star_medium")]
    pub star_medium: [u8; 3],
    #[serde(default = "default_star_light")]
    pub star_light: [u8; 3],
    #[serde(default = "default_star_bright")]
    pub star_bright: [u8; 3],
    #[serde(default = "default_star_brightest")]
    pub star_brightest: [u8; 3],
    
    #[serde(default = "default_status_good")]
    pub status_good: [u8; 3],
    #[serde(default = "default_status_warning")]
    pub status_warning: [u8; 3],
    #[serde(default = "default_status_error")]
    pub status_error: [u8; 3],
    #[serde(default = "default_status_info")]
    pub status_info: [u8; 3],
    
    #[serde(default = "default_border")]
    pub border: [u8; 3],
}

fn default_scale() -> f64 {
    1.15
}

fn default_speed() -> f64 {
    1.0
}

fn default_tilt() -> f64 {
    23.5
}

fn default_lighting() -> bool {
    false
}

fn default_fps() -> u32 {
    60
}

fn default_text_editor() -> String {
    #[cfg(target_os = "macos")]
    {
        "code".to_string()
    }
    #[cfg(not(target_os = "macos"))]
    {
        "code".to_string()
    }
}

fn default_text_dir() -> String {
    dirs::document_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| PathBuf::from("."))
        .to_string_lossy()
        .to_string()
}

fn default_texture_path() -> String {
    "textures".to_string()
}

fn default_text_primary() -> [u8; 3] {
    [255, 255, 255]
}

fn default_text_secondary() -> [u8; 3] {
    [200, 200, 200]
}

fn default_text_selected() -> [u8; 3] {
    [255, 255, 255]
}

fn default_text_accent() -> [u8; 3] {
    [100, 150, 255]
}

fn default_star_dim() -> [u8; 3] {
    [100, 100, 120]
}

fn default_star_medium() -> [u8; 3] {
    [150, 150, 180]
}

fn default_star_light() -> [u8; 3] {
    [200, 200, 220]
}

fn default_star_bright() -> [u8; 3] {
    [230, 230, 250]
}

fn default_star_brightest() -> [u8; 3] {
    [255, 255, 255]
}

fn default_status_good() -> [u8; 3] {
    [0, 255, 0]
}

fn default_status_warning() -> [u8; 3] {
    [255, 255, 0]
}

fn default_status_error() -> [u8; 3] {
    [255, 0, 0]
}

fn default_status_info() -> [u8; 3] {
    [0, 255, 255]
}

fn default_border() -> [u8; 3] {
    [255, 255, 255]
}

fn default_theme() -> ThemeConfig {
    ThemeConfig {
        text_primary: default_text_primary(),
        text_secondary: default_text_secondary(),
        text_selected: default_text_selected(),
        text_accent: default_text_accent(),
        star_dim: default_star_dim(),
        star_medium: default_star_medium(),
        star_light: default_star_light(),
        star_bright: default_star_bright(),
        star_brightest: default_star_brightest(),
        status_good: default_status_good(),
        status_warning: default_status_warning(),
        status_error: default_status_error(),
        status_info: default_status_info(),
        border: default_border(),
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            apps: vec![
                AppEntry {
                    name: "Terminal".to_string(),
                    command: "open".to_string(),
                    args: Some(vec!["-a".to_string(), "Terminal.app".to_string()]),
                },
                AppEntry {
                    name: "VS Code".to_string(),
                    command: "code".to_string(),
                    args: None,
                },
                AppEntry {
                    name: "Finder".to_string(),
                    command: "open".to_string(),
                    args: Some(vec!["-a".to_string(), "Finder.app".to_string()]),
                },
                AppEntry {
                    name: "App 4".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 4".to_string()]),
                },
                AppEntry {
                    name: "App 5".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 5".to_string()]),
                },
                AppEntry {
                    name: "Application 6".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["Application 6".to_string()]),
                },
                AppEntry {
                    name: "App 7".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 7".to_string()]),
                },
                AppEntry {
                    name: "App 8".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 8".to_string()]),
                },
                AppEntry {
                    name: "App 9".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 9".to_string()]),
                },
                AppEntry {
                    name: "App 10".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 10".to_string()]),
                },
                AppEntry {
                    name: "App 11".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 11".to_string()]),
                },
                AppEntry {
                    name: "App 12".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 12".to_string()]),
                },
                AppEntry {
                    name: "App 13".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 13".to_string()]),
                },
                AppEntry {
                    name: "App 14".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 14".to_string()]),
                },
                AppEntry {
                    name: "App 15".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 15".to_string()]),
                },
                AppEntry {
                    name: "App 16".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 16".to_string()]),
                },
                AppEntry {
                    name: "App 17".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 17".to_string()]),
                },
                AppEntry {
                    name: "App 18".to_string(),
                    command: "echo".to_string(),
                    args: Some(vec!["App 18".to_string()]),
                },
            ],
            globe: GlobeConfig {
                scale: 1.15,
                speed: 1.0,
                tilt: 23.5,
                lighting: false,
                texture_path: default_texture_path(),
            },
            ui: UiConfig {
                target_fps: 60,
                text_editor: default_text_editor(),
                default_text_dir: default_text_dir(),
            },
            theme: default_theme(),
        }
    }
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("velocity");
        Ok(config_dir)
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config from {:?}", config_path))?;
        
        let mut config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config from {:?}", config_path))?;
        
        // TEMPORARY: Add fake apps for testing the 15-position layout
        let mut fake_apps = vec![
            AppEntry { name: "App 4".to_string(), command: "echo".to_string(), args: Some(vec!["App 4".to_string()]) },
            AppEntry { name: "App 5".to_string(), command: "echo".to_string(), args: Some(vec!["App 5".to_string()]) },
            AppEntry { name: "Application 6".to_string(), command: "echo".to_string(), args: Some(vec!["Application 6".to_string()]) },
            AppEntry { name: "App 7".to_string(), command: "echo".to_string(), args: Some(vec!["App 7".to_string()]) },
            AppEntry { name: "App 8".to_string(), command: "echo".to_string(), args: Some(vec!["App 8".to_string()]) },
            AppEntry { name: "App 9".to_string(), command: "echo".to_string(), args: Some(vec!["App 9".to_string()]) },
            AppEntry { name: "App 10".to_string(), command: "echo".to_string(), args: Some(vec!["App 10".to_string()]) },
            AppEntry { name: "App 11".to_string(), command: "echo".to_string(), args: Some(vec!["App 11".to_string()]) },
            AppEntry { name: "App 12".to_string(), command: "echo".to_string(), args: Some(vec!["App 12".to_string()]) },
            AppEntry { name: "App 13".to_string(), command: "echo".to_string(), args: Some(vec!["App 13".to_string()]) },
            AppEntry { name: "App 14".to_string(), command: "echo".to_string(), args: Some(vec!["App 14".to_string()]) },
            AppEntry { name: "App 15".to_string(), command: "echo".to_string(), args: Some(vec!["App 15".to_string()]) },
            AppEntry { name: "App 16".to_string(), command: "echo".to_string(), args: Some(vec!["App 16".to_string()]) },
            AppEntry { name: "App 17".to_string(), command: "echo".to_string(), args: Some(vec!["App 17".to_string()]) },
            AppEntry { name: "App 18".to_string(), command: "echo".to_string(), args: Some(vec!["App 18".to_string()]) },
        ];
        config.apps.append(&mut fake_apps);
        
        // Deduplicate apps by name - keep only first occurrence of each app name
        // Do this AFTER appending fake apps so we catch all duplicates
        let mut seen_names = std::collections::HashSet::new();
        config.apps.retain(|app| {
            if seen_names.contains(&app.name) {
                false // Remove duplicate
            } else {
                seen_names.insert(app.name.clone());
                true // Keep first occurrence
            }
        });
        
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;
        fs::create_dir_all(&config_dir)
            .with_context(|| format!("Failed to create config directory {:?}", config_dir))?;
        
        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config to {:?}", config_path))?;
        
        Ok(())
    }
}
