use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::exceptions::SCloudException;
use crate::workers::{SCloudWorker, WorkerType};

pub(crate) async fn generate_channels(workers: Vec<Arc<SCloudWorker>>) -> Result<(), SCloudException> {
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

    let default_worker = vec![Arc::new(SCloudWorker::new(WorkerType::NONE)?)];
    let tcp_acceptor = wl.get("tcp-acceptor").unwrap_or(&default_worker);
    let listeners = wl.get("listener").unwrap_or(&default_worker);
    let decoder = wl.get("decoder").unwrap_or(&default_worker);
    let query_dispatcher = wl.get("query-dispatcher").unwrap_or(&default_worker);
    let cache_lookup = wl.get("cache-lookup").unwrap_or(&default_worker);
    let cache_writers = wl.get("cache-writer").unwrap_or(&default_worker);
    let zone_manager = wl.get("zone-manager").unwrap_or(&default_worker);
    let resolvers = wl.get("resolver").unwrap_or(&default_worker);
    let encoders = wl.get("encoder").unwrap_or(&default_worker);
    let senders = wl.get("sender").unwrap_or(&default_worker);

    // GLOBAL PROC
    // TCP_ACCECPTOR (xN) -> LISTENERS (xN)
    for tcpa in tcp_acceptor {
        tcpa.set_dns_rx(mpsc::channel(1024).1).await;
        for l in listeners {
            let (tx, rx) = mpsc::channel(1024);
            tcpa.set_dns_tx(tx).await;
            l.set_dns_rx(rx).await;
        }
    }

    // GLOBAL PROC
    // LISTENERS (xN) -> DECODERS (xN)
    for l in listeners {
        for d in decoder {
            let (tx, rx) = mpsc::channel(1024);
            l.set_dns_tx(tx).await;
            d.set_dns_rx(rx).await;
        }
    }

    // GLOBAL PROC
    // DECODERS (xN) -> CACHE_LOOKUP (xN)
    for d in decoder {
        for cl in cache_lookup {
            let (tx, rx) = mpsc::channel(1024);
            d.set_dns_tx(tx).await;
            cl.set_dns_rx(rx).await;
        }
    }

    // DIRECT HIT PROC
    // CACHE_LOOKUP (xN) -> CACHE_WRITER (xN)
    for cl in cache_lookup {
        for cw in cache_writers {
            let (tx, rx) = mpsc::channel(1024);
            cl.set_dns_tx(tx).await;
            cw.set_dns_rx(rx).await;
        }
    }

    // ZONE/RESOLVE PROC
    // CACHE_LOOKUP (xN) -> QUERY_DISPATCHER (xN)
    for cl in cache_lookup {
        for qd in query_dispatcher {
            let (tx, rx) = mpsc::channel(1024);
            cl.set_dns_tx(tx).await;
            qd.set_dns_rx(rx).await;
        }
    }

    // ZONE/RESOLVE PROC
    // QUERY_DISPATCHER (xN) -> ZONE_MANAGER (xN)
    for qd in query_dispatcher {
        for zm in zone_manager {
            let (tx, rx) = mpsc::channel(1024);
            qd.set_dns_tx(tx).await;
            zm.set_dns_rx(rx).await;
        }
    }

    // ZONE/RESOLVE PROC
    // QUERY_DISPATCHER (xN) -> RESOLVER (xN)
    for qd in query_dispatcher {
        for r in resolvers {
            let (tx, rx) = mpsc::channel(1024);
            qd.set_dns_tx(tx).await;
            r.set_dns_rx(rx).await;
        }
    }

    // ZONE/RESOLVE PROC
    // ZONE_MANAGER (xN) -> CACHE_WRITER (xN)
    for zm in zone_manager {
        for cw in cache_writers {
            let (tx, rx) = mpsc::channel(1024);
            zm.set_dns_tx(tx).await;
            cw.set_dns_rx(rx).await;
        }
    }

    // ZONE/RESOLVE PROC
    // RESOLVER (xN) -> CACHE_WRITER (xN)
    for r in resolvers {
        for cw in cache_writers {
            let (tx, rx) = mpsc::channel(1024);
            r.set_dns_tx(tx).await;
            cw.set_dns_rx(rx).await;
        }
    }

    // GLOBAL PROC
    // CACHE_WRITER (xN) -> ENCODER (xN)
    for cw in cache_writers {
        for e in encoders {
            let (tx, rx) = mpsc::channel(1024);
            cw.set_dns_tx(tx).await;
            e.set_dns_rx(rx).await;
        }
    }

    // GLOBAL PROC
    // ENCODER (xN) -> SENDER (xN)
    for e in encoders {
        for s in senders {
            let (tx, rx) = mpsc::channel(1024);
            e.set_dns_tx(tx).await;
            s.set_dns_rx(rx).await;
        }
    }

    Ok(())
}