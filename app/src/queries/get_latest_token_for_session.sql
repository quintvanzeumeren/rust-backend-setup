SELECT * FROM refresh_tokens
WHERE refresh_tokens.session_id = $1
ORDER BY refresh_tokens.issued_at DESC
LIMIT 1;