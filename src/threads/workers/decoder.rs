use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use crate::{log_debug, log_trace};
use crate::dns::packet::DNSPacket;
use crate::exceptions::SCloudException;
use crate::threads::SCloudWorker;
use crate::threads::workers::RawDnsMsg;

pub async fn run_dns_decoder(
    worker: Arc<SCloudWorker>,
    mut rx: mpsc::Receiver<RawDnsMsg>,
) -> Result<(), SCloudException> {
    while let Some(msg) = rx.recv().await {
        log_debug!("decoder got {} bytes from {}", msg.data.len(), msg.src);
    }
    Ok(())
}


