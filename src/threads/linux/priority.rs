pub(crate) mod imp {
    use nix::unistd::{getpgid, getpid, gettid, getuid};
    use std::io;
    use crate::threads::{ClassPriority, PriorityScope, ThreadPriority};

    pub(crate) fn set_priority(scope: PriorityScope, p: ThreadPriority) -> io::Result<()> {
        let nice: i32 = match p {
            ThreadPriority::IDLE => 19,
            ThreadPriority::LOW => 10,
            ThreadPriority::BELOW_NORMAL => 5,
            ThreadPriority::NORMAL => 0,
            ThreadPriority::ABOVE_NORMAL => -5,
            ThreadPriority::HIGH => -10,
            ThreadPriority::REALTIME => -20,
        };

        let (which, who) = match scope {
            PriorityScope::THREAD => (libc::PRIO_PROCESS, gettid().as_raw() as libc::id_t),
            PriorityScope::PROCESS => (libc::PRIO_PROCESS, getpid().as_raw() as libc::id_t),
            PriorityScope::USER => (libc::PRIO_USER, getuid().as_raw() as libc::id_t),
            PriorityScope::PROCESS_GROUP => {
                let pgid = getpgid(None).map_err(nix_to_io)?;
                (libc::PRIO_PGRP, pgid.as_raw() as libc::id_t)
            }
        };

        let rc = unsafe { libc::setpriority(which, who, nice) };
        if rc == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub(crate) fn set_class_priority(p: ClassPriority) -> io::Result<()> {
        let nice = p.to_unix_nice();
        let pid: libc::id_t = getpid().as_raw() as libc::id_t;

        let rc = unsafe { libc::setpriority(libc::PRIO_PROCESS, pid, nice) };
        if rc == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    // TODO: change to ScloudException
    fn nix_to_io(e: nix::Error) -> io::Error {
        match e {
            nix::Error::Sys(errno) => io::Error::from_raw_os_error(errno as i32),
            other => io::Error::new(io::ErrorKind::Other, other.to_string()),
        }
    }

    fn apply_unix_nice(priority_class: ClassPriority) -> io::Result<()> {
        let nice = priority_class.to_unix_nice();

        let pid: libc::id_t = unsafe { libc::getpid() } as libc::id_t;
        let rc = unsafe { libc::setpriority(libc::PRIO_PROCESS, pid, nice) };

        if rc == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}