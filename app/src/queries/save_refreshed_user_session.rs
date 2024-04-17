use sqlx::{Postgres, Transaction};

use lib_domain::sessions::state::refreshed::Refreshed;
use lib_domain::sessions::user_session::UserSession;

use crate::queries::save_refresh_token::save_refresh_token;

#[tracing::instrument(
name = "Saving refreshed user session to Postgres",
skip(transaction, session)
)]
pub async fn save_refreshed_session(transaction: &mut Transaction<'_, Postgres>, session: &UserSession<Refreshed>) -> Result<(), sqlx::Error> {
    save_refresh_token(transaction, session.state().new_refresh_token()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use lib_auth::security::token::token::Token;
    use lib_test_util::random::_common::{random_salt, random_secret};
    use lib_test_util::random::user::random_user;
    use lib_test_util::random::user_session::random_newly_created_user_session;

    use crate::queries::get_active_session_by_id::get_active_session_by_id;
    use crate::queries::get_refresh_token_by_id::get_refresh_token_by_id;
    use crate::queries::models::refresh_token_record::RefreshTokenRecord;
    use crate::queries::save_newly_created_user_session::save_newly_created_user_session;
    use crate::queries::save_refreshed_user_session::save_refreshed_session;
    use crate::queries::save_user::save_user;

    #[sqlx::test]
    async fn test_save_refreshed_session(db: PgPool) {
        let mut transaction = db.begin().await.expect("Failed to create transaction");

        // create and save a user
        let salt = random_salt();
        let user = random_user(random_secret(), &salt);
        save_user(&mut transaction, &user).await.expect("Failed to create user");

        // create new session
        let session = random_newly_created_user_session(&user.id);
        save_newly_created_user_session(&mut transaction, &session)
            .await
            .expect("Failed to save newly created user session");

        transaction.commit().await.expect("Failed to commit transaction");
        let mut transaction = db.begin().await.expect("Failed to create transaction");

        // get new session as active session
        let active_session = get_active_session_by_id(&db, &session.id())
            .await
            .expect("Failed to get active session by id")
            .expect("Failed to find active session by id");

        assert_eq!(*active_session.id(), *session.id());

        // refresh the session to get a refreshed session
        let refreshed_session = active_session.refresh(session.state().refresh_token().clone())
            .expect("Failed to get refreshed session from refresh token");

        save_refreshed_session(&mut transaction, &refreshed_session)
            .await
            .expect("Failed to save refreshed session");

        transaction.commit().await.expect("Failed to commit transaction");

        // check if the new refresh token is saved
        let token = get_refresh_token_by_id(&db, refreshed_session.state().new_refresh_token().get_id())
            .await
            .expect("Failed to get just saved refresh token by id")
            .expect("Failed to find refresh token by id");

        let expected = RefreshTokenRecord::from(refreshed_session.state().new_refresh_token());
        let got = RefreshTokenRecord::from(&token);

        assert_eq!(expected, got)
    }
}

