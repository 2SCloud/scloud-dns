use std::sync::atomic::{AtomicU8, AtomicU64, AtomicUsize, Ordering};

#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
mod others;
mod tests;

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
    pub(crate) os_thread_id: AtomicU64,
    pub(crate) stack_size_bytes: AtomicUsize,
    pub(crate) buffer_budget_bytes: AtomicUsize,
    pub(crate) max_stack_size_bytes: AtomicUsize,
    pub(crate) max_buffer_budget_bytes: AtomicUsize,
    pub(crate) priority: AtomicU8,
    pub(crate) priority_scope: AtomicU8,
    last_applied_priority: AtomicU8,
    last_applied_scope: AtomicU8,
    pub(crate) worker_type: WorkerType,
}

#[allow(unused)]
impl ScloudWorker {
    const NEVER_APPLIED: u8 = 0xFF;

    pub(crate) fn new(worker_type: WorkerType) -> Self {
        Self {
            os_thread_id: AtomicU64::new(0),

            stack_size_bytes: AtomicUsize::new(2 * 1024 * 1024),
            buffer_budget_bytes: AtomicUsize::new(4 * 1024 * 1024),
            max_stack_size_bytes: AtomicUsize::new(32 * 1024 * 1024),
            max_buffer_budget_bytes: AtomicUsize::new(256 * 1024 * 1024),

            priority: AtomicU8::new(ThreadPriority::NORMAL as u8),
            priority_scope: AtomicU8::new(PriorityScope::THREAD as u8),

            last_applied_priority: AtomicU8::new(Self::NEVER_APPLIED),
            last_applied_scope: AtomicU8::new(Self::NEVER_APPLIED),

            worker_type,
        }
    }

    #[inline]
    pub(crate) fn set_os_thread_id(&self, tid: u64) {
        self.os_thread_id.store(tid, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn get_os_thread_id(&self) -> u64 {
        self.os_thread_id.load(Ordering::Relaxed)
    }

    #[inline]
    pub(crate) fn set_priority(&self, p: ThreadPriority) {
        self.priority.store(p as u8, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn get_priority(&self) -> ThreadPriority {
        ThreadPriority::from_u8(self.priority.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn set_class_priority(p: ClassPriority) -> std::io::Result<()> {
        thread::priority::set_class_priority(p)
    }

    #[inline]
    pub(crate) fn get_priority_as_u8(&self) -> u8 {
        self.priority.load(Ordering::Relaxed)
    }

    #[inline]
    pub(crate) fn set_priority_scope(&self, s: PriorityScope) {
        self.priority_scope.store(s as u8, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn get_priority_scope(&self) -> PriorityScope {
        PriorityScope::from_u8(self.priority_scope.load(Ordering::Relaxed))
    }

    #[inline]
    pub(crate) fn get_priority_scope_as_u8(&self) -> u8 {
        self.priority_scope.load(Ordering::Relaxed)
    }

    #[inline]
    pub(crate) fn apply_priority_now(&self) -> std::io::Result<()> {
        set_priority(self.get_priority_scope(), self.get_priority())
    }

    pub(crate) fn apply_priority_if_changed(&self) -> std::io::Result<bool> {
        let scope_u8 = self.get_priority_scope_as_u8();
        let prio_u8 = self.get_priority_as_u8();

        let last_scope = self.last_applied_scope.load(Ordering::Relaxed);
        let last_prio = self.last_applied_priority.load(Ordering::Relaxed);

        if last_scope == scope_u8 && last_prio == prio_u8 {
            return Ok(false);
        }

        set_priority(
            PriorityScope::from_u8(scope_u8),
            ThreadPriority::from_u8(prio_u8),
        )?;

        self.last_applied_scope.store(scope_u8, Ordering::Relaxed);
        self.last_applied_priority.store(prio_u8, Ordering::Relaxed);

        Ok(true)
    }

    #[inline]
    pub(crate) fn set_buffer_budget_bytes(&self, v: usize) {
        self.buffer_budget_bytes.store(v, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn buffer_budget_bytes(&self) -> usize {
        self.buffer_budget_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub(crate) fn set_max_buffer_budget_bytes(&self, v: usize) {
        self.max_buffer_budget_bytes.store(v, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn max_buffer_budget_bytes(&self) -> usize {
        self.max_buffer_budget_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub(crate) fn set_stack_size_bytes(&self, v: usize) {
        self.stack_size_bytes.store(v, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn stack_size_bytes(&self) -> usize {
        self.stack_size_bytes.load(Ordering::Relaxed)
    }

    #[inline]
    pub(crate) fn set_max_stack_size_bytes(&self, v: usize) {
        self.max_stack_size_bytes.store(v, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn max_stack_size_bytes(&self) -> usize {
        self.max_stack_size_bytes.load(Ordering::Relaxed)
    }
}

#[allow(unused)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WorkerType {
    Listener,
    Decoder,
    QueryDispatcher,
    CacheLookup,
    ZoneManager,
    Resolver,
    CacheWriter,
    Encoder,
    Sender,

    CacheJanitor,

    Metrics,
    TcpAcceptor,
}

#[repr(u8)]
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
