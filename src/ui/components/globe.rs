use crate::ascii_globe::GlobeRenderer;
use anyhow::Result;
use std::time::Instant;

pub struct GlobeComponent {
    renderer: GlobeRenderer,
    last_update: Instant,
    frame_buffer: Vec<Vec<char>>,
}

impl GlobeComponent {
    pub fn new(texture_dir: &str) -> Result<Self> {
        let renderer = GlobeRenderer::new(texture_dir)?;
        Ok(Self {
            renderer,
            last_update: Instant::now(),
            frame_buffer: Vec::new(),
        })
    }

    pub fn update(&mut self) -> Result<()> {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f64();
        self.last_update = now;
        
        self.renderer.update(delta_time);
        Ok(())
    }

    pub fn render(&mut self, width: usize, height: usize) -> Result<&[Vec<char>]> {
        // Resize frame buffer if needed
        if self.frame_buffer.len() != height || 
           self.frame_buffer.first().map(|r| r.len()) != Some(width) {
            self.frame_buffer = vec![vec![' '; width]; height];
        }
        
        // Clear buffer
        for row in &mut self.frame_buffer {
            row.fill(' ');
        }
        
        // Render globe
        self.renderer.render_frame(&mut self.frame_buffer, width, height);
        
        Ok(&self.frame_buffer)
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.renderer.set_scale(scale);
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.renderer.set_speed(speed);
    }

    pub fn set_tilt(&mut self, tilt: f64) {
        self.renderer.set_tilt(tilt);
    }

    pub fn set_lighting(&mut self, lighting: bool) {
        self.renderer.set_lighting(lighting);
    }

    pub fn get_scale(&self) -> f64 {
        self.renderer.get_scale()
    }
}
