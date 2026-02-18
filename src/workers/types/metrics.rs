use crate::log_error;
use crate::utils::logging::{LOG_SENDER, OtelLog, build_otlp_payload};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{Duration, Instant};

const CHAN_SIZE: usize = 200_000;
const MAX_BATCH: usize = 512;
const FLUSH_EVERY: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

async fn flush_with_retry(client: &reqwest::Client, url: &str, buf: &mut Vec<OtelLog>) {
    if buf.is_empty() {
        return;
    }

    let payload = build_otlp_payload(buf);

    let mut attempts = 0;

    loop {
        attempts += 1;

        let res = client
            .post(url)
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                buf.clear();
                return;
            }
            Ok(resp) => {
                log_error!("OTLP flush failed with status {}", resp.status());
            }
            Err(e) => {
                log_error!("OTLP flush error: {}", e);
            }
        }

        if attempts >= 3 {
            log_error!("OTLP flush aborted after 3 attempts");
            return;
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

pub async fn start_otlp_logger() {
    let (tx, rx) = mpsc::channel::<OtelLog>(CHAN_SIZE);

    if LOG_SENDER.set(tx).is_err() {
        log_error!("OTLP logger already started (LOG_SENDER already set)");
        return;
    }

    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .expect("reqwest client");

    // let url = "http://alloy.scloud-observability.svc:4318/v1/logs";
    let url = "http://localhost:4318/v1/logs";

    let (_shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    run_otlp_logger(rx, shutdown_rx, client, url, MAX_BATCH, FLUSH_EVERY).await;
}

pub(crate) async fn run_otlp_logger(
    mut rx: mpsc::Receiver<OtelLog>,
    mut shutdown: oneshot::Receiver<()>,
    client: reqwest::Client,
    url: &str,
    max_batch: usize,
    flush_every: Duration,
) {
    let mut buf: Vec<OtelLog> = Vec::with_capacity(max_batch);
    let mut ticker = tokio::time::interval(flush_every);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    let mut last_flush = Instant::now();

    loop {
        tokio::select! {
            _ = &mut shutdown => {
                if !buf.is_empty() {
                    flush_with_retry(&client, url, &mut buf).await;
                }
                break;
            }

            Some(log) = rx.recv() => {
                buf.push(log);

                if buf.len() >= max_batch {
                    flush_with_retry(&client, url, &mut buf).await;
                    last_flush = Instant::now();
                }
            }

            _ = ticker.tick() => {
                if !buf.is_empty() && last_flush.elapsed() >= flush_every {
                    flush_with_retry(&client, url, &mut buf).await;
                    last_flush = Instant::now();
                }
            }
        }
    }
}
