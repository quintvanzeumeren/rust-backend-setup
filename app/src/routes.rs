use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::app_state::AppState;
use crate::handlers::internal::v1::auth::login::login::login;
use crate::handlers::internal::v1::auth::logout::logout::logout;
use crate::handlers::internal::v1::auth::refresh::refresh::refresh;
use crate::handlers::internal::v1::current_user::current_user;
use crate::handlers::v1::health_check::health_check;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/v1/health_check", get(health_check))
        .route("/internal/v1/security/login", post(login))
        .route("/internal/v1/security/refresh", post(refresh))
        .route("/internal/v1/security/logout", post(logout))
        .route("/internal/v1/user/current", get(current_user))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(app_state))
}