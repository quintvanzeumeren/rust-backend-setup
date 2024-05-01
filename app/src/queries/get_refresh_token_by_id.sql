SELECT * FROM refresh_tokens
WHERE refresh_tokens.id = $1
LIMIT 1;