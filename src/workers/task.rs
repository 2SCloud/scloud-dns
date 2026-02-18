use crate::workers::WorkerType;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::SystemTime;
use tokio::sync::OwnedSemaphorePermit;
use uuid::Uuid;

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
