use crate::queries::records::user_role_record::SystemRoleType;
use crate::queries::transaction::_transaction::Transaction;
use domain::role::role::SystemRole;
use domain::user::password::Password;
use domain::user::user_id::UserId;
use secrecy::ExposeSecret;
use sqlx::{query_file, Executor};


impl Transaction {
    pub async fn save_new_user(
        &mut self,
        user: &NewUser,
    ) -> sqlx::Result<()> {
        
        let role = match user.system_role {
            None => None,
            Some(r) => Some(SystemRoleType::from(r))  
        };
        
        self.0.execute(query_file!(
            "src/queries/transaction/save_new_user.sql",
            user.id.0,
            user.username,
            user.password.hash().expose_secret(),
            role as Option<SystemRoleType>
        )).await?;

        Ok(())
    }
}

pub struct NewUser {
    pub id: UserId,
    pub username: String,
    pub password: Password,
    pub system_role: Option<SystemRole>
}

#[cfg(test)]
mod tests {
    use sqlx::{query_as, PgPool};

    use crate::queries::database::Database;
    use test_utility::random::_common::{random_salt, random_secret};
    use test_utility::random::user::random_user;

    use crate::queries::records::user_record::UserRecord;

    #[sqlx::test]
    async fn test_save_user(db: PgPool) {
        let db = Database(db);
        let user = random_user(random_secret(), &random_salt());
        let mut transaction = db.new_transaction().await.expect("Failed to start transation");

        transaction.save_new_user(&user)
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
