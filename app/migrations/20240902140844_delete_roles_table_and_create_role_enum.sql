-- Add migration script here
begin;

-- Create new role type enum containing all roles
create type user_role as enum ('Root', 'Admin', 'TeamManager', 'Member');

-- Add the new column containing the new enum
alter table user_roles
add column role user_role;

-- set the correct role for each existing user
UPDATE user_roles
SET role = roles.name::user_role
FROM roles
WHERE user_roles.role_id = roles.id;

-- drop the role_id column and proceed the drop the roles table.
alter table user_roles
    alter column role set not null,
    drop column role_id;

drop table roles;

commit;