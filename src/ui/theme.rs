use ratatui::style::Color;
use crate::config::ThemeConfig;

pub struct Theme {
    config: ThemeConfig,
}

impl Theme {
    pub fn new(config: ThemeConfig) -> Self {
        Self { config }
    }

    pub fn text_primary(&self) -> Color {
        Color::Rgb(self.config.text_primary[0], self.config.text_primary[1], self.config.text_primary[2])
    }

    pub fn text_secondary(&self) -> Color {
        Color::Rgb(self.config.text_secondary[0], self.config.text_secondary[1], self.config.text_secondary[2])
    }

    pub fn text_selected(&self) -> Color {
        Color::Rgb(self.config.text_selected[0], self.config.text_selected[1], self.config.text_selected[2])
    }

    pub fn text_accent(&self) -> Color {
        Color::Rgb(self.config.text_accent[0], self.config.text_accent[1], self.config.text_accent[2])
    }

    pub fn star_color(&self, brightness: u8) -> Color {
        match brightness {
            0..=1 => Color::Rgb(self.config.star_dim[0], self.config.star_dim[1], self.config.star_dim[2]),
            2 => Color::Rgb(self.config.star_medium[0], self.config.star_medium[1], self.config.star_medium[2]),
            3 => Color::Rgb(self.config.star_light[0], self.config.star_light[1], self.config.star_light[2]),
            4 => Color::Rgb(self.config.star_bright[0], self.config.star_bright[1], self.config.star_bright[2]),
            _ => Color::Rgb(self.config.star_brightest[0], self.config.star_brightest[1], self.config.star_brightest[2]),
        }
    }

    pub fn status_good(&self) -> Color {
        Color::Rgb(self.config.status_good[0], self.config.status_good[1], self.config.status_good[2])
    }

    pub fn status_warning(&self) -> Color {
        Color::Rgb(self.config.status_warning[0], self.config.status_warning[1], self.config.status_warning[2])
    }

    pub fn status_error(&self) -> Color {
        Color::Rgb(self.config.status_error[0], self.config.status_error[1], self.config.status_error[2])
    }

    pub fn status_info(&self) -> Color {
        Color::Rgb(self.config.status_info[0], self.config.status_info[1], self.config.status_info[2])
    }

    pub fn border(&self) -> Color {
        Color::Rgb(self.config.border[0], self.config.border[1], self.config.border[2])
    }
}
