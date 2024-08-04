SELECT users.user_id,
       array_agg(roles.name)           AS roles,
       array_agg(team_members.team_id) AS teams
FROM users
         LEFT JOIN user_roles
                   ON user_roles.user_id = $1
         LEFT JOIN roles
                   ON roles.id = user_roles.role_id
         LEFT JOIN team_members
                   ON team_members.user_id = $1
GROUP BY users.user_id;

-- user_id:     3c9579de-0601-436a-8303-3bede39462d0