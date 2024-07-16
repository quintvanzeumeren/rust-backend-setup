use password_hash::SaltString;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::Response;
use secrecy::Secret;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;
use security::hash::scheme::{get_latest_scheme, Scheme};
use crate::util::api_client::ApiClient;
use crate::util::test_user::anonymous::Anonymous;
use crate::util::test_user::logged_in::LoggedIn;
use crate::util::test_user::test_user::TestUser;
use crate::util::test_user::user_state::UserState;


pub struct AbortOnDrop(pub tokio::task::JoinHandle<()>);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort()
    }
}

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub pg_pool: PgPool,
    pub api_client: ApiClient,

    pub _server: AbortOnDrop,
}

impl TestApp {
    pub async fn create_test_user(&self) -> TestUser<Anonymous> {
        let user_id = Uuid::new_v4();
        let username = Uuid::new_v4().to_string();
        let password = Uuid::new_v4().to_string();

        let salt = SaltString::generate(&mut rand::thread_rng());
        let hash_scheme = get_latest_scheme();
        let hashed_password = hash_scheme
            .hash(Secret::new(password.clone()), &salt)
            .expect("Failed to hash TestUser's password")
            .to_string();

        let _ = sqlx::query!(
            r#"
            INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            username,
            hashed_password,
        )
            .execute(&self.pg_pool)
            .await
            .expect("Failed to store test user.");

        TestUser {
            user_id,
            username,
            password,
            state: Anonymous {},
            app: &self
            // api_client: ApiClient {
            //     app_address: test_app.address.clone()
            // },
        }
    }
}


// User Endpoints
impl TestApp {

    pub async fn login(&self, user: &TestUser<'_, Anonymous>) -> Response {
        self.api_client
            .post("/internal/v1/auth/login")
            .json(&json!({
                "username": user.username,
                "password": user.password
            }))
            .send()
            .await
            .expect("Failed to send logic requests")
    }

    pub async fn current_user(&self, user: &TestUser<'_, LoggedIn>) -> Response {
        self.api_client
            .get("/internal/v1/user/current")
            .headers(self.auth_header(user))
            .send()
            .await
            .expect("Failed to send current user request")
    }

    pub async fn refresh(&self, user: &TestUser<'_, LoggedIn>) -> Response {
        self.api_client
            .post("/internal/v1/auth/refresh")
            .json(&json!({
                "refresh_token": user.state.refresh_token.token
            }))
            .send()
            .await
            .expect("Failed to send refresh request")
    }

    pub async fn logout(&self, user: &TestUser<'_, LoggedIn>) -> Response {
        self.api_client
            .post("/internal/v1/auth/logout")
            .headers(self.auth_header(user))
            .send()
            .await
            .expect("Failed to send logout request")
    }

    fn auth_header<T: UserState + Clone>(&self, user: &TestUser<T>) -> HeaderMap {
        match user.state.access_token() {
            None => HeaderMap::new(),
            Some(access_token) => {
                let mut header = HeaderMap::new();
                header.insert(
                    AUTHORIZATION, HeaderValue::from_str(
                        format!("Bearer {}", access_token.token).as_str()
                    ).expect("Failed to create header value for access token")
                );

                header
            }
        }
    }
}

impl TestApp {
    pub async fn get_health(self) -> Response {
        self.api_client
            .get("/v1/health_check")
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login(&self, body: Value) -> Response {
        self.api_client
            .post("/internal/v1/auth/login")
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}