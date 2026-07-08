#[derive(Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Result<Self, ()> {
        if r < 0f32 || r > 1f32 { Err(()) }
        else if g < 0f32 || g > 1f32 { Err(()) }
        else if b < 0f32 || b > 1f32 { Err(()) }
        else if a < 0f32 || a > 1f32 { Err(()) }
        else { Ok(Self { r, g, b, a }) }
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