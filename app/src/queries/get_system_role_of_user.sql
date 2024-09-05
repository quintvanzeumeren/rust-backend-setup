select user_id, system_role AS "system_role!: SystemRoleType" from users
where user_id = $1
limit 1