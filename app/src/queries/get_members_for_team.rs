use std::collections::HashSet;
use sqlx::{query_file, query_file_as};
use uuid::Uuid;
use domain::role::role::SystemRole;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use crate::queries::database::Database;
use crate::queries::records::user_role_record::RoleName;

pub struct TeamMemberRecord {
    user_id: Uuid,
    role: RoleName
}

#[derive(Hash, Eq, PartialEq)]
pub struct TeamMember {
    pub user_id: UserId,
    pub role: SystemRole
}

impl Database {
    
    pub async fn get_team_members(&self, team_id: TeamId) -> sqlx::Result<HashSet<TeamMember>> {
        let result = query_file_as!(
            TeamMemberRecord,
            "src/queries/get_members_for_team.sql",
            team_id.0
        ).fetch_all(self.db()).await?;
        
        Ok(result
            .iter()
            .map(|r| TeamMember {
                    user_id: r.user_id.into(),
                    role: match r.role {
                        RoleName::Root => SystemRole::Root,
                        RoleName::Admin => SystemRole::Admin,
                        RoleName::TeamManager => SystemRole::TeamManager(team_id),
                        RoleName::Member => SystemRole::Member(team_id)
                    }
                }
            )
            .collect()
        )
    }
}