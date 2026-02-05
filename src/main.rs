use crate::config::Config;
use std::path::Path;
use std::sync::Arc;
use crate::threads::{SCloudWorker, SpawnConfig, WorkerType};

mod config;
mod dns;
mod exceptions;
mod threads;
mod utils;

#[tokio::main]
async fn main() {
    let config = Config::from_file(Path::new("./config/config.json")).unwrap();
    utils::logging::init(config.logging.clone()).unwrap();

    let handle = tokio::runtime::Handle::current();

    let worker = Arc::new(SCloudWorker::new(999999, WorkerType::LISTENER));

    let _jh = threads::workers::spawn_worker(
        worker.clone(),
        SpawnConfig { name: Some("listener-0"), stack_size: None },
        handle,
    ).unwrap();

    futures_util::future::pending::<()>().await;
}