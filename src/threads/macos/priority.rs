#![allow(unused)]
#![allow(non_camel_case_types)]

use std::io;
use nix::unistd::{getpgid, getpid, getuid};

use crate::threads::{ClassPriority, PriorityScope, ThreadPriority};

pub(crate) mod imp {
    use super::*;
    use std::ffi::c_int;

    type qos_class_t = u32;
    const QOS_CLASS_BACKGROUND: qos_class_t = 0x09;
    const QOS_CLASS_UTILITY: qos_class_t = 0x11;
    const QOS_CLASS_DEFAULT: qos_class_t = 0x15;
    const QOS_CLASS_USER_INITIATED: qos_class_t = 0x19;
    const QOS_CLASS_USER_INTERACTIVE: qos_class_t = 0x21;

    unsafe extern "C" {
        fn pthread_set_qos_class_self_np(qos_class: qos_class_t, relative_priority: c_int) -> c_int;
    }

    pub(crate) fn set_priority(scope: PriorityScope, p: ThreadPriority) -> io::Result<()> {
        match scope {
            PriorityScope::THREAD => set_thread_priority(p),
            PriorityScope::PROCESS => set_nice(libc::PRIO_PROCESS, getpid().as_raw() as libc::id_t, p),
            PriorityScope::USER => set_nice(libc::PRIO_USER, getuid().as_raw() as libc::id_t, p),
            PriorityScope::PROCESS_GROUP => {
                let pgid = getpgid(None).map_err(nix_to_io)?;
                set_nice(libc::PRIO_PGRP, pgid.as_raw() as libc::id_t, p)
            }
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

    fn set_thread_priority(p: ThreadPriority) -> io::Result<()> {

        let want_bg = matches!(p, ThreadPriority::IDLE | ThreadPriority::LOW | ThreadPriority::BELOW_NORMAL);

        let which = libc::PRIO_DARWIN_THREAD;
        let who: libc::id_t = 0; // current thread
        let prio: c_int = if want_bg { libc::PRIO_DARWIN_BG } else { 0 };

        let rc = unsafe { libc::setpriority(which, who, prio) };
        if rc == -1 {
            return Err(io::Error::last_os_error());
        }

        if !want_bg {
            apply_qos_for_priority(p)?;
        }

        #[cfg(feature = "mach-rt")]
        {
            if p == ThreadPriority::REALTIME {
                apply_mach_realtime_time_constraint()?;
            }
        }

        Ok(())
    }

    fn apply_qos_for_priority(p: ThreadPriority) -> io::Result<()> {
        let (qos_class, rel_prio) = match p {
            ThreadPriority::NORMAL => (QOS_CLASS_DEFAULT, 0),
            ThreadPriority::ABOVE_NORMAL => (QOS_CLASS_USER_INITIATED, 0),
            ThreadPriority::HIGH => (QOS_CLASS_USER_INTERACTIVE, 0),
            ThreadPriority::REALTIME => (QOS_CLASS_USER_INTERACTIVE, 0),

            ThreadPriority::IDLE => (QOS_CLASS_BACKGROUND, 0),
            ThreadPriority::LOW => (QOS_CLASS_UTILITY, 0),
            ThreadPriority::BELOW_NORMAL => (QOS_CLASS_UTILITY, 0),
        };

        let rc = unsafe { pthread_set_qos_class_self_np(qos_class, rel_prio) };
        if rc != 0 {
            return Err(io::Error::from_raw_os_error(rc));
        }
        Ok(())
    }

    fn set_nice(which: c_int, who: libc::id_t, p: ThreadPriority) -> io::Result<()> {
        let nice: c_int = match p {
            ThreadPriority::IDLE => 19,
            ThreadPriority::LOW => 10,
            ThreadPriority::BELOW_NORMAL => 5,
            ThreadPriority::NORMAL => 0,
            ThreadPriority::ABOVE_NORMAL => -5,
            ThreadPriority::HIGH => -10,
            ThreadPriority::REALTIME => -20, // best-effort timesharing NOT true RT
        };

        let rc = unsafe { libc::setpriority(which, who, nice) };
        if rc == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    #[inline]
    fn nix_to_io(e: nix::errno::Errno) -> io::Error {
        io::Error::from_raw_os_error(e as i32)
    }

    #[cfg(feature = "mach-rt")]
    fn apply_mach_realtime_time_constraint() -> io::Result<()> {
        type kern_return_t = i32;
        type thread_act_t = u32;
        type integer_t = i32;

        const KERN_SUCCESS: kern_return_t = 0;
        const THREAD_TIME_CONSTRAINT_POLICY: i32 = 2;

        #[repr(C)]
        struct thread_time_constraint_policy {
            period: u32,
            computation: u32,
            constraint: u32,
            preemptible: i32,
        }

        unsafe extern "C" {
            fn mach_thread_self() -> thread_act_t;
            fn thread_policy_set(
                thread: thread_act_t,
                flavor: i32,
                policy_info: *const integer_t,
                count: u32,
            ) -> kern_return_t;

            fn mach_port_deallocate(task: u32, name: u32) -> kern_return_t;
            fn mach_task_self() -> u32;
        }

        let policy = thread_time_constraint_policy {
            period: 5_000_000,
            computation: 1_000_000,
            constraint: 5_000_000,
            preemptible: 1,
        };

        let count = (std::mem::size_of::<thread_time_constraint_policy>()
            / std::mem::size_of::<integer_t>()) as u32;

        let thread = unsafe { mach_thread_self() };
        let kr = unsafe {
            thread_policy_set(
                thread,
                THREAD_TIME_CONSTRAINT_POLICY,
                &policy as *const _ as *const integer_t,
                count,
            )
        };

        unsafe {
            let _ = mach_port_deallocate(mach_task_self(), thread);
        }

        if kr != KERN_SUCCESS {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("thread_policy_set failed (kern_return_t={})", kr),
            ));
        }

        Ok(())
    }
}