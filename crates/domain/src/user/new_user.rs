use crate::role::role::SystemRole;
use crate::user::password::Password;
use crate::user::user_id::UserId;

pub struct NewUser {
    pub id: UserId,
    pub username: String,
    pub password: Password,
    pub system_role: Option<SystemRole>
}