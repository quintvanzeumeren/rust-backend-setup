use crate::app_state::AppState;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;
use crate::telemetry::TelemetryRecord;
use anyhow::Context;
use axum::async_trait;
use domain::permission::permission::Permission;
use domain::role::role::{Role, UserRoles};
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use std::sync::Arc;

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
        fields(user_id = %user)
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

    type Details = AddTeamMemberDetails;
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
    async fn authorize(&self, details: Self::Details) -> Result<Self::Contract, PolicyRejectionError> {
        self.principle_id.record_in_telemetry("principle_id");
        details.user_to_add.record_in_telemetry("user_to_add");
        details.team_to_add_to.record_in_telemetry("team_to_add_to");
        
        
        todo!("Have a good look at this logic again to see if it even make sense lol");
        
        
        let mut user_to_add_roles: Option<UserRoles> = None;
        for principle_role in self.principle_roles.iter() {
            match principle_role {
                Role::Root => {
                    return Ok(AddMemberContract::new(self.state.clone(), details))
                }
                Role::Admin => {
                    if details.user_to_add == self.principle_id {
                        return Ok(AddMemberContract::new(self.state.clone(), details))
                    }
                    
                    match &user_to_add_roles {
                        None => {
                            user_to_add_roles = Some(
                                self.state.db.get_user_roles(details.user_to_add)
                                    .await
                                    .context("Failed to get retrieve user_details for user_to_add")?
                            );
                        }
                        Some(roles) => {
                            if roles.iter().all(|r| match r {
                                Role::TeamManager(_) => true,
                                Role::Member(_) => true,
                                _ => false,
                            }) {
                                return Ok(AddMemberContract::new(self.state.clone(), details))
                            }
                        }
                    }
                    
                }
                Role::TeamManager(team_id) => {
                    if *team_id != details.team_to_add_to { 
                        continue
                    }
                    
                    match &user_to_add_roles {
                        None => {
                            user_to_add_roles = Some(
                                self.state.db.get_user_roles(details.user_to_add)
                                    .await
                                    .context("Failed to get retrieve user_details for user_to_add")?
                            );
                        }
                        Some(roles) => {
                            if roles.iter().all(|r| match r {
                                Role::Root | Role::Admin => false,
                                Role::TeamManager(_) | Role::Member(_) => true,
                            }) {
                                return Ok(AddMemberContract::new(self.state.clone(), details))
                            }
                        }
                    }
                }
                _ => continue
            }
        }
        
        Err(PolicyRejectionError::Forbidden)
            
        // if self.principle_roles.is_root() {
        //     return Ok(AddMemberContract::new(self.state.clone(), details))
        // }
        //
        // let not_an_admin_either = !self.principle_roles.is_admin();
        // if not_an_admin_either {
        //     return Err(PolicyRejectionError::Forbidden)
        // }
        //
        // if self.principle_roles.id == details.user_to_add {
        //     return Ok(AddMemberContract::new(self.state.clone(), details))
        // }
        //
        // let user_to_add_details = self.state.db.get_user_details(details.user_to_add)
        //     .await
        //     .context("Failed to get retrieve user_details for user_to_add")?;
        //
        // if user_to_add_details.is_root_or_admin() {
        //     return Err(PolicyRejectionError::Forbidden)
        // }
        //
        // Ok(AddMemberContract::new(self.state.clone(), details))
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