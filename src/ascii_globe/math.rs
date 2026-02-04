use std::f64::consts::PI;

pub type Vec3 = [f64; 3];

#[allow(dead_code)]
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn magnitude(r: Vec3) -> f64 {
    (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()
}

pub fn normalize(r: Vec3) -> Vec3 {
    let mag = magnitude(r);
    if mag == 0.0 {
        return r;
    }
    [r[0] / mag, r[1] / mag, r[2] / mag]
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn vector(b: Vec3, c: Vec3) -> Vec3 {
    [b[0] - c[0], b[1] - c[1], b[2] - c[2]]
}

pub fn transform_vector(vec: Vec3, m: &[f64; 16]) -> Vec3 {
    let x = vec[0] * m[0] + vec[1] * m[4] + vec[2] * m[8] + m[12];
    let y = vec[0] * m[1] + vec[1] * m[5] + vec[2] * m[9] + m[13];
    let z = vec[0] * m[2] + vec[1] * m[6] + vec[2] * m[10] + m[14];
    [x, y, z]
}

pub fn rotate_x(vec: Vec3, theta: f64) -> Vec3 {
    let a = theta.sin();
    let b = theta.cos();
    let m = [
        1.0, 0.0, 0.0,
        0.0, b, -a,
        0.0, a, b,
    ];
    transform_vector_2(vec, &m)
}

pub fn transform_vector_2(vec: Vec3, m: &[f64; 9]) -> Vec3 {
    let x = m[0] * vec[0] + m[1] * vec[1] + m[2] * vec[2];
    let y = m[3] * vec[0] + m[4] * vec[1] + m[5] * vec[2];
    let z = m[6] * vec[0] + m[7] * vec[1] + m[8] * vec[2];
    [x, y, z]
}

pub fn clamp(x: f64, min_val: f64, max_val: f64) -> f64 {
    x.max(min_val).min(max_val)
}

pub fn clamp_int(x: i32, min_val: i32, max_val: i32) -> i32 {
    x.max(min_val).min(max_val)
}

pub const PI_CONST: f64 = PI;
