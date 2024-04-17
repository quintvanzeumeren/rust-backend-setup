use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;

use crate::configuration::configuration::Configuration;
use crate::handlers::internal::v1::auth::login::login::login;
use crate::handlers::internal::v1::auth::logout::logout::logout;
use crate::handlers::internal::v1::auth::refresh::refresh::refresh;
use crate::handlers::internal::v1::current_user::current_user;
use crate::handlers::v1::health_check::health_check;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub encryption_key: SymmetricKey<V4>,
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

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/v1/health_check", get(health_check))
        .route("/internal/v1/auth/login", post(login))
        .route("/internal/v1/auth/refresh", post(refresh))
        .route("/internal/v1/auth/logout", post(logout))
        .route("/internal/v1/user/current", get(current_user))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(app_state))
}