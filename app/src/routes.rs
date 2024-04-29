use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use lib_auth::security::token::token::Token;
use lib_auth::security::token::token_encryptor::TokenEncryptor;
use lib_domain::sessions::tokens::RefreshToken;
use crate::app_state::AppState;

use crate::configuration::configuration::Configuration;
use crate::handlers::internal::v1::auth::login::login::login;
use crate::handlers::internal::v1::auth::logout::logout::logout;
use crate::handlers::internal::v1::auth::refresh::refresh::refresh;
use crate::handlers::internal::v1::current_user::current_user;
use crate::handlers::v1::health_check::health_check;

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