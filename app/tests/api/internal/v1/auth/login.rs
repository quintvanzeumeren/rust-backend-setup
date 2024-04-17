use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use tracing::log::__private_api::log;
use uuid::Uuid;
use crate::util::spawn_app::{assert_status_eq, spawn_app};

use crate::util::test_user::anonymous::Anonymous;
use crate::util::test_user::test_user::TestUser;

#[sqlx::test]
async fn login_should_give_unprocessable_entity_for_invalid_body(db: PgPool) {
    let test_app = spawn_app(db).await;
    let invalid_payloads = vec![
        ("missing field 'username' and 'hash'", json!({})),
        ("missing field 'hash'", json!({
            "username": "some_username"
        })),
        ("missing field 'username", json!({
            "hash": "only_a_password"
        }))
    ];

    for (test_name, payload) in invalid_payloads {
        let response = test_app.post_login(payload).await;
        assert_status_eq(&response, StatusCode::UNPROCESSABLE_ENTITY, Some(test_name.into()))
    }
}

#[derive(Serialize)]
struct UserCredentials {
    username: String,
    password: String
}

#[sqlx::test]
async fn login_should_reject_invalid_user_credentials(db: PgPool) {
    let test_app = spawn_app(db).await;

    let user_credentials = UserCredentials {
        username: Uuid::new_v4().into(),
        password: Uuid::new_v4().into()
    };
    
    let response = test_app.post_login(json!(user_credentials)).await;
    assert_status_eq(&response, StatusCode::UNAUTHORIZED, None);
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

#[sqlx::test]
async fn login_should_accept_login_credentials(db: PgPool) {
    let test_app = spawn_app(db).await;
    let test_user = test_app.create_test_user().await;

    let user_credentials = UserCredentials {
        username: test_user.username,
        password: test_user.password
    };

    let response = test_app.post_login(json!(user_credentials)).await;
    assert_status_eq(&response, StatusCode::OK, None);

    let _ = response.json::<LoginResponses>().await.expect("Failed to json");
}

#[sqlx::test]
async fn login_should_reject_an_invalid_password(db: PgPool) {
    let test_app = spawn_app(db).await;
    let test_user = test_app.create_test_user().await;

    let incorrect_pw = loop {
        let pw = Uuid::new_v4().to_string();

        if pw != test_user.password {
            break pw
        }
    };

    let user_credentials = UserCredentials {
        username: test_user.username,
        password: incorrect_pw
    };

    let response = test_app.post_login(json!(user_credentials)).await;
    assert_status_eq(&response, StatusCode::UNAUTHORIZED, None);
}

#[sqlx::test]
async fn login_should_reject_invalid_username(db: PgPool) {
    let test_app = spawn_app(db).await;
    let test_user = test_app.create_test_user().await;

    let incorrect_username = loop {
        let username = Uuid::new_v4().to_string();

        if username != test_user.username {
            break username
        }
    };

    let user_credentials = UserCredentials {
        username: incorrect_username,
        password: test_user.password
    };

    let response = test_app.post_login(json!(user_credentials)).await;
    assert_status_eq(&response, StatusCode::UNAUTHORIZED, None);
}


