use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use anyhow::Context;
use axum::async_trait;
use domain::role::role::SystemRole;
use domain::user::user_details::UserDetails;
use domain::user::user_id::UserId;
use std::sync::Arc;

pub struct ReadUserDetailsPolicy {
    state: Arc<AppState>,
    principle: UserDetails
}

#[async_trait]
impl Policy for ReadUserDetailsPolicy {

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

    type Details = UserId;
    type Contract = ReadUserDetailsContract;

    async fn authorize(&self, user_id: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        
        if self.principle.id == user_id {
            return Ok(ReadUserDetailsContract {
                state: self.state.clone(),
                user_id,
            })
        }
        
        if let Some(role) = self.principle.system_role {
            return match role {
                SystemRole::Root |
                SystemRole::Admin => {
                    Ok(ReadUserDetailsContract {
                        state: self.state.clone(),
                        user_id,
                    })          
                }
            }
        }
        
        let user_details = self.state.db.get_user_details(user_id)
            .await
            .with_context(|| format!("Failed to get UserDetails for user: {}", user_id))?;
        
        if let Some(details) = user_details { 
            let principle_is_manager_of_user = self.principle
                .get_teams_where_manager()
                .iter()
                .any(|t| details.teams.iter().any(|m| m.team_id == *t));
            
            if principle_is_manager_of_user {
                return Ok(ReadUserDetailsContract {
                    state: self.state.clone(),
                    user_id,
                })
            }
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
            return Ok(self.state.db.get_user_details(self.user_id).await?)
        }

        Ok(None)
    }

}