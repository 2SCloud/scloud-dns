#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub enum ThreadPriority {
    Idle,
    Low,
    Normal,
    High,
    Realtime,
}

#[allow(unused)]
pub fn set_current_thread_priority(p: ThreadPriority) -> std::io::Result<()> {
    imp::set_current_thread_priority(p)
}

#[cfg(windows)]
mod imp {
    use super::ThreadPriority;
    use std::io;

    use winapi::shared::minwindef::FALSE;
    use winapi::um::processthreadsapi::{GetCurrentThread, SetThreadPriority};
    use winapi::um::winbase::{
        THREAD_PRIORITY_BELOW_NORMAL, THREAD_PRIORITY_HIGHEST, THREAD_PRIORITY_IDLE,
        THREAD_PRIORITY_NORMAL, THREAD_PRIORITY_TIME_CRITICAL,
    };

    pub fn set_current_thread_priority(p: ThreadPriority) -> io::Result<()> {
        let prio = match p {
            ThreadPriority::Idle => THREAD_PRIORITY_IDLE,
            ThreadPriority::Low => THREAD_PRIORITY_BELOW_NORMAL,
            ThreadPriority::Normal => THREAD_PRIORITY_NORMAL,
            ThreadPriority::High => THREAD_PRIORITY_HIGHEST,
            ThreadPriority::Realtime => THREAD_PRIORITY_TIME_CRITICAL,
        };

        let ok = unsafe { SetThreadPriority(GetCurrentThread(), prio.try_into().unwrap()) };
        if ok == FALSE {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

#[cfg(target_os = "linux")]
mod imp {
    use super::ThreadPriority;
    use nix::sys::resource::{PriorityWhich, setpriority};
    use nix::unistd::gettid;
    use std::io;

    pub fn set_current_thread_priority(p: ThreadPriority) -> io::Result<()> {
        let nice = match p {
            ThreadPriority::Idle => 19,
            ThreadPriority::Low => 10,
            ThreadPriority::Normal => 0,
            ThreadPriority::High => -10,
            ThreadPriority::Realtime => -20,
        };

        let tid = gettid().as_raw() as u32;

        setpriority(PriorityWhich::PRIO_PROCESS, tid, nice)
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;

        Ok(())
    }
}

#[cfg(not(any(windows, target_os = "linux")))]
mod imp {
    use super::ThreadPriority;
    use std::io;

    pub fn set_current_thread_priority(_: ThreadPriority) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Thread priority not supported on this OS",
        ))
    }
}
