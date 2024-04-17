use chrono::{DateTime, Utc};
use crate::sessions::state::state::{SessionEndReason, State};

pub struct AlreadyEnded {
    pub reason_for_ending: SessionEndReason,
    pub session_end_time: DateTime<Utc>
}

impl State for AlreadyEnded {}