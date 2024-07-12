INSERT INTO users (user_id, username, password_hash, admin)
VALUES ($1, $2, $3, true);