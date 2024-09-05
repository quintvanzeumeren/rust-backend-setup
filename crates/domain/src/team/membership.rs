use serde::{Deserialize, Serialize};
use crate::team::team_id::TeamId;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Membership {
    pub team_id: TeamId,
    pub manager: bool
}