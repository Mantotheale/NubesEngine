use std::ops::{Add, Div, Index, Mul, Neg, Sub};
use crate::math::non_zero_f32::NonZeroF32;
use crate::math::vec4f::Vec4f;

#[derive(Debug, Copy, Clone)]
pub struct Mat4f {
    x_col: Vec4f,
    y_col: Vec4f,
    z_col: Vec4f,
    w_col: Vec4f
}

impl Mat4f {
    pub const IDENTITY: Self = Self { x_col: Vec4f::UNIT_X, y_col: Vec4f::UNIT_Y, z_col: Vec4f::UNIT_Z, w_col: Vec4f::UNIT_W };

    pub fn new(a00: f32, a01: f32, a02: f32, a03: f32, a10: f32, a11: f32, a12: f32, a13: f32, a20: f32, a21: f32, a22: f32, a23: f32, a30: f32, a31: f32, a32: f32, a33: f32) -> Self {
        Self {
            x_col: Vec4f::new(a00, a10, a20, a30),
            y_col: Vec4f::new(a01, a11, a21, a31),
            z_col: Vec4f::new(a02, a12, a22, a32),
            w_col: Vec4f::new(a03, a13, a23, a33)
        }
    }

    pub fn from_col_vecs(x_col: Vec4f, y_col: Vec4f, z_col: Vec4f, w_col: Vec4f) -> Self {
        Self { x_col, y_col, z_col, w_col }
    }

    pub fn from_row_vecs(x_row: Vec4f, y_row: Vec4f, z_row: Vec4f, w_row: Vec4f) -> Self {
        Self {
            x_col: Vec4f::new(x_row.x(), y_row.x(), z_row.x(), w_row.x()),
            y_col: Vec4f::new(x_row.y(), y_row.y(), z_row.y(), w_row.y()),
            z_col: Vec4f::new(x_row.z(), y_row.z(), z_row.z(), w_row.z()),
            w_col: Vec4f::new(x_row.w(), y_row.w(), z_row.w(), w_row.w())
        }
    }

    pub fn from_rows(r0: [f32; 4], r1: [f32; 4], r2: [f32; 4], r3: [f32; 4]) -> Self {
        Self {
            x_col: Vec4f::new(r0[0], r1[0], r2[0], r3[0]),
            y_col: Vec4f::new(r0[1], r1[1], r2[1], r3[1]),
            z_col: Vec4f::new(r0[2], r1[2], r2[2], r3[2]),
            w_col: Vec4f::new(r0[3], r1[3], r2[3], r3[3])
        }
    }

    pub fn from_columns(c0: [f32; 4], c1: [f32; 4], c2: [f32; 4], c3: [f32; 4]) -> Self {
        Self {
            x_col: c0.into(),
            y_col: c1.into(),
            z_col: c2.into(),
            w_col: c3.into()
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f32> {
        match col {
            0 => self.x_col.get(row),
            1 => self.y_col.get(row),
            2 => self.z_col.get(row),
            3 => self.w_col.get(row),
            _ => None
        }
    }

    pub fn determinant(&self) -> f32 {
        let a00 = self.x_col.x();
        let a01 = self.y_col.x();
        let a02 = self.z_col.x();
        let a03 = self.w_col.x();
        let a10 = self.x_col.y();
        let a11 = self.y_col.y();
        let a12 = self.z_col.y();
        let a13 = self.w_col.y();
        let a20 = self.x_col.z();
        let a21 = self.y_col.z();
        let a22 = self.z_col.z();
        let a23 = self.w_col.z();
        let a30 = self.x_col.w();
        let a31 = self.y_col.w();
        let a32 = self.z_col.w();
        let a33 = self.w_col.w();

        let high_01 = a00 * a11 - a01 * a10;
        let high_02 = a00 * a12 - a02 * a10;
        let high_03 = a00 * a13 - a03 * a10;
        let high_12 = a01 * a12 - a02 * a11;
        let high_13 = a01 * a13 - a03 * a11;
        let high_23 = a02 * a13 - a03 * a12;

        let low_01 = a20 * a31 - a21 * a30;
        let low_02 = a20 * a32 - a22 * a30;
        let low_03 = a20 * a33 - a23 * a30;
        let low_12 = a21 * a32 - a22 * a31;
        let low_13 = a21 * a33 - a23 * a31;
        let low_23 = a22 * a33 - a23 * a32;

        high_01 * low_23 - high_02 * low_13 + high_03 * low_12 +
            high_12 * low_03 - high_13 * low_02 + high_23 * low_01
    }

    pub fn inverse(&self) -> Option<Self> {
        let a00 = self.x_col.x();
        let a01 = self.y_col.x();
        let a02 = self.z_col.x();
        let a03 = self.w_col.x();
        let a10 = self.x_col.y();
        let a11 = self.y_col.y();
        let a12 = self.z_col.y();
        let a13 = self.w_col.y();
        let a20 = self.x_col.z();
        let a21 = self.y_col.z();
        let a22 = self.z_col.z();
        let a23 = self.w_col.z();
        let a30 = self.x_col.w();
        let a31 = self.y_col.w();
        let a32 = self.z_col.w();
        let a33 = self.w_col.w();

        let high_01 = a00 * a11 - a01 * a10;
        let high_02 = a00 * a12 - a02 * a10;
        let high_03 = a00 * a13 - a03 * a10;
        let high_12 = a01 * a12 - a02 * a11;
        let high_13 = a01 * a13 - a03 * a11;
        let high_23 = a02 * a13 - a03 * a12;

        let low_01 = a20 * a31 - a21 * a30;
        let low_02 = a20 * a32 - a22 * a30;
        let low_03 = a20 * a33 - a23 * a30;
        let low_12 = a21 * a32 - a22 * a31;
        let low_13 = a21 * a33 - a23 * a31;
        let low_23 = a22 * a33 - a23 * a32;

        let det = NonZeroF32::new(high_01 * low_23 - high_02 * low_13 + high_03 * low_12 +
            high_12 * low_03 - high_13 * low_02 + high_23 * low_01)?;

        let adjugate = Self::from_col_vecs(
            Vec4f::new(
                a11 * low_23 - a12 * low_13 + a13 * low_12,
                -(a10 * low_23 - a12 * low_03 + a13 * low_02),
                a10 * low_13 - a11 * low_03 + a13 * low_01,
                -(a10 * low_12 - a11 * low_02 + a12 * low_01),
            ),
            Vec4f::new(
                -(a01 * low_23 - a02 * low_13 + a03 * low_12),
                a00 * low_23 - a02 * low_03 + a03 * low_02,
                -(a00 * low_13 - a01 * low_03 + a03 * low_01),
                a00 * low_12 - a01 * low_02 + a02 * low_01,
            ),
            Vec4f::new(
                a31 * high_23 - a32 * high_13 + a33 * high_12,
                -(a30 * high_23 - a32 * high_03 + a33 * high_02),
                a30 * high_13 - a31 * high_03 + a33 * high_01,
                -(a30 * high_12 - a31 * high_02 + a32 * high_01),
            ),
            Vec4f::new(
                -(a21 * high_23 - a22 * high_13 + a23 * high_12),
                a20 * high_23 - a22 * high_03 + a23 * high_02,
                -(a20 * high_13 - a21 * high_03 + a23 * high_01),
                a20 * high_12 - a21 * high_02 + a22 * high_01,
            ),
        );

        Some(adjugate / det)
    }
}

impl Add for Mat4f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_col_vecs(
            self.x_col + rhs.x_col,
            self.y_col + rhs.y_col,
            self.z_col + rhs.z_col,
            self.w_col + rhs.w_col,
        )
    }
}

