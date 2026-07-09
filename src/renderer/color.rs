use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorComponentOutOfRange {
    pub component: &'static str,
    pub value: f32,
}

impl fmt::Display for ColorComponentOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color component '{}' must be in [0.0, 1.0], got {}", self.component, self.value)
    }
}

impl std::error::Error for ColorComponentOutOfRange {}

#[derive(Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Result<Self, ColorComponentOutOfRange> {
        let in_range = |value: f32| (0f32..=1f32).contains(&value);

        if !in_range(r) { return Err(ColorComponentOutOfRange { component: "r", value: r }); }
        if !in_range(g) { return Err(ColorComponentOutOfRange { component: "g", value: g }); }
        if !in_range(b) { return Err(ColorComponentOutOfRange { component: "b", value: b }); }
        if !in_range(a) { return Err(ColorComponentOutOfRange { component: "a", value: a }); }

        Ok(Self { r, g, b, a })
    }
    
    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    pub fn a(&self) -> f32 {
        self.a
    }
}