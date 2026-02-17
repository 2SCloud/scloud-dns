use std::sync::Mutex;
use once_cell::sync::Lazy;
use tokio::sync::{Mutex as TMutex, Notify};

static SCLOUD_WORKER_ID_LIST: Lazy<Mutex<Vec<u64>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub(crate) fn generate_worker_id() -> u64 {
    let mut list = SCLOUD_WORKER_ID_LIST.lock().unwrap();
    let result = list.len() as u64 + 1;
    list.push(result);
    result
}



pub(crate) struct StartGate {
    next_id: TMutex<u64>,
    notify: Notify,
}

impl StartGate {
    pub(crate) fn new(first_id: u64) -> Self {
        Self { next_id: TMutex::new(first_id), notify: Notify::new() }
    }

    pub(crate) async fn wait_turn(&self, my_id: u64) {
        loop {
            {
                let next = *self.next_id.lock().await;
                if next == my_id {
                    return;
                }
            }
            self.notify.notified().await;
        }
    }

    pub(crate) async fn done(&self) {
        let mut next = self.next_id.lock().await;
        *next += 1;
        drop(next);
        self.notify.notify_waiters();
    }
}
