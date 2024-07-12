use std::path::PathBuf;
use serde::Deserialize;
use crate::configuration::admin::AdminConfig;
use crate::configuration::application::ApplicationConfig;
use crate::configuration::database::DatabaseConfig;
use crate::configuration::telemetry::TelemetryConfig;

#[derive(Deserialize, Clone)]
pub struct Configuration {
    pub application: ApplicationConfig,
    pub database: DatabaseConfig,
    pub admin: AdminConfig,
    // environment: Environment
    pub telemetry: TelemetryConfig
}

/// APP_ENVIRONMENT is the name of the environment variable used to determine the running environment.
const APP_ENVIRONMENT: &'static str = "APP_ENVIRONMENT";

/// BASE_CONFIGURATION is the name of the base configuration file.
/// It contains shared configurations across all environments.
/// e.g. the name of the database, the name of the application, etc.
const BASE_CONFIGURATION: &'static str = "base.yaml";

pub fn get_configuration(configuration_directory: PathBuf) -> Result<Configuration, config::ConfigError> {
    // let base_path = std::env::current_dir()
    //     .expect("Failed to determine the current directory");
    // 
    // let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var(APP_ENVIRONMENT)
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect(format!("Failed to parse {}.", APP_ENVIRONMENT).as_str());

    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join(BASE_CONFIGURATION),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Configuration>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}