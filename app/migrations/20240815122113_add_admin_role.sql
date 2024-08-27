-- Add migration script here
insert into roles (id, name)
values (uuid_generate_v4(), 'admin');
