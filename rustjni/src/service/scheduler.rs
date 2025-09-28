use crate::service::tasks::ScheduledTask;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

async fn run_task(task: Arc<dyn ScheduledTask>, mut stop_rx: oneshot::Receiver<()>) {
    let mut ticker = tokio::time::interval(task.interval());

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                task.run().await;
            }
            _ = &mut stop_rx => {
                println!("Stop signal received, exiting task");
                break;
            }
        }
    }
}

pub struct Scheduler {
    runtime: Option<Runtime>,
    stop_txs: Vec<oneshot::Sender<()>>,
    // tasks: Vec<Arc<dyn ScheduledTask>>,
    tasks: Vec<Arc<dyn ScheduledTask + Send + Sync>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            runtime: None,
            stop_txs: Vec::new(),
            tasks: Vec::new(),
        }
    }

    // pub fn add_task<T: ScheduledTask + 'static>(&mut self, task: T) {
    //     self.tasks.push(Arc::new(task));
    // }
    pub fn add_task(&mut self, task: Arc<dyn ScheduledTask + Send + Sync>) {
        self.tasks.push(task);
    }

    pub fn start(&mut self, rt: &tokio::runtime::Runtime) {
        for task in self.tasks.drain(..) {
            let (tx, rx) = oneshot::channel();
            self.stop_txs.push(tx);

            let task_clone = Arc::clone(&task);
            rt.spawn(async move {
                run_task(task_clone, rx).await;
            });
        }
    }

    pub fn stop(&mut self) {
        for tx in self.stop_txs.drain(..) {
            let _ = tx.send(());
        }
        self.runtime.take();
        println!("Service stopped");
    }
}
