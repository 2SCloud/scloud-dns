use std::sync::atomic::{AtomicU8, AtomicU64, AtomicUsize, AtomicBool, Ordering};
use anyhow::Result;
use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable, BasicProperties, Channel, Connection};
use lapin::message::Delivery;
use serde::{Deserialize, Serialize};
use crate::threads::task::ScloudWorkerTask;

pub(crate) mod tests;
pub(crate) mod task;
mod rabbit_mq;

#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
mod others;
mod queue;

#[cfg(windows)]
mod thread {
    pub(crate) use crate::threads::windows::imp as thread_base;
    pub(crate) use crate::threads::windows::priority::imp as priority;
}

#[cfg(target_os = "linux")]
mod thread {
    pub(crate) use crate::threads::linux::imp as thread_base;
    pub(crate) use crate::threads::linux::priority::imp as priority;
}

#[cfg(target_os = "macos")]
mod thread {
    pub(crate) use crate::threads::macos::imp as thread_base;
    pub(crate) use crate::threads::macos::priority::imp as priority;
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
mod thread {
    pub(crate) use crate::threads::others::imp as thread_base;
    pub(crate) use crate::threads::others::priority::imp as priority;
}

#[allow(unused)]
#[allow(non_camel_case_types)]
/// Internal worker descriptor and runtime controls.
///
/// - Mutable runtime knobs are atomics for lock-free internal control updates.
/// - Non-atomic fields are treated as immutable after construction (engine-dev only).
///
/// Notes:
/// - `os_thread_id` is for diagnostics only (`0` = unset/invalid). Do not treat it as a liveness guarantee.
/// - `priority`/`priority_scope` store the *desired* policy; internal code must explicitly apply it.
/// - On macOS, priority is applied via QoS by default. Enabling the optional `mach-rt` feature may
///   apply a true Mach real-time policy for `ThreadPriority::REALTIME` (advanced/potentially disruptive).
/// - `stack_size_bytes` is a spawn-time knob on most platforms; updates typically only affect *future spawns*
///   (e.g., after respawn/restart), not an already-running thread.
///
/// Incoming:
/// - CPU affinity/processor binding
pub(crate) struct ScloudWorker {
    // IDENTITY
    pub(crate) worker_id: u64,
    pub(crate) os_thread_id: AtomicU64,
    pub(crate) worker_type: WorkerType,

    // RESOURCES/LIMITS
    pub(crate) stack_size_bytes: AtomicUsize,
    pub(crate) buffer_budget_bytes: AtomicUsize,
    pub(crate) max_stack_size_bytes: AtomicUsize,
    pub(crate) max_buffer_budget_bytes: AtomicUsize,

    // SCHEDULING/PRIORITY
    pub(crate) priority: AtomicU8,
    pub(crate) priority_scope: AtomicU8,
    last_applied_priority: AtomicU8,
    last_applied_scope: AtomicU8,

    // RUNTIME STATE
    pub(crate) state: AtomicU8,
    pub(crate) shutdown_requested: AtomicBool,
    pub(crate) shutdown_mode: AtomicU8,

    // BACKPRESSURE/IN-FLIGHT
    pub(crate) in_flight: AtomicUsize,        // should be 0/1
    pub(crate) max_in_flight: AtomicUsize,    // prefetch/internal pool

    // METRICS
    pub(crate) jobs_done: AtomicU64,
    pub(crate) jobs_failed: AtomicU64,
    pub(crate) jobs_retried: AtomicU64,

    pub(crate) last_job_started_ms: AtomicU64,
    pub(crate) last_job_finished_ms: AtomicU64,

    pub(crate) last_error_code: AtomicU64,
    pub(crate) last_error_at_ms: AtomicU64,

    // CORRELATION/TRACING
    pub(crate) last_task_id_hi: AtomicU64,    // 128-bit UUID split
    pub(crate) last_task_id_lo: AtomicU64,

