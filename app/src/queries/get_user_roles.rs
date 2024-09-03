use std::collections::HashMap;
use domain::role::role::{Role, UserRoles};
use domain::user::user_id::UserId;
use sqlx::{query, query_as, query_file, query_file_as};
use tracing::warn;
use uuid::Uuid;
use domain::team::team_id::TeamId;
use crate::queries::database::Database;
use crate::queries::records::user_role_record::{RoleName, UserRoleRecord};

impl Database {
    
    #[tracing::instrument(
        name = "fetching user roles from db",
        skip(self),
        fields (
            user_id = %user_id,
            unknown_role_id = tracing::field::Empty,
            unknown_role = tracing::field::Empty
        )
    )]
     pub async fn get_user_roles(&self, user_id: UserId) -> sqlx::Result<UserRoles> {
        // todo for some reason this query doesn't work when using query_file!
        let records = query_as!(
            UserRoleRecord,
            "SELECT user_id, team_id, role AS \"role!: RoleName\" FROM user_roles WHERE user_id = $1",
            user_id.0
        ).fetch_all(self.db()).await?;

         let user_roles = parse_into_user_roles(records);
        Ok(user_roles)
    }
}

fn parse_into_user_roles(records: Vec<UserRoleRecord>) -> UserRoles {
    let mut roles = HashMap::new();
    for record in records {
        match record.role {
            RoleName::Root => { roles.entry(record.role).or_insert(Role::Root); },
            RoleName::Admin => { roles.entry(record.role).or_insert(Role::Admin); },
            RoleName::TeamManager
            | RoleName::Member => {
                let team_id: TeamId = match record.team_id {
                    None => {
                        warn!("Expected a team id for team_manager role");
                        continue
                    },
                    Some(uuid) => uuid.into()
                };

                if let Some(role) = roles.get(&record.role) {
                    let mut updated_teams = vec![team_id];
                    if let Role::TeamManager { teams } = role {
                        updated_teams.extend(teams);
                        roles.insert(record.role, Role::TeamManager { teams: updated_teams });
                    } else if let Role::Member { teams } = role {
                        updated_teams.extend(teams);
                        roles.insert(record.role, Role::Member { teams: updated_teams });
                    }
                }
            }
        }
    }

    roles.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use uuid::Uuid;
    use domain::role::role::{Role, UserRoles};
    use crate::queries::get_user_roles::parse_into_user_roles;
    use crate::queries::records::user_role_record::{RoleName, UserRoleRecord};

    fn new_root_record() -> UserRoleRecord {
        UserRoleRecord {
            user_id: Uuid::default(),
            team_id: None,
            role: RoleName::Root,
        }
    }

    fn new_admin_record() -> UserRoleRecord {
        UserRoleRecord {
            user_id: Uuid::default(),
            team_id: None,
            role: RoleName::Admin,
        }
    }

    fn new_team_manager_record() -> UserRoleRecord {
        UserRoleRecord {
            user_id: Uuid::default(),
            team_id: Some(Uuid::new_v4()),
            role: RoleName::TeamManager,
        }
    }

    fn new_member_record() -> UserRoleRecord {
        UserRoleRecord {
            user_id: Uuid::default(),
            team_id: Some(Uuid::new_v4()),
            role: RoleName::Member,
        }
    }

    #[test]
    fn new_methods() {
        assert_eq!(new_root_record().role, RoleName::Root);
        assert_eq!(new_admin_record().role, RoleName::Admin);
        assert_eq!(new_team_manager_record().role, RoleName::TeamManager);
        assert_eq!(new_member_record().role, RoleName::Member);
    }

    #[test]
    fn test_parse_into_records_correctly() {
        let root = new_root_record();
        let root2 = new_root_record();
        let admin = new_admin_record();
        let admin2 = new_admin_record();
        let team_manager = new_team_manager_record();
        let team_manager2 = new_team_manager_record();
        let team_manager3  = new_team_manager_record();
        let member = new_member_record();
        let member2 = new_member_record();
        let member3 = new_member_record();

        let mut expected: UserRoles = HashSet::new();
        expected.insert(Role::Root);
        expected.insert(Role::Admin);
        expected.insert(Role::TeamManager {
            teams: vec![
                team_manager.team_id.expect("Expected team id").into(),
                team_manager2.team_id.expect("Expected team id").into(),
                team_manager3.team_id.expect("Expected team id").into(),
            ]
        });
        expected.insert(Role::Member {
            teams: vec![
                member.team_id.expect("Expected team id").into(),
                member2.team_id.expect("Expected team id").into(),
                member3.team_id.expect("Expected team id").into(),
            ]
        });

        let roles = vec![
            root,
            root2,
            admin,
            admin2,
            team_manager,
            team_manager2,
            team_manager3,
            member,
            member2,
            member3
        ];

        let result = parse_into_user_roles(roles);
        assert_eq!(expected, result)
    }
}

