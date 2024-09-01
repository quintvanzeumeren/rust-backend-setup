use std::sync::Arc;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::permissions::read_user_details_permission::ReadUserDetailsPermission;
use domain::permission::user_attributes::UserDetails;
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;

pub struct ReadUserDetailsPolicy {
    state: Arc<AppState>,
    permission: ReadUserDetailsPermission
}

#[async_trait]
impl Policy for ReadUserDetailsPolicy {

    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, PolicyRejectionError> {
        let user_attributes = state.db.get_user_details(user_in_question)
            .await
            .context("Failed to retrieve user details")?;
        
        let permission = ReadUserDetailsPermission::new(user_attributes);
        
        Ok(Self {
            state,
            permission,
        })
    }

    type Details = UserId;
    type Contract = ReadUserDetailsContract;

    async fn authorize(&self, user: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        if self.permission.is_authorized_for(user.clone()) {
            return Ok(ReadUserDetailsContract {
                state: self.state.clone(),
                user_id: user,
            })
        }

        Err(PolicyRejectionError::Forbidden)
    }
}

pub struct ReadUserDetailsContract {
    state: Arc<AppState>,
    user_id: UserId
}

impl ReadUserDetailsContract {

    pub async fn get_user_details(&self) -> Result<Option<UserDetails>, sqlx::Error> {
        if self.state.db.exist_user_of(self.user_id).await? {
            return Ok(Some(self.state.db.get_user_details(self.user_id).await?))
        }

        Ok(None)
    }

}