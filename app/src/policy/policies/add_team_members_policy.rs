use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use crate::telemetry::TelemetryRecord;
use anyhow::Context;
use axum::async_trait;
use domain::role::role::{SystemRole, UserRoles};
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use std::sync::Arc;
use domain::team::member::Member;

pub struct AddTeamMemberPolicy {
    state: Arc<AppState>,
    principle_roles: UserRoles,
    principle_id: UserId
}

#[async_trait]
impl Policy for AddTeamMemberPolicy {

    #[tracing::instrument(
        name = "Initializing a new AddTeamMembersPolicy",
        skip(state),
        fields(user_id = %user_id)
    )]
    async fn new(state: Arc<AppState>, user_id: UserId) -> Result<Self, PolicyRejectionError> {
        let roles = state.db
            .get_user_roles(user_id)
            .await
            .context("Failed to query UserDetails")?;

        Ok(Self {
            state,
            principle_roles: roles,
            principle_id: user_id,
        })
    }

    type Details = TeamId;
    type Contract = AddMemberContract;

    #[tracing::instrument(
        name = "Initializing a new AddTeamMembersPolicy",
        skip_all,
        fields(
            principle_id = tracing::field::Empty,
            user_to_add = tracing::field::Empty,
            team_to_add_too = tracing::field::Empty,
        )
    )]
    async fn authorize(&self, team_to_add_to: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        self.principle_id.record_in_telemetry("principle_id");
        team_to_add_to.record_in_telemetry("team_to_add_to");

        for principle_role in self.principle_roles.iter() {
            match principle_role {
                SystemRole::Root | SystemRole::Admin => {
                    return Ok(AddMemberContract {
                        state: self.state.clone(),
                        team_to_add_too: team_to_add_to,
                    })
                },
                SystemRole::TeamManager(team_id) => {
                    if *team_id == team_to_add_to {
                        return Ok(AddMemberContract {
                            state: self.state.clone(), 
                            team_to_add_too: team_to_add_to,
                        })
                    }
                },
                _ => continue
            }
        }

        Err(PolicyRejectionError::Forbidden)
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

        Ok(())
    }
}