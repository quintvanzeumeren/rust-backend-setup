use axum::Router;
use sqlx::migrate::MigrateError;
use sqlx::PgPool;
use tokio::net::TcpListener;

pub async fn migrate(db: PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(&db)
        .await
}

pub async fn run(
    std_listener: std::net::TcpListener,
    router: Router
) -> anyhow::Result<()> {

    let listener = TcpListener::from_std(std_listener)?;
    axum::serve(listener, router).await?;

    Ok(())
}