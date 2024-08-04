use serde::de::DeserializeOwned;

use domain::permission::permission_authorizer::PermissionAuthorizer;
use domain::permission::permissions::create_team::CreateTeam;
use domain::team::team::Team;
use domain::team::team_id::TeamId;
use crate::extractors::user::permission_extractor::permission_of::PermissionOf;
use crate::extractors::user::user_with::UserWith;

impl <RC> UserWith<PermissionOf<CreateTeam, RC>> 
where
    RC: DeserializeOwned + Into<<CreateTeam as PermissionAuthorizer>::ResourceInQuestion> + Send + Sync + Clone 
{
    pub async fn create_new_team(&self, team_id: TeamId) -> sqlx::Result<Team> {
        let new_team = Team {
            id: team_id
        };
        
        let mut transaction = self.state.db.new_transaction().await?;
        transaction.save_team(&new_team).await?;
        transaction.commit().await?;
        
        Ok(new_team)
    }
    
}