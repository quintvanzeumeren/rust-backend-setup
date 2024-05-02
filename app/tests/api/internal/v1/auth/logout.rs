use reqwest::StatusCode;
use sqlx::PgPool;
use crate::util::spawn_app::{assert_status_eq, spawn_app};

#[sqlx::test]
async fn test_logging_out_will_invalidate_refresh_token(db: PgPool) {
    // Create app and initial user
    let app = spawn_app(db).await;
    let user = app.create_test_user().await;
    let user = user.login().await;
    
    // make sure user can currently refresh its tokens
    let user = user.refresh().await;

    // logout user
    let response = app.logout(&user).await;
    
    // Assert logout went ok
    assert_status_eq(&response, StatusCode::OK, None);
    
    // Assert can no longer refresh tokens
    let response = app.refresh(&user).await;
    assert_status_eq(&response, StatusCode::UNAUTHORIZED, None);
}