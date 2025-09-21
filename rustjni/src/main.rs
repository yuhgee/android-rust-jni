mod data_store;

use std::{sync::{Arc, Mutex}, thread, time::Duration};
use data_store::store::{Store, SharedDataStore};
use data_store::scheduler::start_scheduler;
use data_store::my_data::MyData;

fn main() {
    let store: SharedDataStore<MyData> = Arc::new(Mutex::new(Store::new(5)));
    let mut counter: u64 = 0;

    // スケジューラ開始（10秒ごとに5件出力してクリア）
    start_scheduler(Arc::clone(&store), 10, 20);

    loop {
        thread::sleep(Duration::from_secs(1));
        counter += 1;

        let mut store_guard = store.lock().unwrap();
        let data = MyData::new(counter);
        store_guard.add(data.clone());

        // main で追加時に values 全体を Display 表示
        let snapshot = store_guard.get_values();
        println!(
            "Added: {}, Current queue: [{}]",
            counter,
            snapshot
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
