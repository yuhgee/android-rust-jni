use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Store<T> {
    values: VecDeque<T>,
    limit: usize,
}

impl<T> Store<T> {
    /// 新しい Store を作成
    pub fn new(limit: usize) -> Self {
        Self {
            values: VecDeque::new(),
            limit,
        }
    }

    /// 値を追加。limit を超えたら古いものから削除（FIFO）
    pub fn add(&mut self, value: T) {
        self.values.push_back(value);
        while self.values.len() > self.limit {
            self.values.pop_front();
        }
    }

    /// 現在の値のスナップショットを返す（クローン）
    pub fn get_values(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.values.iter().cloned().collect()
    }

    /// 全ての値をクリア
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

/// スレッドセーフ版 SharedStore
pub type SharedStore<T> = Arc<Mutex<Store<T>>>;
