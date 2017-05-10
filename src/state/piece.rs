use super::{Inner, Generate, Template, Pos, Color, Map, MAX_COLLAPSED_ROWS};

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub template: Template,
    pub pos: Pos,
    pub color: Color,
}

impl Generate for Piece {
    fn generate() -> Self {
        Piece {
            template: Template::generate(),
            pos: Pos { x: 4, y: 1 },
            color: Color::generate(),
        }
    }
}

impl Piece {
    pub fn try_into(&self, map: &Map<Color>) -> Option<Vec<Pos>> {
        let mut result = Vec::<Pos>::with_capacity(MAX_COLLAPSED_ROWS);
        for delta in self.template.0.iter() {
            let dx = delta.dx + self.pos.x as isize;
            let dy = delta.dy + self.pos.y as isize;
            if dx < 0 || dy < 0 {
                return None;
            }
            let pos = Pos { x: dx as usize, y: dy as usize};
            if map.is_inside(pos) {
                result.push(pos);
            }
            else {
                return None;
            }
        }

        Some(result)
    }
}