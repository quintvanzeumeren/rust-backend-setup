use std::collections::HashSet;
use crate::team::team_id::TeamId;
use crate::user::user_id::UserId;

pub struct Team {
    id: TeamId,
    members: HashSet<UserId>
}