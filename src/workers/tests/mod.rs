#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::sync::Arc;
    use crate::{exceptions, workers};
    use std::sync::atomic::Ordering;
    use tokio::sync::mpsc;
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

}
