use std::ops::{Add, Sub};
use crate::math::approx_eq::ApproxEq;
use crate::math::vec3f::Vec3f;

#[derive(Debug, Copy, Clone)]
pub struct Point3f {
    x: f32,
    y: f32,
    z: f32
}

impl Point3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3f { x, y, z }
    }
    
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

impl Add<Vec3f> for Point3f {
    type Output = Self;

    fn add(self, rhs: Vec3f) -> Self::Output {
        Self::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl Sub for Point3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ApproxEq for Point3f {
    fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
        self.x.approx_eq(&other.x, epsilon) &&
            self.y.approx_eq(&other.y, epsilon) &&
            self.z.approx_eq(&other.z, epsilon)
    }
}

impl From<Vec3f> for Point3f {
    fn from(vec: Vec3f) -> Self {
        Self { x: vec.x(), y: vec.y(), z: vec.z() }
    }
}