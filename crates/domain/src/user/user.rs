use std::marker::PhantomData;
use crate::user::password::Password;
use crate::user::user::private::UserRole;
use crate::user::user_id::UserId;

mod private {
    pub trait UserRole {}
}

pub struct Admin;
impl UserRole for Admin {}

pub struct EndUser;
impl UserRole for EndUser {}

pub struct User<Role: UserRole = EndUser> {
    pub id: UserId,
    pub username: String,
    pub hashed_password: Password,
    phantom_data: PhantomData<Role>
}

impl<R: UserRole> User<R> {

    pub fn new(id: UserId, username: String, password: Password) -> Self {
        Self {
            id,
            username,
            hashed_password: password,
            phantom_data: PhantomData,
        }
    }
}