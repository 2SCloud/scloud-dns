use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{mpsc};
use bytes::{Buf, Bytes};
use tokio::net::UdpSocket;
use crate::exceptions::SCloudException;
use crate::workers::{SCloudWorker, WorkerState, WorkerType};
use crate::workers::task::{InFlightTask, SCloudWorkerTask};
use crate::utils;

pub async fn run_dns_listener(
    worker: Arc<SCloudWorker>,
    bind_addr: &str,
    tx: mpsc::Sender<InFlightTask>,
) -> Result<(), SCloudException> {
    let socket = UdpSocket::bind(bind_addr).await.map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;
    let mut buf = [0u8; 65_535];
    worker.set_state(WorkerState::IDLE);

    loop {
        let (len, src) = socket.recv_from(&mut buf).await.map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_RECV_FAILED)?;

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

        let in_flight = InFlightTask {task, _permit: permit };

        if tx.send(in_flight).await.is_err() {
            return Ok(());
        }
    }
}
