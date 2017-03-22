use bit_vec::BitVec;

pub struct State {
    pub frame: Frame,
}

impl State {
    pub fn new() -> Self {
        let frame = Frame::new(10, 22);
        State { frame: frame }
    }
}

pub struct Frame {
    inner: Vec<BitVec>
}

impl Frame {
    fn new(x: usize, y: usize) -> Self {
        let zeroes = BitVec::from_fn(x, |_| { false });
        Frame {
            inner: vec![zeroes; y],
        }
    }

    pub fn x(&self) -> usize {
        self.inner[0].len()
    }

    pub fn y(&self) -> usize {
        self.inner.len()
    }
}
