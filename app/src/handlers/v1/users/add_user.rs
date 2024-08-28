use reqwest::StatusCode;
use secrecy::Secret;
use uuid::Uuid;

pub struct AddUserBody {
    id: Uuid,
    username: String,
    password: Secret<String>,
    roles: Vec<String>
}

pub async fn add_user() -> StatusCode {
    
    todo!()
}