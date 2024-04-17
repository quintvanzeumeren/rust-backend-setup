use crate::util::test_user::user_state::{Token, UserState};

#[derive(Clone)]
pub struct LoggedIn {
    pub access_token: Token,
    pub refresh_token: Token
}

impl UserState for LoggedIn {
    fn access_token(&self) -> Option<&Token> {
        Some(&self.access_token)
    }

    fn refresh_token(&self) -> Option<&Token> {
        Some(&self.refresh_token)
    }
}