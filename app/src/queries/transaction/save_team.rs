use sqlx::{Executor, query_file};
use domain::team::team::Team;
use crate::queries::transaction::_transaction::Transaction;

impl Transaction {
    
    pub async fn save_team(&mut self, team: &Team) -> sqlx::Result<()> {
        
        let query = query_file!(
            "src/queries/transaction/save_team.sql",
            team.id.0
        );
        
        self.0.execute(query).await?;
        Ok(())
    }
}