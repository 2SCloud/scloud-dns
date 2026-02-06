use crate::config::Config;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::exceptions::SCloudException;
use crate::threads::{SCloudWorker, SpawnConfig, WorkerType};
use crate::threads::workers::RawDnsMsg;

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

    let (tx, rx) = mpsc::channel::<RawDnsMsg>(1024);

    let listener = Arc::new(SCloudWorker::new(1, WorkerType::LISTENER)?);
    let decoder  = Arc::new(SCloudWorker::new(2, WorkerType::DECODER)?);

    listener.set_dns_tx(tx).await;
    decoder.set_dns_rx(rx).await;

    let _jh_listener = threads::workers::spawn_worker(
        listener.clone(),
        SpawnConfig { name: Some(listener.os_thread_name.as_str()), stack_size: None },
        handle.clone(),
    )?;

    let _jh_decoder = threads::workers::spawn_worker(
        decoder.clone(),
        SpawnConfig { name: Some(decoder.os_thread_name.as_str()), stack_size: None },
        handle,
    )?;

    futures_util::future::pending::<()>().await;
    Ok(())
}