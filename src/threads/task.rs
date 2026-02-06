use std::net::SocketAddr;
use crate::threads::WorkerType;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use bytes::Bytes;
use uuid::Uuid;
use tokio::sync::OwnedSemaphorePermit;

#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SCloudWorkerTask {
    pub task_id: Uuid,
    pub for_type: WorkerType,
    pub for_who: SocketAddr,
    pub payload: Bytes,
    pub attempts: u8,
    pub max_attempts: u8,
    pub created_at: SystemTime,
    pub deadline_timeout: Option<SystemTime>,
    pub priority: u8,                   // if supported by the broker
    pub reply_to: Option<String>,       // response endpoint
    pub correlation_id: Option<String>, // id request/response
}

pub struct InFlightTask {
    pub task: SCloudWorkerTask,
    pub _permit: OwnedSemaphorePermit,
}

