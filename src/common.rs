#[derive(Clone)]
pub struct Color([f32; 3]);

pub mod color {
    pub const RED:     super::Color = super::Color([1.0f32, 0.0f32, 0.0f32]);
    pub const GREEN:   super::Color = super::Color([0.0f32, 1.0f32, 0.0f32]);
    pub const BLUE:    super::Color = super::Color([0.0f32, 0.0f32, 1.0f32]);
    pub const YELLOW:  super::Color = super::Color([1.0f32, 1.0f32, 0.0f32]);
    pub const MAGENTA: super::Color = super::Color([1.0f32, 0.0f32, 1.0f32]);
    pub const CYAN:    super::Color = super::Color([0.0f32, 1.0f32, 1.0f32]);
    pub const WHITE:   super::Color = super::Color([1.0f32, 1.0f32, 1.0f32]);
    pub const GRAY:    super::Color = super::Color([0.1f32, 0.1f32, 0.1f32]);
}

impl Default for Color {
    fn default() -> Self {
        color::GRAY
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

#[derive(Copy, Clone)]
pub struct Size2 {
    pub w: usize,
    pub h: usize,
}

#[derive(Copy, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Map<T> {
    tiles: Vec<T>,
    size: Size2,
}

impl<T: Clone + Default> Map<T> {
    pub fn new(size: Size2) -> Map<T> {
        let count = size.w * size.h;
        let tiles = ::std::iter::repeat(Default::default()).take(count).collect();
        Map {
            tiles: tiles,
            size: size,
        }
    }

    pub fn size(&self) -> Size2 {
        self.size
    }

    pub fn tile_mut(&mut self, pos: Pos) -> &mut T {
        assert!(self.is_inside(pos));
        let index = self.size.w * pos.y + pos.x;
        &mut self.tiles[index]
    }

    pub fn tile(&mut self, pos: Pos) -> &T {
        assert!(self.is_inside(pos));
        let index = self.size.w * pos.y + pos.x;
        &self.tiles[index]
    }

    pub fn is_inside(&self, pos: Pos) -> bool {
        let x = pos.x;
        let y = pos.y;
        x < self.size.w && y < self.size.h
    }

    pub fn get_iter(&self) -> MapIter {
        MapIter::new(self.size())
    }
}

#[derive(Clone)]
pub struct MapIter {
    cursor: Pos,
    size: Size2,
}

impl MapIter {
    fn new(size: Size2) -> Self {
        MapIter {
            cursor: Pos { x: 0, y: 0 },
            size: size,
        }
    }
}

impl Iterator for MapIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        let curr_pos = if self.cursor.y >= self.size.h {
            None
        } else {
            Some(self.cursor)
        };

        self.cursor.x += 1;
        if self.cursor.x >= self.size.w {
            self.cursor.x = 0;
            self.cursor.y += 1;
        }

        curr_pos
    }
}

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

    pub fn x_dim(&self) -> usize {
        self.inner.size.w
    }

    pub fn y_dim(&self) -> usize {
        self.inner.size.h
    }

    pub fn box_width(&self) -> f32 {
        1.0 / self.x_dim() as f32
    }

    pub fn box_height(&self) -> f32 {
        1.0 / self.y_dim() as f32
    }
}
