INSERT INTO refresh_tokens (id, session_id, user_id, parent_id, issued_at, not_before, expiration)
VALUES ($1, $2, $3, $4, $5, $6, $7);