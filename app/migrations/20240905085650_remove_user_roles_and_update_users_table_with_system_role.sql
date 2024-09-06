-- Add migration script here
begin;

drop table user_roles;
drop type user_role;

create type system_role as enum ('Root', 'Admin');

alter table users
    add column system_role system_role;

commit;
