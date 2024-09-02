use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Deserialize, Serialize)]
pub struct TeamId(pub Uuid);

impl From<Uuid> for TeamId {
    fn from(value: Uuid) -> Self {
        TeamId(value)
    }
}

impl Display for TeamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
