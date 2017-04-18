use super::Inner;

#[derive(Copy, Clone)]
pub struct Size2 {
    pub w: usize,
    pub h: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Map<T> {
    tiles: Vec<T>,
    pub size: Size2,
}

impl<T: Clone + Default> Map<T> {
    pub fn new(size: Size2) -> Map<T> {
        let count = size.w * size.h;
        let tiles = ::std::iter::repeat(Default::default()).take(count).collect();
        Map {
            tiles: tiles,
            size: size,
        }
    }

    pub fn size(&self) -> Size2 {
        self.size
    }

    pub fn tile_mut(&mut self, pos: Pos) -> &mut T {
        assert!(self.is_inside(pos));
        let index = self.size.w * pos.y  + pos.x;
        &mut self.tiles[index]
    }

    pub fn tile(&mut self, pos: Pos) -> &T {
        assert!(self.is_inside(pos));
        let index = self.size.w * pos.y + pos.x;
        &self.tiles[index]
    }

    pub fn get_iter(&self) -> MapIter {
        MapIter::new(self.size())
    }
}

#[derive(Clone)]
pub struct MapIter {
    cursor: Pos,
    size: Size2,
}

impl MapIter {
    fn new(size: Size2) -> Self {
        MapIter {
            cursor: Pos { x: 0, y: 0 },
            size: size,
        }
    }
}

impl Iterator for MapIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        let curr_pos = if self.cursor.y >= self.size.h {
            None
        } else {
            Some(self.cursor)
        };

        self.cursor.x += 1;
        if self.cursor.x >= self.size.w {
            self.cursor.x = 0;
            self.cursor.y += 1;
        }

        curr_pos
    }
}

impl<T> Inner<Pos> for Map<T> {
    fn is_inside(&self, delta: Pos) -> bool {
        let x = delta.x;
        let y = delta.y;
        x < self.size.w && y < self.size.h
    }
}
