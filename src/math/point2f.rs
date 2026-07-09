use super::approx_eq::ApproxEq;

#[derive(Copy, Clone)]
pub struct Point2f {
    x: f32,
    y: f32
}

impl Point2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl ApproxEq for Point2f {
    fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
        self.x.approx_eq(&other.x, epsilon) && self.y.approx_eq(&other.y, epsilon)
    }
}