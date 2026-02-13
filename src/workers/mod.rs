use crate::exceptions::SCloudException;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use crate::{log_error, log_info, log_sdebug, log_strace};
use tokio::sync::{mpsc, Mutex, MutexGuard, Semaphore};
use std::sync::Arc;
use crate::workers::task::InFlightTask;

pub(crate) mod task;
pub(crate) mod tests;
pub(crate) mod queue;
pub(crate) mod types;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct SCloudWorker {
    // IDENTITY
    pub(crate) worker_id: u64,
    pub(crate) worker_type: WorkerType,

    // CHANNEL
    pub(crate) dns_tx: Mutex<Option<mpsc::Sender<InFlightTask>>>,
    pub(crate) dns_rx: Mutex<Option<mpsc::Receiver<InFlightTask>>>,

    // RESOURCES/LIMITS
    pub(crate) stack_size_bytes: AtomicUsize,
    pub(crate) buffer_budget_bytes: AtomicUsize,
    pub(crate) max_stack_size_bytes: AtomicUsize,
    pub(crate) max_buffer_budget_bytes: AtomicUsize,

    // RUNTIME STATE
    pub(crate) state: AtomicU8,
    pub(crate) shutdown_requested: AtomicBool,
    pub(crate) shutdown_mode: AtomicU8,

    // BACKPRESSURE/IN-FLIGHT
    pub(crate) in_flight: AtomicUsize, // for metrics
    pub(crate) in_flight_sem: Arc<Semaphore>,
    pub(crate) max_in_flight: AtomicUsize,

    // METRICS
    pub(crate) jobs_done: AtomicU64,
    pub(crate) jobs_failed: AtomicU64,
    pub(crate) jobs_retried: AtomicU64,

    pub(crate) last_job_started_ms: AtomicU64,
    pub(crate) last_job_finished_ms: AtomicU64,

    pub(crate) last_error_code: AtomicU64,
    pub(crate) last_error_at_ms: AtomicU64,

    // CORRELATION/TRACING
    pub(crate) last_task_id_hi: AtomicU64, // 128-bit UUID split
    pub(crate) last_task_id_lo: AtomicU64,
}

#[allow(unused)]
impl SCloudWorker {
    const NEVER_APPLIED: u8 = 0xFF;

    pub(crate) fn new(worker_id: u64, worker_type: WorkerType) -> Result<Self, SCloudException> {

        Ok(Self {
            worker_id,
            worker_type,
            dns_tx: Mutex::new(None),
            dns_rx: Mutex::new(None),
            stack_size_bytes: AtomicUsize::new(2 * 1024 * 1024),
            buffer_budget_bytes: AtomicUsize::new(4 * 1024 * 1024),
            max_stack_size_bytes: AtomicUsize::new(32 * 1024 * 1024),
            max_buffer_budget_bytes: AtomicUsize::new(256 * 1024 * 1024),
            state: AtomicU8::new(WorkerState::INIT as u8),
            shutdown_requested: AtomicBool::new(false),
            shutdown_mode: AtomicU8::new(ShutdownMode::GRACEFUL as u8),
            in_flight: AtomicUsize::new(0),
            in_flight_sem: Arc::new(Semaphore::new(512)),
            max_in_flight: AtomicUsize::new(512),
            jobs_done: AtomicU64::new(0),
            jobs_failed: AtomicU64::new(0),
            jobs_retried: AtomicU64::new(0),
            last_job_started_ms: AtomicU64::new(0),
            last_job_finished_ms: AtomicU64::new(0),
            last_error_code: AtomicU64::new(0),
            last_error_at_ms: AtomicU64::new(0),
            last_task_id_hi: AtomicU64::new(0),
            last_task_id_lo: AtomicU64::new(0),
        })
    }

