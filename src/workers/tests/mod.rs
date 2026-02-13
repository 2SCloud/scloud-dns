#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::sync::Arc;
    use crate::{exceptions, workers};
    use std::sync::atomic::Ordering;
    use tokio::sync::{mpsc, Semaphore};
    use crate::workers::task::InFlightTask;

    #[test]
    fn test_init_scloud_worker() {
        let worker = workers::SCloudWorker::new(0, workers::WorkerType::LISTENER)
            .unwrap();

        assert_eq!(worker.worker_id, 0);
        assert_eq!(worker.worker_type, workers::WorkerType::LISTENER);

        assert_eq!(worker.stack_size_bytes.load(Ordering::Relaxed), 2 * 1024 * 1024);
        assert_eq!(worker.buffer_budget_bytes.load(Ordering::Relaxed), 4 * 1024 * 1024);
        assert_eq!(worker.max_stack_size_bytes.load(Ordering::Relaxed), 32 * 1024 * 1024);
        assert_eq!(worker.max_buffer_budget_bytes.load(Ordering::Relaxed), 256 * 1024 * 1024);

        assert_eq!(worker.state.load(Ordering::Relaxed), workers::WorkerState::INIT as u8);
        assert_eq!(worker.shutdown_requested.load(Ordering::Relaxed), false);
        assert_eq!(worker.shutdown_mode.load(Ordering::Relaxed), workers::ShutdownMode::GRACEFUL as u8);

        assert_eq!(worker.in_flight.load(Ordering::Relaxed), 0);
        assert_eq!(worker.max_in_flight.load(Ordering::Relaxed), 512);

        assert_eq!(worker.jobs_done.load(Ordering::Relaxed), 0);
        assert_eq!(worker.jobs_failed.load(Ordering::Relaxed), 0);
        assert_eq!(worker.jobs_retried.load(Ordering::Relaxed), 0);

        assert_eq!(worker.max_in_flight.load(Ordering::Relaxed), 512);
    }

    #[tokio::test]
    async fn test_run_listener_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(1, workers::WorkerType::LISTENER).unwrap());

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_decoder_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(2, workers::WorkerType::DECODER).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_decoder_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(2, workers::WorkerType::DECODER).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_query_dispatcher_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::QUERY_DISPATCHER).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_query_dispatcher_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::QUERY_DISPATCHER).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_cache_lookup_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::CACHE_LOOKUP).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_cache_lookup_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::CACHE_LOOKUP).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_zone_manager_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::ZONE_MANAGER).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_zone_manager_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::ZONE_MANAGER).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_resolver_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::RESOLVER).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_resolver_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::RESOLVER).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_cache_writer_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::CACHE_WRITER).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_cache_writer_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::CACHE_WRITER).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_encoder_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::ENCODER).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_encoder_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::ENCODER).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_sender_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::SENDER).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_sender_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::SENDER).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_cache_janitor_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::SENDER).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_tcp_acceptor_fails_if_rx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());

        let (tx, _) = mpsc::channel::<InFlightTask>(8);
        *w.dns_tx.lock().await = Some(tx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_RX_NOT_SET));
    }

    #[tokio::test]
    async fn test_run_tcp_acceptor_fails_if_tx_not_set() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());

        let (_, rx) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx);

        let err = w.clone().run().await.unwrap_err();
        assert!(matches!(err, exceptions::SCloudException::SCLOUD_WORKER_TX_NOT_SET));
    }

    #[test]
    fn test_get_worker_id() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(3, w.get_worker_id());
    }

    #[test]
    fn test_get_worker_type() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(workers::WorkerType::TCP_ACCEPTOR, w.get_worker_type());
    }

    #[tokio::test]
    async fn test_get_dns_rx_tx() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());

        let (tx0, rx0) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx0);
        *w.dns_tx.lock().await = Some(tx0);

        let (rx, tx) = w.get_dns_rx_tx().await.expect("should return rx+tx");

        assert!(w.dns_rx.lock().await.is_none());
        assert!(w.dns_tx.lock().await.is_some());
    }

    #[tokio::test]
    async fn test_get_dns_rx() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());

        let (tx0, rx0) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx0);
        *w.dns_tx.lock().await = Some(tx0);

        let rx = w.get_dns_rx().await.expect("should return rx");

        assert!(w.dns_rx.lock().await.is_none());
    }

    #[tokio::test]
    async fn test_get_dns_tx() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());

        let (tx0, rx0) = mpsc::channel::<InFlightTask>(8);
        *w.dns_rx.lock().await = Some(rx0);
        *w.dns_tx.lock().await = Some(tx0);

        let tx = w.get_dns_tx().await.expect("should return tx");

        assert!(w.dns_tx.lock().await.is_some());
    }

    #[test]
    pub fn test_get_stack_size_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_stack_size_bytes(), 2 * 1024 * 1024);
    }

    #[test]
    pub fn test_get_buffer_budget_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_buffer_budget_bytes(), 4 * 1024 * 1024);
    }

    #[test]
    pub fn test_get_max_stack_size_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_max_stack_size_bytes(), 32 * 1024 * 1024);
    }

    #[test]
    pub fn test_get_max_buffer_budget_bytes() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_max_buffer_budget_bytes(), 256 * 1024 * 1024);
    }

    #[test]
    pub fn test_get_state() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_state(), workers::WorkerState::INIT as u8);
    }

    #[test]
    pub fn test_get_shutdown_requested() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_shutdown_requested(), false)
    }

    #[test]
    pub fn test_get_shutdown_mode() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_shutdown_mode(), workers::ShutdownMode::GRACEFUL as u8)
    }

    #[test]
    pub fn test_get_in_flight() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_in_flight(), 0)
    }

    #[test]
    pub fn test_get_in_flight_sem() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_in_flight_sem(), 512)
    }

    #[test]
    pub fn test_get_max_in_flight() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_max_in_flight(), 512)
    }

    #[test]
    pub fn test_get_jobs_done() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_jobs_done(), 0)
    }

    #[test]
    pub fn test_get_jobs_failed() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_jobs_failed(), 0)
    }

    #[test]
    pub fn test_get_jobs_retried() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_jobs_retried(), 0)
    }

    #[test]
    pub fn test_get_last_job_started_ms() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_job_started_ms(), 0)
    }

    #[test]
    pub fn test_get_last_job_finished_ms() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_job_finished_ms(), 0)
    }

    #[test]
    pub fn test_get_last_error_code() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_error_code(), 0)
    }

    #[test]
    pub fn test_get_last_error_at_ms() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_error_at_ms(), 0)
    }

    #[test]
    pub fn test_get_last_task_id_hi() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_task_id_hi(), 0)
    }

    #[test]
    pub fn test_get_last_task_id_lo() {
        let w = Arc::new(workers::SCloudWorker::new(3, workers::WorkerType::TCP_ACCEPTOR).unwrap());
        assert_eq!(w.get_last_task_id_lo(), 0)
    }

}
