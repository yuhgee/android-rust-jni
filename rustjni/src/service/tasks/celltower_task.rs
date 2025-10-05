use crate::service::store::SharedStore;
use crate::service::tasks::ScheduledTask;
use crate::data::cell_tower_info::CellTowerInfo;
use async_trait::async_trait;
use std::time::Duration;
use std::sync::Mutex;
use crate::jni::jni_cell_tower::{fetch_cell_towers_safe, fetch_cell_towers_safe_with_context};
use crate::jni::jni_impl::get_context_ref;

pub struct CellTowerTask {
    store: SharedStore<CellTowerInfo>,
    interval: Duration,
}

impl CellTowerTask {
    pub fn new(interval: Duration, limit: usize) -> Self {
        Self {
            store: SharedStore::new(Mutex::new(crate::service::store::Store::new(limit))),
            interval,
        }
    }

    pub fn get_store(&self) -> SharedStore<CellTowerInfo> {
        std::sync::Arc::clone(&self.store)
    }

    pub fn clear(&self) {
        let mut s = self.store.lock().unwrap();
        s.clear();
    }
}

#[async_trait]
impl ScheduledTask for CellTowerTask {
    async fn run(&self) {
        let s = match self.get_cell_towers_from_java() {
            Ok(s) => s,
            Err(e) => {
                log::error!("CellTowerTask error fetching string: {:?}", e);
                return;
            }
        };

        let new_data = match self.get_cell_towers_info(&s) {
            Ok(data) => data,
            Err(e) => {
                log::error!("CellTowerTask error parsing data: {:?}", e);
                return;
            }
        };

        {
            let mut s = self.store.lock().unwrap();
            s.add(new_data);
        }

        let snapshot = {
            let s = self.store.lock().unwrap();
            s.get_values()
        };
        let size = snapshot.len();
        log::info!("CellTowerTask: Added new data, total size: {}", size);
        log::debug!("CellTowerTask snapshot: {:?}", snapshot);        
    }

    fn interval(&self) -> Duration {
        self.interval
    }

}

impl CellTowerTask {
    fn get_cell_towers_from_java(&self) -> Result<String, Box<dyn std::error::Error>> {
        // let context_guard = get_context_ref();
        // let context_ref = context_guard
        //     .as_ref()
        //     .expect("Context not initialized");
        // let context_obj = context_ref.as_obj();
        // match fetch_cell_towers_safe_with_context(context_obj) {
        //     Ok(json) => {
        //         Ok(json)
        //     },
        //     Err(e) => {
        //         Err(e)
        //     }
        // }

        match fetch_cell_towers_safe() {
            Ok(json) => {
                Ok(json)
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    fn  get_cell_towers_info(&self, json_str: &str) -> Result<CellTowerInfo, Box<dyn std::error::Error>> {
        let cell_towers: Vec<CellTowerInfo> = serde_json::from_str(json_str)?;
        if let Some(first) = cell_towers.first() {
            return Ok(first.clone());
        }
        Err("No cell tower info found".into())
    }
}