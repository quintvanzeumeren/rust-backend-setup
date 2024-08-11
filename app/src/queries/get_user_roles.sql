SELECT roles.id, roles.name FROM user_roles
JOIN roles ON
    user_roles.user_id = $1 AND user_roles.role_id = roles.id;
