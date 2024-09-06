
insert into team_members (user_id, team_id, manager)
values ($1, $2, $3)
on conflict(user_id, team_id) do update set manager = EXCLUDED.manager