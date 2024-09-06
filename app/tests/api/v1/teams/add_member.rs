use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;
use crate::util::spawn_app::{assert_status_eq, spawn_app};
use crate::util::test_app::TestApp;
use crate::util::test_user::logged_in::LoggedIn;
use crate::util::test_user::test_user::TestUser;

#[sqlx::test]
async fn test_root_can_add_himself_to_team(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;

    test_add_to_team(&app, &root, root.user_id, true).await;
}

#[sqlx::test]
async fn test_root_can_add_root_to_team(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let new_member = root.create_root().await;

    test_add_to_team(&app, &root, new_member.user_id, true).await;
}

#[sqlx::test]
async fn test_root_can_add_admin_to_team(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let new_member = root.create_admin().await;

    test_add_to_team(&app, &root, new_member.user_id, true).await;
}

#[sqlx::test]
async fn test_root_can_add_user_to_team(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let new_member = root.create_user().await;

    test_add_to_team(&app, &root, new_member.user_id, true).await;
}

#[sqlx::test]
async fn test_admin_can_add_himself_to_a_team(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let admin = root.create_admin().await;

    test_add_to_team(&app, &admin, admin.user_id, true).await;
}

#[sqlx::test]
async fn test_admin_can_add_user_with_system_role_admin_to_a_team(db: PgPool)  {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let admin = root.create_admin().await;
    let admin_to_add = root.create_admin().await;

    test_add_to_team(&app, &admin, admin_to_add.user_id, true).await;
}

#[sqlx::test]
async fn test_admin_can_add_user_with_system_role_root_to_a_team(db: PgPool)  {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let admin = root.create_admin().await;

    test_add_to_team(&app, &admin, root.user_id, true).await;
}

#[sqlx::test]
async fn test_admin_can_add_user(db: PgPool)  {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let admin = root.create_admin().await;
    let user_to_add = root.create_user().await;

    test_add_to_team(&app, &admin, user_to_add.user_id, true).await;
}

#[sqlx::test]
async fn test_user_cannot_add_anyone(db: PgPool)  {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let admin = root.create_admin().await;
    let user = root.create_user().await;

    test_add_to_team(&app, &user, root.user_id, false).await;
    test_add_to_team(&app, &user, admin.user_id, false).await;
    test_add_to_team(&app, &user, user.user_id, false).await;
}

async fn test_add_to_team(app: &TestApp, user: &TestUser<'_, LoggedIn>, user_id: Uuid, should_succeed: bool) {
    let root = app.get_root_user().await;
    let team_id = root.create_team().await;
    let members = root.get_team_members(team_id).await;
    assert_eq!(members.len(), 0);

    let response= app.add_team_member(user, team_id, user_id).await;
    if should_succeed {
        assert_status_eq(&response, StatusCode::OK, None);
    } else {
        assert_status_eq(&response, StatusCode::FORBIDDEN, None);
    }

    let members = root.get_team_members(team_id).await;
    if should_succeed {
        assert_eq!(members.len(), 1);
        assert!(members.contains(&user_id));
    } else {
        assert!(members.is_empty());
    }

}

async fn test_add_to_team_(app: &TestApp, user: &TestUser<'_, LoggedIn>, user_id: Uuid) {
    let root = app.get_root_user().await;
    let team_id = root.create_team().await;
    let members = root.get_team_members(team_id).await;
    assert_eq!(members.len(), 0);

    let response= app.add_team_member(user, team_id, user_id).await;
    assert_status_eq(&response, StatusCode::OK, None);

    let members = root.get_team_members(team_id).await;
    assert_eq!(members.len(), 1);
    assert!(members.contains(&user_id))
}


