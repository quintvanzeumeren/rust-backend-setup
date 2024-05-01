UPDATE user_sessions
SET ended_at        = $2,
    ending_reason   = $3,
    ending_token_id = $4
WHERE user_sessions.id = $1;