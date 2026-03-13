use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::workers::{SCloudWorker, WorkerType};

// maybe we don't need this return
pub(crate) async fn generate_channels(workers: Vec<Arc<SCloudWorker>>) /* -> HashMap<&str, Vec<&SCloudWorker>> */ {
    let mut wl: HashMap<&str, Vec<Arc<SCloudWorker>>> = HashMap::new();
    for w in workers {
        let key = match &w.get_worker_type() {
            WorkerType::LISTENER         => "listener",
            WorkerType::DECODER          => "decoder",
            WorkerType::QUERY_DISPATCHER => "query-dispatcher",
            WorkerType::CACHE_LOOKUP     => "cache-lookup",
            WorkerType::ZONE_MANAGER     => "zone-manager",
            WorkerType::RESOLVER         => "resolver",
            WorkerType::CACHE_WRITER     => "cache-writer",
            WorkerType::ENCODER          => "encoder",
            WorkerType::SENDER           => "sender",
            WorkerType::CACHE_JANITOR    => "cache-janitor",
            WorkerType::METRICS          => "metrics",
            WorkerType::TCP_ACCEPTOR     => "tcp-acceptor",
            WorkerType::NONE             => "none",
        };
        wl.entry(key).or_insert_with(Vec::new).push(Arc::clone(&w));
    }

    let listeners = wl.get("listener").unwrap();
    let decoder = wl.get("listener").unwrap();
    let query_dispatcher = wl.get("query-dispatcher").unwrap();

    for l in listeners {
        l.set_dns_rx(mpsc::channel(1024).1).await;
        for d in decoder {
            let (tx, rx) = mpsc::channel(1024);
            l.set_dns_tx(tx).await;
            d.set_dns_rx(rx).await;
        }
    }

    for d in query_dispatcher {
        for qd in query_dispatcher {
            let (tx, rx) = mpsc::channel(1024);
            d.set_dns_tx(tx).await;
            qd.set_dns_rx(rx).await;
        }
    }

}