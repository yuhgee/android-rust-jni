use std::sync::Arc;
use tokio::runtime::Runtime;
use crate::service::scheduler::Scheduler;
use crate::service::tasks::logger_task::LoggerTask;
use std::time::Duration;

pub struct DataTableScheduler {
    scheduler: Scheduler,
}

impl DataTableScheduler {
    /// 新規作成
    pub fn new() -> Self {
        let mut scheduler = Scheduler::new();
        let logger_task = Arc::new(LoggerTask::new(Duration::from_millis(1000), 5));
        scheduler.add_task(logger_task);
        // scheduler.add_task(logger_task as Arc<dyn ScheduledTask>);
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
        // self.scheduler.initialize_tasks();
    }

    /// 後片付け処理
    pub fn finalize(&mut self) {
        // self.scheduler.finalize_tasks();
    }
}
