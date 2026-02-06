use std::sync::Arc;
use tokio::sync::{mpsc};
use bytes::Bytes;
use tokio::net::UdpSocket;
use crate::exceptions::SCloudException;
use crate::threads::SCloudWorker;
use crate::threads::workers::RawDnsMsg;

pub async fn run_dns_listener(
    worker: Arc<SCloudWorker>,
    bind_addr: &str,
    tx: mpsc::Sender<RawDnsMsg>,
) -> Result<(), SCloudException> {
    let socket = UdpSocket::bind(bind_addr).await.map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED)?;
    let mut buf = [0u8; 65_535];

    loop {
        let (len, src) = socket.recv_from(&mut buf).await.map_err(|_| SCloudException::SCLOUD_WORKER_LISTENER_RECV_FAILED)?;

        let permit = match worker.in_flight_sem.clone().try_acquire_owned() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let msg = RawDnsMsg {
            src,
            data: Bytes::copy_from_slice(&buf[..len]),
        };

        let tx2 = tx.clone();
        tokio::spawn(async move {
            let _permit = permit;
            let _ = tx2.send(msg).await;
        });
    }
}
