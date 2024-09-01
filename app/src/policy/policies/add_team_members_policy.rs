use std::collections::HashSet;
use std::sync::Arc;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::permissions::add_team_members::AddTeamMembers;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyAuthorizationError;

pub struct AddTeamMembersPolicy {
    state: Arc<AppState>,
    permission: AddTeamMembers
}

#[async_trait]
impl Policy for AddTeamMembersPolicy {
    type Rejection = sqlx::Error;

    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, Self::Rejection> {
        let user = state.db.get_user_attributes(user_in_question).await?;
        let permission = AddTeamMembers::new(user);

        Ok(Self {
            state,
            permission
        })
    }

    type Details = TeamId;
    type Contract = AddMemberContract;
    type AuthorizationRejection = PolicyAuthorizationError;

    fn authorize(&self, details: Self::Details) -> Result<Self::Contract, Self::AuthorizationRejection> {
        if !self.permission.is_authorized_for(details) {
            return Err(PolicyAuthorizationError::Forbidden)
        }

        Ok(AddMemberContract {
            team_id: details,
            state: self.state.clone(),
        })
    }
}

/// Contract

pub struct AddMemberContract {
    team_id: TeamId,
    state: Arc<AppState>,
}

impl AddMemberContract {

    pub async fn add_member(&self, user_id: UserId) -> Result<(), sqlx::Error> {
        let mut transaction = self.state.db.new_transaction().await?;
        
        transaction.add_member_to_team(self.team_id, user_id).await?;
        transaction.commit().await?;

        Ok(())
    }

}