impl Sub for Mat4f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_col_vecs(
            self.x_col - rhs.x_col,
            self.y_col - rhs.y_col,
            self.z_col - rhs.z_col,
            self.w_col - rhs.w_col
        )
    }
}

impl Neg for Mat4f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_col_vecs(-self.x_col, -self.y_col, -self.z_col, -self.w_col)
    }
}

impl Mul<f32> for Mat4f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_col_vecs(self.x_col * rhs, self.y_col * rhs, self.z_col * rhs, self.w_col * rhs)
    }
}

impl Mul<Mat4f> for f32 {
    type Output = Mat4f;

    fn mul(self, rhs: Mat4f) -> Self::Output {
        rhs * self
    }
}

impl Mul for Mat4f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x_row = Vec4f::new(self.x_col.x(), self.y_col.x(), self.z_col.x(), self.w_col.x());
        let y_row = Vec4f::new(self.x_col.y(), self.y_col.y(), self.z_col.y(), self.w_col.y());
        let z_row = Vec4f::new(self.x_col.z(), self.y_col.z(), self.z_col.z(), self.w_col.z());
        let w_row = Vec4f::new(self.x_col.w(), self.y_col.w(), self.z_col.w(), self.w_col.w());

        Self::new(
            x_row.dot(rhs.x_col), x_row.dot(rhs.y_col), x_row.dot(rhs.z_col), x_row.dot(rhs.w_col),
            y_row.dot(rhs.x_col), y_row.dot(rhs.y_col), y_row.dot(rhs.z_col), y_row.dot(rhs.w_col),
            z_row.dot(rhs.x_col), z_row.dot(rhs.y_col), z_row.dot(rhs.z_col), z_row.dot(rhs.w_col),
            w_row.dot(rhs.x_col), w_row.dot(rhs.y_col), w_row.dot(rhs.z_col), w_row.dot(rhs.w_col),
        )
    }
}

impl Mul<Vec4f> for Mat4f {
    type Output = Vec4f;

    fn mul(self, rhs: Vec4f) -> Self::Output {
        self.x_col * rhs.x() + self.y_col * rhs.y() + self.z_col * rhs.z() + self.w_col * rhs.w()
    }
}

impl Div<NonZeroF32> for Mat4f {
    type Output = Self;

    fn div(self, rhs: NonZeroF32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Index<(usize, usize)> for Mat4f {
    type Output = f32;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        match col {
            0 => &self.x_col[row],
            1 => &self.y_col[row],
            2 => &self.z_col[row],
            3 => &self.w_col[row],
            _ => panic!("Column out of bounds"),
        }
    }
}

impl From<[[f32; 4]; 4]> for Mat4f {
    fn from(mat: [[f32; 4]; 4]) -> Self {
        Self::from_rows(mat[0], mat[1], mat[2], mat[3])
    }
}