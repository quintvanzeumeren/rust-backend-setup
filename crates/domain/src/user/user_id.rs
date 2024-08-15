use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct UserId(pub Uuid);

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        UserId(value)
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

