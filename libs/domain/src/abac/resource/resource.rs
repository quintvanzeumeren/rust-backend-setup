use crate::abac::resource::resource_id::ResourceId;
use crate::abac::resource::resource_type::ResourceType;

struct Resource<T: ResourceType> {
    id: ResourceId,
    resource_type: T
}