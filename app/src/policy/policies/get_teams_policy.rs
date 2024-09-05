use anyhow::Context;
use axum::async_trait;
use domain::role::role::{SystemRole};
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use std::collections::HashSet;
use std::sync::Arc;
use domain::user::user_details::UserDetails;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;

pub struct GetTeamsPolicy {
    state: Arc<AppState>,
    principle: UserDetails
}

#[async_trait]
impl Policy for GetTeamsPolicy {
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
    type Contract = ViewTeamsContract;

    async fn authorize(&self, _: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {

        if let Some(role) = self.principle.system_role {
            return match role {
                SystemRole::Root | SystemRole::Admin => {
                    Ok(ViewTeamsContract {
                        state: self.state.clone(),
                        viewable_teams: ViewableTeams::Every
                    })
                }
            }
        }

        if self.principle.teams.is_empty() {
            return Err(PolicyRejectionError::Forbidden)
        }

        Ok(ViewTeamsContract {
            state: self.state.clone(),
            viewable_teams: ViewableTeams::SelectedOnly(
                self.principle.teams.iter().map(|m| m.team_id).collect()
            )
        })
    }
}

pub struct ViewTeamsContract {
    state: Arc<AppState>,
    viewable_teams: ViewableTeams
}

pub enum ViewableTeams {
    /// Can see every team
    Every,

    /// Can see selected teams only
    SelectedOnly(HashSet<TeamId>)
}

impl ViewTeamsContract {

    pub async fn get_teams(&self) -> sqlx::Result<HashSet<TeamId>> {
        match &self.viewable_teams {
            ViewableTeams::Every => Ok(self.state.db.get_teams().await?),
            ViewableTeams::SelectedOnly(teams) => Ok(teams.clone())
        }
    }

}