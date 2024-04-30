use sqlx::{Executor, Postgres, query, Transaction};

use domain::user::user::User;

use crate::queries::models::user_record::UserRecord;

pub async fn save_user(transaction: &mut Transaction<'_, Postgres>, user: &User) -> sqlx::Result<()> {
    let user_record = UserRecord::from(user);
    transaction.execute(query!(
        r#"
            INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)
        "#,
        user_record.user_id,
        user_record.username,
        user_record.password_hash
    )).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::{PgPool, query_as};

    use test_utility::random::_common::{random_salt, random_secret};
    use test_utility::random::user::random_user;

    use crate::queries::models::user_record::UserRecord;
    use crate::queries::save_user::save_user;

    #[sqlx::test]
    async fn test_save_user(db: PgPool) {
        let user = random_user(random_secret(), &random_salt());
        
        let mut transaction = db.begin()
            .await
            .expect("Failed to start transation");
        
        save_user(&mut transaction, &user).await.expect("Failed to save user");
        transaction.commit().await.expect("Failed to commit transaction");
        
        let user_record = query_as!(
            UserRecord,
            r#"
                SELECT * FROM users
                WHERE user_id = $1
            "#,
            user.id
        )
            .fetch_optional(&db)
            .await
            .expect("Failed to select added user");
        
        assert!(user_record.is_some());
        let user_record = user_record.unwrap();
        assert_eq!(user_record, UserRecord::from(&user))
    }
    
}