use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::role::role::{SystemRole};
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use std::collections::HashSet;
use std::sync::Arc;
use domain::team::member::Member;
use domain::user::user_details::UserDetails;

pub struct GetTeamMembersPolicy {
    state: Arc<AppState>,
    principle: UserDetails
}

#[async_trait]
impl Policy for GetTeamMembersPolicy {
    async fn new(state: Arc<AppState>, principle_id: UserId) -> Result<Self, PolicyRejectionError> {
        let principle = state.db.get_user_details(principle_id).await
            .context("Failed to user details for principle")?;

        match principle {
            None => Err(PolicyRejectionError::Forbidden),
            Some(principle) => Ok(Self {
                state,
                principle
            })
        }
    }

    type Details = TeamId;
    type Contract = GetTeamMembersContract;

    async fn authorize(&self, team_id: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        if let Some(role) = self.principle.system_role {
            return match role {
                SystemRole::Root | SystemRole::Admin => {
                    Ok(GetTeamMembersContract {
                        team_id,
                        state: self.state.clone(),
                    })
                }
            }
        }
        
        if self.principle.teams.iter().any(|t| t.team_id == team_id) { 
            return Ok(GetTeamMembersContract {
                team_id,
                state: self.state.clone(),
            })
        }
        
        Err(PolicyRejectionError::Forbidden)
    }
}

pub struct GetTeamMembersContract {
    team_id: TeamId,
    state: Arc<AppState>
}

impl GetTeamMembersContract {
    
    pub async fn fetch_team_members(&self) -> Result<HashSet<Member>, sqlx::Error> {
        Ok(self.state.db.get_members_by_team_id(self.team_id).await?)
    }
}