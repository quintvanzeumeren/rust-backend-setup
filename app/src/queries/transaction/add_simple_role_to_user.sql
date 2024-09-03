INSERT INTO user_roles (user_id, role)
VALUES ($1, $2)
ON CONFLICT DO NOTHING;