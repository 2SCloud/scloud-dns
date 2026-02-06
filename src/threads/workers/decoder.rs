use std::sync::Arc;
use bytes::Buf;
use tokio::sync::mpsc;
use crate::{log_debug, log_trace};
use crate::exceptions::SCloudException;
use crate::threads::SCloudWorker;
use crate::threads::task::InFlightTask;

pub async fn run_dns_decoder(
    worker: Arc<SCloudWorker>,
    mut rx: mpsc::Receiver<InFlightTask>,
) -> Result<(), SCloudException> {
    loop {
        while let Some(msg) = rx.recv().await {
            log_debug!("decoder got {} bytes from {}", msg.task.payload.len(), msg.task.for_who);
            log_trace!("bytes: {:?}", msg.task.payload.chunk())
        }
    }
    Ok(())
}
