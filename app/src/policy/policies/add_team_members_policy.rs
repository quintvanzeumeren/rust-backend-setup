use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use crate::telemetry::TelemetryRecord;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::permission::user_attributes::UserDetails;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use std::sync::Arc;

pub struct AddTeamMemberPolicy {
    state: Arc<AppState>,
    user: UserDetails,
}

#[async_trait]
impl Policy for AddTeamMemberPolicy {

    #[tracing::instrument(
        name = "Initializing a new AddTeamMembersPolicy",
        skip(state),
        fields(user_id = %user)
    )]
    async fn new(state: Arc<AppState>, user: UserId) -> Result<Self, PolicyRejectionError> {
        let user = state.db
            .get_user_details(user)
            .await
            .context("Failed to query UserDetails")?;

        Ok(Self {
            state,
            user
        })
    }

    type Details = AddTeamMemberDetails;
    type Contract = AddMemberContract;

    #[tracing::instrument(
        name = "Initializing a new AddTeamMembersPolicy",
        skip_all,
        fields(
            user_of_policy = tracing::field::Empty,
            user_to_add = tracing::field::Empty,
            team_to_add_too = tracing::field::Empty,
        )
    )]
    async fn authorize(&self, details: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        self.user.id.record_in_telemetry("user_of_policy");
        details.user_to_add.record_in_telemetry("user_to_add");
        details.team_to_add_to.record_in_telemetry("team_to_add_to");

        if self.user.is_root() {
            return Ok(AddMemberContract::new(self.state.clone(), details))
        }

        let not_an_admin_either = !self.user.is_admin();
        if not_an_admin_either {
            return Err(PolicyRejectionError::Forbidden)
        }

        if self.user.id == details.user_to_add {
            return Ok(AddMemberContract::new(self.state.clone(), details))
        }

        let user_to_add_details = self.state.db.get_user_details(details.user_to_add)
            .await
            .context("Failed to get retrieve user_details for user_to_add")?;

        if user_to_add_details.is_root_or_admin() {
            return Err(PolicyRejectionError::Forbidden)
        }

        Ok(AddMemberContract::new(self.state.clone(), details))
    }
}

pub struct AddTeamMemberDetails {
    pub user_to_add: UserId,
    pub team_to_add_to: TeamId
}

pub struct AddMemberContract {
    state: Arc<AppState>,
    details: AddTeamMemberDetails
}

impl AddMemberContract {

    pub async fn add_member(&self) -> Result<(), sqlx::Error> {
        let mut transaction = self.state.db.new_transaction().await?;
        
        transaction.add_member_to_team(self.details.team_to_add_to, self.details.user_to_add).await?;
        transaction.commit().await?;

        Ok(())
    }

    fn new(state: Arc<AppState>, details: AddTeamMemberDetails) -> Self {
        Self { state, details }
    }
}