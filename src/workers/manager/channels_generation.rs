// HERE MAKE THE CHANNELS GENERATION BETWEEN ALL THE WORKERS TYPES CURRENTLY RUNNING
// AND DON'T FORGET ABOUT IF A WORKER IS REGENERATED, GENERATED NEW CHANNELS WHILE STILL RUNNING THE DNS

use std::collections::HashMap;
use crate::workers::{SCloudWorker, WorkerType};

pub(crate) fn generate_channels(workers: &[SCloudWorker]) -> HashMap<&str, Vec<&SCloudWorker>> {
    let mut wl: HashMap<&str, Vec<&SCloudWorker>> = HashMap::new();
    for w in workers {
        let key = match w.get_worker_type() {
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
        wl.entry(key).or_insert_with(Vec::new).push(w);
    }
    wl
}