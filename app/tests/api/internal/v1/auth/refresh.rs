use reqwest::StatusCode;
use sqlx::PgPool;

use crate::util::spawn_app::{assert_status_eq, spawn_app};
use crate::util::test_user::test_user::ExpectedRefreshResponse;

#[sqlx::test]
async fn test_should_receive_refresh_tokens_upon_sending_valid_refresh_token(db: PgPool) {
    let app = spawn_app(db).await;
    let user = app.create_test_user().await;
    let user = user.login().await;

    let response = app.refresh(&user).await;
    assert_status_eq(&response, StatusCode::CREATED, None);

    response.json::<ExpectedRefreshResponse>().await.expect("Failed to parse ExpectedRefreshResponse");
}

#[sqlx::test]
async fn test_access_token_after_refresh(db: PgPool) {
    let app = spawn_app(db).await;
    let user = app.create_test_user().await;
    let user = user.login().await;

    let response = user.current_user().await;
    assert_eq!(response.user_id, user.user_id);

    let refresh_user = user.refresh().await;
    let response = refresh_user.current_user().await;
    assert_eq!(response.user_id, user.user_id);
    assert_eq!(refresh_user.user_id, user.user_id);
}

#[sqlx::test]
async fn test_refresh_token_can_refresh_itself_again(db: PgPool) {
    let app = spawn_app(db).await;
    let user = app.create_test_user().await;
    let mut user = user.login().await;
    
    let initial_id = user.user_id.clone();

    let mut i = 0;
    while i < 10 {
        user = user.refresh().await;
        let response = user.current_user().await;
        assert_eq!(response.user_id, user.user_id);
        assert_eq!(initial_id, user.user_id);
        
        i = i + 1;
    }
}

#[sqlx::test]
async fn test_that_refresh_token_cannot_be_used_twice(db: PgPool) {
    let app = spawn_app(db).await;
    let user = app.create_test_user().await;
    let user = user.login().await;

    // create initial refresh
    let refreshed_user = user.refresh().await;
    let response = refreshed_user.current_user().await;
    assert_eq!(response.user_id, user.user_id);
    
    // refresh with the token a second time
    let response = app.refresh(&user).await;
    assert_status_eq(&response, StatusCode::UNAUTHORIZED, None);

    // check if refresh user will not also no longer work
    let response = app.refresh(&refreshed_user).await;
    assert_status_eq(&response, StatusCode::UNAUTHORIZED, None);
}