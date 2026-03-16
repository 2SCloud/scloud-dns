use crate::exceptions::SCloudException;
use crate::workers::SCloudWorker;
use crate::workers::task::InFlightTask;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn run_dns_sender(
    worker: Arc<SCloudWorker>,
    mut rx: mpsc::Receiver<InFlightTask>,
) -> Result<(), SCloudException> {
    loop {
        while let Some(mut msg) = rx.recv().await {

        }
    }
}
