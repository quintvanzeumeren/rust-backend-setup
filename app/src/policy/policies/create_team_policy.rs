use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use anyhow::Context;
use axum::async_trait;
use domain::role::role::SystemRole;
use domain::team::team::Team;
use domain::team::team_id::TeamId;
use domain::user::user_details::UserDetails;
use domain::user::user_id::UserId;
use std::sync::Arc;

pub struct CreateTeamPolicy {
    state: Arc<AppState>,
    principle: UserDetails
}

#[async_trait]
impl Policy for CreateTeamPolicy {

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

    type Details = ();
    type Contract = CreateTeamContract;

    async fn authorize(&self, _: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        if let Some(role) = self.principle.system_role {
            return match role {
                SystemRole::Root | SystemRole::Admin => {
                    Ok(CreateTeamContract {
                        state: self.state.clone(),
                    })
                }
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