use sqlx::{Executor, query_file};

use domain::sessions::state::newly_created::NewlyCreated;
use domain::sessions::user_session::UserSession;

use crate::queries::models::user_session_record::UserSessionRecord;
use crate::queries::transaction::_transaction::Transaction;


impl Transaction {
    #[tracing::instrument(
    name = "Saving newly created user session to Postgres",
    skip(self, session)
    )]
    pub async fn save_newly_created_user_session(
        &mut self,
        session: &UserSession<NewlyCreated>,
    ) -> Result<(), sqlx::Error> {
        let session_record = UserSessionRecord::from(session);
        self.0.execute(query_file!("src/queries/transaction/save_newly_created_user_session.sql",
            session_record.id,
            session_record.user_id,
            session_record.created_at
        )).await?;

        self.save_refresh_token(session.state().refresh_token()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{query_as, PgPool};

    use test_utility::random::_common::{random_salt, random_secret};
    use test_utility::random::user::random_user;
    use test_utility::random::user_session::random_newly_created_user_session;
    use crate::queries::database::Database;

    use crate::queries::models::refresh_token_record::RefreshTokenRecord;
    use crate::queries::models::user_session_record::UserSessionRecord;

    #[sqlx::test]
    async fn test_save_new_session(db: PgPool) {
        let database = Database(db);
        let mut transaction = database.new_transaction().await.expect("Failed to create transaction");
        let salt = random_salt();
        let user = random_user(random_secret(), &salt);
        transaction.save_user(&user).await
            .expect("Failed to save user");

        let session = random_newly_created_user_session(&user.id);
        transaction.save_newly_created_user_session(&session).await
            .expect("Failed to save session");

        transaction.commit().await
            .expect("Failed to commit transaction");

        let expected = UserSessionRecord::from(&session);
        let got = query_as!(
            UserSessionRecord,
            r#"
                SELECT * FROM user_sessions
                WHERE user_sessions.id = $1
                LIMIT 1
            "#,
            session.id().clone()
        )
        .fetch_one(database.db())
        .await
        .expect("Failed to find saved user session");

        assert_eq!(expected, got);

        let expected = RefreshTokenRecord::from(session.state().refresh_token());
        let got = query_as!(
            RefreshTokenRecord,
            r#"
                SELECT * FROM refresh_tokens
                WHERE refresh_tokens.id = $1
                LIMIT 1
            "#,
            session.state().refresh_token().id.clone()
        )
        .fetch_one(database.db())
        .await
        .expect("Failed to find saved user session");

        assert_eq!(expected, got);
    }
}
