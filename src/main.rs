use crate::config::Config;
use crate::exceptions::SCloudException;
use crate::workers::manager::StartGate;
use crate::workers::{SCloudWorker, WorkerType};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;

mod config;
mod dns;
mod exceptions;
mod utils;
mod workers;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), SCloudException> {
    let config = Config::from_file(Path::new("./config/config.json"))?;
    let gate = Arc::new(StartGate::new(1));
    utils::logging::init(config.logging.clone())?;

    let mut workers: Vec<Arc<SCloudWorker>> = vec![
        Arc::new(SCloudWorker::new(WorkerType::LISTENER)?),
        Arc::new(SCloudWorker::new(WorkerType::LISTENER)?),
        Arc::new(SCloudWorker::new(WorkerType::LISTENER)?),
        Arc::new(SCloudWorker::new(WorkerType::DECODER)?),
        Arc::new(SCloudWorker::new(WorkerType::DECODER)?),
        Arc::new(SCloudWorker::new(WorkerType::DECODER)?),
        Arc::new(SCloudWorker::new(WorkerType::DECODER)?),
    ];

    workers::manager::channels_generation::generate_channels(workers.clone()).await;

    workers.sort_by_key(|w| w.get_worker_id());

    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    for w in workers {
        handles.push(workers::spawn_worker(w, gate.clone()));
    }

    futures_util::future::pending::<()>().await;
    Ok(())
}
