use crate::ascii_globe::math::{self, Vec3, clamp, clamp_int};

pub const PALETTE: &str = " .:;',wiogOLXHWYV@";

pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    matrix: [f64; 16],
}

impl Camera {
    pub fn new(r: f64, alfa: f64, beta: f64) -> Self {
        let a = alfa.sin();
        let b = alfa.cos();
        let c = beta.sin();
        let d = beta.cos();
        
        let x = r * b * d;
        let y = r * a * d;
        let z = r * c;
        
        let matrix = [
            -a, b, 0.0, 0.0,
            b * c, a * c, -d, 0.0,
            b * d, a * d, c, 0.0,
            x, y, z, 1.0,
        ];
        
        Self { x, y, z, matrix }
    }
    
    pub fn render_sphere(
        &self,
        canvas: &mut [Vec<char>],
        radius: f64,
        angle_offset: f64,
        earth: &[Vec<char>],
        earth_night: &[Vec<char>],
        scale: f64,
        tilt: f64,
        lighting: bool,
        canvas_width: usize,
        canvas_height: usize,
    ) {
        let light: Vec3 = [0.0, 999999.0, 0.0];
        
        let texture_height = earth.len();
        if texture_height == 0 {
            return;
        }
        
        let texture_width = earth[0].len();
        if texture_width == 0 {
            return;
        }
        
        let radius = radius * scale;
        let tilt_rad = tilt.to_radians();
        
        for yi in 0..canvas_height {
            for xi in 0..canvas_width {
                let o: Vec3 = [self.x, self.y, self.z];
                let mut u: Vec3 = [
                    -((xi as f64 - (canvas_width as f64) / 2.0) + 0.5) / (canvas_width as f64 / 2.0) * 1.2,
                    ((yi as f64 - (canvas_height as f64) / 2.0) + 0.5) / (canvas_height as f64 / 2.0),
                    -1.0,
                ];
                
                u = math::transform_vector(u, &self.matrix);
                u = [
                    u[0] - self.x,
                    u[1] - self.y,
                    u[2] - self.z,
                ];
                u = math::normalize(u);
                
                let discriminant = math::dot(u, o) * math::dot(u, o) - math::dot(o, o) + radius * radius;
                if discriminant < 0.0 {
                    continue;
                }
                
                let distance = -discriminant.sqrt() - math::dot(u, o);
                let inter: Vec3 = [
                    o[0] + distance * u[0],
                    o[1] + distance * u[1],
                    o[2] + distance * u[2],
                ];
                
                let n = math::normalize(inter);
                let l = math::normalize(math::vector(light, inter));
                let luminance = if lighting {
                    clamp(5.0 * math::dot(n, l) + 0.5, 0.0, 1.0)
                } else {
                    1.0
                };
                
                let temp = math::rotate_x(inter, -tilt_rad);
                
                let phi = -temp[2] / radius / 2.0 + 0.5;
                let mut theta = -temp[1].atan2(temp[0]) / math::PI_CONST + 0.5 + angle_offset / 2.0 / math::PI_CONST;
                theta -= theta.floor();
                
                let earth_x = clamp_int((theta * (texture_width - 1) as f64) as i32, 0, (texture_width - 1) as i32);
                let earth_y = clamp_int((phi * (texture_height - 1) as f64) as i32, 0, (texture_height - 1) as i32);
                
                if let (Some(day_char), Some(night_char)) = (
                    earth.get(earth_y as usize).and_then(|row| row.get(earth_x as usize)),
                    earth_night.get(earth_y as usize).and_then(|row| row.get(earth_x as usize)),
                ) {
                    let day = find_index(*day_char, PALETTE);
                    let night = find_index(*night_char, PALETTE);
                    
                    if day >= 0 && night >= 0 {
                        let index = ((1.0 - luminance) * night as f64 + luminance * day as f64) as usize;
                        let index = clamp_int(index as i32, 0, (PALETTE.len() - 1) as i32) as usize;
                        draw_point(canvas, xi, yi, PALETTE.chars().nth(index).unwrap_or(' '), canvas_width, canvas_height);
                    }
                }
            }
        }
    }
}

fn find_index(c: char, s: &str) -> i32 {
    s.chars().position(|x| x == c).map(|i| i as i32).unwrap_or(-1)
}

fn draw_point(canvas: &mut [Vec<char>], x: usize, y: usize, c: char, canvas_width: usize, canvas_height: usize) {
    if x < canvas_width && y < canvas_height {
        if let Some(row) = canvas.get_mut(y) {
            if let Some(cell) = row.get_mut(x) {
                *cell = c;
            }
        }
    }
}
