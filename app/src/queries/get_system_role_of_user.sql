select user_id, system_role AS "system_role!: Option<SystemRoleType>" from users
where user_id = $1
limit 1