use crate::ascii_globe::camera::Camera;
use crate::ascii_globe::math::PI_CONST;
use crate::ascii_globe::texture::load_texture;
use anyhow::{Context, Result};

pub struct GlobeRenderer {
    camera: Camera,
    earth: Vec<Vec<char>>,
    earth_night: Vec<Vec<char>>,
    angle_offset: f64,
    scale: f64,
    speed: f64,
    tilt: f64,
    lighting: bool,
}

impl GlobeRenderer {
    pub fn new(texture_dir: &str) -> Result<Self> {
        let earth_path = format!("{}/earth.txt", texture_dir);
        let earth_night_path = format!("{}/earth_night.txt", texture_dir);
        
        let earth = load_texture(&earth_path)
            .with_context(|| format!("Failed to load {}", earth_path))?;
        let earth_night = load_texture(&earth_night_path)
            .with_context(|| format!("Failed to load {}", earth_night_path))?;
        
        if earth.is_empty() || earth_night.is_empty() {
            anyhow::bail!("Failed to load textures");
        }
        
        Ok(Self {
            camera: Camera::new(2.0, 0.0, 0.0),
            earth,
            earth_night,
            angle_offset: 0.0,
            scale: 1.0,
            speed: 1.0,
            tilt: 23.5,
            lighting: true,
        })
    }
    
    pub fn render_frame(&mut self, canvas: &mut [Vec<char>], width: usize, height: usize) {
        self.camera.render_sphere(
            canvas,
            1.0,
            self.angle_offset,
            &self.earth,
            &self.earth_night,
            self.scale,
            self.tilt,
            self.lighting,
            width,
            height,
        );
    }
    
    pub fn update(&mut self, delta_time: f64) {
        // Update rotation based on speed and delta time
        // Original code used: angle_offset += (2.0 * PI_CONST / 18.0) * speed
        // For 60 FPS with 100ms sleep: delta_time ≈ 0.1s
        // Original increment per frame: (2.0 * PI / 18.0) * speed ≈ 0.349 * speed
        // So per second: 0.349 * speed * 10 ≈ 3.49 * speed radians/second
        let rotation_rate = 3.49 * self.speed; // radians per second
        self.angle_offset += rotation_rate * delta_time;
        // Keep angle_offset in [0, 2π) range
        self.angle_offset = self.angle_offset % (2.0 * PI_CONST);
    }
    
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }
    
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }
    
    pub fn set_tilt(&mut self, tilt: f64) {
        self.tilt = tilt;
    }
    
    pub fn set_lighting(&mut self, lighting: bool) {
        self.lighting = lighting;
    }
    
    pub fn get_scale(&self) -> f64 {
        self.scale
    }
    
    #[allow(dead_code)]
    pub fn get_speed(&self) -> f64 {
        self.speed
    }
    
    #[allow(dead_code)]
    pub fn get_tilt(&self) -> f64 {
        self.tilt
    }
    
    #[allow(dead_code)]
    pub fn get_lighting(&self) -> bool {
        self.lighting
    }
}
