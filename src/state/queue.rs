use super::Generate;

use ::std::collections::VecDeque;

pub struct Queue<T> {
    pub data: VecDeque<T>
}

impl<T: Generate> Queue<T> {
    pub fn with_capacity(size: usize) -> Self {
        let mut data = VecDeque::<T>::with_capacity(size);
        for _ in 0..3 {
            data.push_back(T::generate());
        }
        Queue { data: data }
    }

    pub fn next(&mut self) -> T {
        let next = self.data.pop_front();
        self.data.push_back(T::generate());
        next.unwrap()
    }
}