    pub(crate) async fn run(self: Arc<Self>) -> Result<(), SCloudException> {
        log_sdebug!("Running SCloudWorker [ID: {}][TYPE: {:?}]", self.worker_id, self.worker_type);
        match self.worker_type {
            WorkerType::LISTENER => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let tx = self.get_dns_tx().await?;
                types::listener::run_dns_listener(self, "0.0.0.0:5353", tx).await?;
            }
            WorkerType::DECODER => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::decoder::run_dns_decoder(self.clone(), rx, tx).await?;
            }
            WorkerType::QUERY_DISPATCHER => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::query_dispatcher::run_dns_query_dispatcher(self.clone(), rx, tx).await?;
            }
            WorkerType::CACHE_LOOKUP => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::cache_lookup::run_dns_cache_lookup(self.clone(), rx, tx).await?;
            }
            WorkerType::ZONE_MANAGER => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::zone_manager::run_dns_zone_manager(self.clone(), rx, tx).await?;
            }
            WorkerType::RESOLVER => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::resolver::run_dns_resolver(self.clone(), rx, tx).await?;
            }
            WorkerType::CACHE_WRITER => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::cache_writer::run_dns_cache_writer(self.clone(), rx, tx).await?;
            }
            WorkerType::ENCODER => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::encoder::run_dns_encoder(self.clone(), rx, tx).await?;
            }
            WorkerType::SENDER => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::sender::run_dns_sender(self.clone(), rx, tx).await?;
            }
            WorkerType::CACHE_JANITOR => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let rx = self.get_dns_rx().await?;
                types::cache_janitor::run_dns_cache_janitor(self.clone(), rx).await?;
            }
            WorkerType::METRICS => {
                self.clone().set_state(WorkerState::IDLE as u8);
                types::metrics::start_otlp_logger().await;
            }
            WorkerType::TCP_ACCEPTOR => {
                self.clone().set_state(WorkerState::IDLE as u8);
                let (rx, tx) = self.get_dns_rx_tx().await?;
                types::tcp_acceptor::run_dns_tcp_acceptor(self.clone(), rx, tx).await?;
            }
            _ => {

            }
        }
        Ok(())
    }

    #[inline]
    pub fn get_worker_id(&self) -> u64 {
        self.worker_id
    }

    #[inline]
    pub fn get_worker_type(&self) -> WorkerType {
        self.worker_type
    }

    #[inline]
    pub async fn get_dns_rx_tx(&self) -> (
        Result<
            (mpsc::Receiver<InFlightTask>,
            mpsc::Sender<InFlightTask>),
            SCloudException
        >
    ) {
        let rx = self.dns_rx
            .lock()
            .await
            .take()
            .ok_or(SCloudException::SCLOUD_WORKER_RX_NOT_SET);

        let tx = self.dns_tx
            .lock()
            .await
            .as_ref()
            .cloned()
            .ok_or(SCloudException::SCLOUD_WORKER_TX_NOT_SET);


        Ok((rx?, tx?))
    }

    #[inline]
    pub async fn get_dns_rx(&self) -> Result<mpsc::Receiver<InFlightTask>, SCloudException> {
        self.dns_rx
            .lock()
            .await
            .take()
            .ok_or(SCloudException::SCLOUD_WORKER_RX_NOT_SET)
    }

    #[inline]
    pub async fn get_dns_tx(&self) -> Result<mpsc::Sender<InFlightTask>, SCloudException> {
        self.dns_tx
            .lock()
            .await
            .as_ref()
            .cloned()
            .ok_or(SCloudException::SCLOUD_WORKER_TX_NOT_SET)
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
    pub fn get_in_flight_sem(&self) -> usize {
        self.in_flight_sem.available_permits()
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
    pub fn set_worker_id(&mut self, worker_id: u64) {
        self.worker_id = worker_id;
    }

    #[inline]
    pub fn set_worker_type(&mut self, worker_type: WorkerType) {
        self.worker_type = worker_type;
    }

    #[inline]
    pub async fn set_dns_tx(&self, tx: mpsc::Sender<InFlightTask>) {
        *self.dns_tx.lock().await = Some(tx);
    }

    #[inline]
    pub async fn set_dns_rx(&self, rx: mpsc::Receiver<InFlightTask>) {
        *self.dns_rx.lock().await = Some(rx);
    }

    #[inline]
    pub fn set_stack_size_bytes(&mut self, stack_size_bytes: usize) {
        self.stack_size_bytes
            .store(stack_size_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_buffer_budget_bytes(&mut self, buffer_budget_bytes: usize) {
        self.buffer_budget_bytes
            .store(buffer_budget_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max_stack_size_bytes(&mut self, max_stack_size_bytes: usize) {
        self.max_stack_size_bytes
            .store(max_stack_size_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_max_buffer_budget_bytes(&mut self, max_buffer_budget_bytes: usize) {
        self.max_buffer_budget_bytes
            .store(max_buffer_budget_bytes, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_state(&self, state: u8) {
        self.state.store(state, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_shutdown_requested(&mut self, shutdown_requested: bool) {
        self.shutdown_requested
            .store(shutdown_requested, Ordering::Relaxed);
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
        self.last_job_started_ms
            .store(last_job_started_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_job_finished_ms(&mut self, last_job_finished_ms: u64) {
        self.last_job_finished_ms
            .store(last_job_finished_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_error_code(&mut self, last_error_code: u64) {
        self.last_error_code
            .store(last_error_code, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_error_at_ms(&mut self, last_error_at_ms: u64) {
        self.last_error_at_ms
            .store(last_error_at_ms, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_task_id_hi(&mut self, last_task_id_hi: u64) {
        self.last_task_id_hi
            .store(last_task_id_hi, Ordering::Relaxed);
    }

    #[inline]
    pub fn set_last_task_id_lo(&mut self, last_task_id_lo: u64) {
        self.last_task_id_lo
            .store(last_task_id_lo, Ordering::Relaxed);
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

pub fn spawn_worker(
    worker: Arc<SCloudWorker>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {

        if let Err(e) = worker.clone().run().await {
            log_error!(
                "Worker {} failed: {:?}",
                worker.worker_id,
                e
            );
        } else {
            log_info!(
                "Worker {} ({:?}) started",
                worker.worker_id,
                worker.worker_type
            );
        }
    })
}
