use crate::queries::records::user_role_record::RoleName;
use crate::queries::transaction::_transaction::Transaction;
use domain::role::role::SystemRole;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use sqlx::{query_file, Executor};

impl Transaction {

    pub async fn save_new_role_to_user(&mut self, user_id: UserId, role: &SystemRole) -> sqlx::Result<()> {
        
        match role {
            SystemRole::Root => self.save_root_or_admin_role(user_id, RoleName::Root).await?,
            SystemRole::Admin => self.save_root_or_admin_role(user_id, RoleName::Admin).await?,
            SystemRole::TeamManager(team_id) => self.save_role_with_team_id(user_id, RoleName::TeamManager, *team_id).await?,
            SystemRole::Member(team_id) => self.save_role_with_team_id(user_id, RoleName::Member, *team_id).await?
        }

        Ok(())
    }

    async fn save_root_or_admin_role(&mut self, user_id: UserId, role: RoleName) -> sqlx::Result<()> {
        self.0.execute(query_file!(
            "src/queries/transaction/save_new_user_role.sql",
            user_id.0,
            role.clone() as RoleName
        )).await?;

        Ok(())
    }

    async fn save_role_with_team_id(&mut self, id: UserId, role: RoleName, team_id: TeamId) -> sqlx::Result<()> {
        self.0.execute(query_file!(
            "src/queries/transaction/save_new_user_role_with_team_id.sql",
            id.0,
            role.clone() as RoleName,
            team_id.0
        )).await?;

        Ok(())
    }
}