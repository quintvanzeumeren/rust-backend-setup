use chrono::{DateTime, Duration, Utc};

pub fn within_duration(date_time_1: DateTime<Utc>, date_time_2: DateTime<Utc>, duration: Duration) -> bool {
    let difference = (date_time_1 - date_time_2).abs();
    difference <= duration
}
