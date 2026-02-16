use crate::config::Config;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::exceptions::SCloudException;
use crate::workers::manager::StartGate;
use crate::workers::{SCloudWorker, WorkerType};

mod config;
mod dns;
mod exceptions;
mod workers;
mod utils;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), SCloudException> {
    let config = Config::from_file(Path::new("./config/config.json"))?;
    let gate = Arc::new(StartGate::new(1));
    utils::logging::init(config.logging.clone())?;

    let (tx_l2d, rx_l2d) = mpsc::channel(1024);
    let (tx_d2qd, rx_d2qd) = mpsc::channel(1024);
    let (tx_qd2r, rx_qd2r) = mpsc::channel(1024);
    let (tx_r2s, rx_r2s) = mpsc::channel(1024);

    let mut workers: Vec<Arc<SCloudWorker>> = vec![
        Arc::new(SCloudWorker::new(WorkerType::LISTENER)?),
        Arc::new(SCloudWorker::new(WorkerType::DECODER)?),
        Arc::new(SCloudWorker::new(WorkerType::QUERY_DISPATCHER)?),
        Arc::new(SCloudWorker::new(WorkerType::RESOLVER)?),
        Arc::new(SCloudWorker::new(WorkerType::METRICS)?)
    ];

    // TODO: should automatically detect worker's type and create channel between them.
    workers[0].set_dns_tx(tx_l2d).await;
    workers[1].set_dns_rx(rx_l2d).await;
    workers[1].set_dns_tx(tx_d2qd).await;
    workers[2].set_dns_rx(rx_d2qd).await;
    workers[2].set_dns_tx(tx_qd2r).await;
    workers[3].set_dns_rx(rx_qd2r).await;
    workers[3].set_dns_tx(tx_r2s).await;

    workers.sort_by_key(|w| w.get_worker_id());

    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    for w in workers {
        handles.push(workers::spawn_worker(w, gate.clone()));
    }

    futures_util::future::pending::<()>().await;
    Ok(())
}
