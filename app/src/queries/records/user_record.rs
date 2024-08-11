use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use domain::user::password::Password;
use domain::user::user::User;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserRecord {
    pub user_id: Uuid,
    pub username: String,
    pub password_hash: String,
}

impl From<&User> for UserRecord {
    fn from(user: &User) -> Self {
        UserRecord {
            user_id: user.id.0,
            username: user.username.clone(),
            password_hash: user.hashed_password.hash_string().expose_secret().clone(),
        }
    }
}

impl TryInto<User> for UserRecord {
    type Error = password_hash::Error;

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User {
            id: self.user_id.into(),
            username: self.username,
            hashed_password: Password::try_from(self.password_hash)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use domain::user::user::User;
    use test_utility::random::_common::{random_salt, random_secret};
    use test_utility::random::user::random_user;

    use crate::queries::records::user_record::UserRecord;

    #[test]
    fn test_from() {
        let salt = random_salt();
        let user1 = random_user(random_secret(), &salt);

        let record = UserRecord::from(&user1);
        assert_eq!(record.user_id, user1.id.0);
        assert_eq!(record.username, user1.username);
        assert_eq!(record.password_hash, user1.hashed_password.hash_string().expose_secret().clone());

        let record2 = UserRecord::from(&user1);
        assert_eq!(record, record2)
    }

    #[test]
    fn test_into() {
        let salt = random_salt();
        let user1 = random_user(random_secret(), &salt);

        let record = UserRecord::from(&user1);
        let into_user: User = record.try_into().expect("Failed to transform UserRecord into User");
        assert_eq!(user1.id, into_user.id);
        assert_eq!(user1.username, into_user.username);
        assert_eq!(user1.hashed_password.hash_string().expose_secret().clone(), into_user.hashed_password.hash_string().expose_secret().clone());
    }
}