pub const MAX_CATCH_UP_UPDATES: u8 = 10;
pub const FLOAT_EPS: f32 = 1e-5;
pub const MAX_COLORED_RECTANGLES: usize = 100;
pub const VERTICES_PER_RECTANGLE: usize = 4;
pub const INDICES_PER_RECTANGLE: usize = 6;
pub const MAX_COLORED_LINES: usize = 100;
pub const VERTICES_PER_LINE: usize = 2;
pub const ASSETS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");