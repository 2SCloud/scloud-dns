use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;

fn build_dns_query_a(qname: &str, id: u16) -> Vec<u8> {
    let mut msg = Vec::with_capacity(512);

    msg.extend_from_slice(&id.to_be_bytes());
    msg.extend_from_slice(&0x0100u16.to_be_bytes());
    msg.extend_from_slice(&1u16.to_be_bytes());
    msg.extend_from_slice(&0u16.to_be_bytes());
    msg.extend_from_slice(&0u16.to_be_bytes());
    msg.extend_from_slice(&0u16.to_be_bytes());

    for label in qname.trim_end_matches('.').split('.') {
        let len = label.len();
        if len == 0 || len > 63 {
            continue;
        }
        msg.push(len as u8);
        msg.extend_from_slice(label.as_bytes());
    }
    msg.push(0);

    msg.extend_from_slice(&1u16.to_be_bytes());
    msg.extend_from_slice(&1u16.to_be_bytes());

    msg
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // Usage:
    // cargo run --release --bin dns_stress_test -- 127.0.0.1:5353 10 20 0 github.com.
    // 1 worker can handle 19 clients approx. (with my computer performance)
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "Usage: {} <target ip:port> <duration_s> <clients> <sleep_ns> [qname]\n\
             Example: {} 127.0.0.1:5353 10 200 0 example.com.",
            args[0], args[0]
        );
        std::process::exit(2);
    }

    let target: SocketAddr = args[1].parse()?;
    let duration_s: u64 = args[2].parse()?;
    let clients: usize = args[3].parse()?;
    let sleep_ns: u64 = args[4].parse()?;
    let qname = args.get(5).map(|s| s.as_str()).unwrap_or("example.com.");

    let sent = Arc::new(AtomicU64::new(0));
    let send_err = Arc::new(AtomicU64::new(0));

    let start = Instant::now();
    let deadline = start + Duration::from_secs(duration_s);

    let mut handles = Vec::with_capacity(clients);
    for client_idx in 0..clients {
        let sent = sent.clone();
        let send_err = send_err.clone();
        let qname = qname.to_string();

        let h = tokio::spawn(async move {
            let sock = UdpSocket::bind("0.0.0.0:0").await.expect("bind client socket");
            sock.connect(target).await.expect("connect client socket");

            let mut id: u16 = (client_idx as u16).wrapping_mul(7919).wrapping_add(1);

            while Instant::now() < deadline {
                id = id.wrapping_add(1);
                let q = build_dns_query_a(&qname, id);

                match sock.send(&q).await {
                    Ok(_) => {
                        sent.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(_) => {
                        send_err.fetch_add(1, Ordering::Relaxed);
                    }
                }

                if sleep_ns > 0 {
                    tokio::time::sleep(Duration::from_nanos(sleep_ns)).await;
                }
            }
        });

        handles.push(h);
    }

    let mut last = Instant::now();
    let mut last_sent = 0u64;
    let mut last_err = 0u64;

    while Instant::now() < deadline {
        tokio::time::sleep(Duration::from_secs(1)).await;

        let now = Instant::now();
        let dt = (now - last).as_secs_f64().max(1e-9);

        let s = sent.load(Ordering::Relaxed);
        let e = send_err.load(Ordering::Relaxed);

        let ds = s - last_sent;
        let de = e - last_err;

        println!(
            "1s: sent/s={:.0} send_err/s={:.0} (total sent={})",
            ds as f64 / dt,
            de as f64 / dt,
            s
        );

        last = now;
        last_sent = s;
        last_err = e;
    }

    for h in handles {
        let _ = h.await;
    }

    let elapsed = (Instant::now() - start).as_secs_f64().max(1e-9);
    let s = sent.load(Ordering::Relaxed);
    let e = send_err.load(Ordering::Relaxed);

    println!("\n=== RESULT ===");
    println!("target: {target}, qname: {qname}, clients: {clients}, duration: {duration_s}s");
    println!("sent: {s} ({:.0}/s)", s as f64 / elapsed);
    println!("send_err: {e} ({:.0}/s)", e as f64 / elapsed);

    Ok(())
}
