use anyhow::Context;
use app::configuration::configuration::get_configuration;
use app::database::get_connection_pool;
use app::routes::{AppState, router};
use app::startup::{migrate, run};
use app::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup tracing
    let subscriber = get_subscriber("rust_backend_setup".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Setup app
    let configuration_directory = std::env::current_dir()
        .context("Failed to determine the current directory")?
        .join("configuration");
    
    let configuration = get_configuration(configuration_directory)
        .context("Failed to get configuration")?;
    
    let db_pool = get_connection_pool(&configuration.database);
    
    let listener = configuration.application.tcp_listener()
        .context("Failed to get a tcp listener")?;
    
    let app_state = AppState::try_from(configuration)
        .context("Failed to create app_state")?;

    // Start app
    migrate(db_pool).await.context("Failed to migrate db")?;
    run(listener, router(app_state)).await.context("Failed to start http server")?;
    Ok(())
}


