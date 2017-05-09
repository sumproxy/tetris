use super::Piece;

use ::std::collections::VecDeque;

pub struct PieceQueue {
    pub data: VecDeque<Piece>
}

impl PieceQueue {
    pub fn with_capacity(size: usize) -> Self {
        let mut data = VecDeque::<Piece>::with_capacity(size);
        for _ in 0..3 {
            data.push_back(Piece::generate());
        }
        PieceQueue { data: data }
    }

    pub fn next(&mut self) -> Piece {
        let next = self.data.pop_front();
        self.data.push_back(Piece::generate());
        next.unwrap()
    }
}
