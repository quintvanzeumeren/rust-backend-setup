use sqlx::{Executor, query_file};
use domain::role::role_name::RoleName;
use domain::user::user_id::UserId;
use crate::queries::transaction::_transaction::Transaction;

impl Transaction {

    pub async fn add_role_to_user(&mut self, id: UserId, role: RoleName) -> sqlx::Result<()> {
        self.0.execute(
            query_file!("src/queries/transaction/add_role_to_user.sql",
                role.0.0,
                id.0
            )
        ).await?;

        Ok(())
    }
}