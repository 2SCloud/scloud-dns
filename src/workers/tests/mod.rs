#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use crate::workers;
    use crate::workers::{ShutdownMode, WorkerState};
    use std::sync::atomic::Ordering;

    #[tokio::test]
    async fn test_init_scloud_worker() {
        let worker = workers::SCloudWorker::new(0, workers::WorkerType::LISTENER)
            .unwrap();

        assert_eq!(worker.worker_id, 0);
        assert_eq!(worker.worker_type, workers::WorkerType::LISTENER);

        assert_eq!(worker.stack_size_bytes.load(Ordering::Relaxed), 2 * 1024 * 1024);
        assert_eq!(worker.buffer_budget_bytes.load(Ordering::Relaxed), 4 * 1024 * 1024);
        assert_eq!(worker.max_stack_size_bytes.load(Ordering::Relaxed), 32 * 1024 * 1024);
        assert_eq!(worker.max_buffer_budget_bytes.load(Ordering::Relaxed), 256 * 1024 * 1024);

        assert_eq!(worker.state.load(Ordering::Relaxed), WorkerState::IDLE as u8);
        assert_eq!(worker.shutdown_requested.load(Ordering::Relaxed), false);
        assert_eq!(worker.shutdown_mode.load(Ordering::Relaxed), ShutdownMode::GRACEFUL as u8);

        assert_eq!(worker.in_flight.load(Ordering::Relaxed), 0);
        assert_eq!(worker.max_in_flight.load(Ordering::Relaxed), 512);

        assert_eq!(worker.jobs_done.load(Ordering::Relaxed), 0);
        assert_eq!(worker.jobs_failed.load(Ordering::Relaxed), 0);
        assert_eq!(worker.jobs_retried.load(Ordering::Relaxed), 0);

        assert_eq!(worker.max_in_flight.load(Ordering::Relaxed), 512);
    }

}
