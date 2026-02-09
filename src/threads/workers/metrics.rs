use tokio::sync::mpsc;
use crate::utils::logging::LOG_SENDER;

pub async fn start_otlp_logger() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    LOG_SENDER.set(tx).ok();

    let client = reqwest::Client::new();

    while let Some(log) = rx.recv().await {
        let payload = serde_json::json!({
            "resourceLogs": [{
                "resource": {
                    "attributes": [
                        {"key": "service.name", "value": {"stringValue": "scloud-dns"}},
                        {"key": "service.instance.id", "value": {"stringValue": "scloud-dns-01"}}
                    ]
                },
                "scopeLogs": [{
                    "scope": { "name": log.target },
                    "logRecords": [{
                        "timeUnixNano": log.timestamp,
                        "severityText": log.severity,
                        "body": { "stringValue": log.message }
                    }]
                }]
            }]
        });

        let _ = client
            .post("http://127.0.0.1:14318/v1/logs")
            .json(&payload)
            .send()
            .await;
    }
}
