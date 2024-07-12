use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct TelemetryConfig {
    pub otlp_endpoint: Secret<String>,
    pub dataset_name: String
}