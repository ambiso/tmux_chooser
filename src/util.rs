use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_seconds(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs / 60 < 60 {
        format!("{}m", secs / 60)
    } else if secs / (60 * 60) < 24 {
        format!("{}h", secs / (60 * 60))
    } else {
        format!("{}d", secs / (60 * 60 * 24))
    }
}

pub fn seconds_since_unix_timestamp(timestamp: u64) -> u64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .expect("Could not compute current time")
        .as_secs()
        - timestamp
}
