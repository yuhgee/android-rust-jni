use crate::service::store::SharedStore;
use crate::service::tasks::ScheduledTask;
use async_trait::async_trait;
use std::time::Duration;
use std::sync::Mutex;

pub struct CounterTask {
    store: SharedStore<i32>,
    interval: Duration,
}

impl CounterTask {
    pub fn new(interval: Duration, limit: usize) -> Self {
        Self {
            store: SharedStore::new(Mutex::new(crate::service::store::Store::new(limit))),
            interval,
        }
    }

    pub fn get_store(&self) -> SharedStore<i32> {
        std::sync::Arc::clone(&self.store)
    }
}

#[async_trait]
impl ScheduledTask for CounterTask {
    async fn run(&self) {
        let mut s = self.store.lock().unwrap();
        let last = s.get_values().last().cloned().unwrap_or(0);
        s.add(last + 1);
        println!("CounterTask last: {}", last + 1);
    }

    fn interval(&self) -> Duration {
        self.interval
    }
}
