use domain::sessions::state::refreshed::Refreshed;
use domain::sessions::user_session::UserSession;

use crate::queries::transaction::_transaction::Transaction;

impl Transaction {
    #[tracing::instrument(
    name = "Saving refreshed user session to Postgres",
    skip(self, session)
    )]
    pub async fn save_refreshed_session(
        &mut self,
        session: &UserSession<Refreshed>,
    ) -> Result<(), sqlx::Error> {
        self.save_refresh_token(session.state().new_refresh_token()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use security::token::token::Token;
    use test_utility::random::_common::{random_salt, random_secret};
    use test_utility::random::user::random_user;
    use test_utility::random::user_session::random_newly_created_user_session;

    use crate::queries::database::Database;
    use crate::queries::models::refresh_token_record::RefreshTokenRecord;

    #[sqlx::test]
    async fn test_save_refreshed_session(db: PgPool) {
        let database = Database(db);
        let mut transaction = database.new_transaction().await.expect("Failed to create transaction");

        // create and save a user
        let salt = random_salt();
        let user = random_user(random_secret(), &salt);
        transaction.save_new_user(&user)
            .await
            .expect("Failed to create user");

        // create new session
        let session = random_newly_created_user_session(user.id);
        // save_newly_created_user_session(&mut transaction, &session)
        transaction.save_newly_created_user_session(&session)
            .await
            .expect("Failed to save newly created user session");

        transaction
            .commit()
            .await
            .expect("Failed to commit transaction");

        let mut transaction = database.new_transaction().await.expect("Failed to create transaction");

        // get new session as active session
        let active_session = database.get_active_session_by_id(&session.id())
            .await
            .expect("Failed to get active session by id")
            .expect("Failed to find active session by id");

        assert_eq!(*active_session.id(), *session.id());

        // refresh the session to get a refreshed session
        let refreshed_session = active_session
            .refresh(session.state().refresh_token().clone())
            .expect("Failed to get refreshed session from refresh token");

        transaction.save_refreshed_session(&refreshed_session)
            .await
            .expect("Failed to save refreshed session");

        transaction
            .commit()
            .await
            .expect("Failed to commit transaction");

        // check if the new refresh token is saved
        let token = database
            .get_refresh_token_by_id(refreshed_session.state().new_refresh_token().get_id())
            .await
            .expect("Failed to get just saved refresh token by id")
            .expect("Failed to find refresh token by id");

        let expected = RefreshTokenRecord::from(refreshed_session.state().new_refresh_token());
        let got = RefreshTokenRecord::from(&token);

        assert_eq!(expected, got)
    }
}
