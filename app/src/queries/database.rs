use crate::queries::transaction::_transaction::Transaction;
use sqlx::{Error, PgPool};

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
