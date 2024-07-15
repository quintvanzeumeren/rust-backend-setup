use uuid::Uuid;

pub trait Permission<T> {
    
    /// Name returns the str value for the permission
    fn name() -> &'static str;
    
    /// granted checks whenever the member is granted the permission
    fn granted(member: Member, t: T) -> bool;
}

pub struct Member {
    
}

struct Resource {
    id: Uuid,
    resource_type: String,
    resource_id: Uuid
}

