mod ascii_globe;
mod config;
mod launcher;
mod recent_files;
mod system_stats;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};
use ui::{App, AppState};
use ui::components::GlobeComponent;
use ui::views::{render_home, render_apps, render_recent, render_settings};

fn find_texture_path(config_path: &str) -> String {
    // Check if the configured path exists
    if Path::new(config_path).exists() {
        return config_path.to_string();
    }
    
    // Try common locations relative to current directory
    let candidates = vec![
        config_path, // Try configured path first
        "textures",  // Default bundled location
    ];
    
    for candidate in candidates {
        if Path::new(candidate).exists() {
            return candidate.to_string();
        }
    }
    
    // Fallback to configured path even if it doesn't exist yet
    // (user might need to create it or it will error with a helpful message)
    config_path.to_string()
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize app
    let mut app = App::new()?;
    // Find texture path - check multiple locations
    let texture_path = find_texture_path(&app.config.globe.texture_path);
    let mut globe = GlobeComponent::new(&texture_path)?;
    
    // Apply config to globe
    globe.set_scale(app.config.globe.scale);
    globe.set_speed(app.config.globe.speed);
    globe.set_tilt(app.config.globe.tilt);
    globe.set_lighting(app.config.globe.lighting);
    
    let target_fps = app.config.ui.target_fps;
    let frame_time = Duration::from_secs_f64(1.0 / target_fps as f64);
    
    // Main event loop
    let mut last_frame = Instant::now();
    
    loop {
        let mut needs_immediate_redraw = false;
        
        // Handle events
        if crossterm::event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Resize(width, height) => {
                    // Terminal was resized - force immediate redraw and resize stars if needed
                    needs_immediate_redraw = true;
                    if let Some(ref mut stars) = app.stars {
                        // Resize stars to full screen dimensions
                        stars.resize(width, height);
                    }
                }
                Event::Key(key) => {
                    app.handle_key(key)?;
                    if app.should_quit {
                        break;
                    }
                }
                _ => {}
            }
        }
        
        // Update
        app.update();
        globe.update()?;
        
        // Render
        let theme = app.theme();
        terminal.draw(|f| {
            match app.state {
                AppState::Home => {
                    render_home(f, &mut globe, &mut app.system_stats, &theme);
                }
                AppState::Apps => {
                    // Initialize stars only if they don't exist or dimensions changed
                    let area = f.size();
                    if area.width > 0 && area.height > 0 {
                        use ui::components::NightSky;
                        let needs_init = app.stars.is_none() || 
                            app.stars.as_ref().map(|s| s.initialized_width != area.width || s.initialized_height != area.height).unwrap_or(true);
                        if needs_init {
                            app.stars = Some(NightSky::new(area.width, area.height));
                        }
                    }
                    render_apps(f, &mut globe, app.app_selection, &app.config, app.stars.as_mut(), &theme);
                }
                AppState::RecentFiles => {
                    // Initialize stars only if they don't exist or dimensions changed
                    let area = f.size();
                    if area.width > 0 && area.height > 0 {
                        use ui::components::NightSky;
                        let needs_init = app.stars.is_none() || 
                            app.stars.as_ref().map(|s| s.initialized_width != area.width || s.initialized_height != area.height).unwrap_or(true);
                        if needs_init {
                            app.stars = Some(NightSky::new(area.width, area.height));
                        }
                    }
                    render_recent(f, &mut globe, &app.recent_files, app.recent_selection, app.stars.as_mut(), &theme);
                }
                AppState::Settings => {
                    // Initialize stars only if they don't exist or dimensions changed
                    let area = f.size();
                    if area.width > 0 && area.height > 0 {
                        use ui::components::NightSky;
                        let needs_init = app.stars.is_none() || 
                            app.stars.as_ref().map(|s| s.initialized_width != area.width || s.initialized_height != area.height).unwrap_or(true);
                        if needs_init {
                            app.stars = Some(NightSky::new(area.width, area.height));
                        }
                    }
                    render_settings(f, &mut globe, &app.config, app.settings_selection, app.stars.as_mut(), &theme);
                }
            }
        })?;
        
        // Frame rate limiting (skip if resize occurred for immediate response)
        if !needs_immediate_redraw {
            let elapsed = last_frame.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        }
        last_frame = Instant::now();
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
