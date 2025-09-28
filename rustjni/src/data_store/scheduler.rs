use tokio::time::{interval, Duration};
use crate::data_store::store::SharedStore;
use crate::jni::jni_celltower::fetch_cell_towers_safe;
use crate::jni::jni_impl::get_context_ref;


/// 非同期スケジューラを開始する
pub async fn start_scheduler<T>(store: SharedStore<T>, interval_sec: u64, batch_size: usize)
where
    T: std::fmt::Display + Clone + Send + 'static,
{
    let mut ticker = interval(Duration::from_secs(interval_sec));

    tokio::spawn(async move {
        loop {
            ticker.tick().await;
            // ワーカースレッドからの呼び出し
            let context_guard = get_context_ref();
            let context_ref = context_guard
                .as_ref()
                .expect("Context not initialized");
            let context_obj = context_ref.as_obj();
            let result = fetch_cell_towers_safe(context_obj);
    
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
