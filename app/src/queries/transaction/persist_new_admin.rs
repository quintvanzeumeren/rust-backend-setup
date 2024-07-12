use sqlx::{Executor, query_file};

use domain::user::user::{Admin, User};

use crate::queries::models::user_record::UserRecord;
use crate::queries::transaction::_transaction::Transaction;

impl Transaction {
    pub async fn persist_new_admin(
        &mut self,
        user: &User<Admin>,
    ) -> sqlx::Result<()> {
        let user_record = UserRecord::from(user);
        self.0.execute(query_file!(
            "src/queries/transaction/persist_new_admin.sql",
            user_record.user_id,
            user_record.username,
            user_record.password_hash
        )).await?;

        Ok(())
    }

}
#[cfg(test)]
mod tests {
    use sqlx::{PgPool, query_as};

    use test_utility::random::_common::{random_salt, random_secret};
    use test_utility::random::user::random_admin;

    use crate::queries::database::Database;
    use crate::queries::models::user_record::UserRecord;

    #[sqlx::test]
    async fn test_save_user(db: PgPool) {
        let db = Database(db);
        let user = random_admin(random_secret(), &random_salt());
        let mut transaction = db.new_transaction().await.expect("Failed to start transation");

        transaction.persist_new_admin(&user)
            .await
            .expect("Failed to save user");
        transaction
            .commit()
            .await
            .expect("Failed to commit transaction");

        let user_record = query_as!(
            UserRecord,
            r#"
                SELECT * FROM users
                WHERE user_id = $1
            "#,
            user.id.0
        )
        .fetch_optional(&db.0)
        .await
        .expect("Failed to select added user");

        assert!(user_record.is_some());
        let user_record = user_record.unwrap();
        assert_eq!(user_record, UserRecord::from(&user))
    }
}
