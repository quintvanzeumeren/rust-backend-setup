use anyhow::Context;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::handlers::v1::auth::authentication_error::AuthenticationError;

impl AuthenticatedUser {
    pub async fn logout(self) -> Result<(), AuthenticationError> {
        let session = self.state.db
            .get_active_session_by_id(&self.session_id)
            .await
            .context("Failed to query database to get active session")?
            .ok_or(AuthenticationError::SessionNotActive)?
            .end_by_user_logout();

        let mut transaction = self.state.db.new_transaction()
            .await
            .expect("Failed to begin a transaction to store updated user session");

        transaction.save_just_ended_session(&session)
            .await
            .context("Failed to save updated user session to database")?;

        transaction.commit()
            .await
            .context("Failed to commit transaction containing updated database")?;

        Ok(())
    }
}
