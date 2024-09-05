-- Add migration script here
alter table team_members
    add column manager bool not null default false;