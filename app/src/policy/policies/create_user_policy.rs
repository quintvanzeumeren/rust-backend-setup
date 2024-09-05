use std::collections::HashSet;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::role::role::{SystemRole, UserRoles};
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

        let mut possible_roles: UserRoles = HashSet::new();
        for role in self.roles_of_principle.iter() {
            match role {
                SystemRole::Root => {
                    return Ok(CreateUserContract {
                        state: self.state.clone(),
                        roles_for_new_users,
                    })
                },
                SystemRole::Admin => {
                    let creatable_roles_for_admin = roles_for_new_users.iter().all(|r| match r {
                        SystemRole::TeamManager { .. } | SystemRole::Member { .. } => true,
                        _ => false,
                    });

                    if creatable_roles_for_admin {
                        return Ok(CreateUserContract {
                            state: self.state.clone(),
                            roles_for_new_users,
                        });
                    }
                }
                SystemRole::TeamManager(team_id) => {
                    let roles: Vec<&SystemRole> = roles_for_new_users.iter().filter(|r | match r {
                        SystemRole::TeamManager(id) => *id == *team_id,
                        SystemRole::Member(id) => *id == *team_id,
                        _ => false,
                    }).collect();

                    for role in roles {
                        possible_roles.insert(role.clone());
                    }
                }
                _ => continue
            }
        }
        
        if possible_roles.eq(&roles_for_new_users) {
            return Ok(CreateUserContract {
                state: self.state.clone(),
                roles_for_new_users,
            });
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
            transaction.save_new_role_to_user(user_id, user_role).await?;
        }
        
        transaction.commit().await?;
        Ok(())
    }
    
}

pub struct NewUserDetails {
    pub(crate) user: User
}