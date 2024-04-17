use crate::util::test_user::user_state::{Token, UserState};

#[derive(Clone)]
pub struct Anonymous {}

impl UserState for Anonymous {
    fn access_token(&self) -> Option<&Token> {
        None
    }

    fn refresh_token(&self) -> Option<&Token> {
        None
    }
}