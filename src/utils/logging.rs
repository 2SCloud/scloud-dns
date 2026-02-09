use crate::config::{LogFormat, LogLevel, LoggingConfig};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;
use crate::exceptions::SCloudException;
use crate::utils::time::{format_system_time, now_epoch_ms};
use once_cell::sync::OnceCell;
use tokio::sync::mpsc;

struct Logger {
    cfg: LoggingConfig,
    file: File,
}

pub struct OtelLog {
    pub target: String,
    pub severity: &'static str,
    pub message: String,
    pub timestamp: String,
}

pub static LOG_SENDER: OnceCell<mpsc::UnboundedSender<OtelLog>> = OnceCell::new();
static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

/// Initialize global logger.</br>
/// Call once at startup.
pub fn init(cfg: LoggingConfig) -> Result<(), SCloudException> {
    let path = Path::new(&cfg.file);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e|  {
            eprintln!("failed to create log dir {:?}: {}", parent, e);
            SCloudException::SCLOUD_LOGGING_PATH_CREATION_FAILED
        })?;
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path).map_err(|_| SCloudException::SCLOUD_LOGGING_FILE_CREATION_OR_OPENING_FAILED)?;

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

    if level < g.cfg.level {
        if g.cfg.live_print == true {
            let now = SystemTime::now();
            println!("[{}][{:?}][{}] - {}", format_system_time(now), level, target, msg);
        }
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
    format!("[{}][{}][{}] {}", format_system_time(SystemTime::now()), level.as_str(), target, msg)
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
macro_rules! __log_internal {
    ($lvl:expr, $otel_lvl:expr, $($arg:tt)*) => {{
        let target = concat!(module_path!(), ":", line!());

        if let Some(sender) = $crate::utils::logging::LOG_SENDER.get() {
            let _ = sender.send($crate::utils::logging::OtelLog {
                target: target.to_string(),
                severity: $otel_lvl,
                message: format!($($arg)*),
                timestamp: $crate::utils::time::now_unix_nano(),
            });
        }

        $crate::utils::logging::log(
            $lvl,
            target,
            &format!($($arg)*),
        );
    }};
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::TRACE, "TRACE", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::DEBUG, "DEBUG", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::INFO, "INFO", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::WARN, "WARN", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::ERROR, "ERROR", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::FATAL, "FATAL", $($arg)*);
    };
}

