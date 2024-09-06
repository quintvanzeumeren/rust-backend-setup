use crate::util::spawn_app::{assert_status_eq, spawn_app};
use crate::util::test_app::NewUserBody;
use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

fn create_new_user_body(system_role: Option<&'static str>) -> NewUserBody {
    NewUserBody {
        id: Uuid::new_v4(),
        username: Uuid::new_v4().to_string(),
        password: Uuid::new_v4().to_string(),
        role: system_role,
    }
}

fn create_new_user_body_root() -> NewUserBody {
    create_new_user_body(Some("Root"))
}

fn create_new_user_body_admin() -> NewUserBody {
    create_new_user_body(Some("Admin"))
}

#[sqlx::test]
async fn test_root_can_create_root_user(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;

    let new_root = create_new_user_body_root();
    let response = app.create_user(&root, new_root.clone()).await;
    assert_status_eq(&response, StatusCode::CREATED, None);
    
    let new_root = app.test_user_from(new_root.id, new_root.username, new_root.password);
    let new_root = new_root.login().await;
    let details = new_root.get_user_details().await;
    assert_eq!(details.system_role, Some("Root".to_string()));
}

#[sqlx::test]
async fn test_root_can_create_admin_user(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;

    let new_admin = create_new_user_body_admin();
    let response = app.create_user(&root, new_admin.clone()).await;
    assert_status_eq(&response, StatusCode::CREATED, None);
    
    let new_admin = app.test_user_from(new_admin.id, new_admin.username, new_admin.password);
    let new_admin = new_admin.login().await;
    let details = new_admin.get_user_details().await;
    assert_eq!(details.system_role, Some("Admin".to_string()));
}

#[sqlx::test]
async fn test_root_can_create_user_without_roles(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;

    let user = create_new_user_body(None);
    let response = app.create_user(&root, user.clone()).await;
    assert_status_eq(&response, StatusCode::CREATED, None);
    
    let user = app.test_user_from(user.id, user.username, user.password);
    let user = user.login().await;
    let details = user.get_user_details().await;
    assert!(details.system_role.is_none());
}

#[sqlx::test]
async fn test_admin_cannot_create_root_user(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let admin = root.create_admin().await;

    let new_root = create_new_user_body_root();
    let response = app.create_user(&admin, new_root.clone()).await;
    assert_status_eq(&response, StatusCode::FORBIDDEN, None);
    
    let response = app.get_user_details(&root, new_root.id).await;
    assert_status_eq(&response, StatusCode::NOT_FOUND, Some("Expected to receive 404 when retrieving root that doesn't exist".to_string()));
}

#[sqlx::test]
async fn test_admin_can_create_admin_user(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let admin = root.create_admin().await;

    let new_admin = create_new_user_body_admin();
    let response = app.create_user(&admin, new_admin.clone()).await;
    assert_status_eq(&response, StatusCode::CREATED, None);

    let user = app.test_user_from(new_admin.id, new_admin.username, new_admin.password);
    let user = user.login().await;
    let details = user.get_user_details().await;
    assert_eq!(details.system_role, Some("Admin".to_string()));
}

#[sqlx::test]
async fn test_admin_can_create_user_without_roles(db: PgPool) {
    let app = spawn_app(db).await;
    let root = app.get_root_user().await;
    let admin = root.create_admin().await;

    // test creation for new role-less user.
    let user = create_new_user_body(None);
    let response = app.create_user(&admin, user.clone()).await;
    assert_status_eq(&response, StatusCode::CREATED, None);
    
    let user = app.test_user_from(user.id, user.username, user.password);
    let user = user.login().await;
    let details = user.get_user_details().await;
    assert!(details.system_role.is_none());
}
