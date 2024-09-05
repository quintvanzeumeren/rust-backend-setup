use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::permissions::create_team::CreateTeam;
use domain::team::team::Team;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use std::sync::Arc;
use domain::role::role::{SystemRole, UserRoles};

pub struct CreateTeamPolicy {
    state: Arc<AppState>,
    // permission: CreateTeam
    principle_roles: UserRoles,
    principle_id: UserId
}

#[async_trait]
impl Policy for CreateTeamPolicy {

    async fn new(state: Arc<AppState>, user_id: UserId) -> Result<Self, PolicyRejectionError> {
        let roles = state.db.get_user_roles(user_id)
            .await
            .context("Failed to retrieve user details")?;

        Ok(Self {
            state,
            principle_roles: roles,
            principle_id: user_id
        })
    }

    type Details = ();
    type Contract = CreateTeamContract;

    async fn authorize(&self, _: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        for principle_role in &self.principle_roles {
            match principle_role {
                SystemRole::Root | SystemRole::Admin => {
                    return Ok(CreateTeamContract {
                        state: self.state.clone()
                    })
                }
                _ => continue
            }
        }

        Err(PolicyRejectionError::Forbidden)
    }
}

pub struct CreateTeamContract {
    state: Arc<AppState>
}

impl CreateTeamContract {
    pub async fn create_team(&self, team_id: TeamId) -> sqlx::Result<Team> {
        let new_team = Team {
            id: team_id
        };

        let mut transaction = self.state.db.new_transaction().await?;
        transaction.save_team(&new_team).await?;
        transaction.commit().await?;

        Ok(new_team)
    }
}