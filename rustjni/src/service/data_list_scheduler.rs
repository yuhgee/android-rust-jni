use std::sync::Arc;
use tokio::runtime::Runtime;
use crate::service::scheduler::Scheduler;
use crate::service::tasks::celltower_task::CellTowerTask;
use std::time::Duration;

pub struct DataListScheduler {
    scheduler: Scheduler,
}

impl DataListScheduler {
    /// 新規作成
    pub fn new() -> Self {
        let mut scheduler = Scheduler::new();
        let cell_task = Arc::new(CellTowerTask::new(Duration::from_millis(1000), 5));
        scheduler.add_task(cell_task);
        Self { scheduler }
    }

    /// Scheduler を開始（Runtime は Service から渡す）
    pub fn start(&mut self, rt: &Runtime) {
        self.scheduler.start(rt);
    }

    /// Scheduler を停止
    pub fn stop(&mut self) {
        self.scheduler.stop();
    }

    /// 初期化処理
    pub fn initialize(&mut self) {
    }

    /// 後片付け処理
    pub fn finalize(&mut self) {
    }
}
