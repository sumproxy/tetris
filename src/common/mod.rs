pub mod template;
pub mod color;
pub mod map;

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

#[derive(Debug)]
pub struct State {
    pub inner: Map<Color>,
    pub piece: Piece,
}

impl State {
    pub fn new() -> Self {
        let mut state = State {
            inner: Map::<Color>::new(Size2 { w: 10, h: 22 }),
            piece: Piece::generate(),
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

    pub fn bake_piece(&mut self) {
        self.draw_piece(Visible::Yes);
        self.piece = Piece::generate();
        self.draw_piece(Visible::Yes);
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
        while let Ok(()) = self.move_piece(DeltaPos { dx: 0, dy: 1 }) {};
    }

    fn is_colliding(&self, piece: Piece) -> bool {
        let old_coords;
        if let Some(coords) = self.piece.try_into(&self) {
            old_coords = coords;
        }
        else {
            return true;
        }

        if let Some(coords) = piece.try_into(&self) {
            coords
                .iter()
                .filter(|x| !old_coords.contains(x))
                .map(|&pos| *self.inner.tile(pos))
                .any(|col| col != Color::default())
        }
        else {
            true
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
