use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::role::role::{Role, UserRoles};
use domain::user::user::User;
use domain::user::user_id::UserId;
use std::sync::Arc;

pub struct CreateUserPolicy {
    state: Arc<AppState>,
    
    /// roles_of_principle refers to the entity performing an action  
    roles_of_principle: UserRoles,
}

#[async_trait]
impl Policy for CreateUserPolicy {
    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, PolicyRejectionError> {
        let user_roles = state.db.get_user_roles(user_in_question).await
            .context("Failed to user roles")?;
         
        Ok(Self {
            state,
            roles_of_principle: user_roles
        })
    }

    type Details = UserRoles;
    type Contract = CreateUserContract;

    async fn authorize(&self, roles_for_new_users: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        for role in self.roles_of_principle.iter() {
            return match role {
                Role::Root => Ok(CreateUserContract {
                    state: self.state.clone(),
                    roles_for_new_users,
                }),
                Role::Admin => {
                    let creatable_roles_for_admin = roles_for_new_users.iter().all(|r| match r {
                        Role::TeamManager { .. } | Role::Member { .. } => true,
                        _ => false,
                    });
                    
                    if creatable_roles_for_admin { 
                        return Ok(CreateUserContract {
                            state: self.state.clone(),
                            roles_for_new_users,
                        });     
                    }
                     
                    Err(PolicyRejectionError::Forbidden)
                }
                Role::TeamManager { teams } => {
                    let teams_where_manager = teams;
                    let creatable_roles_for_team_manager = roles_for_new_users.iter().all(|r| match r {
                        Role::TeamManager { teams } => teams.iter().all(|t| teams_where_manager.contains(t)),
                        Role::Member { teams } => teams.iter().all(|t| teams_where_manager.contains(t)),
                        _ => false,
                    });
                    
                    if creatable_roles_for_team_manager {
                        return Ok(CreateUserContract {
                            state: self.state.clone(),
                            roles_for_new_users,
                        });
                    }

                    Err(PolicyRejectionError::Forbidden)
                }
                _ => Err(PolicyRejectionError::Forbidden)
            }
        }

        Err(PolicyRejectionError::Forbidden)
    }
}

pub struct CreateUserContract {
    state: Arc<AppState>,
    roles_for_new_users: UserRoles
}

impl CreateUserContract {
    
    pub async fn create_user(&self, new_user: NewUserDetails) -> sqlx::Result<()> {
        let mut transaction = self.state.db.new_transaction().await?;
        transaction.save_new_user(&new_user.user).await?;
        
        let user_id = new_user.user.id;
        for user_role in &self.roles_for_new_users {
            transaction.add_role_to_user(user_id, user_role).await?;
        }
        
        transaction.commit().await?;
        Ok(())
    }
    
}

pub struct NewUserDetails {
    pub(crate) user: User
}