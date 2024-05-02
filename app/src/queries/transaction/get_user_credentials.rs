use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::{PgPool, query};
use uuid::Uuid;
use domain::user::password::Password;
use crate::queries::database::Database;

impl Database {
    #[tracing::instrument(name = "Fetching user credentials for username", skip(self, username))]
    pub async fn get_user_credentials(
        &self,
        username: &String,
    ) -> sqlx::Result<Option<UserCredentials>> {
        let row = query!(
            r#"
               SELECT user_id, password_hash FROM users
               WHERE username = $1
            "#,
            username
        )
            .fetch_optional(self.db())
            .await?
            .map(|row| (row.user_id, Secret::new(row.password_hash)));

        if row.is_none() {
            return Ok(None);
        }

        let (user_id, pw_hash) = row.unwrap();
        Ok(Some(UserCredentials {
            user_id,
            password_hash: pw_hash,
        }))
    }
}

pub struct UserCredentials {
    pub user_id: Uuid,
    pub password_hash: Secret<String>,
}