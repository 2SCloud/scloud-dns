pub(crate) mod imp {
    use crate::threads::{ClassPriority, PriorityScope, ThreadPriority};
    use std::io;
    use winapi::shared::minwindef::FALSE;
    use winapi::um::processthreadsapi::{
        GetCurrentProcess, GetCurrentThread, SetPriorityClass, SetThreadPriority,
    };
    use winapi::um::winbase::*;

    pub(crate) fn set_priority(scope: PriorityScope, p: ThreadPriority) -> io::Result<()> {
        match scope {
            PriorityScope::THREAD => set_thread(p),
            PriorityScope::PROCESS => set_process(p),

            PriorityScope::USER | PriorityScope::PROCESS_GROUP => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Priority scope not supported on Windows",
            )),
        }
    }

    pub(crate) fn set_class_priority(p: ClassPriority) -> io::Result<()> {
        let class = match p {
            ClassPriority::IDLE => IDLE_PRIORITY_CLASS,
            ClassPriority::BELOW_NORMAL => BELOW_NORMAL_PRIORITY_CLASS,
            ClassPriority::NORMAL => NORMAL_PRIORITY_CLASS,
            ClassPriority::ABOVE_NORMAL => ABOVE_NORMAL_PRIORITY_CLASS,
            ClassPriority::HIGH => HIGH_PRIORITY_CLASS,
            ClassPriority::REALTIME => REALTIME_PRIORITY_CLASS,
        };

        let ok = unsafe { SetPriorityClass(GetCurrentProcess(), class) };
        if ok == FALSE {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn set_thread(p: ThreadPriority) -> io::Result<()> {
        let prio = match p {
            ThreadPriority::IDLE => THREAD_PRIORITY_IDLE,
            ThreadPriority::LOW => THREAD_PRIORITY_LOWEST,
            ThreadPriority::BELOW_NORMAL => THREAD_PRIORITY_BELOW_NORMAL,
            ThreadPriority::NORMAL => THREAD_PRIORITY_NORMAL,
            ThreadPriority::ABOVE_NORMAL => THREAD_PRIORITY_ABOVE_NORMAL,
            ThreadPriority::HIGH => THREAD_PRIORITY_HIGHEST,
            ThreadPriority::REALTIME => THREAD_PRIORITY_TIME_CRITICAL,
        };

        let ok = unsafe { SetThreadPriority(GetCurrentThread(), prio.try_into().unwrap()) };

        if ok == FALSE {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn apply_process_class(class_priority: ClassPriority) -> io::Result<()> {
        type BOOL = i32;
        type HANDLE = *mut core::ffi::c_void;
        type DWORD = u32;

        const IDLE_PRIORITY_CLASS: DWORD = 0x00000040;
        const BELOW_NORMAL_PRIORITY_CLASS: DWORD = 0x00004000;
        const NORMAL_PRIORITY_CLASS: DWORD = 0x00000020;
        const ABOVE_NORMAL_PRIORITY_CLASS: DWORD = 0x00008000;
        const HIGH_PRIORITY_CLASS: DWORD = 0x00000080;
        const REALTIME_PRIORITY_CLASS: DWORD = 0x00000100;

        extern "system" {
            fn GetCurrentProcess() -> HANDLE;
            fn SetPriorityClass(hProcess: HANDLE, dwPriorityClass: DWORD) -> BOOL;
        }

        let klass: DWORD = match class_priority {
            ClassPriority::IDLE => IDLE_PRIORITY_CLASS,
            ClassPriority::BELOW_NORMAL => BELOW_NORMAL_PRIORITY_CLASS,
            ClassPriority::NORMAL => NORMAL_PRIORITY_CLASS,
            ClassPriority::ABOVE_NORMAL => ABOVE_NORMAL_PRIORITY_CLASS,
            ClassPriority::HIGH => HIGH_PRIORITY_CLASS,
            ClassPriority::REALTIME => REALTIME_PRIORITY_CLASS,
        };

        let ok = unsafe { SetPriorityClass(GetCurrentProcess(), klass) };
        if ok == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}