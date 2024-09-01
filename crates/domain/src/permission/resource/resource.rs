use std::hash::Hash;
use crate::permission::resource::resource_id::ResourceId;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Resource<T: PartialEq + Clone + Hash + PartialEq + Eq> {
    pub id: ResourceId,
    pub resource: T
}