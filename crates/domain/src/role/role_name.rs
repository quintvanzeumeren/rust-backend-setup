use crate::shared::slug::Slug;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoleName(pub Slug);

impl From<&'static str> for RoleName {
    fn from(value: &'static str) -> Self {
        Self(Slug::new(value.to_string()))
    }
}
