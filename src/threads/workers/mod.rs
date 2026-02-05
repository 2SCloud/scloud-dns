pub(crate) mod listener;

use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::exceptions::SCloudException;
use crate::{log_debug, log_error};
use crate::threads::{SCloudWorker, SpawnConfig, WorkerState};
use tokio::runtime::Handle;

pub fn spawn_worker(
    worker: Arc<SCloudWorker>,
    cfg: SpawnConfig<'static>,
    handle: Handle,
) -> Result<std::thread::JoinHandle<()>, SCloudException> {
    let worker_clone = worker.clone();

    crate::threads::new(cfg, move || {
        let tid = crate::threads::thread::thread_base::current_thread_id();
        worker_clone.os_thread_id.store(tid as u64, Ordering::Relaxed);

        log_debug!(
            "SCloudWorker (ID: {}) linked to ThreadOS (TID: {})",
            worker_clone.worker_id,
            tid
        );

        worker_clone.state.store(WorkerState::BUSY as u8, Ordering::Relaxed);

        let res = handle.block_on(worker_clone.run());

        if let Err(e) = res {
            log_error!("worker failed: {:?}", e);
        }
    })
        .map_err(|_| SCloudException::SCLOUD_WORKER_FAILED_TO_SPAWN)
}