    // BROKER RELATED
    pub(crate) consumer_tag_hash: AtomicU64,  // find which consumer RabbitMQ (hash)
}


#[allow(unused)]
impl ScloudWorker {
    const NEVER_APPLIED: u8 = 0xFF;

    pub(crate) fn new(worker_id: u64, worker_type: WorkerType) -> Self {
        Self {
            worker_id,
            os_thread_id: AtomicU64::new(0),
            worker_type,
            stack_size_bytes: AtomicUsize::new(2 * 1024 * 1024),
            buffer_budget_bytes: AtomicUsize::new(4 * 1024 * 1024),
            max_stack_size_bytes: AtomicUsize::new(32 * 1024 * 1024),
            max_buffer_budget_bytes: AtomicUsize::new(256 * 1024 * 1024),
            priority: AtomicU8::new(ThreadPriority::NORMAL as u8),
            priority_scope: AtomicU8::new(PriorityScope::THREAD as u8),
            last_applied_priority: AtomicU8::new(Self::NEVER_APPLIED),
            last_applied_scope: AtomicU8::new(Self::NEVER_APPLIED),
            state: AtomicU8::new(WorkerState::IDLE as u8),
            shutdown_requested: AtomicBool::new(false),
            shutdown_mode: AtomicU8::new(ShutdownMode::GRACEFUL as u8),
            in_flight: AtomicUsize::new(0),
            max_in_flight: AtomicUsize::new(1),
            jobs_done: AtomicU64::new(0),
            jobs_failed: AtomicU64::new(0),
            jobs_retried: AtomicU64::new(0),
            last_job_started_ms: AtomicU64::new(0),
            last_job_finished_ms: AtomicU64::new(0),
            last_error_code: AtomicU64::new(0),
            last_error_at_ms: AtomicU64::new(0),
            last_task_id_hi: AtomicU64::new(0),
            last_task_id_lo: AtomicU64::new(0),
            consumer_tag_hash: AtomicU64::new(0),
        }
    }

    pub(crate) async fn run(conn: &Connection) -> Result<()> {
        let channel = conn.create_channel().await?;

        channel
            .basic_qos(1, BasicQosOptions::default())
            .await?;

        let mut consumer = channel
            .basic_consume(
                "scloud.jobs.waiting",
                "worker-1",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        println!("Worker started");

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery?;
            Self::handle_delivery(&channel, delivery).await?;
        }

        Ok(())
    }

    async fn handle_delivery(channel: &Channel, delivery: Delivery) -> Result<()> {
        let task: ScloudWorkerTask = serde_json::from_slice((&delivery.data).as_ref())?;

        let response = serde_json::json!({
        "ok": true,
        "for": task.for_who,
        "task_id": task.task_id.to_string(),
    });

        if let Some(reply_to) = delivery.properties.reply_to() {
            let mut props = BasicProperties::default();

            if let Some(cid) = delivery.properties.correlation_id().clone() {
                props = props.with_correlation_id(cid);
            }

            channel
                .basic_publish(
                    "",
                    reply_to.as_str(),
                    BasicPublishOptions::default(),
                    &serde_json::to_vec(&response)?,
                    props,
                )
                .await?
                .await?;
        }

        delivery.ack(BasicAckOptions::default()).await?;
        Ok(())
    }


    #[inline]
    pub fn get_worker_id(&self) -> u64 {
        self.worker_id
    }

