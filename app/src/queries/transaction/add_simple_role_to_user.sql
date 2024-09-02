WITH role_id_cte AS (
    SELECT id
    FROM roles
    WHERE name = $1
)
INSERT INTO user_roles (user_id, role_id)
VALUES ($2, (SELECT id from role_id_cte))
ON CONFLICT DO NOTHING;