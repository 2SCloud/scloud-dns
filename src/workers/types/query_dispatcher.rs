use crate::exceptions::SCloudException;
use crate::workers::SCloudWorker;
use crate::workers::task::InFlightTask;
use crate::{log_debug, log_trace};
use bytes::Buf;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn run_dns_query_dispatcher(
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
