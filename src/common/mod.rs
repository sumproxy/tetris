pub mod piece;
pub mod color;
pub mod map;

use self::map::Map;
use self::map::Size2;
use self::color::Color;

pub struct State {
    pub inner: Map<Color>,
}

impl State {
    pub fn new() -> Self {
        let mut state = State { inner: Map::<Color>::new(Size2 { w: 10, h: 22 }) };
        let colors = [color::WHITE, color::CYAN, color::MAGENTA, color::YELLOW, color::BLUE, color::GREEN, color::RED];
        let mut iter = colors.iter().cycle();
        for pos in state.inner.get_iter() {
            *state.inner.tile_mut(pos) = iter.next().unwrap().clone();
        }
        state
    }

    pub fn dim(&self) -> Size2 {
        self.inner.size
    }

    pub fn box_width(&self) -> f32 {
        1.0 / self.dim().w as f32
    }

    pub fn box_height(&self) -> f32 {
        1.0 / self.dim().h as f32
    }
}