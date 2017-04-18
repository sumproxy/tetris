pub mod template;
pub mod color;
pub mod map;

use self::map::{Map, Pos, Size2};
use self::template::{Template, DeltaPos};
use self::color::Color;

trait Inner<T> {
    fn is_inside(&self, delta: T) -> bool;
}

pub struct Piece {
    template: Template,
    pos: Pos,
    color: Color,
}

impl Piece {
    fn into(&self, board: &State) -> [Pos; 4] {
        [Pos { x: 0, y: 0 }; 4]
    }
}

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
        
        // let mut iter = colors.iter().cycle();
        // for pos in state.inner.get_iter() {
        //     *state.inner.tile_mut(pos) = iter.next().unwrap().clone();
        // }
        state
    }

    pub fn move_piece(&mut self, mut delta: DeltaPos) {
        if self.is_inside(delta) {
            self.remove_piece();
            self.put_piece();
        }
    }
    
    pub fn put_piece(&mut self) {
        
    }

    pub fn remove_piece(&mut self) {
        let kind = self.piece.template;
        let pos = self.piece.pos;
        for delta in kind.0.iter() {
            //pos.x + delta.dx
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
            0 < dx && 0 < dy && dx < self.dim().w as isize && dy < self.dim().h as isize
        })
    }
}
