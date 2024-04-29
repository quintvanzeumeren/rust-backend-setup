use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use infrastructure::paseto::paseto_token_encryptor::LocalPasetoV4TokenEncryptor;

use crate::configuration::configuration::Configuration;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub encryption_key: SymmetricKey<V4>,
}

impl <'a> AppState {
    pub fn new_token_encryptor(&'a self) -> LocalPasetoV4TokenEncryptor {
        LocalPasetoV4TokenEncryptor {
            symmetrick_key: self.encryption_key.clone()
        }
    }
}

impl TryFrom<Configuration> for AppState {
    type Error = anyhow::Error;

    fn try_from(config: Configuration) -> Result<Self, Self::Error> {
        let pg_pool = PgPoolOptions::new()
            .connect_lazy_with(config.database.with_db());

        return Ok(AppState {
            db: pg_pool,
            encryption_key: config.application.encryption_key()?
        })
    }
}