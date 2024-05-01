use anyhow::Context;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;
use crate::queries::get_active_session_by_id::get_active_session_by_id;
use crate::queries::save_just_ended_user_session::save_just_ended_session;

impl AuthenticatedUser {
    pub async fn logout(self) -> Result<(), AuthenticationError> {

        let session = get_active_session_by_id(&self.state.db, &self.session_id)
            .await
            .context("Failed to query database to get active session")?
            .ok_or(AuthenticationError::SessionNotActive)?
            .end_duo_user_by_logout();

        let mut transaction = self.state.db.begin()
            .await
            .expect("Failed to begin a transaction to store updated user session");

        save_just_ended_session(&mut transaction, &session)
            .await
            .context("Failed to save updated user session to database")?;

        transaction.commit().await.context("Failed to commit transaction containing updated database")?;

        Ok(())
    }
}
