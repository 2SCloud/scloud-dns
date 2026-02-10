pub(crate) mod listener;
pub(crate) mod decoder;
pub(crate) mod query_dispatcher;
pub(crate) mod resolver;
pub(crate) mod metrics;

use std::sync::Arc;
use crate::{log_error, log_info};
use crate::threads::SCloudWorker;

pub fn spawn_worker(
    worker: Arc<SCloudWorker>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        log_info!(
            "Worker {} ({:?}) started",
            worker.worker_id,
            worker.worker_type
        );

        if let Err(e) = worker.clone().run().await {
            log_error!(
                "Worker {} failed: {:?}",
                worker.worker_id,
                e
            );
        }
    })
}

