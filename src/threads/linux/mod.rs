pub(crate) mod priority;

pub(crate) mod imp {
    use crate::exceptions::SCloudException;
    use crate::log_error;
    use super::super::SpawnConfig;

    pub fn new<F, T>(cfg: SpawnConfig<'_>, f: F) -> Result<std::thread::JoinHandle<T>, SCloudException>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let mut b = std::thread::Builder::new();
        if let Some(name) = cfg.name {
            b = b.name(name.to_string());
        }
        if let Some(sz) = cfg.stack_size {
            b = b.stack_size(sz);
        }

        b.spawn(f).map_err(|_| {
            log_error!("{}", SCloudException::SCLOUD_THREADS_FAILED_TO_SPAWN.to_str());
            SCloudException::SCLOUD_THREADS_FAILED_TO_SPAWN
        })
    }

    pub fn current_thread_id() -> usize {
        unsafe { libc::syscall(libc::SYS_gettid) as usize }
    }
}
