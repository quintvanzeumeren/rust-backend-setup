use std::collections::HashSet;
use std::sync::Arc;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::permissions::read_user_details_permission::ReadUserDetailsPermission;
use domain::permission::user_attributes::UserDetails;
use domain::role::role::{SystemRole, UserRoles};
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;

pub struct ReadUserDetailsPolicy {
    state: Arc<AppState>,
    principle_roles: UserRoles,
    principle_id: UserId
}

#[async_trait]
impl Policy for ReadUserDetailsPolicy {

    async fn new(state: Arc<AppState>, principle_id: UserId) -> Result<Self, PolicyRejectionError> {
        let principle_roles = state.db.get_user_roles(principle_id)
            .await
            .context("Failed to retrieve user details")?;
        
        Ok(Self {
            state,
            principle_roles,
            principle_id
        })
    }

    type Details = UserId;
    type Contract = ReadUserDetailsContract;

    async fn authorize(&self, user_id: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {

        if self.principle_id == user_id {
            return Ok(ReadUserDetailsContract {
                state: self.state.clone(),
                user_id,
            })
        }

        let mut roles_of_user: Option<UserRoles> = None;
        for principle_role in &self.principle_roles {
            match principle_role {
                SystemRole::Root | SystemRole::Admin => {
                    return Ok(ReadUserDetailsContract {
                        state: self.state.clone(),
                        user_id,
                    })
                }
                SystemRole::TeamManager(team_id) => {

                    if roles_of_user.is_none() {
                        roles_of_user = Some(
                            self.state.db.get_user_roles(user_id)
                                .await
                                .context("Failed to get user roles")?
                        );
                    }

                    if let Some(roles) = &roles_of_user {
                        for role in roles {
                            let is_of_same_team = match role {
                                SystemRole::TeamManager(t_id) => t_id == team_id,
                                SystemRole::Member(t_id) => t_id == team_id,
                                _ => continue
                            };

                            if is_of_same_team {
                                return Ok(ReadUserDetailsContract {
                                    state: self.state.clone(),
                                    user_id,
                                })
                            }
                        }
                    }
                }
                _ => continue
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