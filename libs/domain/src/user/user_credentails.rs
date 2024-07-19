use crate::user::password::Password;
use crate::user::user_id::UserId;

pub struct UserCredentials {
    id: UserId,
    username: String,
    password: Password
}