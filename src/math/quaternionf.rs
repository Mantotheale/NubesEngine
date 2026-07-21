use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::math::non_zero_f32::NonZeroF32;
use crate::math::vec3f::Vec3f;

#[derive(Debug, Copy, Clone)]
pub struct Quaternionf {
    vector: Vec3f,
    scalar: f32
}

impl Quaternionf {
    pub const IDENTITY: Self = Self { vector: Vec3f::ZERO, scalar: 1.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { vector: Vec3f::new(x, y, z), scalar: w }
    }

    pub fn from_vec(vector: Vec3f) -> Self {
        Self { vector, scalar: 0.0 }
    }

    pub fn from_scalar(scalar: f32) -> Self {
        Self { vector: Vec3f::ZERO, scalar }
    }

    pub fn from_vec_and_scalar(vector: Vec3f, scalar: f32) -> Self {
        Self { vector, scalar }
    }

    pub fn squared_len(&self) -> f32 {
        self.vector.squared_len() + self.scalar * self.scalar
    }

    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn conjugate(&self) -> Self {
        Self { vector: -self.vector, scalar: self.scalar }
    }

    pub fn inverse(&self) -> Option<Self> {
        let squared_len = NonZeroF32::new(self.squared_len())?;
        Some(self.conjugate() / squared_len)
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

impl Add for Quaternionf {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_vec_and_scalar(self.vector + rhs.vector, self.scalar + rhs.scalar)
    }
}

impl Sub for Quaternionf {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_vec_and_scalar(self.vector - rhs.vector, self.scalar - rhs.scalar)
    }
}

impl Neg for Quaternionf {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_vec_and_scalar(-self.vector, -self.scalar)
    }
}

impl Mul<f32> for Quaternionf {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_vec_and_scalar(self.vector * rhs, self.scalar * rhs)
    }
}

impl Mul<Quaternionf> for f32 {
    type Output = Quaternionf;

    fn mul(self, rhs: Quaternionf) -> Self::Output {
        rhs * self
    }
}

impl Div<NonZeroF32> for Quaternionf {
    type Output = Self;

    fn div(self, rhs: NonZeroF32) -> Self::Output {
        let inv = 1.0 / rhs;
        self * inv
    }
}