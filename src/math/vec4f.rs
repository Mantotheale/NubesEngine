use std::ops::{Add, Div, Index, Mul, Neg, Sub};
use crate::math::approx_eq::ApproxEq;
use crate::math::non_zero_f32::NonZeroF32;

#[derive(Debug, Copy, Clone)]
pub struct Vec4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Vec4f {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };

    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };

    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };

    pub const UNIT_Z: Self = Self { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    
    pub const UNIT_W: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn get(&self, idx: usize) -> Option<f32> {
        match idx {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            3 => Some(self.w),
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

    pub fn w(&self) -> f32 {
        self.w
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
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl Add for Vec4f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl Sub for Vec4f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl Neg for Vec4f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f32> for Vec4f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul<Vec4f> for f32 {
    type Output = Vec4f;

    fn mul(self, rhs: Vec4f) -> Self::Output {
        rhs * self
    }
}

impl Div<NonZeroF32> for Vec4f {
    type Output = Vec4f;

    fn div(self, rhs: NonZeroF32) -> Self::Output {
        let inv = 1.0 / rhs;
        self * inv
    }
}

impl Index<usize> for Vec4f {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl ApproxEq for Vec4f {
    fn approx_eq(&self, other: &Self, epsilon: f32) -> bool { 
        self.x.approx_eq(&other.x, epsilon) &&
            self.y.approx_eq(&other.y, epsilon) &&
            self.z.approx_eq(&other.z, epsilon) &&
            self.w.approx_eq(&other.w, epsilon)
    }
}

impl From<[f32; 4]> for Vec4f {
    fn from(arr: [f32; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }
}