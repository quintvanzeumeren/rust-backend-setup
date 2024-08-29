use std::sync::Arc;

use axum::{middleware, Router};
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::app_state::AppState;
use crate::handlers::v1::auth::login::login::login;
use crate::handlers::v1::auth::logout::logout::logout;
use crate::handlers::v1::auth::refresh::refresh::refresh;
use crate::handlers::v1::current_user::current_user;
use crate::handlers::v1::teams::get_teams::get_teams;
use crate::handlers::v1::teams::new_team::new_team;
use crate::handlers::v1::teams::users::add_member::add_member;
use crate::handlers::v1::teams::users::get_team_members::get_team_members;
use crate::handlers::v1::users::me::me;
use crate::handlers::v1::health_check::health_check;
use crate::handlers::v1::users::add_user::add_user;
use crate::handlers::v1::users::get_user_details::get_user_details;
use crate::middleware::capture_trace_data::print_request_response;

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/v1/health_check", get(health_check))
        .route("/v1/auth/login", post(login))
        .route("/v1/auth/refresh", post(refresh))
        .route("/v1/auth/logout", post(logout))
        .route("/v1/users/:user_id", get(get_user_details))
        .route("/v1/users", post(add_user))
        .route("/v1/user/current", get(current_user))
        .route("/v1/users/me", get(me))
        .route("/v1/teams", post(new_team))
        .route("/v1/teams", get(get_teams))
        .route("/v1/teams/:team_id/users/:user_id", post(add_member))
        .route("/v1/teams/:team_id/users", get(get_team_members))
        .layer(middleware::from_fn(print_request_response))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(app_state))
}