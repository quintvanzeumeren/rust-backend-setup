use anyhow::Context;
use axum::Router;
use password_hash::SaltString;
use secrecy::ExposeSecret;
use sqlx::migrate::MigrateError;
use sqlx::PgPool;
use tokio::net::TcpListener;
use uuid::Uuid;
use domain::role::role::{Role, ROLE_ROOT};
use domain::user::password::Password;
use domain::user::user::User;
use crate::configuration::configuration::Configuration;
use crate::queries::database::Database;

pub async fn migrate(db: PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(&db)
        .await
}

pub async fn create_root_user(db: &Database, config: &Configuration, salt_string: &SaltString) -> anyhow::Result<()> {
    let user = db.get_user_credentials(&config.admin.username.expose_secret())
        .await
        .context("Failed to get user")?;

    if user.is_some() {
        return Ok(())
    }

    let new_root = User {
        id: Uuid::new_v4().into(),
        username: config.admin.username.expose_secret().to_string(),
        password: Password::new(config.admin.password.clone(), salt_string)
            .context("Could not parse and hash admin password")?,
    };

    let mut transaction = db.new_transaction().await.context("Failed to start transaction")?;
    transaction.save_new_user(&new_root).await
        .context("Failed to insert initial user")?;

    transaction.save_new_user_role(new_root.id, &Role::Root).await
        .context("Failed to add role of root to the new root user")?;

    transaction.commit().await.context("Failed to commit transaction")?;
    Ok(())
}

pub async fn run(
    std_listener: std::net::TcpListener,
    router: Router
) -> anyhow::Result<()> {

    let listener = TcpListener::from_std(std_listener)?;
    axum::serve(listener, router).await?;

    Ok(())
}