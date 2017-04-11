pub mod color {
    pub const RED:     [f32; 3] = [1.0f32, 0.0f32, 0.0f32];
    pub const GREEN:   [f32; 3] = [0.0f32, 1.0f32, 0.0f32];
    pub const BLUE:    [f32; 3] = [0.0f32, 0.0f32, 1.0f32];
    pub const YELLOW:  [f32; 3] = [1.0f32, 1.0f32, 0.0f32];
    pub const MAGENTA: [f32; 3] = [1.0f32, 0.0f32, 1.0f32];
    pub const CYAN:    [f32; 3] = [0.0f32, 1.0f32, 1.0f32];
    pub const WHITE:   [f32; 3] = [1.0f32, 1.0f32, 1.0f32];
    pub const GRAY:    [f32; 3] = [0.1f32, 0.1f32, 0.1f32];
}

pub struct State {
    pub frame: Vec<Vec<Option<[f32; 3]>>>
}

impl State {
    pub fn new() -> Self {
        let mut state = State { frame: vec![vec![None; 10]; 22]};
        state.frame[0][0] = Some(color::WHITE);
        state.frame[0][1] = Some(color::CYAN);
        state.frame[0][2] = Some(color::MAGENTA);
        state.frame[0][3] = Some(color::YELLOW);
        state.frame[0][4] = Some(color::BLUE);
        state.frame[0][5] = Some(color::GREEN);
        state.frame[0][6] = Some(color::RED);
        state
    }

    pub fn x_dim(&self) -> usize {
        self.frame[0].len()
    }

    pub fn y_dim(&self) -> usize {
        self.frame.len()
    }

    pub fn box_width(&self) -> f32 {
        1.0 / self.x_dim() as f32
    }

    pub fn box_height(&self) -> f32 {
        1.0 / self.y_dim() as f32
    }
}
