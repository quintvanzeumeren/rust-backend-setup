use chrono::{DateTime, Utc};
use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;

use crate::sessions::state::state::{SessionEndReason, State};
use crate::sessions::tokens::RefreshToken;

#[derive(Debug)]
pub struct JustEnded {
    pub(in crate::sessions) latest_refresh_token: LocalPasetoV4Token<RefreshToken>,
    pub(in crate::sessions) reason_for_ending: SessionEndReason,
    pub(in crate::sessions) session_end_time: DateTime<Utc>
}

impl JustEnded {
    pub fn latest_refresh_token(&self) -> &LocalPasetoV4Token<RefreshToken> {
        &self.latest_refresh_token
    }
    pub fn reason_for_ending(&self) -> &SessionEndReason {
        &self.reason_for_ending
    }
    pub fn session_end_time(&self) -> DateTime<Utc> {
        self.session_end_time
    }
}

impl State for JustEnded {
    
}