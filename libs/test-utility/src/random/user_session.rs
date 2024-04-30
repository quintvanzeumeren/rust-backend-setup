use uuid::Uuid;

use domain::sessions::state::newly_created::NewlyCreated;
use domain::sessions::user_session::UserSession;

pub fn random_newly_created_user_session(user_id: &Uuid) -> UserSession<NewlyCreated> {
    UserSession::<NewlyCreated>::new(user_id)
}