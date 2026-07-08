use super::approx_eq::ApproxEq;

pub struct Point2f {
    x: f32,
    y: f32
}

impl ApproxEq for Point2f {
    fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
        self.x.approx_eq(&other.x, epsilon) && self.y.approx_eq(&other.y, epsilon)
    }
}