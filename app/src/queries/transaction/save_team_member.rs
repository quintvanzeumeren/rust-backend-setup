use crate::queries::transaction::_transaction::Transaction;
use domain::team::member::Member;
use sqlx::{query_file, Executor};
use crate::telemetry::TelemetryRecord;

impl Transaction {
    
    #[tracing::instrument(
        name = "Saving new team member",
        skip(self),
        fields (
            new_member_id = tracing::field::Empty,
            team_id = tracing::field::Empty,
            manager = tracing::field::Empty
        )
    )]
    pub async fn save_team_member(&mut self, member: Member) -> sqlx::Result<()> {
        member.user_id.record_in_telemetry("new_member_id");
        member.team_id.record_in_telemetry("team_id");
        member.manager.record_in_telemetry("manager");
        
        let query = query_file!(
            "src/queries/transaction/save_team_member.sql",
            member.user_id.0,
            member.team_id.0,
            member.manager,
        );
        
        self.0.execute(query).await?;
        
        Ok(())
    }
}