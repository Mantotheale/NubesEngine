pub trait ApproxEq {
    fn approx_eq(&self, other: &Self, epsilon: f32) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
        (self - other).abs() <= epsilon
    }
}