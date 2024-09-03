use crate::queries::records::user_role_record::RoleName;
use crate::queries::transaction::_transaction::Transaction;
use domain::role::role::Role;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use sqlx::{query_file, Executor};
use std::collections::HashSet;

impl Transaction {

    pub async fn add_role_to_user(&mut self, id: UserId, role: &Role) -> sqlx::Result<()> {
        match role {
            Role::Root => self.add_simple_role(id, RoleName::Root).await?,
            Role::Admin => self.add_simple_role(id, RoleName::Admin).await?,
            Role::TeamManager { teams } => self.add_team_manager_role(id, RoleName::TeamManager, teams).await?,
            Role::Member { teams } => self.add_team_manager_role(id, RoleName::Member, teams).await?
        }

        Ok(())
    }

    async fn add_simple_role(&mut self, id: UserId, role: RoleName) -> sqlx::Result<()> {
        self.0.execute(
            query_file!("src/queries/transaction/add_simple_role_to_user.sql",
                id.0,
                role as RoleName,
            )
        ).await?;

        Ok(())
    }

    async fn add_team_manager_role(&mut self, id: UserId, role: RoleName, teams: &HashSet<TeamId>) -> sqlx::Result<()> {
        for team_id in teams {
            self.0.execute(
                query_file!("src/queries/transaction/add_team_manager_role_to_user.sql",
                id.0,
                role.clone() as RoleName,
                team_id.0
            )).await?;
        }

        Ok(())
    }
}