use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoleId(pub Uuid);

impl From<Uuid> for RoleId {
    fn from(value: Uuid) -> Self {
        RoleId(value)
    }
}

impl Display for RoleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}