// src/threads/others/priority.rs
pub(crate) mod imp {
    use crate::threads::{ClassPriority, PriorityScope, ThreadPriority};
    use std::io;

    pub(crate) fn set_priority(_scope: PriorityScope, _p: ThreadPriority) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Thread priority is not supported on this platform",
        ))
    }

    pub(crate) fn set_class_priority(_p: ClassPriority) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Class priority is not supported on this platform",
        ))
    }
}
