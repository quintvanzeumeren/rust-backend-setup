use crate::util::spawn_app::{assert_status_eq, spawn_app};
use reqwest::StatusCode;
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

#[sqlx::test]
async fn create_new_team_test(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let team_id = Uuid::new_v4();
    
    let response = app.create_team(&root, team_id).await;
    assert_status_eq(&response, StatusCode::CREATED, None);
    
    let response = app.get_teams(&root).await;
    assert_status_eq(&response, StatusCode::OK, None);

    let teams = response.json::<HashSet<Uuid>>().await
        .expect("Failed to parse get_teams result");

    assert_eq!(teams.len(), 1);
    assert!(teams.contains(&team_id));
}