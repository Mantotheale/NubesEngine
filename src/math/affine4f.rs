use std::ops::Mul;
use crate::math::angle::Angle;
use crate::math::mat3f::Mat3f;
use crate::math::point3f::Point3f;
use crate::math::vec3f::Vec3f;

#[derive(Debug, Copy, Clone)]
pub struct Affine4f {
    mat: Mat3f,
    translation: Vec3f
}

impl Affine4f {
    pub fn new(mat: Mat3f, translation: Vec3f) -> Self {
        Self { mat, translation }
    }

    pub fn from_translation(translation: Vec3f) -> Self {
        Self { mat: Mat3f::IDENTITY, translation }
    }

    pub fn from_axis_rotation(axis: Vec3f, rotation: Angle) -> Option<Self> {
        Some(Self { mat: Mat3f::from_axis_rotation(axis, rotation)?, translation: Vec3f::ZERO })
    }

    pub fn from_reflection_perp_axis(axis: Vec3f) -> Option<Self> {
        Some(Self { mat: Mat3f::from_reflection_perp_axis(axis)?, translation: Vec3f::ZERO })
    }

    pub fn from_scaling_vec(scaling_vec: Vec3f) -> Self {
        Self { mat: Mat3f::from_scaling_vec(scaling_vec), translation: Vec3f::ZERO }
    }

    pub fn from_scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Self { mat: Mat3f::from_scaling(sx, sy, sz), translation: Vec3f::ZERO }
    }

    pub fn from_uniform_scaling(s: f32) -> Self {
        Self { mat: Mat3f::from_uniform_scaling(s), translation: Vec3f::ZERO }
    }

    pub fn from_axis_scaling(axis: Vec3f, scaling: f32) -> Option<Self> {
        Some(Self { mat: Mat3f::from_axis_scaling(axis, scaling)?, translation: Vec3f::ZERO })
    }

    pub fn determinant(&self) -> f32 {
        self.mat.determinant()
    }

    pub fn inverse(&self) -> Option<Self> {
        let inv_mat = self.mat.inverse()?;

        Some(Self { mat: inv_mat, translation: -(inv_mat * self.translation) })
    }
}

impl Mul for Affine4f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { mat: self.mat * rhs.mat, translation: self.mat * rhs.translation + self.translation }
    }
}

impl Mul<Vec3f> for Affine4f {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        self.mat * rhs
    }
}

impl Mul<Point3f> for Affine4f {
    type Output = Point3f;

    fn mul(self, rhs: Point3f) -> Self::Output {
        self.mat * rhs + self.translation
    }
}