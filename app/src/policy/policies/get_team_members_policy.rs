use std::collections::HashSet;
use std::sync::Arc;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::permissions::read_team_members::ReadTeamMembers;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;

pub struct GetTeamMembersPolicy {
    state: Arc<AppState>,
    permission: ReadTeamMembers,
    user_id: UserId
}

#[async_trait]
impl Policy for GetTeamMembersPolicy {
    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, PolicyRejectionError> {
        let user = state.db.get_user_details(user_in_question)
            .await
            .context("Failed to retrieve user details")?;
        
        Ok(Self {
            state,
            permission: ReadTeamMembers::new(user),
            user_id: user_in_question,
        })
    }

    type Details = TeamId;
    type Contract = GetTeamMembersContract;

    async fn authorize(&self, team_id: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        if self.permission.is_authorized_for(team_id) {
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
    
    pub async fn fetch_team_members(&self) -> Result<HashSet<UserId>, sqlx::Error> {
        Ok(self.state.db.get_team_members(self.team_id).await?)
    }
}