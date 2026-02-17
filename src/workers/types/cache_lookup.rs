use std::sync::Arc;
use tokio::sync::mpsc;
use crate::exceptions::SCloudException;
use crate::workers::SCloudWorker;
use crate::workers::task::InFlightTask;

pub async fn run_dns_cache_lookup(
    worker: Arc<SCloudWorker>,
    mut rx: mpsc::Receiver<InFlightTask>,
    mut tx: mpsc::Sender<InFlightTask>,
) -> Result<(), SCloudException> {
    loop {
        while let Some(mut msg) = rx.recv().await {
            if tx.send(msg).await.is_err() {
                return Ok(());
            }
        }
    }
}