use std::sync::atomic::Ordering;
use tokio::net::UdpSocket;
use crate::{log_debug, log_trace};

pub async fn run_dns_listener(
    worker: std::sync::Arc<crate::threads::SCloudWorker>,
    bind_addr: &str,
) -> std::io::Result<()> {
    let socket = UdpSocket::bind(bind_addr).await?;
    log_debug!("DNS UDP listener bound on {}", bind_addr);

    let mut buf = [0u8; 65_535];

    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        let packet = buf[..len].to_vec(); // OK for now but after we need to change it

        let max = worker.max_in_flight.load(Ordering::Relaxed);
        let cur = worker.in_flight.load(Ordering::Relaxed);

        if cur >= max {
            // saturated -> drop or rate-limited log
            continue;
        }

        worker.in_flight.fetch_add(1, Ordering::Relaxed);

        let worker2 = worker.clone();
        tokio::spawn(async move {
            log_debug!("packet from {}: {} bytes", src, packet.len());
            log_trace!("bytes: {:?}", packet);

            // ... do stuff ...

            worker2.in_flight.fetch_sub(1, Ordering::Relaxed);
        });
    }
}

