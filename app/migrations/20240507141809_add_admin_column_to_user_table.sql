-- Add migration script here
alter table users add column admin boolean not null default false;
