use sqlx::{Error, Postgres};
pub struct Transaction(pub sqlx::Transaction<'static, Postgres>);
impl Transaction {
    pub async fn commit(self) -> Result<(), Error> {
        // todo write test for commit
        self.0.commit().await
    }

    pub async fn rollback(self) -> Result<(), Error> {
        // todo write test for rollback
        self.0.rollback().await
    }
}
