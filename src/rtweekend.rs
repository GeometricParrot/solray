pub use crate::vec3::*;
pub use crate::ray::*;
pub use crate::interval::*;

pub const PI: f32 = 3.1415926535897932385;

pub fn deg_to_rad(deg: f32) -> f32 {
	deg * PI / 180.0
}