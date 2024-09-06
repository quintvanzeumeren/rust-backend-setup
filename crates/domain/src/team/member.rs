use crate::team::team_id::TeamId;
use crate::user::user_id::UserId;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Member {
    pub user_id: UserId,
    pub team_id: TeamId,
    pub manager: bool
}

impl Member {
    pub fn is_manager_of_team(&self) -> bool {
        self.manager == true
    }
}