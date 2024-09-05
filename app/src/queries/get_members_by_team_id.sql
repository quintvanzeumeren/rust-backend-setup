select user_id, team_id, manager from team_members
where team_id = $1;