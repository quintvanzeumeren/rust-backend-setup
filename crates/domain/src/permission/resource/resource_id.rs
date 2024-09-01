
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ResourceId(pub Uuid);

impl From<Uuid> for ResourceId {
    fn from(value: Uuid) -> Self {
        ResourceId(value)
    }
}

impl Display for ResourceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