    #[inline]
    pub fn get_os_thread_id(&self) -> u64 {
        self.os_thread_id.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_worker_type(&self) -> WorkerType {
        self.worker_type
    }

    #[inline]
    pub fn get_stack_size_bytes(&self) -> usize {
        self.stack_size_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_buffer_budget_bytes(&self) -> usize {
        self.buffer_budget_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_max_stack_size_bytes(&self) -> usize {
        self.max_stack_size_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_max_buffer_budget_bytes(&self) -> usize {
        self.max_buffer_budget_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_priority(&self) -> u8 {
        self.priority.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_priority_scope(&self) -> u8 {
        self.priority_scope.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_applied_priority(&self) -> u8 {
        self.last_applied_priority.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_applied_scope(&self) -> u8 {
        self.last_applied_scope.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_state(&self) -> u8 {
        self.state.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_shutdown_mode(&self) -> u8 {
        self.shutdown_mode.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_in_flight(&self) -> usize {
        self.in_flight.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_max_in_flight(&self) -> usize {
        self.max_in_flight.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_jobs_done(&self) -> u64 {
        self.jobs_done.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_jobs_failed(&self) -> u64 {
        self.jobs_failed.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_jobs_retried(&self) -> u64 {
        self.jobs_retried.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_job_started_ms(&self) -> u64 {
        self.last_job_started_ms.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_job_finished_ms(&self) -> u64 {
        self.last_job_finished_ms.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_error_code(&self) -> u64 {
        self.last_error_code.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_error_at_ms(&self) -> u64 {
        self.last_error_at_ms.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_task_id_hi(&self) -> u64 {
        self.last_task_id_hi.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_last_task_id_lo(&self) -> u64 {
        self.last_task_id_lo.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn get_consumer_tag_hash(&self) -> u64 {
        self.consumer_tag_hash.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn set_worker_id(&mut self, worker_id: u64) {
        self.worker_id = worker_id;
    }

    #[inline]
    pub fn set_os_thread_id(&mut self, os_thread_id: u64) {
        self.os_thread_id.store(os_thread_id, Ordering::Relaxed)
    }

    #[inline]
    pub fn set_worker_type(&mut self, worker_type: WorkerType) {
        self.worker_type = worker_type;
    }

    #[inline]
    pub fn set_stack_size_bytes(&mut self, stack_size_bytes: usize) {
        self.stack_size_bytes.store(stack_size_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_buffer_budget_bytes(&mut self, buffer_budget_bytes: usize) {
        self.buffer_budget_bytes.store(buffer_budget_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max_stack_size_bytes(&mut self, max_stack_size_bytes: usize) {
        self.max_stack_size_bytes.store(max_stack_size_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max_buffer_budget_bytes(&mut self, max_buffer_budget_bytes: usize) {
        self.max_buffer_budget_bytes.store(max_buffer_budget_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_priority(&mut self, priority: u8) {
        self.priority.store(priority, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_priority_scope(&mut self, priority_scope: u8) {
        self.priority_scope.store(priority_scope, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_applied_priority(&mut self, last_applied_priority: u8) {
        self.last_applied_priority.store(last_applied_priority, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_applied_scope(&mut self, last_applied_scope: u8) {
        self.last_applied_scope.store(last_applied_scope, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_state(&mut self, state: u8) {
        self.state.store(state, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_shutdown_requested(&mut self, shutdown_requested: bool) {
        self.shutdown_requested.store(shutdown_requested, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_shutdown_mode(&mut self, shutdown_mode: u8) {
        self.shutdown_mode.store(shutdown_mode, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_in_flight(&mut self, in_flight: usize) {
        self.in_flight.store(in_flight, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max_in_flight(&mut self, max_in_flight: usize) {
        self.max_in_flight.store(max_in_flight, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_jobs_done(&mut self, jobs_done: u64) {
        self.jobs_done.store(jobs_done, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_jobs_failed(&mut self, jobs_failed: u64) {
        self.jobs_failed.store(jobs_failed, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_jobs_retried(&mut self, jobs_retried: u64) {
        self.jobs_retried.store(jobs_retried, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_job_started_ms(&mut self, last_job_started_ms: u64) {
        self.last_job_started_ms.store(last_job_started_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_job_finished_ms(&mut self, last_job_finished_ms: u64) {
        self.last_job_finished_ms.store(last_job_finished_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_error_code(&mut self, last_error_code: u64) {
        self.last_error_code.store(last_error_code, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_error_at_ms(&mut self, last_error_at_ms: u64) {
        self.last_error_at_ms.store(last_error_at_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_task_id_hi(&mut self, last_task_id_hi: u64) {
        self.last_task_id_hi.store(last_task_id_hi, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_task_id_lo(&mut self, last_task_id_lo: u64) {
        self.last_task_id_lo.store(last_task_id_lo, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_consumer_tag_hash(&mut self, consumer_tag_hash: u64) {
        self.consumer_tag_hash.store(consumer_tag_hash, Ordering::Relaxed);
    }
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub(crate) enum WorkerState {
    INIT = 0,
    IDLE = 1,
    BUSY = 2,
    PAUSED = 3,
    STOPPING = 4,
    STOPPED = 5,
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub(crate) enum ShutdownMode {
    GRACEFUL = 0,
    IMMEDIATE = 1,
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum WorkerType {
    LISTENER,
    DECODER,
    QUERY_DISPATCHER,
    CACHE_LOOKUP,
    ZONE_MANAGER,
    RESOLVER,
    CACHE_WRITER,
    ENCODER,
    SENDER,

    CACHE_JANITOR,

    METRICS,
    TCP_ACCEPTOR,
}

#[repr(u8)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClassPriority {
    IDLE = 0,
    BELOW_NORMAL = 1,
    NORMAL = 2,
    ABOVE_NORMAL = 3,
    HIGH = 4,
    REALTIME = 5,
}

impl ClassPriority {
    #[inline]
    #[allow(unused)]
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::IDLE,
            1 => Self::BELOW_NORMAL,
            2 => Self::NORMAL,
            3 => Self::ABOVE_NORMAL,
            4 => Self::HIGH,
            5 => Self::REALTIME,
            _ => {
                debug_assert!(false, "invalid ClassPriority value: {}", v);
                Self::NORMAL
            }
        }
    }

    #[inline]
    #[allow(unused)]
    pub fn to_unix_nice(self) -> i32 {
        match self {
            Self::IDLE => 19,
            Self::BELOW_NORMAL => 10,
            Self::NORMAL => 0,
            Self::ABOVE_NORMAL => -5,
            Self::HIGH => -10,
            Self::REALTIME => -20, // Not true RT; this is "strongly favored timesharing" at best.
        }
    }
}

#[repr(u8)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ThreadPriority {
    IDLE = 0,
    LOW = 1,
    BELOW_NORMAL = 2,
    NORMAL = 3,
    ABOVE_NORMAL = 4,
    HIGH = 5,
    REALTIME = 6,
}

impl ThreadPriority {
    #[inline]
    #[allow(unused)]
    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::IDLE,
            1 => Self::LOW,
            2 => Self::BELOW_NORMAL,
            3 => Self::NORMAL,
            4 => Self::ABOVE_NORMAL,
            5 => Self::HIGH,
            6 => Self::REALTIME,
            _ => {
                debug_assert!(false, "invalid ThreadPriority value: {}", v);
                Self::NORMAL
            }
        }
    }
}

#[repr(u8)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PriorityScope {
    THREAD = 0,
    PROCESS = 1,
    USER = 2,
    PROCESS_GROUP = 3,
}

impl PriorityScope {
    #[inline]
    #[allow(unused)]
    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::THREAD,
            1 => Self::PROCESS,
            2 => Self::USER,
            3 => Self::PROCESS_GROUP,
            _ => {
                debug_assert!(false, "invalid PriorityScope value: {}", v);
                Self::THREAD
            }
        }
    }
}

pub struct SpawnConfig<'a> {
    pub name: Option<&'a str>,
    pub stack_size: Option<usize>,
}

impl<'a> Default for SpawnConfig<'a> {
    fn default() -> Self {
        Self {
            name: None,
            stack_size: None,
        }
    }
}

#[allow(unused)]
pub fn new<F, T>(cfg: SpawnConfig<'_>, f: F) -> std::thread::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    thread::thread_base::new(cfg, f)
}

// TODO: should return an ScloudException
#[allow(unused)]
pub fn set_priority(scope: PriorityScope, p: ThreadPriority) -> std::io::Result<()> {
    thread::priority::set_priority(scope, p)
}
