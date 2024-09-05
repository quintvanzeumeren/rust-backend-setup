use crate::queries::transaction::_transaction::Transaction;
use domain::team::member::Member;
use sqlx::{query_file, Executor};

impl Transaction {
    
    #[tracing::instrument(
        name = "Saving new team member",
        skip(self),
        fields (
            team_id = %team_id,
            new_member = %new_member_id
        )
    )]
    pub async fn save_team_member(&mut self, member: Member) -> sqlx::Result<()> {
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