pub mod template;
pub mod color;
pub mod map;

use ::std::time::{Duration, Instant};

use self::map::{Map, Pos, Size2};
use self::template::{Template, DeltaPos};
use self::color::Color;

trait Inner<T> {
    fn is_inside(&self, delta: T) -> bool;
}

pub enum Visible {
    Yes,
    No,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub template: Template,
    pos: Pos,
    color: Color,
}

impl Piece {
    fn try_into(&self, board: &State) -> Option<Vec<Pos>> {
        let mut result = Vec::<Pos>::with_capacity(4);
        for delta in self.template.0.iter() {
            let dx = delta.dx + self.pos.x as isize;
            let dy = delta.dy + self.pos.y as isize;
            if dx < 0 || dy < 0 {
                return None;
            }
            let pos = Pos { x: dx as usize, y: dy as usize};
            if board.inner.is_inside(pos) {
                result.push(pos);
            }
            else {
                return None;
            }
        }

        Some(result)
    }

    fn generate() -> Self {
        Piece {
            template: Template::generate(),
            pos: Pos { x: 4, y: 1 },
            color: Color::generate(),
        }
    }
}

pub struct Timer {
    accumulator: Instant,
    threshold: Duration,
}

impl Timer {
    fn new() -> Self {
        Timer {
            accumulator: Instant::now(),
            threshold: Duration::from_millis(350),
        }
    }

    pub fn tick(&mut self) -> Option<()> {
        if self.accumulator.elapsed() > self.threshold {
            self.accumulator = Instant::now();
            Some(())
        } else {
            None
        }
    }
}

pub struct State {
    pub inner: Map<Color>,
    pub piece: Piece,
    pub timer: Timer,
}

impl State {
    pub fn new() -> Self {
        let mut state = State {
            inner: Map::<Color>::new(Size2 { w: 10, h: 22 }),
            piece: Piece::generate(),
            timer: Timer::new(),
        };

        state.draw_piece(Visible::Yes);
        state
    }

    pub fn draw_piece(&mut self, visible: Visible) {
        let piece = self.piece.clone();
        if let Some(coords) = piece.try_into(&self) {
            for pos in coords {
                *self.inner.tile_mut(pos) = match visible {
                    Visible::No => Color::default(),
                    Visible::Yes => piece.color,
                };
            }
        }
    }

    pub fn spawn_piece(&mut self) {
        self.piece = Piece::generate();
    }

    pub fn move_piece(&mut self, delta: DeltaPos) -> Result<(), ()> {
        let mut moved = self.piece;
        let mut pos = moved.pos;
        pos.x = (pos.x as isize + delta.dx) as usize;
        pos.y = (pos.y as isize + delta.dy) as usize;
        moved.pos = pos;

        if self.is_inside(moved) && !self.is_colliding(moved) {
            self.piece.pos = moved.pos;
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn rotate_piece(&mut self) {
        let mut rotated = self.piece;
        rotated.template = rotated.template.rotate_right();
        if self.is_inside(rotated) && !self.is_colliding(rotated) {
            self.piece = rotated;
        }
    }

    pub fn hard_drop(&mut self) {
        let down = DeltaPos { dx: 0, dy: 1 };
        while let Ok(()) = self.move_piece(down) {};
    }

    pub fn collapse_rows(&mut self) {
        let mut filled_rows = self.filled_rows();
        while let Some(row) = filled_rows.pop() {
            self.remove_row(row);
            for row in filled_rows.iter_mut() {
                *row += 1;
            }
        }
    }

    fn is_colliding(&self, piece: Piece) -> bool {
        if let (Some(old_coords), Some(new_coords)) = (self.piece.try_into(&self), piece.try_into(&self)) {
            new_coords
                .iter()
                .filter(|pos| !old_coords.contains(pos))
                .map(|&pos| *self.inner.tile(pos))
                .any(|color| color != Color::default())
        }
        else {
            true
        }
    }

    fn filled_rows(&self) -> Vec<usize> {
        let mut result = Vec::with_capacity(4); // can't get more than 4
        for y in 0..self.inner.size().h {
            if self.is_row_filled(y) {
                result.push(y)
            }
        }
        result
    }

    fn is_row_filled(&self, y: usize) -> bool {
        for x in 0..self.inner.size().w {
            let pos = Pos {x: x, y: y};
            if *self.inner.tile(pos) == Color::default() {
                return false;
            }
        }
        true
    }

    fn remove_row(&mut self, y: usize) {
        for row in (1..y+1).rev() {
            self.move_row_down(row);
        }
        for x in 0..self.inner.size().w {
            let pos = Pos {x: x, y: 0};
            *self.inner.tile_mut(pos) = Color::default();
        }
    }

    fn move_row_down(&mut self, y: usize) {
        for x in 0..self.inner.size().w {
            let (old_pos, new_pos) = (Pos { x: x, y: y-1 }, Pos { x: x, y: y });
            *self.inner.tile_mut(new_pos) = *self.inner.tile(old_pos);
        }
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

impl Inner<Piece> for State {
    fn is_inside(&self, piece: Piece) -> bool {
        let kind = piece.template;
        let pos = piece.pos;
        
        kind.0.iter().all(|tile| {
            let dx = tile.dx + pos.x as isize;
            let dy = tile.dy + pos.y as isize;
            0 <= dx && 0 <= dy && dx < self.dim().w as isize && dy < self.dim().h as isize
        })
    }
}
