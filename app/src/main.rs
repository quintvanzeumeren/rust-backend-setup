use anyhow::Context;
use password_hash::SaltString;
use tracing::info;
use app::app_state::AppState;
use app::configuration::configuration::get_configuration;
use app::database::get_connection_pool;
use app::routes::router;
use app::startup::{create_root_user, migrate, run};
use app::telemetry::{get_subscriber, init_subscriber, init_tracer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup app
    let configuration_directory = std::env::current_dir()
        .context("Failed to determine the current directory")?
        .join("configuration");

    let configuration = get_configuration(configuration_directory)
        .context("Failed to get configuration")?;

    // Setup telemetry & tracing:
    let provider = init_tracer(&configuration.telemetry)
        .context("Failed to initialise provider for open telemetry")?;

    let subscriber = get_subscriber(
        "rust_backend_setup".into(),
        "info".into(),
        std::io::stdout,
        &configuration.telemetry,
        &provider
    );
    init_subscriber(subscriber);

    // Setup database
    let db_pool = get_connection_pool(&configuration.database);

    // Prepare app
    let listener = configuration.application.tcp_listener()
        .context("Failed to get a tcp listener")?;
    
    let app_state = AppState::try_from(configuration.clone())
        .context("Failed to create app_state")?;

    // Migrate database & create initial admin if none exist
    migrate(db_pool).await.context("Failed to migrate db")?;
    let salt = SaltString::generate(&mut rand::thread_rng());
    create_root_user(&app_state.db, &configuration, &salt).await.context("Failed to migrate db")?;

    // start app
    info!("Starting app on {}", listener.local_addr().context("Failed to get local address")?.to_string());
    run(listener, router(app_state)).await.context("Failed to start http server")?;
    Ok(())
}


