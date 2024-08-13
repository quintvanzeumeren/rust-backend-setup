use std::collections::HashSet;
use std::sync::Arc;
use axum::async_trait;
use domain::permission::permissions::view_teams::ViewTeam;
use domain::permission::user_attributes::UserAttributes;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;

use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyAuthorizationError;

pub struct ViewTeamsPolicy {
    user_id: UserId,
    state: Arc<AppState>,
    permission: ViewTeam
}

#[async_trait]
impl Policy for ViewTeamsPolicy {
    type Rejection = sqlx::Error;

    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, Self::Rejection> {
        let user_attributes = state.db.get_user_attributes(user_in_question).await?;

        Ok(Self {
            user_id: user_in_question,
            state,
            permission: ViewTeam {
                user_attributes,
                // todo retrieve viewable teams for the user
                viewable_teams: HashSet::default()
            },
        })
    }

    type Details = ();
    type Contract = ViewTeamsContract;
    type AuthenticationRejection = PolicyAuthorizationError;

    fn authorize(&self, _: Self::Details) -> Result<Self::Contract, Self::AuthenticationRejection> {
        Ok(ViewTeamsContract {
            state: self.state.clone(),
            user_attributes: self.permission.user_attributes.clone(),
            viewable_teams: Default::default(),
        })
    }
}

pub struct ViewTeamsContract {
    state: Arc<AppState>,
    user_attributes: UserAttributes,
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