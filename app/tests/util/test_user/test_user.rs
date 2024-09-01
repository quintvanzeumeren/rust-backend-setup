use std::collections::HashSet;
use chrono::{DateTime, Utc};
use reqwest::{Response, StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use domain::team::team_id::TeamId;
use crate::util::spawn_app::assert_status_eq;
use crate::util::test_app::{NewUserBody, TestApp};
use crate::util::test_user::anonymous::Anonymous;
use crate::util::test_user::logged_in::LoggedIn;
use crate::util::test_user::user_state::{Token, UserState};

#[derive(Clone)]
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
    pub async fn login(self) -> TestUser<'a, LoggedIn> {
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
    
    pub async fn create_team(&self) -> Uuid {
        let team_id = Uuid::new_v4();
        let response = self.app.create_team(self, team_id).await;
        assert_status_eq(&response, StatusCode::CREATED, Some("Incorrect statuscode for when a team was created".to_string()));
            
        team_id
    }
    
    pub async fn get_teams(&self) -> HashSet<Uuid> {
        self.app.get_teams(self)
            .await
            .json::<HashSet<Uuid>>()
            .await
            .expect("Failed to parse get_teams result")
    }
    
    pub async fn get_team_members(&self, team_id: Uuid) -> HashSet<Uuid> {
        self.app.get_team_members(self, team_id)
            .await
            .json()
            .await
            .expect("Failed to parse get_team_members result")
    }

    pub async fn get_user_details(&self) -> GetUserDetailsResponse {
        self.get_user_details_of(self.user_id).await
    }

    pub async fn get_user_details_of(&self, user_id: Uuid) -> GetUserDetailsResponse {
        self.app.get_user_details(&self, user_id)
            .await
            .json()
            .await
            .expect("Failed to parse get_user_details result")
    }

    pub async fn create_admin(&self) -> TestUser<'a, LoggedIn> {
        let admin = NewUserBody {
            id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
            roles: vec!["admin".to_string()],
        };
        let response = self.app.create_user(&self, admin.clone()).await;
        assert_status_eq(&response, StatusCode::CREATED, Some("Failed to create admin".to_string()));
        let new_admin = self.app.test_user_from(admin.id, admin.username, admin.password);
        let new_admin = new_admin.login().await;

        new_admin
    }

    pub async fn create_root(&self) -> TestUser<'a, LoggedIn> {
        let admin = NewUserBody {
            id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
            roles: vec!["root".to_string()],
        };
        let response = self.app.create_user(&self, admin.clone()).await;
        assert_status_eq(&response, StatusCode::CREATED, Some("Failed to create admin".to_string()));
        let new_admin = self.app.test_user_from(admin.id, admin.username, admin.password);
        let new_admin = new_admin.login().await;

        new_admin
    }

    pub async fn create_user(&self) -> TestUser<'a, LoggedIn> {
        let admin = NewUserBody {
            id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
            roles: vec![],
        };
        let response = self.app.create_user(&self, admin.clone()).await;
        assert_status_eq(&response, StatusCode::CREATED, Some("Failed to create admin".to_string()));
        let new_admin = self.app.test_user_from(admin.id, admin.username, admin.password);
        let new_admin = new_admin.login().await;

        new_admin
    }
}

#[derive(Deserialize)]
pub struct GetUserDetailsResponse {
    pub id: Uuid,
    pub teams: HashSet<Uuid>,
    pub roles: HashSet<String>
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

