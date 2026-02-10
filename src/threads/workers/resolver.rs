use std::sync::Arc;
use bytes::Buf;
use tokio::sync::mpsc;
use crate::{log_debug, log_trace};
use crate::exceptions::SCloudException;
use crate::threads::SCloudWorker;
use crate::threads::task::InFlightTask;

pub async fn run_dns_resolver(
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
