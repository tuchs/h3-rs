use num::Float;

#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

/**
 * Calculates the magnitude of a 2D cartesian vector.
 * @param v The 2D cartesian vector.
 * @return The magnitude of the vector.
 */
pub fn _v2dMag(v: &Vec2d) -> f64 {
    return (v.x * v.x + v.y * v.y).sqrt();
}
