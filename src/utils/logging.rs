use crate::config::{LogFormat, LogLevel, LoggingConfig};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;
use crate::utils::time::{format_system_time, now_epoch_ms};

struct Logger {
    cfg: LoggingConfig,
    file: File,
}

static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

/// Initialize global logger.</br>
/// Call once at startup.
pub fn init(cfg: LoggingConfig) -> io::Result<()> {
    let path = Path::new(&cfg.file);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let logger = Logger { cfg, file };
    let _ = LOGGER.set(Mutex::new(logger));
    Ok(())
}


/// Write one log line (internal).</br>
/// Safe to call from any thread.
pub fn log(level: LogLevel, target: &str, msg: &str) {
    let Some(lock) = LOGGER.get() else {
        return;
    };

    let mut g = match lock.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    };

    if g.cfg.live_print == true {
        let now = SystemTime::now();
        println!("[{}][{:?}][{}] - {}", format_system_time(now), level, target, msg);
    }

    if level < g.cfg.level {
        return;
    }

    if g.cfg.rotate {
        let max_bytes = g.cfg.max_size_mb.saturating_mul(1024 * 1024);
        if max_bytes > 0 {
            if let Ok(meta) = g.file.metadata() {
                if meta.len() >= max_bytes {
                    let _ = rotate_file(&mut *g);
                }
            }
        }
    }

    let line = match g.cfg.format {
        LogFormat::JSON => format_json_line(level, target, msg),
        LogFormat::TEXT => format_text_line(level, target, msg),
    };

    let _ = g.file.write_all(line.as_bytes());
    let _ = g.file.write_all(b"\n");
    let _ = g.file.flush();
}

fn rotate_file(logger: &mut Logger) -> io::Result<()> {
    let path = Path::new(&logger.cfg.file);

    let epoch_ms = now_epoch_ms();
    let rotated = rotated_name(path, epoch_ms);

    let _ = logger.file.flush();

    let _ = fs::rename(path, &rotated);

    logger.file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    Ok(())
}

fn rotated_name(path: &Path, epoch_ms: u128) -> PathBuf {
    let mut p = path.as_os_str().to_owned();
    let suffix = format!(".{}", epoch_ms);
    p.push(suffix);
    PathBuf::from(p)
}

fn format_text_line(level: LogLevel, target: &str, msg: &str) -> String {
    format!("{} {} {} {}", now_epoch_ms(), level.as_str(), target, msg)
}

fn format_json_line(level: LogLevel, target: &str, msg: &str) -> String {
    let msg_esc = json_escape(msg);
    let target_esc = json_escape(target);
    format!(
        r#"{{"ts":{},"level":"{}","target":"{}","msg":"{}"}}"#,
        now_epoch_ms(),
        level.as_str(),
        target_esc,
        msg_esc
    )
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            _ => out.push(c),
        }
    }
    out
}

#[macro_export]
macro_rules! log_error {
    ($target:expr, $($arg:tt)*) => {
        $crate::utils::logging::log($crate::utils::logging::LogLevel::Error, $target, &format!($($arg)*))
    };
}
#[macro_export]
macro_rules! log_warn {
    ($target:expr, $($arg:tt)*) => {
        $crate::utils::logging::log($crate::utils::logging::LogLevel::Warn, $target, &format!($($arg)*))
    };
}
#[macro_export]
macro_rules! log_info {
    ($target:expr, $($arg:tt)*) => {
        $crate::utils::logging::log($crate::utils::logging::LogLevel::Info, $target, &format!($($arg)*))
    };
}
#[macro_export]
macro_rules! log_debug {
    ($target:expr, $($arg:tt)*) => {
        $crate::utils::logging::log($crate::utils::logging::LogLevel::Debug, $target, &format!($($arg)*))
    };
}
#[macro_export]
macro_rules! log_trace {
    ($target:expr, $($arg:tt)*) => {
        $crate::utils::logging::log($crate::utils::logging::LogLevel::Trace, $target, &format!($($arg)*))
    };
}
