use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use lib_domain::sessions::state::active::Active;
use lib_domain::sessions::state::just_ended::JustEnded;
use lib_domain::sessions::state::newly_created::NewlyCreated;
use lib_domain::sessions::state::refreshed::Refreshed;
use lib_domain::sessions::state::state::SessionEndReason;
use lib_domain::sessions::user_session_token::UserSessionToken;
use lib_domain::sessions::tokens::RefreshToken;
use lib_domain::sessions::user_session::UserSession;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserSessionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub ending_reason: Option<String>,
    pub ending_token_id: Option<Uuid>,
}

impl UserSessionRecord {
    pub fn to_active_session(self, latest_token: UserSessionToken<RefreshToken>) -> UserSession<Active> {
        UserSession::<Active>::new(
            self.id,
            self.user_id,
            self.created_at.and_utc(),
            Active {
                latest_refresh_token: latest_token,
            }
        )
    }
}

impl From<&UserSession<NewlyCreated>> for UserSessionRecord {
    fn from(session: &UserSession<NewlyCreated>) -> Self {
        UserSessionRecord {
            id: session.id().clone(),
            user_id: session.user_id().clone(),
            created_at: session.created_at().naive_utc(),
            ended_at: None,
            ending_reason: None,
            ending_token_id: None,
        }
    }
}

impl From<&UserSession<Refreshed>> for UserSessionRecord {
    fn from(session: &UserSession<Refreshed>) -> Self {
        UserSessionRecord {
            id: session.id().clone(),
            user_id: session.user_id().clone(),
            created_at: session.created_at().naive_utc(),
            ended_at: None,
            ending_reason: None,
            ending_token_id: None,
        }
    }
}

impl From<&UserSession<JustEnded>> for UserSessionRecord {
    fn from(session: &UserSession<JustEnded>) -> Self {
        let ending_token_id: Option<Uuid> = {
            match session.state().reason_for_ending() {
                SessionEndReason::AttemptedToReuseRefreshToken { caused_by } => Some(caused_by.id),
                _ => None
            }
        };

        UserSessionRecord {
            id: session.id().clone(),
            user_id: session.user_id().clone(),
            created_at: session.created_at().naive_utc(),
            ended_at: Some(session.state().session_end_time().naive_utc()),
            ending_reason: Some(session.state().reason_for_ending().to_string().to_string()),
            ending_token_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use lib_test_util::random::user_session::random_newly_created_user_session;

    use crate::queries::models::user_session_record::UserSessionRecord;

    #[test]
    fn test_from_newly_created() {
        let user_id = Uuid::new_v4();
        let session = random_newly_created_user_session(&user_id);

        let expected = UserSessionRecord {
            id: session.id().clone(),
            user_id,
            created_at: session.created_at().naive_utc().clone(),
            ended_at: None,
            ending_reason: None,
            ending_token_id: None,
        };

        let got = UserSessionRecord::from(&session);
        assert_eq!(expected, got)
    }

}