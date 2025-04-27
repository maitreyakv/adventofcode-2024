use std::collections::VecDeque;

#[derive(Debug)]
pub struct FixedSizeQueue<T> {
    size: usize,
    data: VecDeque<T>,
}

impl<T> FixedSizeQueue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: VecDeque::with_capacity(size),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.data.len() == self.size {
            self.data.pop_front();
        };
        self.data.push_back(item);
    }
}

impl<T> std::ops::Index<usize> for FixedSizeQueue<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
