use super::Generate;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    I, T, O, J, L, S, Z,
}

#[derive(Copy, Clone, Debug)]
pub struct DeltaPos {
    pub dx: isize,
    pub dy: isize,
}

#[derive(Clone, Copy, Debug)]
pub struct Template(pub [DeltaPos; 4], pub Kind);

impl Generate for Template {
    fn generate() -> Self {
        use rand;
        use rand::Rng;

        let mut rng = rand::thread_rng();
        static TEMPLATES: [Template; 7] = [I, T, O, J, L, S, Z];
        TEMPLATES[rng.gen_range(0, 7)]
    }
}

impl Template {
    #[allow(dead_code)]
    pub fn rotate_left(&self) -> Self {
        let mut piece = self.clone();
        if self.1 == Kind::O {
            return piece;
        }

        for (output, input) in piece.0.iter_mut().zip(self.0.iter()) {
            output.dx = -input.dy;
            output.dy =  input.dx;
        }
        piece
    }

    pub fn rotate_right(&self) -> Self {
        let mut piece = self.clone();
        if self.1 == Kind::O {
            return piece;
        }

        for (output, input) in piece.0.iter_mut().zip(self.0.iter()) {
            output.dx =  input.dy;
            output.dy = -input.dx;
        }
        piece
    }
}

pub const I: Template = Template([DeltaPos { dx: -1, dy:  0 }, DeltaPos { dx:  0, dy:  0 }, DeltaPos { dx:  1, dy: 0 }, DeltaPos { dx: 2, dy: 0 }], Kind::I);
pub const T: Template = Template([DeltaPos { dx:  0, dy: -1 }, DeltaPos { dx: -1, dy:  0 }, DeltaPos { dx:  0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }], Kind::T);
pub const O: Template = Template([DeltaPos { dx:  0, dy: -1 }, DeltaPos { dx:  1, dy: -1 }, DeltaPos { dx:  0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }], Kind::O);
pub const J: Template = Template([DeltaPos { dx: -1, dy: -1 }, DeltaPos { dx: -1, dy:  0 }, DeltaPos { dx:  0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }], Kind::J);
pub const L: Template = Template([DeltaPos { dx:  1, dy: -1 }, DeltaPos { dx: -1, dy:  0 }, DeltaPos { dx:  0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }], Kind::L);
pub const S: Template = Template([DeltaPos { dx:  0, dy: -1 }, DeltaPos { dx:  1, dy: -1 }, DeltaPos { dx: -1, dy: 0 }, DeltaPos { dx: 0, dy: 0 }], Kind::S);
pub const Z: Template = Template([DeltaPos { dx: -1, dy: -1 }, DeltaPos { dx: 0, dy: -1 }, DeltaPos { dx: 0, dy: 0 }, DeltaPos { dx: 1, dy: 0 }], Kind::Z);
