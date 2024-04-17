use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Token {
    pub token: String,
    pub expiration: DateTime<Utc>
}

pub trait UserState {
    fn access_token(&self) -> Option<&Token>;

    fn refresh_token(&self) -> Option<&Token>;
}
