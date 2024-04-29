use sqlx::{PgPool, query_as};
use uuid::Uuid;
use lib_domain::sessions::token::UserSessionToken;

use lib_domain::sessions::tokens::RefreshToken;

use crate::queries::models::refresh_token_record::RefreshTokenRecord;

#[tracing::instrument(
name = "Querying Postgres for latest refresh token by session id",
skip(db, session_id),
fields(
session_id = % session_id
)
)]
pub async fn get_latest_token_for_session(
    db: &PgPool,
    session_id: &Uuid,
) -> Result<Option<UserSessionToken<RefreshToken>>, sqlx::Error> {
    let query_result = query_as!(
        RefreshTokenRecord,
        r#"
        SELECT * FROM refresh_tokens
        WHERE refresh_tokens.session_id = $1
        ORDER BY refresh_tokens.issued_at DESC
        LIMIT 1
        "#,
        session_id
    )
        .fetch_optional(db)
        .await?;

    if let Some(token) = query_result {
        return Ok(Some(token.into()))
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use lib_test_util::random::_common::{random_salt, random_secret};
    use lib_test_util::random::refresh_token::random_refresh_token_from;
    use lib_test_util::random::user::random_user;
    use lib_test_util::random::user_session::random_newly_created_user_session;

    use crate::queries::get_latest_token_for_session::get_latest_token_for_session;
    use crate::queries::models::refresh_token_record::RefreshTokenRecord;
    use crate::queries::save_newly_created_user_session::save_newly_created_user_session;
    use crate::queries::save_refresh_token::save_refresh_token;
    use crate::queries::save_user::save_user;

    #[sqlx::test]
    async fn test_latest_token_for_session_2(db: PgPool) {
        let mut transaction = db.begin().await.expect("Failed to create transaction");

        // create and save a user
        let salt = random_salt();
        let user = random_user(random_secret(), &salt);
        save_user(&mut transaction, &user)
            .await
            .expect("Failed to create user");

        // create new session
        let session = random_newly_created_user_session(&user.id);
        save_newly_created_user_session(&mut transaction, &session)
            .await
            .expect("Failed to save newly created user session");

        transaction.commit().await.expect("Failed to commit transaction");

        let mut latest_refresh_token = session.state().refresh_token().clone();
        for i in 0..10 {

            let mut transaction = db.begin()
                .await
                .expect("Failed to create transaction");

            println!("\nold: {}", latest_refresh_token.id);

            // get new session
            latest_refresh_token = random_refresh_token_from(&latest_refresh_token);
            save_refresh_token(&mut transaction, &latest_refresh_token)
                .await
                .expect("Failed to save refresh token");

            println!("newest: {}", latest_refresh_token.id);
            transaction.commit().await.expect("Failed to commit new refresh token");

            // get latest refresh token
            let token = get_latest_token_for_session(&db, session.id())
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