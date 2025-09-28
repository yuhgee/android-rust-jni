use crate::service::store::SharedStore;
use crate::service::tasks::ScheduledTask;
use async_trait::async_trait;
use std::time::Duration;
use std::sync::Mutex;

pub struct LoggerTask {
    store: SharedStore<String>,
    interval: Duration,
}

impl LoggerTask {
    pub fn new(interval: Duration, limit: usize) -> Self {
        Self {
            store: SharedStore::new(Mutex::new(crate::service::store::Store::new(limit))),
            interval,
        }
    }

    pub fn get_store(&self) -> SharedStore<String> {
        std::sync::Arc::clone(&self.store)
    }

    pub fn clear(&self) {
        let mut s = self.store.lock().unwrap();
        s.clear();
    }
}

#[async_trait]
impl ScheduledTask for LoggerTask {
    async fn run(&self) {
        let snapshot = self.store.lock().unwrap().get_values();
        println!("LoggerTask snapshot: {:?}", snapshot);
    }

    fn interval(&self) -> Duration {
        self.interval
    }
}
