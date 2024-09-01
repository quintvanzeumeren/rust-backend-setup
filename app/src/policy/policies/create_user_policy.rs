use std::sync::Arc;
use anyhow::Context;
use axum::async_trait;
use fake::faker::internet::raw::Username;
use password_hash::PasswordHash;
use domain::permission::permission::Permission;
use domain::permission::permissions::create_user::CreateUser;
use domain::role::role_name::RoleName;
use domain::user::password::Password;
use domain::user::user::User;
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;

pub struct CreateUserPolicy {
    state: Arc<AppState>,
    permission: CreateUser
}

#[async_trait]
impl Policy for CreateUserPolicy {
    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, PolicyRejectionError> {
        let user_attributes = state.db.get_user_details(user_in_question)
            .await
            .context("Failed to retrieve user details")?;
        
        Ok(Self {
            state,
            permission: CreateUser {
                user_attributes
            }
        })
    }

    type Details = Vec<RoleName>;
    type Contract = CreateUserContract;

    async fn authorize(&self, details: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        if self.permission.is_authorized_for(details.clone()) {
           return Ok(CreateUserContract {
               state: self.state.clone(),
               user_roles: details,
           }) 
        }
        
        Err(PolicyRejectionError::Forbidden)
    }
}

pub struct CreateUserContract {
    state: Arc<AppState>,
    user_roles: Vec<RoleName>
}

impl CreateUserContract {
    
    pub async fn create_user(&self, new_user: NewUserDetails) -> sqlx::Result<()> {
        let mut transaction = self.state.db.new_transaction().await?;
        transaction.save_new_user(&new_user.user).await?;
        
        let user_id = new_user.user.id;
        for user_role in &self.user_roles {
            transaction.add_role_to_user(user_id, user_role.clone()).await?;
        }
        
        transaction.commit().await?;
        Ok(())
    }
    
}

pub struct NewUserDetails {
    pub(crate) user: User
}