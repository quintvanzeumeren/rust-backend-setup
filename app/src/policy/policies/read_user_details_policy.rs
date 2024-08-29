use std::sync::Arc;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::permissions::read_user_details_permission::ReadUserDetailsPermission;
use domain::permission::user_attributes::UserAttributes;
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyAuthorizationError;

pub struct ReadUserDetailsPolicy {
    state: Arc<AppState>,
    permission: ReadUserDetailsPermission
}

#[async_trait]
impl Policy for ReadUserDetailsPolicy {
    type Rejection = sqlx::Error;

    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, Self::Rejection> {
        let user_attributes = state.db.get_user_attributes(user_in_question).await?;
        let permission = ReadUserDetailsPermission::new(user_attributes);

        Ok(Self {
            state,
            permission,
        })
    }

    type Details = UserId;
    type Contract = ReadUserDetailsContract;
    type AuthorizationRejection = PolicyAuthorizationError;

    fn authorize(&self, user: Self::Details) -> Result<Self::Contract, Self::AuthorizationRejection> {
        if self.permission.is_authorized_for(user.clone()) {
            return Ok(ReadUserDetailsContract {
                state: self.state.clone(),
                user_id: user,
            })
        }

        Err(PolicyAuthorizationError::Forbidden)
    }
}

pub struct ReadUserDetailsContract {
    state: Arc<AppState>,
    user_id: UserId
}

impl ReadUserDetailsContract {

    pub async fn get_user_details(&self) -> Result<UserAttributes, sqlx::Error> {
        Ok(self.state.db.get_user_attributes(self.user_id).await?)
    }

}