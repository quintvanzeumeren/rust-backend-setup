use reqwest::StatusCode;
use sqlx::PgPool;
use crate::util::spawn_app::{assert_status_eq, spawn_app};

#[sqlx::test]
async fn health_check_should_return_ok_when_app_running(db: PgPool) {
    let test_app = spawn_app(db).await;
    let response = test_app.get_health().await;

    assert_status_eq(&response, StatusCode::OK, None)
}
