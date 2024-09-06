use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use crate::telemetry::TelemetryRecord;
use anyhow::Context;
use axum::async_trait;
use domain::role::role::SystemRole;
use domain::team::member::Member;
use domain::team::team_id::TeamId;
use domain::user::user_details::UserDetails;
use domain::user::user_id::UserId;
use std::sync::Arc;

pub struct AddTeamMemberPolicy {
    state: Arc<AppState>,
    principle: UserDetails
}

#[async_trait]
impl Policy for AddTeamMemberPolicy {

    #[tracing::instrument(
        name = "Initializing a new AddTeamMembersPolicy",
        skip(state),
        fields(user_id = %user_id)
    )]
    async fn new(state: Arc<AppState>, user_id: UserId) -> Result<Self, PolicyRejectionError> {
        let principle = state.db.get_user_details(user_id).await
            .context("Failed to user details for principle")?;

        match principle {
            None => Err(PolicyRejectionError::Forbidden),
            Some(principle) => Ok(Self {
                state,
                principle
            })
        }
    }

    type Details = TeamId;
    type Contract = AddMemberContract;

    #[tracing::instrument(
        name = "Initializing a new AddTeamMembersPolicy",
        skip_all,
        fields(
            principle_id = tracing::field::Empty,
            team_to_add_too = %team_to_add_to,
        )
    )]
    async fn authorize(&self, team_to_add_to: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        self.principle.id.record_in_telemetry("principle_id");
        
        if let Some(role) = self.principle.system_role {
            return match role {
                SystemRole::Root | SystemRole::Admin => Ok(AddMemberContract {
                    state: self.state.clone(),
                    team_to_add_too: team_to_add_to,
                })
            }
        }
        
        match self.principle.teams.iter().find(|t| t.team_id == team_to_add_to) {
            None => Err(PolicyRejectionError::Forbidden),
            Some(membership) => {
                if membership.manager {
                    return Ok(AddMemberContract {
                        state: self.state.clone(),
                        team_to_add_too: team_to_add_to,
                    })
                }
                
                Err(PolicyRejectionError::Forbidden)
            }
        }
    }
}

pub struct AddMemberContract {
    state: Arc<AppState>,
    team_to_add_too: TeamId,
}

impl AddMemberContract {

    pub async fn add_member(&self, new_member_id: UserId, should_become_team_manager: bool) -> Result<(), sqlx::Error> {
        let mut transaction = self.state.db.new_transaction().await?;
        transaction.save_team_member(Member {
            user_id: new_member_id,
            team_id: self.team_to_add_too,
            manager: should_become_team_manager,
        }).await?;
        
        transaction.commit().await?;

        Ok(())
    }
}