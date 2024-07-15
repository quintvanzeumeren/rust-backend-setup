use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr)]
pub enum Permissions {
    ReadUsersOfOrganisation
}

