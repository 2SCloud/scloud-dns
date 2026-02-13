use tokio::sync::mpsc;
use tokio::time::{Duration, Instant};
use crate::utils::logging::{build_otlp_payload, OtelLog, LOG_SENDER};

const CHAN_SIZE: usize = 200_000;
const MAX_BATCH: usize = 512;
const FLUSH_EVERY: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn start_otlp_logger() {
    let (tx, mut rx) = mpsc::channel::<OtelLog>(CHAN_SIZE);

    if LOG_SENDER.set(tx).is_err() {
        eprintln!("OTLP logger already started (LOG_SENDER already set)");
        return;
    }

    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .expect("reqwest client");

    let url = "http://alloy.scloud-observability.svc:4318/v1/logs";

    let mut buf: Vec<OtelLog> = Vec::with_capacity(MAX_BATCH);
    let mut ticker = tokio::time::interval(FLUSH_EVERY);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    let mut last_flush = Instant::now();

    loop {
        tokio::select! {
            Some(log) = rx.recv() => {
                buf.push(log);

                if buf.len() >= MAX_BATCH {
                    flush_with_retry(&client, url, &mut buf).await;
                    last_flush = Instant::now();
                }
            }

            _ = ticker.tick() => {
                if !buf.is_empty() && last_flush.elapsed() >= FLUSH_EVERY {
                    flush_with_retry(&client, url, &mut buf).await;
                    last_flush = Instant::now();
                }
            }
        }
    }
}

async fn flush_with_retry(client: &reqwest::Client, url: &str, buf: &mut Vec<OtelLog>) {
    let payload = build_otlp_payload(buf.as_slice());

    let mut backoff = Duration::from_millis(200);
    let max_backoff = Duration::from_secs(5);
    let mut attempts = 0u32;

    loop {
        attempts += 1;

        let res = client.post(url).json(&payload).send().await;

        match res {
            Ok(r) if r.status().is_success() => {
                buf.clear();
                return;
            }

            Ok(r) => {
                let status = r.status();
                let body = r.text().await.unwrap_or_default();

                if status.as_u16() == 429 || status.is_server_error() {
                    eprintln!("OTLP flush got HTTP {status} ({} logs). retrying in {:?}. Body: {}",
                              buf.len(), backoff, truncate(&body, 200));

                    if attempts >= 6 {
                        eprintln!("OTLP flush giving up after {attempts} attempts, dropping {} logs", buf.len());
                        buf.clear();
                        return;
                    }

                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(max_backoff);
                    continue;
                }

                eprintln!("OTLP flush failed HTTP {status} ({} logs). Body: {}",
                          buf.len(), truncate(&body, 200));
                buf.clear();
                return;
            }

            Err(e) => {
                eprintln!("OTLP flush error ({} logs): {e}. retrying in {:?}", buf.len(), backoff);

                if attempts >= 6 {
                    eprintln!("OTLP flush giving up after {attempts} attempts, dropping {} logs", buf.len());
                    buf.clear();
                    return;
                }

                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(max_backoff);
            }
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { return s.to_string(); }
    format!("{}…", &s[..max])
}
