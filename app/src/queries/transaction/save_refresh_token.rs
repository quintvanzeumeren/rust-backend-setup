use crate::queries::records::refresh_token_record::RefreshTokenRecord;
use domain::sessions::tokens::RefreshToken;
use domain::sessions::user_session_token::UserSessionToken;
use sqlx::{Executor};
use crate::queries::transaction::_transaction::Transaction;

impl Transaction {
    #[tracing::instrument(
    name = "Saving refresh token to Postgres",
    skip(self, refresh_token)
    )]
    pub async fn save_refresh_token(
        &mut self,
        refresh_token: &UserSessionToken<RefreshToken>,
    ) -> sqlx::Result<()> {
        let refresh_token_record = RefreshTokenRecord::from(refresh_token);
        self.0.execute(sqlx::query_file!(
            "src/queries/transaction/save_refresh_token.sql",
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
}
