use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::time::Instant;
use rand::Rng;
use crate::ui::Theme;

pub struct Star {
    x: u16,
    y: u16,
    brightness: u8,
    twinkle_speed: f32,
}

pub struct NightSky {
    stars: Vec<Star>,
    start_time: Instant,
    pub initialized_width: u16,
    pub initialized_height: u16,
}

impl NightSky {
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        
        // Calculate star count: approximately 1 star per 20 cells, capped at 500
        let star_count = ((width as usize * height as usize) / 20).min(500);
        
        let mut stars = Vec::with_capacity(star_count);
        
        for _ in 0..star_count {
            stars.push(Star {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..height),
                brightness: rng.gen_range(1..=5),
                twinkle_speed: rng.gen_range(0.1..=0.5),
            });
        }
        
        Self {
            stars,
            start_time: Instant::now(),
            initialized_width: width,
            initialized_height: height,
        }
    }
    
    pub fn update(&mut self) {
        // Time-based animation - no need to do anything here
        // We'll use elapsed time in render()
    }
    
    pub fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        self.render_with_occupied_positions(frame, area, &std::collections::HashSet::new(), theme);
    }
    
    pub fn render_with_occupied_positions(&mut self, frame: &mut Frame, area: Rect, occupied_positions: &std::collections::HashSet<(u16, u16)>, theme: &Theme) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        
        // Reinitialize stars if area dimensions don't match initialized dimensions
        if area.width != self.initialized_width || area.height != self.initialized_height {
            *self = Self::new(area.width, area.height);
        }
        
        let elapsed = self.start_time.elapsed().as_secs_f64();
        
        for star in &self.stars {
            // Bounds checking - ensure star is within the render area
            if star.x >= area.width || star.y >= area.height {
                continue;
            }
            
            // Calculate absolute position
            let abs_x = area.x.saturating_add(star.x);
            let abs_y = area.y.saturating_add(star.y);
            
            // Skip if this position is occupied (globe or app text)
            if occupied_positions.contains(&(abs_x, abs_y)) {
                continue;
            }
            
            
            // Calculate twinkling effect using sine wave
            // Use elapsed time directly with twinkle_speed as a slow multiplier
            // twinkle_speed (0.1-0.5) controls how fast each star twinkles
            // Maximum twinkling: very wide range (0.0 to 1.8) for very dramatic variation
            let twinkle_raw = (elapsed * star.twinkle_speed as f64 * 0.5).sin();
            // Map sin(-1 to 1) to (0.0 to 1.8) - stars dim to invisible and brighten significantly beyond base
            let twinkle = ((twinkle_raw + 1.0) / 2.0) * 1.8;
            let current_brightness = (star.brightness as f32 * twinkle as f32).clamp(0.0, 6.0) as u8;
            
            // Map brightness to character
            let star_char = if current_brightness <= 1 {
                "·"
            } else if current_brightness <= 3 {
                "•"
            } else {
                "✦"
            };
            
            // Map brightness to theme color
            let color = theme.star_color(current_brightness);
            
            // Render star at its position
            let star_area = Rect {
                x: abs_x,
                y: abs_y,
                width: 1,
                height: 1,
            };
            
            // Ensure we don't overflow
            if star_area.x < area.x.saturating_add(area.width) && 
               star_area.y < area.y.saturating_add(area.height) {
                let star_widget = Paragraph::new(Line::from(Span::styled(star_char, Style::default().fg(color))));
                frame.render_widget(star_widget, star_area);
            }
        }
    }
    
    pub fn resize(&mut self, width: u16, height: u16) {
        // Reinitialize stars when terminal is resized
        *self = Self::new(width, height);
    }
}
