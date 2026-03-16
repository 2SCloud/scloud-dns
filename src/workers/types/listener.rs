use std::sync::Arc;
use std::time::SystemTime;

use bytes::Bytes;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use crate::exceptions::SCloudException;
use crate::utils;
use crate::workers::task::{InFlightTask, SCloudWorkerTask};
use crate::workers::{SCloudWorker, WorkerState, WorkerType};

pub async fn run_dns_listener_with_socket(
    worker: Arc<SCloudWorker>,
    socket: UdpSocket,
    rx: Vec<mpsc::Receiver<InFlightTask>>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    let mut buf = [0u8; 65_535];
    worker.set_state(WorkerState::IDLE);

    loop {
        let (len, src) = socket
            .recv_from(&mut buf)
            .await
            .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_RECV_FAILED)?;

        let permit = match worker.in_flight_sem.clone().try_acquire_owned() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let task = SCloudWorkerTask {
            task_id: utils::uuid::generate_uuid(),
            for_type: WorkerType::LISTENER,
            for_who: src,
            payload: Bytes::copy_from_slice(&buf[..len]),
            attempts: 0,
            max_attempts: 0,
            created_at: SystemTime::now(),
            deadline_timeout: None,
            priority: 0,
            reply_to: None,
            correlation_id: None,
        };

        let in_flight = InFlightTask {
            task,
            _permit: permit,
        };

        let mut current = Some(in_flight);

        for tx_channel in tx.iter() {
            match tx_channel.try_send(current.take().unwrap()) {
                Ok(_) => break,
                Err(mpsc::error::TrySendError::Full(returned)) => {
                    current = Some(returned);
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    return Ok(());
                }
            }
        }

        if let Some(unsent) = current {
            if tx[0].send(unsent).await.is_err() {
                return Ok(());
            }
        }
    }
}

pub async fn run_dns_listener(
    worker: Arc<SCloudWorker>,
    bind_addr: &str,
    rx: Vec<mpsc::Receiver<InFlightTask>>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    let socket = UdpSocket::bind(bind_addr)
        .await
        .map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;

    run_dns_listener_with_socket(worker, socket, rx, tx).await
}
