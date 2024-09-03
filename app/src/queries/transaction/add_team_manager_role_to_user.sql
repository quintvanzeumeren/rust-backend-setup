INSERT INTO user_roles (user_id, role, team_id)
VALUES ($1, $2, $3)
ON CONFLICT DO NOTHING;