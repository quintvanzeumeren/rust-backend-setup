use crate::user::password::Password;
use crate::user::user_id::UserId;

pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: Password,
}