use std::sync::atomic::{AtomicU64, AtomicU8};
use std::time::SystemTime;
use uuid::Uuid;
use crate::threads::WorkerType;

#[allow(non_camel_case_types)]
pub(crate) struct ScloudWorkerTask {
    pub task_id: Uuid,
    pub for_type: WorkerType,
    pub for_who: u64,
    pub payload: Vec<u8>,
    pub attempts: AtomicU8,
    pub max_attempts: u8,
    pub created_at: SystemTime,
    pub deadline_timeout: Option<SystemTime>,
    pub priority: u8, // if supported by the broker
    pub reply_to: Option<String>, // response endpoint
    pub correlation_id: Option<String>, // id request/response
}
