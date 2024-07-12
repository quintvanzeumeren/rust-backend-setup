use uuid::Uuid;

use domain::sessions::state::newly_created::NewlyCreated;
use domain::sessions::user_session::UserSession;
use domain::user::user_id::UserId;

pub fn random_newly_created_user_session(user_id: UserId) -> UserSession<NewlyCreated> {
    UserSession::<NewlyCreated>::new(user_id)
}