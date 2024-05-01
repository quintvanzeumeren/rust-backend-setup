use sqlx::query_file_as;
use uuid::Uuid;
use domain::sessions::user_session_token::UserSessionToken;
use domain::sessions::tokens::RefreshToken;
use crate::queries::database::Database;
use crate::queries::models::refresh_token_record::RefreshTokenRecord;


impl Database {
    #[tracing::instrument(
    name = "Querying Postgres for refresh token for id",
    skip(self, refresh_token_id),
    fields(
    refresh_token_id = % refresh_token_id
    )
    )]
    pub async fn get_refresh_token_by_id(
        &self,
        refresh_token_id: &Uuid,
    ) -> Result<Option<UserSessionToken<RefreshToken>>, sqlx::Error> {
        let query_result = query_file_as!(
            RefreshTokenRecord,
            "src/queries/get_refresh_token_by_id.sql",
            refresh_token_id
        ).fetch_optional(self.db()).await?;

        if let Some(token) = query_result {
            return Ok(Some(token.into()))
        }

        Ok(None)
    }
}