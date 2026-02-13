use crate::config::Config;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::exceptions::SCloudException;
use crate::workers::{SCloudWorker, WorkerType};

mod config;
mod dns;
mod exceptions;
mod workers;
mod utils;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), SCloudException> {
    let config = Config::from_file(Path::new("./config/config.json"))?;
    utils::logging::init(config.logging.clone())?;

    tokio::spawn(workers::types::metrics::start_otlp_logger());

    let (tx_l2d, rx_l2d) = mpsc::channel(1024);
    let (tx_d2qd, rx_d2qd) = mpsc::channel(1024);
    let (tx_qd2r, rx_qd2r) = mpsc::channel(1024);
    let (tx_r2s, rx_r2s) = mpsc::channel(1024);

    let listener = Arc::new(SCloudWorker::new(1, WorkerType::LISTENER)?);
    let decoder  = Arc::new(SCloudWorker::new(2, WorkerType::DECODER)?);
    let qd       = Arc::new(SCloudWorker::new(3, WorkerType::QUERY_DISPATCHER)?);
    let resolver = Arc::new(SCloudWorker::new(4, WorkerType::RESOLVER)?);

    listener.set_dns_tx(tx_l2d).await;
    decoder.set_dns_rx(rx_l2d).await;
    decoder.set_dns_tx(tx_d2qd).await;
    qd.set_dns_rx(rx_d2qd).await;
    qd.set_dns_tx(tx_qd2r).await;
    resolver.set_dns_rx(rx_qd2r).await;
    resolver.set_dns_tx(tx_r2s).await;

    workers::spawn_worker(listener);
    workers::spawn_worker(decoder);
    workers::spawn_worker(qd);
    workers::spawn_worker(resolver);

    futures_util::future::pending::<()>().await;
    Ok(())
}
