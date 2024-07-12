use std::path::Path;
use once_cell::sync::Lazy;
use pasetors::keys::{Generate, SymmetricKey};
use pasetors::version4::V4;
use reqwest::{Response, StatusCode};
use secrecy::Secret;
use sqlx::PgPool;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use app::app_state::AppState;

use app::configuration::configuration::get_configuration;
use app::configuration::telemetry::TelemetryConfig;
use app::queries::database::Database;
use app::routes::{router};
use app::telemetry::{get_subscriber, init_subscriber, init_tracer};
use crate::util::api_client::ApiClient;
use crate::util::test_app::{AbortOnDrop, TestApp};


static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_test_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_test_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub fn get_test_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
// This "weird" syntax is a higher-ranked trait bound (HRTB)
// It basically means that Sink implements the `MakeWriter`
// trait for all choices of the lifetime parameter `'a`
// Check out https://doc.rust-lang.org/nomicon/hrtb.html
// for more details.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name.into(), sink);

    return Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub async fn spawn_app(db: PgPool) -> TestApp {
    // Start logging
    Lazy::force(&TRACING);

    // Setup app
    let configuration = {
        let mut current_dir = std::env::current_dir().expect("Failed to determine the current directory");
        current_dir.pop();

        // For whatever reason current_dir.join(...) stopped working properly,
        // seemed to replace the current_dir with just /configuration
        // println!("{}", format!("{}/configuration", current_dir.to_str().unwrap().to_string()).as_str());
        let configuration_path = format!("{}/configuration", current_dir.to_str().unwrap().to_string());
        let configuration_dir = Path::new(configuration_path.as_str());

        let mut config = get_configuration(configuration_dir.to_path_buf())
            .expect("Failed to read configuration");

        // By setting application port to 0, the http server will
        // serve on a random port
        config.application.port = 0;
        config
    };

    let listener = configuration
        .application
        .tcp_listener()
        .expect("Failed to create tcp listener");
    
    let app_port = listener.local_addr().unwrap().port();
    
    let app_db = db.clone();
    let _server = AbortOnDrop(tokio::spawn(async move {
        let app = router(
            // AppState::try_from(app_config).expect("Failed to build AppState")
            AppState {
                db: Database(app_db),
                encryption_key: SymmetricKey::<V4>::generate()
                    .expect("Failed to random encryption key")
            }
        );
        let listener = tokio::net::TcpListener::from_std(listener)
            .expect("Failed to get tcp listener from tokio");

        axum::serve(listener, app).await.expect("Failed to serve axum server");
    }));

    let address = format!("http://localhost:{}", app_port);
    let test_app = TestApp {
        address: address.clone(),
        api_client: ApiClient {
            app_address: address.clone()
        },
        port: app_port,
        pg_pool: db,

        // server must be saved in order for the task that starts,
        // the http server (axum) to keep running, or its lifetime gets dropped.
        // the server aborts when TestApp goes out of scope
        _server
    };

    return test_app
}
pub fn assert_status_eq(
    response: &Response,
    status_code: StatusCode,
    description: Option<String>
) {
    match description {
        None => {
            assert_eq!(response.status().as_u16(), status_code.as_u16())
        }
        Some(std) => {
            assert_eq!(response.status().as_u16(), status_code.as_u16(), "{}", std)
        }
    }
}
