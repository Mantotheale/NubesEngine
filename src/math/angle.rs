use std::f32::consts::TAU;
use crate::math::approx_eq::ApproxEq;

#[derive(Debug, Copy, Clone)]
pub struct Angle {
    angle_rad: f32
}

impl Angle {
    pub fn from_radians(angle_rad: f32) -> Self {
        Self { angle_rad: angle_rad.rem_euclid(TAU) }
    }

    pub fn from_degrees(angle_deg: f32) -> Self {
        Self::from_radians(angle_deg.to_radians())
    }

    pub fn sin(&self) -> f32 {
        self.angle_rad.sin()
    }

    pub fn cos(&self) -> f32 {
        self.angle_rad.cos()
    }
}

impl ApproxEq for Angle {
    fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
        let abs_diff = (self.angle_rad - other.angle_rad).abs();
        abs_diff.approx_eq(&0.0, epsilon) || abs_diff.approx_eq(&TAU, epsilon)
    }
}