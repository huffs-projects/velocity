use crate::config::Config;
use crate::recent_files::RecentFiles;
use crate::system_stats::SystemStats;
use crate::ui::components::NightSky;
use crate::ui::Theme;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Home,
    Apps,
    RecentFiles,
    Settings,
}

pub struct App {
    pub state: AppState,
    pub config: Config,
    pub recent_files: RecentFiles,
    pub system_stats: SystemStats,
    pub app_selection: usize,
    pub recent_selection: Option<usize>,
    pub settings_selection: Option<usize>,
    pub should_quit: bool,
    pub stars: Option<NightSky>,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let recent_files = RecentFiles::new()?;
        let system_stats = SystemStats::new();
        
        // Select first item if available
        let files = recent_files.get_files().unwrap_or_default();
        let recent_selection = if !files.is_empty() { Some(0) } else { None };
        
        // Start app selection in the middle of the list
        let app_selection = if config.apps.is_empty() {
            0
        } else {
            config.apps.len().saturating_sub(1) / 2
        };
        
        Ok(Self {
            state: AppState::Home,
            config,
            recent_files,
            system_stats,
            app_selection,
            recent_selection,
            settings_selection: Some(0),
            should_quit: false,
            stars: None,
        })
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => {
                self.should_quit = true;
            }
            crossterm::event::KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            crossterm::event::KeyCode::Esc => {
                // Esc returns home from submenus, quits from home
                if self.state == AppState::Home {
                    self.should_quit = true;
                } else {
                    self.state = AppState::Home;
                }
            }
            crossterm::event::KeyCode::Char('a') => {
                self.state = AppState::Apps;
                // Start selection in the middle of the list
                if !self.config.apps.is_empty() {
                    self.app_selection = self.config.apps.len().saturating_sub(1) / 2;
                } else {
                    self.app_selection = 0;
                }
            }
            crossterm::event::KeyCode::Char('r') => {
                self.state = AppState::RecentFiles;
                // Refresh selection when entering RecentFiles view
                let files = self.recent_files.get_files().unwrap_or_default();
                if !files.is_empty() {
                    self.recent_selection = Some(0);
                } else {
                    self.recent_selection = None;
                }
            }
            crossterm::event::KeyCode::Char('s') => {
                self.state = AppState::Settings;
                self.settings_selection = Some(0);
            }
            crossterm::event::KeyCode::Char('h') => {
                // 'h' returns home from submenus
                if self.state != AppState::Home {
                    self.state = AppState::Home;
                }
            }
            crossterm::event::KeyCode::Char('j') => {
                // 'j' moves down (same as Down arrow)
                match self.state {
                    AppState::Apps => {
                        if self.app_selection < self.config.apps.len().saturating_sub(1) {
                            self.app_selection += 1;
                        }
                    }
                    AppState::RecentFiles => {
                        let files = self.recent_files.get_files().unwrap_or_default();
                        if let Some(selected) = self.recent_selection {
                            if selected < files.len().saturating_sub(1) {
                                self.recent_selection = Some(selected + 1);
                            }
                        }
                    }
                    AppState::Settings => {
                        if let Some(selected) = self.settings_selection {
                            if selected < 4 {
                                self.settings_selection = Some(selected + 1);
                            }
                        }
                    }
                    _ => {}
                }
            }
            crossterm::event::KeyCode::Char('k') => {
                // 'k' moves up (same as Up arrow)
                match self.state {
                    AppState::Apps => {
                        if self.app_selection > 0 {
                            self.app_selection -= 1;
                        }
                    }
                    AppState::RecentFiles => {
                        if let Some(selected) = self.recent_selection {
                            if selected > 0 {
                                self.recent_selection = Some(selected - 1);
                            }
                        }
                    }
                    AppState::Settings => {
                        if let Some(selected) = self.settings_selection {
                            if selected > 0 {
                                self.settings_selection = Some(selected - 1);
                            }
                        }
                    }
                    _ => {}
                }
            }
            crossterm::event::KeyCode::Char('l') => {
                // 'l' selects/launches (same as Enter) in submenus
                match self.state {
                    AppState::Apps => {
                        if let Some(app) = self.config.apps.get(self.app_selection) {
                            crate::launcher::launch_app(app)?;
                        }
                    }
                    AppState::RecentFiles => {
                        let files = self.recent_files.get_files()?;
                        if let Some(selected) = self.recent_selection {
                            if let Some(file) = files.get(selected) {
                                crate::launcher::open_file(file)?;
                                self.recent_files.add_file(file.clone())?;
                            }
                        }
                    }
                    AppState::Settings => {
                        // Settings Enter behavior - currently no-op, but 'l' key should work
                    }
                    _ => {}
                }
            }
            crossterm::event::KeyCode::Char('n') => {
                // Create new file - only works from home view
                if self.state == AppState::Home {
                    let file_path = crate::launcher::create_and_open_text_file(
                        &self.config.ui.text_editor,
                        &self.config.ui.default_text_dir,
                    )?;
                    self.recent_files.add_file(file_path)?;
                }
            }
            crossterm::event::KeyCode::Enter => {
                match self.state {
                    AppState::Apps => {
                        if let Some(app) = self.config.apps.get(self.app_selection) {
                            crate::launcher::launch_app(app)?;
                        }
                    }
                    AppState::RecentFiles => {
                        let files = self.recent_files.get_files()?;
                        if let Some(selected) = self.recent_selection {
                            if let Some(file) = files.get(selected) {
                                crate::launcher::open_file(file)?;
                                self.recent_files.add_file(file.clone())?;
                            }
                        }
                    }
                    AppState::Home => {
                        crate::launcher::launch_terminal()?;
                    }
                    _ => {}
                }
            }
            crossterm::event::KeyCode::Up => {
                match self.state {
                    AppState::Apps => {
                        if self.app_selection > 0 {
                            self.app_selection -= 1;
                        }
                    }
                    AppState::RecentFiles => {
                        if let Some(selected) = self.recent_selection {
                            if selected > 0 {
                                self.recent_selection = Some(selected - 1);
                            }
                        }
                    }
                    AppState::Settings => {
                        if let Some(selected) = self.settings_selection {
                            if selected > 0 {
                                self.settings_selection = Some(selected - 1);
                            }
                        }
                    }
                    _ => {}
                }
            }
            crossterm::event::KeyCode::Down => {
                match self.state {
                    AppState::Apps => {
                        if self.app_selection < self.config.apps.len().saturating_sub(1) {
                            self.app_selection += 1;
                        }
                    }
                    AppState::RecentFiles => {
                        let files = self.recent_files.get_files().unwrap_or_default();
                        if let Some(selected) = self.recent_selection {
                            if selected < files.len().saturating_sub(1) {
                                self.recent_selection = Some(selected + 1);
                            }
                        }
                    }
                    AppState::Settings => {
                        // 5 settings items
                        if let Some(selected) = self.settings_selection {
                            if selected < 4 {
                                self.settings_selection = Some(selected + 1);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        Ok(())
    }

    pub fn update(&mut self) {
        self.system_stats.refresh();
        if let Some(ref mut stars) = self.stars {
            stars.update();
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::new(self.config.theme.clone())
    }
}
