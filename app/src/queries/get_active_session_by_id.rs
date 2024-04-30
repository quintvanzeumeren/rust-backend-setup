use sqlx::{PgPool, query_as};
use tracing::warn;
use uuid::Uuid;
use domain::sessions::state::active::Active;
use domain::sessions::user_session::UserSession;
use crate::queries::get_latest_token_for_session::get_latest_token_for_session;
use crate::queries::models::user_session_record::UserSessionRecord;

#[tracing::instrument(
name = "Querying Postgres for active session by id",
skip(db, session_id),
fields(
    session_id = % session_id
)
)]
pub async fn get_active_session_by_id(db: &PgPool, session_id: &Uuid) -> Result<Option<UserSession<Active>>, sqlx::Error> {
    let query_result = query_as!(
        UserSessionRecord,
        r#"
        SELECT * FROM user_sessions
        WHERE user_sessions.id = $1
        "#, session_id)
        .fetch_optional(db)
        .await?;

    if query_result.is_none() {
        return Ok(None)
    }

    let session = query_result.expect("Failed to get session after verifying it was Some");
    if let Some(time) = session.ended_at {
        warn!("Asked for an active session but found session (id: {}) that has ended already (at: {})", session_id, time);
        return Ok(None)
    }

    match get_latest_token_for_session(db, &session_id).await? {
        None => {
            warn!("Expected to find a refresh token for the active session of id: {}, but did not find one", session_id);
            return Ok(None)
        }
        Some(token) => Ok(Some(session.to_active_session(token)))
    }
}

// #[cfg(test)]
