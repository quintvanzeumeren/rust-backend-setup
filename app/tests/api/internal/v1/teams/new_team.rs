use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use crate::util::spawn_app::{assert_status_eq, spawn_app};

#[sqlx::test]
async fn create_new_team_test(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let team_id = Uuid::new_v4();
    
    let response = app.create_team(&root, team_id).await;
    assert_status_eq(&response, StatusCode::CREATED, None);
    
    // todo verify if exist
}

// todo test duplicate team ids

// #[derive(Deserialize)]
// pub struct ExpectedNewTeamResponse {
//     pub team_id: Uuid
// }