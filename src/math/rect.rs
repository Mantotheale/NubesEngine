use crate::math::point2f::Point2f;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidRectDimension {
    pub dimension: &'static str,
    pub value: f32,
}

impl fmt::Display for InvalidRectDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rect's {} must be positive, it was {}", self.dimension, self.value)
    }
}

impl std::error::Error for InvalidRectDimension {}

#[derive(Copy, Clone)]
pub struct Rect {
    center: Point2f,
    width: f32,
    height: f32
}

impl Rect {
    pub fn new(center: Point2f, width: f32, height: f32) -> Result<Self, InvalidRectDimension> {
        if width <= 0f32 { return Err(InvalidRectDimension { dimension: "width", value: width }); }
        if height <= 0f32 { return Err(InvalidRectDimension { dimension: "height", value: height }); }

        Ok(Self { center, width, height })
    }

    pub fn center(&self) -> Point2f {
        self.center
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn left(&self) -> f32 {
        self.center.x() - (0.5 * self.width)
    }

    pub fn right(&self) -> f32 {
        self.center.x() + (0.5 * self.width)
    }

    pub fn top(&self) -> f32 {
        self.center.y() + (0.5 * self.height)
    }

    pub fn bottom(&self) -> f32 {
        self.center.y() - (0.5 * self.height)
    }

    pub fn bottom_left(&self) -> Point2f {
        Point2f::new(self.left(), self.bottom())
    }

    pub fn bottom_right(&self) -> Point2f {
        Point2f::new(self.right(), self.bottom())
    }

    pub fn top_right(&self) -> Point2f {
        Point2f::new(self.right(), self.top())
    }

    pub fn top_left(&self) -> Point2f {
        Point2f::new(self.left(), self.top())
    }
}