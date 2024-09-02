use crate::queries::transaction::_transaction::Transaction;
use domain::role::role::{NameOfRole, Role};
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use sqlx::{query_file, Executor};

impl Transaction {

    pub async fn add_role_to_user(&mut self, id: UserId, role: Role) -> sqlx::Result<()> {
        let role_name = role.name();
        match role {
            Role::Root | Role::Admin => self.add_simple_role(id, role_name).await?,
            Role::TeamManager { teams } => self.add_team_manager_role(id, role_name, teams).await?
        }

        Ok(())
    }

    async fn add_simple_role(&mut self, id: UserId, name: NameOfRole) -> sqlx::Result<()> {
        self.0.execute(
            query_file!("src/queries/transaction/add_simple_role_to_user.sql",
                name,
                id.0
            )
        ).await?;

        Ok(())
    }

    async fn add_team_manager_role(&mut self, id: UserId, name: NameOfRole, teams: Vec<TeamId>) -> sqlx::Result<()> {
        for team_id in teams {
            self.0.execute(
                query_file!("src/queries/transaction/add_team_manager_role_to_user.sql",
                name,
                id.0,
                team_id.0
            )).await?;
        }

        Ok(())
    }
}