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

        let in_flight = InFlightTask { task, _permit: permit };
        forward_task(in_flight, &tx).await;
    }
}

#[cfg(target_os = "windows")]
pub async fn run_dns_listener_with_shared_socket(
    worker: Arc<SCloudWorker>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    use std::net::SocketAddr;

    let udp = SHARED_UDP_SOCKET
        .get()
        .ok_or(SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?
        .clone();

    let mut buf = [0u8; 65_535];
    worker.set_state(WorkerState::IDLE);

    loop {
        let (len, src) = udp
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

        let in_flight = InFlightTask { task, _permit: permit };
        forward_task(in_flight, &tx).await;
    }
}

async fn forward_task(
    task: InFlightTask,
    tx: &[mpsc::Sender<InFlightTask>],
) {
    let mut current = Some(task);
    for tx_channel in tx.iter() {
        match tx_channel.try_send(current.take().unwrap()) {
            Ok(_) => return,
            Err(mpsc::error::TrySendError::Full(returned)) => {
                current = Some(returned);
            }
            Err(mpsc::error::TrySendError::Closed(_)) => return,
        }
    }
    if let Some(unsent) = current {
        if let Some(tx_channel) = tx.first() {
            let _ = tx_channel.send(unsent).await;
        }
    }
}

#[cfg(target_os = "windows")]
pub static SHARED_UDP_SOCKET: std::sync::OnceLock<Arc<UdpSocket>> = std::sync::OnceLock::new();