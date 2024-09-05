use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use crate::queries::transaction::save_new_user::NewUser;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::user::user_details::UserDetails;
use domain::role::role::SystemRole;
use domain::team::member::Member;
use domain::team::team_id::TeamId;
use domain::user::user_credentials::UserCredentials;
use domain::user::user_id::UserId;
use std::sync::Arc;

pub struct CreateUserPolicy {
    state: Arc<AppState>,

    /// roles_of_principle refers to the entity performing an action
    principle: UserDetails
}

#[async_trait]
impl Policy for CreateUserPolicy {
    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, PolicyRejectionError> {
        let principle = state.db.get_user_details(user_in_question).await
            .context("Failed to user details for principle")?;

        match principle {
            None => Err(PolicyRejectionError::Forbidden),
            Some(principle) => Ok(Self {
                state,
                principle
            })
        }
    }

    type Details = CreateUserDetails;
    type Contract = CreateUserContract;

    async fn authorize(&self, new_user_details: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {

        if let Some(role) = &new_user_details.role {
            if let Some(principle_role) = &self.principle.system_role {
                return match (principle_role, role) {
                    (SystemRole::Root, _) |
                    (SystemRole::Admin, SystemRole::Admin) => Ok(CreateUserContract {
                        state: self.state.clone(),
                        details: new_user_details,
                    }),
                    (_, _) => Err(PolicyRejectionError::Forbidden)
                }
            }

            return Err(PolicyRejectionError::Forbidden)
        }

        if let Some(new_user_team) = new_user_details.team_to_part_of {
            let principle_is_manager_of_new_user_team = self.principle.teams.iter()
                .any(|m| m.team_id == new_user_team && m.manager);

            if principle_is_manager_of_new_user_team {
                return Ok(CreateUserContract {
                    state: self.state.clone(),
                    details: new_user_details,
                })
            }
        }

        Err(PolicyRejectionError::Forbidden)
    }
}

pub struct CreateUserDetails {
    pub role: Option<SystemRole>,
    pub team_to_part_of: Option<TeamId>
}

pub struct CreateUserContract {
    state: Arc<AppState>,
    details: CreateUserDetails
}

impl CreateUserContract {
    
    pub async fn create_user(&self, new_user: UserCredentials) -> sqlx::Result<()> {
        let mut transaction = self.state.db.new_transaction().await?;

        transaction.save_new_user(&NewUser {
            id: new_user.id,
            username: new_user.username,
            password: new_user.password,
            system_role: self.details.role,
        }).await?;

        if let Some(team_id) = self.details.team_to_part_of {
            transaction.save_team_member(Member {
                user_id: new_user.id,
                team_id,
                manager: false,
            }).await?;
        }
        
        transaction.commit().await?;
        Ok(())
    }
}