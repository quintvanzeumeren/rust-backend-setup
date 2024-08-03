use std::sync::Arc;

use axum::{middleware, Router};
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::app_state::AppState;
use crate::handlers::internal::v1::auth::login::login::login;
use crate::handlers::internal::v1::auth::logout::logout::logout;
use crate::handlers::internal::v1::auth::refresh::refresh::refresh;
use crate::handlers::internal::v1::current_user::current_user;
use crate::handlers::internal::v1::teams::new_team::new_team;
use crate::handlers::internal::v1::users::me::me;
use crate::handlers::v1::health_check::health_check;
use crate::middleware::capture_trace_data::print_request_response;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/v1/health_check", get(health_check))
        .route("/internal/v1/auth/login", post(login))
        .route("/internal/v1/auth/refresh", post(refresh))
        .route("/internal/v1/auth/logout", post(logout))
        .route("/internal/v1/user/current", get(current_user))
        .route("/internal/v1/users/me", get(me))
        .route("/internal/v1/teams", post(new_team))
        .layer(middleware::from_fn(print_request_response))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(app_state))
}