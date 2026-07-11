use std::ops::{Deref, Div};
use crate::constants;
use crate::math::approx_eq::ApproxEq;

#[derive(Debug, Copy, Clone)]
pub struct NonZeroF32 {
    value: f32
}

impl NonZeroF32 {
    pub fn new(value: f32) -> Option<Self> {
        if value.approx_eq(&0f32, constants::FLOAT_EPS) { None }
        else { Some(Self { value }) }
    }
}

impl Deref for NonZeroF32 {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AsRef<f32> for NonZeroF32 {
    fn as_ref(&self) -> &f32 {
        self.deref()
    }
}

impl Div<NonZeroF32> for f32 {
    type Output = f32;

    fn div(self, rhs: NonZeroF32) -> Self::Output {
        self / rhs.value
    }
}