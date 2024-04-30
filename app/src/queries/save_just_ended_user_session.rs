use sqlx::{Executor, Postgres, query, Transaction};
use domain::sessions::state::just_ended::JustEnded;
use domain::sessions::user_session::UserSession;
use crate::queries::models::user_session_record::UserSessionRecord;

async fn _save_just_ended_session(transaction: &mut Transaction<'_, Postgres>, user_session_record: UserSessionRecord) -> Result<(), sqlx::Error> {
    transaction.execute(query!(
        r#"
            UPDATE user_sessions SET
                ended_at = $2,
                ending_reason = $3,
                ending_token_id = $4
            WHERE user_sessions.id = $1
        "#,
        user_session_record.id,
        user_session_record.ended_at,
        user_session_record.ending_reason,
        user_session_record.ending_token_id,
    )).await?;

    Ok(())
}

#[tracing::instrument(
name = "Saving just ended user session to Postgres",
skip(transaction, session)
)]
pub async fn save_just_ended_session(transaction: &mut Transaction<'_, Postgres>, session: &UserSession<JustEnded>) -> Result<(), sqlx::Error> {
    _save_just_ended_session(transaction, session.into()).await?;
    Ok(())
}
