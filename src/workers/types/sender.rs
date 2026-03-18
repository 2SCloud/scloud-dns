use crate::exceptions::SCloudException;
use crate::workers::SCloudWorker;
use crate::workers::task::InFlightTask;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn run_dns_sender(
    worker: Arc<SCloudWorker>,
    mut rx: Vec<mpsc::Receiver<InFlightTask>>,
) -> Result<(), SCloudException> {
    loop {
        for rx_channel in rx.iter_mut() {
            while let Some(msg) = rx_channel.recv().await {
                // need to send back to the user in task::SCloudWorkerTask.for_who
            }
        }
    }
}
