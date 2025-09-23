mod data_store;
mod jni;

use data_store::service::Service;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut service = Service::new();

    service.start();

    for i in 1..=60 {
        service.add_data(i);
        sleep(Duration::from_secs(1)).await;
    }

    service.stop();
}
