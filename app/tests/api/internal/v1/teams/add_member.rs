use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use crate::util::spawn_app::{assert_status_eq, spawn_app};

#[sqlx::test]
async fn test_add_user_to_team(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let team_id = root.create_team().await;
    
    let response= app.add_team_member(&root, team_id, root.user_id).await;
    assert_status_eq(&response, StatusCode::OK, None);
    
    // todo verify user is added to team of users
    // get: internal/api/v1/teams/{team_id}/users
}