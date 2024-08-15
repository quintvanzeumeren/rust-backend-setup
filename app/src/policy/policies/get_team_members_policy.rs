use std::collections::HashSet;
use std::sync::Arc;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::permissions::read_team_members::ReadTeamMembers;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyAuthorizationError;

pub struct GetTeamMembersPolicy {
    state: Arc<AppState>,
    permission: ReadTeamMembers,
    user_id: UserId
}

#[async_trait]
impl Policy for GetTeamMembersPolicy {
    type Rejection = sqlx::Error;

    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, Self::Rejection> {
        let user = state.db.get_user_attributes(user_in_question).await?;
        
        Ok(Self {
            state,
            permission: ReadTeamMembers {
                user_attributes: user,
                // todo fetch user resources
                resources: Default::default(),
            },
            user_id: user_in_question,
        })
    }

    type Details = TeamId;
    type Contract = GetTeamMembersContract;
    type AuthorizationRejection = PolicyAuthorizationError;

    fn authorize(&self, team_id: Self::Details) -> Result<Self::Contract, Self::AuthorizationRejection> {
        if self.permission.is_authorized_for(team_id) {
            return Ok(GetTeamMembersContract {
                team_id,
                state: self.state.clone(),
            })
        }

        Err(PolicyAuthorizationError::Forbidden)
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