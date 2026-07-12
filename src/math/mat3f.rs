use std::ops::{Add, Div, Index, Mul, Neg, Sub};
use crate::math::angle::Angle;
use crate::math::non_zero_f32::NonZeroF32;
use crate::math::vec3f::Vec3f;

#[derive(Debug, Copy, Clone)]
pub struct Mat3f {
    x_col: Vec3f,
    y_col: Vec3f,
    z_col: Vec3f,
}

impl Mat3f {
    pub const IDENTITY: Self = Self { x_col: Vec3f::UNIT_X, y_col: Vec3f::UNIT_Y, z_col: Vec3f::UNIT_Z };

    pub fn new(a00: f32, a01: f32, a02: f32, a10: f32, a11: f32, a12: f32, a20: f32, a21: f32, a22: f32) -> Self {
        Self {
            x_col: Vec3f::new(a00, a10, a20),
            y_col: Vec3f::new(a01, a11, a21),
            z_col: Vec3f::new(a02, a12, a22)
        }
    }

    pub fn from_col_vecs(x_col: Vec3f, y_col: Vec3f, z_col: Vec3f) -> Self {
        Self { x_col, y_col, z_col }
    }

    pub fn from_row_vecs(x_row: Vec3f, y_row: Vec3f, z_row: Vec3f) -> Self {
        Self {
            x_col: Vec3f::new(x_row.x(), y_row.x(), z_row.x()),
            y_col: Vec3f::new(x_row.y(), y_row.y(), z_row.y()),
            z_col: Vec3f::new(x_row.z(), y_row.z(), z_row.z())
        }
    }

    pub fn from_rows(r0: [f32; 3], r1: [f32; 3], r2: [f32; 3]) -> Self {
        Self {
            x_col: Vec3f::new(r0[0], r1[0], r2[0]),
            y_col: Vec3f::new(r0[1], r1[1], r2[1]),
            z_col: Vec3f::new(r0[2], r1[2], r2[2])
        }
    }

    pub fn from_columns(c0: [f32; 3], c1: [f32; 3], c2: [f32; 3]) -> Self {
        Self {
            x_col: c0.into(),
            y_col: c1.into(),
            z_col: c2.into()
        }
    }

    pub fn from_axis_rotation(axis: Vec3f, rotation: Angle) -> Option<Self> {
        let axis = axis.normalize()?;
        let s = rotation.sin();
        let c = rotation.cos();
        let one_minus_c = 1.0 - c;
        let sx = axis.x() * s;
        let sy = axis.y() * s;
        let sz = axis.z() * s;
        let xy_one_minus_c = axis.x() * axis.y() * one_minus_c;
        let yz_one_minus_c = axis.y() * axis.z() * one_minus_c;
        let xz_one_minus_c = axis.x() * axis.z() * one_minus_c;

        Some(Self::new(
            axis.x() * axis.x() * one_minus_c + c,
            xy_one_minus_c - sz,
            xz_one_minus_c + sy,
            xy_one_minus_c + sz,
            axis.y() * axis.y() * one_minus_c + c,
            yz_one_minus_c - sx,
            xz_one_minus_c - sy,
            yz_one_minus_c + sx,
            axis.z() * axis.z() * one_minus_c + c,
        ))
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&f32> {
        match col {
            0 => self.x_col.get(row),
            1 => self.y_col.get(row),
            2 => self.z_col.get(row),
            _ => None
        }
    }

    pub fn determinant(&self) -> f32 {
        self.x_col.dot(self.y_col.cross(self.z_col))
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = NonZeroF32::new(self.determinant())?;

        let adjugate = Self::from_row_vecs(
            self.y_col.cross(self.z_col),
            self.z_col.cross(self.x_col),
            self.x_col.cross(self.y_col)
        );

        Some(adjugate / det)
    }
}

impl Add for Mat3f {
    type Output = Mat3f;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_col_vecs(
            self.x_col + rhs.x_col,
            self.y_col + rhs.y_col,
            self.z_col + rhs.z_col
        )
    }
}

impl Sub for Mat3f {
    type Output = Mat3f;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_col_vecs(
            self.x_col - rhs.x_col,
            self.y_col - rhs.y_col,
            self.z_col - rhs.z_col
        )
    }
}

impl Neg for Mat3f {
    type Output = Mat3f;

    fn neg(self) -> Self::Output {
        Self::from_col_vecs(-self.x_col, -self.y_col, -self.z_col)
    }
}

impl Mul<f32> for Mat3f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_col_vecs(self.x_col * rhs, self.y_col * rhs, self.z_col * rhs)
    }
}

impl Mul<Mat3f> for f32 {
    type Output = Mat3f;

    fn mul(self, rhs: Mat3f) -> Self::Output {
        rhs * self
    }
}

impl Mul for Mat3f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x_row = Vec3f::new(self.x_col.x(), self.y_col.x(), self.z_col.x());
        let y_row = Vec3f::new(self.x_col.y(), self.y_col.y(), self.z_col.y());
        let z_row = Vec3f::new(self.x_col.z(), self.y_col.z(), self.z_col.z());
        Self::new(
            x_row.dot(rhs.x_col),
            x_row.dot(rhs.y_col),
            x_row.dot(rhs.z_col),
            y_row.dot(rhs.x_col),
            y_row.dot(rhs.y_col),
            y_row.dot(rhs.z_col),
            z_row.dot(rhs.x_col),
            z_row.dot(rhs.y_col),
            z_row.dot(rhs.z_col),
        )
    }
}

impl Mul<Vec3f> for Mat3f {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        self.x_col * rhs.x() + self.y_col * rhs.y() + self.z_col * rhs.z()
    }
}

impl Div<NonZeroF32> for Mat3f {
    type Output = Self;

    fn div(self, rhs: NonZeroF32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Index<(usize, usize)> for Mat3f {
    type Output = f32;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        match col {
            0 => &self.x_col[row],
            1 => &self.y_col[row],
            2 => &self.z_col[row],
            _ => panic!("Column out of bounds"),
        }
    }
}

impl From<[[f32; 3]; 3]> for Mat3f {
    fn from(mat: [[f32; 3]; 3]) -> Self {
        Self::from_rows(mat[0], mat[1], mat[2])
    }
}