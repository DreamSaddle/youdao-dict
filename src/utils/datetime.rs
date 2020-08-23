
use chrono::prelude::*;

pub fn current_timestamp_millis() -> i64 {
    let dt = Local::now();
    dt.timestamp_millis()
}