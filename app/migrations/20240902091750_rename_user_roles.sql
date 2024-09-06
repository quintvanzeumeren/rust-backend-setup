-- Add migration script here
begin;

update roles
set name = 'Root'
where name = 'root';

update roles
set name = 'Admin'
where name = 'admin';

commit;