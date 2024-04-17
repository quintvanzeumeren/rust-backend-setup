use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::configuration::database::DatabaseConfig;

pub fn get_connection_pool(db_config: &DatabaseConfig) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(db_config.with_db())
}