use enum_dispatch::enum_dispatch;

use domain::permission::permission::{Permission, PermissionName};

use crate::queries::database::Database;
use crate::queries::permissions::permission_querier::PermissionQuerier;

#[enum_dispatch(PermissionQuerier)]
enum PermissionQuerierScheme {
    ReadOrganisationUsers()
}

pub fn getPermissionQuerier<T: Permission>(permission_name: PermissionName) -> Option<impl PermissionQuerier<T>> {
    match permission_name { 
        
        _ => Option::None
    }
}