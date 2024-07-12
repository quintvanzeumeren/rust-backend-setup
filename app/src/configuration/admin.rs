use secrecy::Secret;

#[derive(serde::Deserialize, Clone)]
pub struct AdminConfig {
    pub username: Secret<String>,
    pub password: Secret<String>,
}