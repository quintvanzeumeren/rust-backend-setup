use sqlx::{Executor, Postgres, Transaction};
use lib_domain::sessions::token::UserSessionToken;
use lib_domain::sessions::tokens::RefreshToken;
use crate::queries::models::refresh_token_record::RefreshTokenRecord;

#[tracing::instrument(
    name = "Saving refresh token to Postgres",
    skip(transaction, refresh_token)
)]
pub async fn save_refresh_token(
    transaction: &mut Transaction<'_, Postgres>,
    refresh_token: &UserSessionToken<RefreshToken>
) -> sqlx::Result<()> {
    let refresh_token_record = RefreshTokenRecord::from(refresh_token);
    transaction.execute(sqlx::query!(
        r#"
        INSERT INTO refresh_tokens (id, session_id, user_id, parent_id, issued_at, not_before, expiration)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        refresh_token_record.id,
        refresh_token_record.session_id,
        refresh_token_record.user_id,
        refresh_token_record.parent_id,
        refresh_token_record.issued_at,
        refresh_token_record.not_before,
        refresh_token_record.expiration,
    )).await?;
    
    Ok(())
}