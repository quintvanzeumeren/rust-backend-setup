use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use crate::util::spawn_app::{assert_status_eq, spawn_app};

#[sqlx::test]
async fn test_add_user_to_team(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let team_id = root.create_team().await;

    let members = root.get_team_members(team_id).await;
    assert_eq!(members.len(), 0);
    
    let response= app.add_team_member(&root, team_id, root.user_id).await;
    assert_status_eq(&response, StatusCode::OK, None);
    
    let members = root.get_team_members(team_id).await;
    assert_eq!(members.len(), 1);
    assert!(members.contains(&root.user_id))
    
}