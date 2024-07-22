use crate::permission::resource::resource_id::ResourceId;

pub struct Resource<T> {
    pub id: ResourceId,
    pub resource: T
}