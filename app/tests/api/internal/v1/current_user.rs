use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::spawn_app::{assert_status_eq, spawn_app};

#[derive(Deserialize)]
struct CurrentUser {
    user_id: Uuid
}

#[sqlx::test]
async fn test_get_current_user_after_login(db: PgPool) {
    let app = spawn_app(db).await;
    let user = app.create_test_user().await;
    let user = user.login().await;

    let response = app.current_user(&user).await;

    assert_status_eq(&response, StatusCode::OK, None);
    let current_user = response.json::<CurrentUser>().await
        .expect("Failed to parse current user response");
    assert_eq!(user.user_id, current_user.user_id, "current user id is different from current user id")
 }