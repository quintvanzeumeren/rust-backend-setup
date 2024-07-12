use anyhow::Context;
use axum::Router;
use password_hash::SaltString;
use secrecy::ExposeSecret;
use sqlx::migrate::MigrateError;
use sqlx::PgPool;
use tokio::net::TcpListener;
use uuid::Uuid;
use domain::user::password::Password;
use domain::user::user::{Admin, User};
use crate::configuration::configuration::Configuration;
use crate::queries::database::Database;

pub async fn migrate(db: PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(&db)
        .await
}

pub async fn create_user_if_no_users(db: &Database, config: &Configuration, salt_string: &SaltString) -> anyhow::Result<()> {
    let user = db.get_user_credentials(&config.admin.username.expose_secret())
        .await
        .context("Failed to get user")?;

    if user.is_some() {
        return Ok(())
    }

    let admin: User<Admin> = User::new(
        Uuid::new_v4().into(),
        config.admin.username.expose_secret().to_string(),
        Password::new(config.admin.password.clone(), salt_string)
            .context("Could not parse and hash admin password")?,
    );

    let mut transaction = db.new_transaction().await.context("Failed to start transaction")?;
    transaction.persist_new_admin(&admin).await.context("Failed to insert initial user")?;
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