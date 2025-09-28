pub mod logger_task;
pub mod counter_task;
pub mod celltower_task;

pub use logger_task::LoggerTask;
pub use counter_task::CounterTask;
pub use celltower_task::CellTowerTask;

use async_trait::async_trait;

#[async_trait]
pub trait ScheduledTask: Send + Sync {
    async fn run(&self);
    fn interval(&self) -> std::time::Duration;
}