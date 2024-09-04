use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::role::role::{Role, UserRoles};
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use std::collections::HashSet;
use std::sync::Arc;

pub struct GetTeamMembersPolicy {
    state: Arc<AppState>,
    principle_roles: UserRoles
}

#[async_trait]
impl Policy for GetTeamMembersPolicy {
    async fn new(state: Arc<AppState>, principle_id: UserId) -> Result<Self, PolicyRejectionError> {
        let principle_roles = state.db.get_user_roles(principle_id)
            .await
            .context("Failed to retrieve user details")?;
        
        Ok(Self {
            state,
            principle_roles
        })
    }

    type Details = TeamId;
    type Contract = GetTeamMembersContract;

    async fn authorize(&self, team_id: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        for principle_role in &self.principle_roles {
            match principle_role {
                Role::Root | Role::Admin => {
                    return Ok(GetTeamMembersContract {
                        team_id,
                        state: self.state.clone(),
                    })
                }
                Role::TeamManager(tm_id) => {
                    if team_id == *tm_id {
                        return Ok(GetTeamMembersContract {
                            team_id,
                            state: self.state.clone(),
                        })
                    }
                }
                Role::Member(tm_id) => {
                    if team_id == *tm_id {
                        return Ok(GetTeamMembersContract {
                            team_id,
                            state: self.state.clone(),
                        })
                    }
                }
            }
        }

        Err(PolicyRejectionError::Forbidden)
    }
}

pub struct GetTeamMembersContract {
    team_id: TeamId,
    state: Arc<AppState>
}

impl GetTeamMembersContract {
    
    pub async fn fetch_team_members(&self) -> Result<HashSet<UserId>, sqlx::Error> {
        Ok(self.state.db.get_team_members(self.team_id).await?)
    }
}