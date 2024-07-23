use crate::team::team_id::TeamId;
use crate::user::user_id::UserId;

/// UserAttributes contains attributes of the User by which a Permission can determine if the user
/// is authorized.
pub struct UserAttributes {
    pub id: UserId,
    pub teams: Vec<TeamId>
}