use std::ops::Mul;
use crate::math::vec3f::Vec3f;

#[derive(Debug, Copy, Clone)]
pub struct Quaternionf {
    vector: Vec3f,
    scalar: f32
}

impl Quaternionf {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { vector: Vec3f::new(x, y, z), scalar: w }
    }

    pub fn from_vec_and_scalar(vector: Vec3f, scalar: f32) -> Self {
        Self { vector, scalar }
    }
}

impl Mul for Quaternionf {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            vector: self.vector.cross(rhs.vector)
                + self.scalar * rhs.vector + rhs.scalar * self.vector,
            scalar: self.scalar * rhs.scalar - self.vector.dot(rhs.vector)
        }
    }
}