use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Store<T> {
    values: VecDeque<T>,
    limit: usize,
}

impl<T> Store<T> {
    pub fn new(limit: usize) -> Self {
        Self {
            values: VecDeque::new(),
            limit,
        }
    }

    pub fn add(&mut self, value: T) {
        self.values.push_back(value);
        while self.values.len() > self.limit {
            self.values.pop_front();
        }
    }

    pub fn get_values(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.values.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

pub type SharedStore<T> = Arc<Mutex<Store<T>>>;
