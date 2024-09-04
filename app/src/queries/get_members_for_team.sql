select user_id, role AS "role!: RoleName"
from user_roles
where team_id = $1 and (role = 'TeamManager'::user_role or role = 'Member'::user_role);