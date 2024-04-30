use uuid::Uuid;
use crate::user::password::Password;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: Password
}