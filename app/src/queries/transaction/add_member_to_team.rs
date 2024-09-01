use sqlx::{Executor, query_file};
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use crate::queries::transaction::_transaction::Transaction;

impl Transaction {
    
    #[tracing::instrument(
        name = "Saving new team member",
        skip(self),
        fields (
            team_id = %team_id,
            new_member = %new_member_id
        )
    )]
    pub async fn add_member_to_team(&mut self, team_id: TeamId, new_member_id: UserId) -> sqlx::Result<()> {
        let query = query_file!(
            "src/queries/transaction/add_member_to_team.sql",
            team_id.0,
            new_member_id.0,
        );
        
        self.0.execute(query).await?;
        
        Ok(())
    }
}