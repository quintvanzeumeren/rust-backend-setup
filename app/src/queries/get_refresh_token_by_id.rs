use sqlx::{PgPool, query_as};
use uuid::Uuid;
use domain::sessions::user_session_token::UserSessionToken;
use domain::sessions::tokens::RefreshToken;
use crate::queries::models::refresh_token_record::RefreshTokenRecord;

#[tracing::instrument(
    name = "Querying Postgres for refresh token for id",
    skip(db, refresh_token_id),
    fields(
        refresh_token_id = % refresh_token_id
    )
)]
pub async fn get_refresh_token_by_id(
    db: &PgPool,
    refresh_token_id: &Uuid,
) -> Result<Option<UserSessionToken<RefreshToken>>, sqlx::Error> {
    let query_result = query_as!(
        RefreshTokenRecord,
        r#"
        SELECT * FROM refresh_tokens
        WHERE refresh_tokens.id = $1
        LIMIT 1
        "#,
        refresh_token_id
    )
        .fetch_optional(db)
        .await?;

    if let Some(token) = query_result {
        return Ok(Some(token.into()))
    }

    Ok(None)
}