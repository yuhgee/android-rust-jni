// mod data_store;
mod jni;
mod data;
mod service;
use tokio::runtime::Runtime;

use service::{LoggerTask, CounterTask, Scheduler, CellTowerTask};
use std::thread;
use std::time::Duration;
use log::LevelFilter;
use std::sync::Arc;

fn log_init() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_secs();
    builder.filter_level(LevelFilter::Debug);
    builder.init();
}


fn main() {
    log_init();

    let mut scheduler = Scheduler::new();

    let logger_task = Arc::new(LoggerTask::new(Duration::from_millis(3000), 5));
    let counter_task = Arc::new(CounterTask::new(Duration::from_millis(2000), 10));
    let cell_tower_task = Arc::new(CellTowerTask::new(Duration::from_millis(1000), 5));


    scheduler.add_task(logger_task);
    scheduler.add_task(counter_task);
    scheduler.add_task(cell_tower_task);

    let rt = Runtime::new().unwrap();
    scheduler.start(&rt);

    thread::sleep(Duration::from_secs(12));
    scheduler.stop();
}
