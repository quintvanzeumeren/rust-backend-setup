use secrecy::ExposeSecret;
use sqlx::Executor;
use uuid::Uuid;
use domain::user::password::Password;
use crate::queries::transaction::_transaction::Transaction;


impl Transaction {
    #[tracing::instrument(
    name = "Saving updated password of user to Postgres",
    skip(self, user_id, new_password)
    )]
    pub async fn update_user_password(
        &mut self,
        user_id: Uuid,
        new_password: Password,
    ) -> Result<(), sqlx::Error> {
        let password_hash = new_password.hash_string();
        let query = sqlx::query_file!(
            "src/queries/transaction/update_user_password.sql",
            password_hash.expose_secret(),
            user_id
        );

        self.0.execute(query).await?;
        return Ok(());
    }
}