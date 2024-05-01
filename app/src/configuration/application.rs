use pasetors::errors::Error;
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::net::TcpListener;

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationConfig {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub encryption_key: Secret<String>,
}

impl ApplicationConfig {
    pub fn tcp_listener(&self) -> std::io::Result<TcpListener> {
        let address = format!("{}:{}", self.host, self.port);

        let tcp_listener = TcpListener::bind(address)?;

        // the listener should be set to nonblocking for axum to function properly.
        // see: https://github.com/tokio-rs/axum/issues/2459
        tcp_listener.set_nonblocking(true)?;

        return Ok(tcp_listener);
    }

    pub fn encryption_key(&self) -> Result<SymmetricKey<V4>, Error> {
        SymmetricKey::<V4>::from(self.encryption_key.expose_secret().as_bytes())
    }
}
