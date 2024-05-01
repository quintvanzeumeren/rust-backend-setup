use sqlx::{Executor, query, query_file};

use domain::sessions::state::just_ended::JustEnded;
use domain::sessions::user_session::UserSession;

use crate::queries::models::user_session_record::UserSessionRecord;
use crate::queries::transaction::_transaction::Transaction;

impl Transaction {
    async fn _save_just_ended_session(&mut self, user_session_record: UserSessionRecord) -> Result<(), sqlx::Error> {
        self.0.execute(
            query_file!("src/queries/transaction/save_just_ended_user_session.sql",
            user_session_record.id,
            user_session_record.ended_at,
            user_session_record.ending_reason,
            user_session_record.ending_token_id,
        )).await?;

        Ok(())
    }

    #[tracing::instrument(
    name = "Saving just ended user session to Postgres",
    skip(self, session)
    )]
    pub async fn save_just_ended_session(&mut self, session: &UserSession<JustEnded>) -> Result<(), sqlx::Error> {
        self._save_just_ended_session(session.into()).await?;
        Ok(())
    }
}
