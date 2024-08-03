use chrono::{DateTime, Utc};
use reqwest::Response;
use serde::Deserialize;
use uuid::Uuid;

use crate::util::test_app::TestApp;
use crate::util::test_user::anonymous::Anonymous;
use crate::util::test_user::logged_in::LoggedIn;
use crate::util::test_user::user_state::{Token, UserState};

pub struct TestUser<'a, State: UserState + Clone> {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub state: State,
    pub app: &'a TestApp
}

impl<'a, State: UserState + Clone> TestUser<'a, State> {

    pub fn with_password(&self, new_password: String) -> TestUser<State> {
        TestUser {
            user_id: self.user_id.clone(),
            username: self.username.clone(),
            password: new_password,
            state: self.state.clone(),
            app: self.app,
        }
    }

    pub fn with_username(&self, new_password: String) -> TestUser<State> {
        TestUser {
            user_id: self.user_id.clone(),
            username: self.username.clone(),
            password: new_password,
            state: self.state.clone(),
            app: self.app,
        }
    }
}

impl <'a> TestUser<'a, Anonymous> {
    pub async fn login(&self) -> TestUser<LoggedIn> {
        let login_response = self.app.login(&self)
            .await
            .error_for_status()
            .expect("Failed to receive 200 status code")
            .json::<LoginResponses>()
            .await
            .expect("Failed to map login response");

        return match login_response {
            LoginResponses::UserLoggedInSuccessfully {
                access_token,
                access_token_expiration,
                refresh_token,
                refresh_token_expiration
            } => {
                TestUser {
                    user_id: self.user_id.clone(),
                    username: self.username.clone(),
                    password: self.password.clone(),
                    state: LoggedIn {
                        access_token: Token {
                            token: access_token,
                            expiration: access_token_expiration,
                        },
                        refresh_token: Token {
                            token: refresh_token,
                            expiration: refresh_token_expiration,
                        }
                    },
                    app: self.app,
                }
            },
        }
    }
}

#[derive(Deserialize)]
pub struct ExpectedRefreshResponse {
    pub refresh_token: String,
    pub refresh_token_expiration: DateTime<Utc>,
    pub access_token: String,
    pub access_token_expiration: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ExpectedCurrentUserResponse {
    pub user_id: Uuid
}

impl<'a> TestUser<'a, LoggedIn> {

    pub async fn refresh(&self) -> TestUser<'a, LoggedIn> {
        let response = self.app.refresh(&self)
            .await
            .json::<ExpectedRefreshResponse>()
            .await
            .expect("Failed to parse response into ExpectedRefreshResponse");

        TestUser {
            user_id: self.user_id.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            state: LoggedIn {
                access_token: Token {
                    token: response.access_token,
                    expiration: response.access_token_expiration,
                },
                refresh_token: Token {
                    token: response.refresh_token,
                    expiration: response.refresh_token_expiration,
                }
            },
            app: self.app,
        }
    }
    
    pub async fn logout(&self) -> TestUser<'a, Anonymous> {
        todo!()
    }

    pub async fn current_user(&self) -> ExpectedCurrentUserResponse {
        self.app.current_user(&self)
            .await
            .json::<ExpectedCurrentUserResponse>()
            .await
            .expect("Failed to parse response to ExpectedCurrentUserResponse")
    }
    
    pub async fn create_team(&self, team_id: Uuid) -> Response {
        self.app.create_team(self, team_id).await
    }

}

#[derive(Deserialize, Debug)]
enum LoginResponses {
    UserLoggedInSuccessfully {
        access_token: String,
        access_token_expiration: DateTime<Utc>,
        refresh_token: String,
        refresh_token_expiration: DateTime<Utc>,
    },
}

