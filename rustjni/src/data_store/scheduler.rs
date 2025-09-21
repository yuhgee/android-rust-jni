use std::{thread, time::Duration};
use crate::data_store::store::SharedDataStore;

pub fn start_scheduler<T>(store: SharedDataStore<T>, interval_sec: u64, batch_size: usize)
where
    T: std::fmt::Display + Clone + Send + 'static,
{
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(interval_sec));

            let mut store_guard = store.lock().unwrap();

            let snapshot: Vec<T> = store_guard
                .get_values()
                .into_iter()
                .take(batch_size)
                .collect();

            if !snapshot.is_empty() {
                println!(
                    "Scheduler output (snapshot size: {}, items: [{}])",
                    snapshot.len(),
                    snapshot
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                store_guard.clear();
            }
        }
    });
}
