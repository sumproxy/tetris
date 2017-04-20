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

    pub fn bake_piece(&mut self) {
        self.draw_piece(Visible::Yes);
        self.piece = Piece::generate();
        self.draw_piece(Visible::Yes);
    }

    pub fn move_piece(&mut self, delta: DeltaPos) {
        if self.is_inside(delta) {
            let mut moved = self.piece;
            let mut pos = moved.pos;
            pos.x = (pos.x as isize + delta.dx) as usize;
            pos.y = (pos.y as isize + delta.dy) as usize;
            moved.pos = pos;

            if !self.is_colliding(moved) {
                self.piece.pos = moved.pos;
            }
        }
    }

    pub fn rotate_piece(&mut self) {
        let mut copy = self.piece;
        let backup_template = copy.template;
        let rotated_template = copy.template.rotate_right();
        copy.template = rotated_template;
        let is_colliding = self.is_colliding(copy);
        self.piece.template = rotated_template;
        if self.is_inside(DeltaPos { dx: 0, dy: 0 }) && !is_colliding {
            self.piece.template = backup_template;
            self.draw_piece(Visible::No);
            self.piece.template = rotated_template;
            self.draw_piece(Visible::Yes);
        }
        else {
            self.piece.template = backup_template;
        }
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

impl Inner<DeltaPos> for State {
    fn is_inside(&self, delta: DeltaPos) -> bool {
        let kind = self.piece.template;
        let pos = self.piece.pos;
        
        kind.0.iter().all(|tile| {
            let dx = tile.dx + pos.x as isize + delta.dx;
            let dy = tile.dy + pos.y as isize + delta.dy;
            0 <= dx && 0 <= dy && dx < self.dim().w as isize && dy < self.dim().h as isize
        })
    }
}
