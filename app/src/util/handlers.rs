use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;
use tracing::Span;

#[derive(Serialize)]
pub struct InternalErrorResponse {
    pub request_id: Option<u64>
}

impl From<tracing::Span> for InternalErrorResponse {
    fn from(value: Span) -> Self {
        match value.id() {
            None => InternalErrorResponse {
                request_id: None
            },
            Some(id) => InternalErrorResponse {
                request_id: Some(id.into_u64())
            }
        }
    }
}

impl IntoResponse for InternalErrorResponse {
    fn into_response(self) -> Response {
        // TODO: figure out how to long trace id and potential open telemetry setup
        // https://www.shuttle.rs/blog/2024/01/09/getting-started-tracing-rust
        match self.request_id {
            None => {
                println!("no trace id");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
            Some(id) => {
                println!("should have trace id: {}", id.to_string());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "request_id": id,
                    }))
                ).into_response()
            }
        }
    }
}