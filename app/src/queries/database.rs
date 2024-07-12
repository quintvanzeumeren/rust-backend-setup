use crate::queries::transaction::_transaction::Transaction;
use sqlx::{Error, PgPool};
use domain::user::user_id::UserId;
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;

#[derive(Clone, Debug)]
pub struct Database(pub PgPool);
impl Database {
    pub async fn new_transaction(&self) -> Result<Transaction, Error> {
        let tx = self.0.begin().await?;
        Ok(Transaction(tx))
    }

    pub fn db(&self) -> &PgPool {
        &self.0
    }
}
