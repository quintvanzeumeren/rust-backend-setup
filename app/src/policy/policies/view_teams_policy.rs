use std::collections::HashSet;
use std::sync::Arc;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permissions::view_teams::ViewTeam;
use domain::permission::user_attributes::UserDetails;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;

use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;

pub struct ViewTeamsPolicy {
    user_id: UserId,
    state: Arc<AppState>,
    permission: ViewTeam
}

#[async_trait]
impl Policy for ViewTeamsPolicy {
    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, PolicyRejectionError> {
        let user_attributes = state.db.get_user_details(user_in_question)
            .await
            .context("Failed to retrieve user details")?;
        
        Ok(Self {
            user_id: user_in_question,
            state,
            permission: ViewTeam::new(user_attributes),
        })
    }

    type Details = ();
    type Contract = ViewTeamsContract;

    async fn authorize(&self, _: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        Ok(ViewTeamsContract {
            state: self.state.clone(),
            user_attributes: self.permission.user_attributes.clone(),
            viewable_teams: Default::default(),
        })
    }
}

pub struct ViewTeamsContract {
    state: Arc<AppState>,
    user_attributes: UserDetails,
    viewable_teams: HashSet<TeamId>,
}

impl ViewTeamsContract {

    pub async fn get_teams(&self) -> sqlx::Result<HashSet<TeamId>> {
        if self.user_attributes.is_root() {
            return Ok(self.state.db.get_teams().await?);
        }

        let mut teams = HashSet::new();
        teams.extend(self.user_attributes.teams.clone());
        teams.extend(self.viewable_teams.clone());

        Ok(teams)
    }

}