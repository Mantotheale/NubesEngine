use std::ops::{Add, Div, Index, Mul, Neg, Sub};
use crate::math::approx_eq::ApproxEq;
use crate::math::non_zero_f32::NonZeroF32;

#[derive(Debug, Copy, Clone)]
pub struct Vec3f {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3f {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };

    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };

    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    
    pub const UNIT_Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }
    
    pub fn get(&self, idx: usize) -> Option<&f32> {
        match idx {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            _ => None
        }
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

    pub fn squared_len(&self) -> f32 {
        self.dot(*self)
    }

    pub fn len(&self) -> f32 {
        f32::sqrt(self.squared_len())
    }

    pub fn normalize(&self) -> Option<Self> {
        let len = NonZeroF32::new(self.len())?;
        Some(*self / len)
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn project(&self, rhs: Self) -> Option<Self> {
        let rhs_len_sq = NonZeroF32::new(rhs.squared_len())?;
        Some((self.dot(rhs) / rhs_len_sq) * rhs)
    }

    pub fn reject(&self, rhs: Self) -> Option<Self> {
        Some(*self - self.project(rhs)?)
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vec3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        rhs * self
    }
}

impl Div<NonZeroF32> for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: NonZeroF32) -> Self::Output {
        let inv = 1.0 / rhs;
        self * inv
    }
}

impl Index<usize> for Vec3f {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl ApproxEq for Vec3f {
    fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
        f32::abs(self.x - other.x) <= epsilon &&
            f32::abs(self.y - other.y) <= epsilon &&
            f32::abs(self.z - other.z) <= epsilon
    }
}

impl From<[f32; 3]> for Vec3f {
    fn from(arr: [f32; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}