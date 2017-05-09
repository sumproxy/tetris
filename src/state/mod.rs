pub mod color;
pub mod template;
mod timer;
mod piece;
mod queue;
mod map;

use self::color::Color;
use self::template::{Template, DeltaPos, Kind};
use self::timer::Timer;
use self::piece::Piece;
use self::queue::PieceQueue;
use self::map::{Map, Pos, Size2};

const MAX_COLLAPSED_ROWS: usize = 4;

trait Inner<T> {
    fn is_inside(&self, delta: T) -> bool;
}

pub enum Visible {
    Yes,
    No,
}

pub struct State {
    pub main: Map<Color>,
    pub preview: Map<Color>,
    pub queue: PieceQueue,
    pub piece: Piece,
    pub timer: Timer,
    pub score: u64,
    pub is_gameover: bool,
}

impl State {
    pub fn new() -> Self {
        let mut state = State {
            main: Map::<Color>::new(Size2 { w: 10, h: 22 }),
            preview: Map::<Color>::new(Size2 { w: 4, h: 22}),
            queue: PieceQueue::with_capacity(3),
            piece: Piece::generate(),
            timer: Timer::new(),
            score: 0,
            is_gameover: false,
        };

        state.redraw_preview();
        state.draw_piece(Visible::Yes);
        state
    }

    pub fn draw_piece(&mut self, visible: Visible) {
        let piece = self.piece.clone();
        if let Some(coords) = piece.try_into(&self.main) {
            for pos in coords {
                *self.main.tile_mut(pos) = match visible {
                    Visible::No => Color::default(),
                    Visible::Yes => piece.color,
                };
            }
        }
    }

    pub fn redraw_preview(&mut self) {
        for pos in self.preview.get_iter() {
            *self.preview.tile_mut(pos) = color::PREVIEW;
        }
        let mut bottom = 1;
        for piece in self.queue.data.iter() {
            let mut piece = piece.clone();
            if piece.template.1 == Kind::I {
                bottom -= 1;
            }
            piece.pos = Pos { x: 1, y: bottom };
            let coords = piece.try_into(&self.preview).unwrap();
            bottom = coords.iter().map(|pos| pos.y).max().unwrap() + 3;
            for pos in coords {
                *self.preview.tile_mut(pos) = piece.color;
            }
        }
    }

    pub fn spawn_piece(&mut self) -> Result<(), ()> {
        let piece = self.queue.next();
        if let Some(coords) = piece.try_into(&self.main) {
            self.redraw_preview();

            let is_colliding = coords
                .iter()
                .map(|&pos| *self.main.tile(pos))
                .any(|color| color != Color::default());

            if is_colliding {
                self.is_gameover = true;
                Err(())
            }
            else {
                self.piece = piece;
                Ok(())
            }
        }
        else {
            Err(())
        }
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
        if filled_rows.len() > 0 {
            self.timer.lower_threshold();
        }
        self.score += match filled_rows.len() {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0,
        };

        while let Some(row) = filled_rows.pop() {
            self.remove_row(row);
            for row in filled_rows.iter_mut() {
                *row += 1;
            }
        }
    }

    fn is_colliding(&self, piece: Piece) -> bool {
        if let (Some(old_coords), Some(new_coords)) = (self.piece.try_into(&self.main), piece.try_into(&self.main)) {
            new_coords
                .iter()
                .filter(|pos| !old_coords.contains(pos))
                .map(|&pos| *self.main.tile(pos))
                .any(|color| color != Color::default())
        }
        else {
            true
        }
    }

    fn filled_rows(&self) -> Vec<usize> {
        let mut result = Vec::with_capacity(MAX_COLLAPSED_ROWS);
        for y in 0..self.main.size().h {
            if self.is_row_filled(y) {
                result.push(y)
            }
        }
        result
    }

    fn is_row_filled(&self, y: usize) -> bool {
        for x in 0..self.main.size().w {
            let pos = Pos {x: x, y: y};
            if *self.main.tile(pos) == Color::default() {
                return false;
            }
        }
        true
    }

    fn remove_row(&mut self, y: usize) {
        for row in (1..y+1).rev() {
            self.move_row_down(row);
        }
        for x in 0..self.main.size().w {
            let pos = Pos {x: x, y: 0};
            *self.main.tile_mut(pos) = Color::default();
        }
    }

    fn move_row_down(&mut self, y: usize) {
        for x in 0..self.main.size().w {
            let (old_pos, new_pos) = (Pos { x: x, y: y-1 }, Pos { x: x, y: y });
            *self.main.tile_mut(new_pos) = *self.main.tile(old_pos);
        }
    }

    pub fn dim(&self) -> Size2 {
        let main = self.main.size;
        let preview = self.preview.size;
        Size2 { w: main.w + preview.w, h: main.h }
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
