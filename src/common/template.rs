#[derive(Copy, Clone, Debug)]
struct DeltaPos {
    pub dx: isize,
    pub dy: isize,
}

#[derive(Clone, Copy, Debug)]
pub struct Template([DeltaPos; 4]);

impl Template {
    pub fn rotate_left(&self) -> Self {
        let mut piece = self.clone();
        for (output, input) in piece.0.iter_mut().zip(self.0.iter()) {
            output.dx = -input.dy;
            output.dy =  input.dx;
        }
        piece
    }

    pub fn rotate_right(&self) -> Self {
        let mut piece = self.clone();
        for (output, input) in piece.0.iter_mut().zip(self.0.iter()) {
            output.dx =  input.dy;
            output.dy = -input.dx;
        }
        piece
    }
}

pub const I: Template = Template([DeltaPos { dx: -1, dy:  0 }, DeltaPos { dx:  0, dy:  0 }, DeltaPos { dx:  1, dy: 0 }, DeltaPos { dx: 2, dy: 0 }]);
pub const T: Template = Template([DeltaPos { dx:  0, dy: -1 }, DeltaPos { dx: -1, dy:  0 }, DeltaPos { dx:  0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }]);
pub const O: Template = Template([DeltaPos { dx:  0, dy: -1 }, DeltaPos { dx:  1, dy: -1 }, DeltaPos { dx:  0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }]);
pub const J: Template = Template([DeltaPos { dx: -1, dy: -1 }, DeltaPos { dx: -1, dy:  0 }, DeltaPos { dx:  0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }]);
pub const L: Template = Template([DeltaPos { dx:  1, dy: -1 }, DeltaPos { dx: -1, dy:  0 }, DeltaPos { dx:  0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }]);
pub const S: Template = Template([DeltaPos { dx:  0, dy: -1 }, DeltaPos { dx:  1, dy: -1 }, DeltaPos { dx: -1, dy: 0 }, DeltaPos { dx: 0, dy: 0 }]);
pub const Z: Template = Template([DeltaPos { dx: -1, dy: -1 }, DeltaPos { dx: 0, dy: -1 }, DeltaPos { dx: 0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }]);
