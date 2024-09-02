SELECT user_id, team_id, role AS "role!: RoleName" FROM user_roles
WHERE user_id = $1;
