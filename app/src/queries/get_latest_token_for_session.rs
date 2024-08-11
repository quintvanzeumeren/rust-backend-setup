use sqlx::query_file_as;
use uuid::Uuid;

use domain::sessions::tokens::RefreshToken;
use domain::sessions::user_session_token::UserSessionToken;

use crate::queries::database::Database;
use crate::queries::records::refresh_token_record::RefreshTokenRecord;

impl Database {
    #[tracing::instrument(
    name = "Querying Postgres for latest refresh token by session id",
    skip(self, session_id),
    fields(session_id = % session_id)
    )]
    pub async fn get_latest_token_for_session(
        &self,
        session_id: &Uuid,
    ) -> Result<Option<UserSessionToken<RefreshToken>>, sqlx::Error> {
        let query_result = query_file_as!(
        RefreshTokenRecord,
        "src/queries/get_latest_token_for_session.sql",
        session_id
    )
            .fetch_optional(self.db())
            .await?;

        if let Some(token) = query_result {
            return Ok(Some(token.into()))
        }

        Ok(None)
    }

}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use test_utility::random::_common::{random_salt, random_secret};
    use test_utility::random::refresh_token::random_refresh_token_from;
    use test_utility::random::user::random_user;
    use test_utility::random::user_session::random_newly_created_user_session;

    use crate::queries::database::Database;
    use crate::queries::records::refresh_token_record::RefreshTokenRecord;

    #[sqlx::test]
    async fn test_latest_token_for_session_2(db: PgPool) {
        let db = Database(db);
        let mut transaction = db.new_transaction().await.expect("Failed to create transaction");

        // create and save a user
        let salt = random_salt();
        let user = random_user(random_secret(), &salt);
        transaction.save_new_user(&user)
            .await
            .expect("Failed to create user");

        // create new session
        let session = random_newly_created_user_session(user.id);
        transaction.save_newly_created_user_session(&session)
            .await
            .expect("Failed to save newly created user session");

        transaction.commit().await.expect("Failed to commit transaction");

        let mut latest_refresh_token = session.state().refresh_token().clone();
        for _ in 0..10 {

            let mut transaction = db.new_transaction()
                .await
                .expect("Failed to create transaction");

            println!("\nold: {}", latest_refresh_token.id);

            // get new session
            latest_refresh_token = random_refresh_token_from(&latest_refresh_token);
            transaction.save_refresh_token(&latest_refresh_token)
                .await
                .expect("Failed to save refresh token");

            println!("newest: {}", latest_refresh_token.id);
            transaction.commit().await.expect("Failed to commit new refresh token");

            // get latest refresh token
            let token = db.get_latest_token_for_session(session.id())
                .await
                .expect("Failed to query database to get latest session token")
                .expect("Failed to find latest session token");

            let expected = RefreshTokenRecord::from(&latest_refresh_token);
            let got = RefreshTokenRecord::from(&token);

            println!("expected: {}", expected.id);
            println!("got: {}", got.id);
            assert_eq!(expected, got)
        }

    }

}