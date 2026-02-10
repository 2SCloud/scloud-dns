use tokio::sync::mpsc;
use tokio::time::{Duration, Instant};
use crate::utils::logging::{LOG_SENDER, OtelLog};
use crate::utils::logging::build_otlp_payload;

pub async fn start_otlp_logger() {
    let (tx, mut rx) = mpsc::channel::<OtelLog>(10_000);

    if LOG_SENDER.set(tx).is_err() {
        eprintln!("OTLP logger already started (LOG_SENDER already set)");
        return;
    }

    let client = reqwest::Client::new();
    let url = "http://127.0.0.1:14318/v1/logs";

    let mut buf: Vec<OtelLog> = Vec::with_capacity(512);
    let mut last_flush = Instant::now();

    loop {
        tokio::select! {
            Some(log) = rx.recv() => {
                buf.push(log);
                if buf.len() >= 512 {
                    flush(&client, url, &mut buf).await;
                    last_flush = Instant::now();
                }
            }

            _ = tokio::time::sleep(Duration::from_millis(200)) => {
                if !buf.is_empty() && last_flush.elapsed() >= Duration::from_millis(200) {
                    flush(&client, url, &mut buf).await;
                    last_flush = Instant::now();
                }
            }
        }
    }
}

async fn flush(client: &reqwest::Client, url: &str, buf: &mut Vec<OtelLog>) {
    let payload = build_otlp_payload(buf.as_ref());

    let res = client.post(url).json(&payload).send().await;

    if let Err(e) = res {
        eprintln!("OTLP flush failed ({} logs): {e}", buf.len());
    }

    buf.clear();
}
