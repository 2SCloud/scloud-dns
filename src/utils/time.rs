use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn now_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

pub(crate) fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };

    (
        (y + if m <= 2 { 1 } else { 0 }) as i32,
        m as u32,
        d as u32,
    )
}

pub(crate) fn format_unix_timestamp(secs: u64) -> String {
    const SECS_PER_DAY: u64 = 86_400;

    let days = secs / SECS_PER_DAY;
    let mut rem = secs % SECS_PER_DAY;

    let hour = rem / 3600;
    rem %= 3600;
    let min = rem / 60;
    let sec = rem % 60;

    let (year, month, day) = civil_from_days(days as i64);

    format!(
        "{:02}/{:02}/{:04}-{:02}:{:02}:{:02}",
        month, day, year, hour, min, sec
    )
}

pub(crate) fn format_system_time(t: SystemTime) -> String {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format_unix_timestamp(secs)
}

pub fn now_unix_nano() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    now.as_nanos().to_string()
}