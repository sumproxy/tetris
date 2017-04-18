#[derive(Copy, Clone, Debug)]
pub struct Color([f32; 3]);

pub const RED:     Color = Color([0.8f32, 0.0f32, 0.0f32]);
pub const GREEN:   Color = Color([0.0f32, 0.8f32, 0.0f32]);
pub const BLUE:    Color = Color([0.0f32, 0.0f32, 0.8f32]);
pub const YELLOW:  Color = Color([0.8f32, 0.8f32, 0.0f32]);
pub const MAGENTA: Color = Color([0.8f32, 0.0f32, 0.8f32]);
pub const CYAN:    Color = Color([0.0f32, 0.8f32, 0.8f32]);
pub const WHITE:   Color = Color([0.8f32, 0.8f32, 0.8f32]);
pub const GRAY:    Color = Color([0.1f32, 0.1f32, 0.1f32]);

impl Color {
    pub fn generate() -> Self {
        use rand;
        use rand::Rng;
        
        static COLORS: [Color; 7] = [RED, GREEN, BLUE, YELLOW, MAGENTA, CYAN, WHITE];
        let mut rng = rand::thread_rng();
        COLORS[rng.gen_range(0, 7)]
    }
}

impl Default for Color {
    fn default() -> Self {
        GRAY
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        self.0
    }
}

impl<'a> Into<[f32; 3]> for &'a Color {
    fn into(self) -> [f32; 3] {
        self.0
    }
}
