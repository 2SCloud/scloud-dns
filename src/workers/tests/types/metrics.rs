#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::sync::{mpsc, oneshot};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use crate::utils::logging::OtelLog;
    use crate::workers;


    fn mk_log(i: usize) -> OtelLog {
        OtelLog {
            target: format!("test.target.{i}"),
            severity: "INFO",
            message: format!("hello {i}"),
            timestamp_unix_nano: format!("{}", 1_700_000_000_000_000_000u128 + i as u128),
        }
    }

    async fn wait_for_requests(server: &MockServer, n: usize) {
        let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
        loop {
            let reqs = server.received_requests().await.unwrap();
            if reqs.len() >= n {
                return;
            }
            if tokio::time::Instant::now() >= deadline {
                panic!("timeout waiting for {n} request(s), got {}", reqs.len());
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    #[tokio::test]
    async fn flushes_on_max_batch_and_payload_contains_logs() {
        use wiremock::{Mock, MockServer, ResponseTemplate};
        use wiremock::matchers::{method, path};

        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/logs"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let (tx, rx) = mpsc::channel::<OtelLog>(1000);
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap();

        let url = format!("{}/v1/logs", server.uri());

        let max_batch = 5usize;
        let flush_every = Duration::from_secs(60);

        let handle = tokio::spawn(async move {
            workers::types::metrics::run_otlp_logger(rx, shutdown_rx, client, &url, max_batch, flush_every).await;
        });

        for i in 0..max_batch {
            tx.send(mk_log(i)).await.unwrap();
        }

        wait_for_requests(&server, 1).await;

        let _ = shutdown_tx.send(());
        let _ = handle.await;

        server.verify().await;

        let reqs = server.received_requests().await.unwrap();
        assert_eq!(reqs.len(), 1);

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        let records = &body["resourceLogs"][0]["scopeLogs"][0]["logRecords"];
        let arr = records.as_array().expect("logRecords must be an array");
        assert_eq!(arr.len(), max_batch);

        let msgs: Vec<String> = arr
            .iter()
            .map(|r| r["body"]["stringValue"].as_str().unwrap().to_string())
            .collect();

        assert!(msgs.contains(&"hello 0".to_string()));
        assert!(msgs.contains(&"hello 4".to_string()));
    }


    #[tokio::test]
    async fn flushes_on_timer_tick_when_buffer_non_empty() {
        tokio::time::pause();

        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/logs"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let (tx, rx) = mpsc::channel::<OtelLog>(1000);
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let client = reqwest::Client::builder().timeout(Duration::from_secs(2)).build().unwrap();
        let url = format!("{}/v1/logs", server.uri());

        let max_batch = 999usize;
        let flush_every = Duration::from_secs(10);

        let handle = tokio::spawn(async move {
            workers::types::metrics::run_otlp_logger(rx, shutdown_rx, client, &url, max_batch, flush_every).await;
        });

        tx.send(mk_log(1)).await.unwrap();

        tokio::time::advance(Duration::from_secs(11)).await;
        tokio::task::yield_now().await;

        let _ = shutdown_tx.send(());
        let _ = handle.await;

        server.verify().await;
    }

    #[tokio::test]
    async fn does_not_flush_when_buffer_empty() {
        tokio::time::pause();

        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/logs"))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&server)
            .await;

        let (_tx, rx) = mpsc::channel::<OtelLog>(1000);
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let client = reqwest::Client::builder().timeout(Duration::from_secs(2)).build().unwrap();
        let url = format!("{}/v1/logs", server.uri());

        let handle = tokio::spawn(async move {
            workers::types::metrics::run_otlp_logger(rx, shutdown_rx, client, &url, 10, Duration::from_secs(10)).await;
        });

        tokio::time::advance(Duration::from_secs(30)).await;
        tokio::task::yield_now().await;

        let _ = shutdown_tx.send(());
        let _ = handle.await;

        server.verify().await;
    }
}
