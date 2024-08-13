use std::sync::Arc;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::permissions::create_team::CreateTeam;
use domain::sessions::state::state::State;
use domain::team::team::Team;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyAuthorizationError;

pub struct CreateTeamPolicy {
    state: Arc<AppState>,
    permission: CreateTeam
}

#[async_trait]
impl Policy for CreateTeamPolicy {
    type Rejection = sqlx::Error;

    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, Self::Rejection> {
        let user = state.db.get_user_attributes(user_in_question).await?;
        let permission = CreateTeam {
            user,
        };

        Ok(Self {
            state,
            permission,
        })
    }

    type Details = ();
    type Contract = CreateTeamContract;
    type AuthenticationRejection = PolicyAuthorizationError;

    fn authorize(&self, _: Self::Details) -> Result<Self::Contract, Self::AuthenticationRejection> {
        if self.permission.is_authorized_for(()) {
            return Ok(CreateTeamContract {
                state: self.state.clone(),
            })
        }

        return Err(PolicyAuthorizationError::Forbidden)
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