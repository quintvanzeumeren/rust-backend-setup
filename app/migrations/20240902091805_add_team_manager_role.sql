-- Add migration script here
begin;

insert into roles (id, name)
values (uuid_generate_v4(), 'TeamManager');

alter table user_roles
    add column team_id uuid null;

alter table user_roles
    add constraint fk_team
        foreign key (team_id)
            references teams (id);

commit;

