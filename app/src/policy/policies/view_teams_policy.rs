use anyhow::Context;
use axum::async_trait;
use domain::role::role::{Role, UserRoles};
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use std::collections::HashSet;
use std::sync::Arc;

use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;

pub struct ViewTeamsPolicy {
    state: Arc<AppState>,
    principle_roles: UserRoles
}

#[async_trait]
impl Policy for ViewTeamsPolicy {
    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, PolicyRejectionError> {
        let principle_roles = state.db.get_user_roles(user_in_question)
            .await
            .context("Failed to retrieve user details")?;
        
        Ok(Self {
            state,
            principle_roles
        })
    }

    type Details = ();
    type Contract = ViewTeamsContract;

    async fn authorize(&self, _: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        for principle_role in &self.principle_roles {
            return match principle_role {
                Role::Root | Role::Admin => Ok(ViewTeamsContract {
                    state: self.state.clone(),
                    viewable_teams: ViewableTeams::Every
                }),
                Role::TeamManager { teams } => Ok(ViewTeamsContract {
                    state: self.state.clone(),
                    viewable_teams: ViewableTeams::SelectedOnly(teams.clone())
                }),
                Role::Member { teams } => Ok(ViewTeamsContract {
                    state: self.state.clone(),
                    viewable_teams: ViewableTeams::SelectedOnly(teams.clone())
                }),
            }
        }

        Err(PolicyRejectionError::Forbidden)
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