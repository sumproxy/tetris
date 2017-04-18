pub mod template;
pub mod color;
pub mod map;

use self::map::{Map, Pos, Size2};
use self::template::{Template, DeltaPos};
use self::color::Color;

trait Inner<T> {
    fn is_inside(&self, delta: T) -> bool;
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
}

#[derive(Debug)]
pub struct State {
    pub inner: Map<Color>,
    pub piece: Piece,
}

impl State {
    pub fn new() -> Self {
        let kind = Template::generate();
        let color = Color::generate();
        let pos = Pos { x: 4, y: 1 };
        let mut state = State {
            inner: Map::<Color>::new(Size2 { w: 10, h: 22 }),
            piece: Piece { template: kind, pos: pos, color: color },
        };

        state.draw_piece(true);
        state
    }

    pub fn move_piece(&mut self, delta: DeltaPos) {
        if self.is_inside(delta) {
            self.draw_piece(false);
            let mut pos = self.piece.pos;
            pos.x = (pos.x as isize + delta.dx) as usize;
            pos.y = (pos.y as isize + delta.dy) as usize;
            self.piece.pos = pos;
            self.draw_piece(true);
        }
    }

    pub fn rotate_piece(&mut self) {
        let backup = self.piece.template;
        let rotated = backup.rotate_right();
        self.piece.template = rotated;
        if self.is_inside(DeltaPos { dx: 0, dy: 0 }) {
            self.piece.template = backup;
            self.draw_piece(false);
            self.piece.template = rotated;
            self.draw_piece(true);
        }
        else {
            self.piece.template = backup;
        }
    }

    pub fn draw_piece(&mut self, visible: bool) {
        let piece = self.piece.clone();
        if let Some(coords) = piece.try_into(&self) {
            for pos in coords {
                *self.inner.tile_mut(pos) = if !visible { color::Color::default() } else { piece.color };
            }
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
