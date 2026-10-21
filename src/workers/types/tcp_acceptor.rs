use super::listener::run_dns_listener_with_socket;
use crate::exceptions::SCloudException;
use crate::workers::SCloudWorker;
use crate::workers::task::InFlightTask;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

#[cfg(target_os = "windows")]
use super::listener::run_dns_listener_with_shared_socket;

pub async fn run_dns_tcp_acceptor(
    worker: Arc<SCloudWorker>,
    tx: Vec<mpsc::Sender<InFlightTask>>,
) -> Result<(), SCloudException> {
    #[cfg(not(target_os = "windows"))]
    {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
            .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;

        socket
            .set_reuse_port(true)
            .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;
        socket
            .set_reuse_address(true)
            .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;
        socket
            .set_nonblocking(true)
            .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;
        socket
            .set_recv_buffer_size(16 * 1024 * 1024)
            .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;
        socket
            .set_send_buffer_size(16 * 1024 * 1024)
            .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;

        let addr: SocketAddr = "0.0.0.0:5353".parse().unwrap();
        socket
            .bind(&addr.into())
            .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_BIND_FAILED)?;

        let std_socket: std::net::UdpSocket = socket.into();
        let udp = UdpSocket::from_std(std_socket)
            .map_err(|_| SCloudException::SCLOUD_WORKER_TCPA_SOCKET_CREATION_FAILED)?;

        run_dns_listener_with_socket(worker, udp, vec![], tx).await
    }

    #[cfg(target_os = "windows")]
    {
        run_dns_listener_with_shared_socket(worker, tx).await
    }
}
