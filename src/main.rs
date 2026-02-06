use crate::config::Config;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::exceptions::SCloudException;
use crate::threads::{SCloudWorker, SpawnConfig, WorkerType};
use crate::threads::task::InFlightTask;

mod config;
mod dns;
mod exceptions;
mod threads;
mod utils;

#[tokio::main]
async fn main() -> Result<(), SCloudException> {
    let config = Config::from_file(Path::new("./config/config.json")).unwrap();
    utils::logging::init(config.logging.clone()).unwrap();

    let handle = tokio::runtime::Handle::current();

    let (tx_l2d, rx_l2d) = mpsc::channel::<InFlightTask>(1024);
    let (tx_d2qd, rx_d2qd) = mpsc::channel::<InFlightTask>(1024);
    let (tx_qd2r, rx_qd2r) = mpsc::channel::<InFlightTask>(1024);

    let listener = Arc::new(SCloudWorker::new(1, WorkerType::LISTENER)?);
    let decoder  = Arc::new(SCloudWorker::new(2, WorkerType::DECODER)?);
    let query_dispatcher  = Arc::new(SCloudWorker::new(3, WorkerType::QUERY_DISPATCHER)?);

    let resolver  = Arc::new(SCloudWorker::new(3, WorkerType::RESOLVER)?);

    listener.set_dns_tx(tx_l2d.clone()).await;
    decoder.set_dns_rx(rx_l2d).await;
    decoder.set_dns_tx(tx_d2qd.clone()).await;
    query_dispatcher.set_dns_rx(rx_d2qd).await;
    query_dispatcher.set_dns_tx(tx_qd2r.clone()).await;

    let _jh_listener = threads::workers::spawn_worker(
        listener.clone(),
        SpawnConfig { name: Some(listener.os_thread_name.as_str()), stack_size: None },
        handle.clone(),
    )?;

    let _jh_decoder = threads::workers::spawn_worker(
        decoder.clone(),
        SpawnConfig { name: Some(decoder.os_thread_name.as_str()), stack_size: None },
        handle.clone(),
    )?;

    let _jh_query_dispatcher = threads::workers::spawn_worker(
        query_dispatcher.clone(),
        SpawnConfig { name: Some(query_dispatcher.os_thread_name.as_str()), stack_size: None },
        handle,
    )?;

    futures_util::future::pending::<()>().await;
    Ok(())
}