use anyhow::{Context, Result};
use crate::config::AppEntry;
use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::fs;
use chrono::Local;

pub fn launch_app(app: &AppEntry) -> Result<()> {
    let mut cmd = Command::new(&app.command);
    
    if let Some(ref args) = app.args {
        cmd.args(args);
    }
    
    // Launch in background, detached from terminal
    cmd.stdin(Stdio::null())
       .stdout(Stdio::null())
       .stderr(Stdio::null());
    
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0); // Create new process group
    }
    
    cmd.spawn()
        .with_context(|| format!("Failed to launch {}", app.name))?;
    
    Ok(())
}

pub fn launch_terminal() -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let app = AppEntry {
            name: "Terminal".to_string(),
            command: "open".to_string(),
            args: Some(vec!["-a".to_string(), "Terminal.app".to_string()]),
        };
        launch_app(&app)
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        // Try common terminal emulators
        let terminals = ["xterm", "gnome-terminal", "konsole", "alacritty", "kitty"];
        for term in &terminals {
            if Command::new(term).arg("-e").arg("bash").spawn().is_ok() {
                return Ok(());
            }
        }
        anyhow::bail!("Could not find a terminal emulator")
    }
}

pub fn open_file(path: &std::path::Path) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .with_context(|| format!("Failed to open file {:?}", path))?;
        Ok(())
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        // Try xdg-open on Linux
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .with_context(|| format!("Failed to open file {:?}", path))?;
        Ok(())
    }
}

pub fn create_and_open_text_file(text_editor: &str, default_dir: &str) -> Result<PathBuf> {
    let default_dir_path = PathBuf::from(default_dir);
    // Ensure the directory exists
    fs::create_dir_all(&default_dir_path)
        .with_context(|| format!("Failed to create directory {:?}", default_dir_path))?;
    
    // Generate a unique filename with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("new_file_{}.txt", timestamp);
    let file_path = default_dir_path.join(&filename);
    
    // Create an empty file
    fs::File::create(&file_path)
        .with_context(|| format!("Failed to create file {:?}", file_path))?;
    
    // Open the file with the configured text editor
    let mut cmd = Command::new(text_editor);
    cmd.arg(&file_path);
    
    // Launch in background, detached from terminal
    cmd.stdin(Stdio::null())
       .stdout(Stdio::null())
       .stderr(Stdio::null());
    
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0); // Create new process group
    }
    
    cmd.spawn()
        .with_context(|| format!("Failed to launch text editor '{}' for file {:?}", text_editor, file_path))?;
    
    Ok(file_path)
}

