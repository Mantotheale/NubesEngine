use std::fmt;
use crate::constants;
use crate::math::approx_eq::ApproxEq;
use crate::math::point2f::Point2f;
use crate::math::vec2f::Vec2f;

#[derive(Debug)]
pub struct SegmentPointsCoincide {
    origin: Point2f,
    destination: Point2f
}

impl fmt::Display for SegmentPointsCoincide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Segment's defining points must not coincide, they were {:?}, {:?}", self.origin, self.destination)
    }
}

impl std::error::Error for SegmentPointsCoincide {}

pub struct Segment {
    origin: Point2f,
    destination: Point2f
}

impl Segment {
    pub fn new(origin: Point2f, destination: Point2f) -> Result<Self, SegmentPointsCoincide> {
        if origin.approx_eq(&destination, constants::FLOAT_EPS) {
            Err(SegmentPointsCoincide { origin, destination })
        } else { 
            Ok(Self { origin, destination })
        }
    }
    
    pub fn origin(&self) -> Point2f {
        self.origin
    }

    pub fn destination(&self) -> Point2f {
        self.destination
    }
    
    pub fn direction(&self) -> Vec2f {
        self.destination - self.origin
    }
}