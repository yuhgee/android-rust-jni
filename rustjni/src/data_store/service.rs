use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

use crate::data_store::store::{Store, SharedStore};
use crate::data_store::my_data::MyData;
use crate::data_store::scheduler::start_scheduler;

pub struct Service {
    runtime: Option<Runtime>,
    stop_tx: Option<oneshot::Sender<()>>,
    store: SharedStore<MyData>, // SharedStore を使用
}

impl Service {
    pub fn new() -> Self {
        let store: SharedStore<MyData> = Arc::new(Mutex::new(Store::new(100)));
        Service {
            runtime: None,
            stop_tx: None,
            store,
        }
    }

    /// サービスを開始（Tokio runtime を起動してスケジューラ実行）
    pub fn start(&mut self) {
        if self.runtime.is_some() {
            println!("Service already running");
            return;
        }

        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        let store_clone = Arc::clone(&self.store);

        let (tx, rx) = oneshot::channel::<()>();
        self.stop_tx = Some(tx);

        rt.spawn(async move {
            tokio::select! {
                _ = start_scheduler(store_clone, 5, 20) => {},
                _ = rx => {
                    println!("Stop signal received, shutting down scheduler");
                }
            }
        });

        self.runtime = Some(rt);
        println!("Service started");
    }

    /// サービスを停止（スケジューラを終了）
    pub fn stop(&mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(()); // 停止シグナル送信
        }
        self.runtime.take(); // runtime を drop してスレッド停止
        println!("Service stopped");
    }

    /// SharedStore にデータを追加して、リストを表示
    pub fn add_data(&self, value: u64) {
        let store_clone = Arc::clone(&self.store);
        std::thread::spawn(move || {
            let mut store_guard = store_clone.lock().unwrap();
            let data = MyData::new(value);
            store_guard.add(data);

            let snapshot = store_guard.get_values();
            println!(
                "Added: {}, Current queue (size={}): [{}]",
                value,
                snapshot.len(),
                snapshot
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        });
    }
}
