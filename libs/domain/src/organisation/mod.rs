use crate::shared::slug::Slug;

pub mod organisation;
pub mod organisation_id;
pub mod member;

/// COMPANY_ORGANISATION_NAME is the main organisation that manages to application.
/// todo add further documentation later
pub const COMPANY_ORGANISATION_NAME: &'static str = "MyCompany";

pub fn main_company_slug() -> Slug {
    return Slug::new(COMPANY_ORGANISATION_NAME.to_string());
}


