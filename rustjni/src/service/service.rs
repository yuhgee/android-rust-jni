use std::sync::Arc;
use tokio::runtime::Runtime;
use crate::service::{DataListScheduler, DataTableScheduler};

pub struct Service {
    pub data_list_scheduler: DataListScheduler,
    pub data_table_scheduler: DataTableScheduler,
    runtime: Option<Runtime>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            data_list_scheduler: DataListScheduler::new(),
            data_table_scheduler: DataTableScheduler::new(),
            runtime: None,
        }
    }

    /// 初期化処理
    pub fn initialize(&mut self) {
    }

    /// サービス起動
    pub fn start(&mut self) {
        if self.runtime.is_some() {
            println!("Service already running");
            return;
        }

        let rt = Runtime::new().expect("Failed to create Tokio runtime");

        self.data_list_scheduler.start(&rt);
        self.data_table_scheduler.start(&rt);

        self.runtime = Some(rt);
        println!("Service started");
    }

    /// サービス停止
    pub fn stop(&mut self) {
        self.data_list_scheduler.stop();
        self.data_table_scheduler.stop();
        self.runtime.take();
        println!("Service stopped");
    }

    /// 後片付け
    pub fn finalize(&mut self) {
    }
}
