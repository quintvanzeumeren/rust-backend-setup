use sqlx::{Error, PgPool};
use domain::permission::permission::Permission;
use crate::queries::permissions::querier::Querier;
use crate::queries::transaction::_transaction::Transaction;

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

impl Database {
    pub fn get_permission_querier<P: Permission>(&self) -> Querier<P> {
        Querier::new(self.0.clone())
    }
}
