use sqlx::{query_file_as};
use tracing::warn;
use uuid::Uuid;
use domain::sessions::state::active::Active;
use domain::sessions::user_session::UserSession;
use crate::queries::database::Database;
use crate::queries::models::user_session_record::UserSessionRecord;


impl Database {
    #[tracing::instrument(
    name = "Querying Postgres for active session by id",
    skip(self, session_id),
    fields(session_id = % session_id)
    )]
    pub async fn get_active_session_by_id(
        &self,
        session_id: &Uuid
    ) -> Result<Option<UserSession<Active>>, sqlx::Error> {
        let query_result = query_file_as!(
            UserSessionRecord,
            "src/queries/get_active_session_by_id.sql",
            session_id
        ).fetch_optional(self.db()).await?;

        if query_result.is_none() {
            return Ok(None)
        }

        let session = query_result.expect("Failed to get session after verifying it was Some");
        if let Some(time) = session.ended_at {
            warn!("Asked for an active session but found session (id: {}) that has ended already (at: {})", session_id, time);
            return Ok(None)
        }

        match self.get_latest_token_for_session(&session_id).await? {
            None => {
                warn!("Expected to find a refresh token for the active session of id: {}, but did not find one", session_id);
                return Ok(None)
            }
            Some(token) => Ok(Some(session.to_active_session(token)))
        }
    }
}
