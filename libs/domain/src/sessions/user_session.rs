use chrono::{DateTime, Utc};
use uuid::Uuid;
use security::token::token::Token;
use crate::sessions::state::active::Active;
use crate::sessions::state::already_ended::AlreadyEnded;
use crate::sessions::state::just_ended::JustEnded;
use crate::sessions::state::newly_created::NewlyCreated;
use crate::sessions::state::refreshed::Refreshed;
use crate::sessions::state::state::{SessionEndReason, State};
use crate::sessions::user_session_token::UserSessionToken;
use crate::sessions::tokens::{AccessToken, RefreshToken};
use crate::user::user_id::UserId;

#[derive(Clone, PartialEq, Debug)]
pub struct UserSession<T: State> {
    pub(crate) id: Uuid,
    pub(crate) user_id: UserId,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) state: T
}

impl<T: State> UserSession<T> {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn state(&self) -> &T {
        &self.state
    }
}

impl UserSession<NewlyCreated> {
    pub fn new(user_id: UserId) -> UserSession<NewlyCreated> {
        let created_at = Utc::now();
        let session_id = Uuid::new_v4();

        let refresh_token: UserSessionToken<RefreshToken> = RefreshToken {
            user_id: user_id.0,
            session_id: session_id.clone(),
            parent_id: None,
        }.into();

        let access_token: UserSessionToken<AccessToken> = AccessToken {
            user_id: user_id.0,
            session_id: session_id.clone(),
            refresh_token_id: refresh_token.get_id().clone(),
        }.into();

        UserSession {
            id: session_id,
            user_id,
            created_at,
            state: NewlyCreated {
                refresh_token,
                access_token
            }
        }
    }
}

impl UserSession<Active> {
    pub fn new(
        id: Uuid,
        user_id: UserId,
        created_at: DateTime<Utc>,
        state: Active
    ) -> UserSession<Active> {
        UserSession {
            id,
            user_id,
            created_at,
            state
        }
    }

    pub fn refresh(
        self,
        refresh_token: UserSessionToken<RefreshToken>
    ) -> Result<UserSession<Refreshed>, UserSession<JustEnded>> {
        let latest_refresh_token = &self.state.latest_refresh_token;

        // There can only be one valid refresh token at the time,
        // therefor it must only be the latest token
        let refresh_token_used_previously = *latest_refresh_token.get_id() != *refresh_token.get_id();
        if refresh_token_used_previously {
            return Err(UserSession {
                id: self.id,
                user_id: self.user_id,
                created_at: self.created_at,
                state: JustEnded {
                    latest_refresh_token: self.state.latest_refresh_token,
                    reason_for_ending: SessionEndReason::AttemptedToReuseRefreshToken {
                        caused_by: refresh_token,
                    },
                    session_end_time: Utc::now(),
                }
            })
        }

        if self.state.latest_refresh_token.expired() {
            return Err(UserSession {
                id: self.id,
                user_id: self.user_id,
                created_at: self.created_at,
                state: JustEnded {
                    latest_refresh_token: self.state.latest_refresh_token,
                    reason_for_ending: SessionEndReason::LatestRefreshTokenExpired,
                    session_end_time: Utc::now(),
                }
            })
        }

        // no need to check this currently as paseto token implementation will 
        // raise an error when decrypting the token and time `not before` claim has not yet passed.
        // therefor we currently cannot ever receive a token which is not yet activated.
        // if self.state.latest_refresh_token.not_before.has_not_passed() {
        // }

        let new_access_token = AccessToken {
            user_id: self.user_id.0,
            session_id: self.id.clone(),
            refresh_token_id: latest_refresh_token.get_id().clone(),
        }.into();

        let new_refresh_token = RefreshToken {
            user_id: self.user_id.0,
            session_id: self.id.clone(),
            parent_id: Some(latest_refresh_token.get_id().clone()),
        }.into();

        Ok(UserSession {
            id: self.id,
            user_id: self.user_id,
            created_at: self.created_at,
            state: Refreshed {
                new_access_token,
                new_refresh_token,
                old_refresh_token: self.state.latest_refresh_token,
            },
        })
    }
    
    pub fn end_by_user_logout(self) -> UserSession<JustEnded> {
        UserSession {
            id: self.id,
            user_id: self.user_id,
            created_at: self.created_at,
            state: JustEnded {
                latest_refresh_token: self.state.latest_refresh_token,
                reason_for_ending: SessionEndReason::UserLogout,
                session_end_time: Utc::now(),
            }
        }
    }
}

impl UserSession<AlreadyEnded> {
    pub fn new(
        id: Uuid,
        user_id: UserId,
        created_at: DateTime<Utc>,
        state: Active
    ) -> UserSession<Active> {
        UserSession {
            id,
            user_id,
            created_at,
            state
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Duration, Utc};
    use uuid::Uuid;
    use security::token::token::Token;
    use crate::sessions::state::active::Active;
    use crate::sessions::state::just_ended::JustEnded;
    use crate::sessions::state::newly_created::NewlyCreated;
    use crate::sessions::state::state::SessionEndReason;
    use crate::sessions::user_session_token::UserSessionToken;
    use crate::sessions::tokens::RefreshToken;
    use crate::sessions::user_session::UserSession;

    #[test]
    fn test_new_session() {
        let user_id = Uuid::new_v4().into();
        let session = UserSession::<NewlyCreated>::new(user_id);

        // Check if user session has correct properties
        assert_eq!(session.user_id, user_id);
        assert_ne!(session.id, user_id.0);

        // Check if access token has correct properties
        assert_eq!(session.state.access_token.get_custom_claims().user_id, user_id.0);
        assert_eq!(session.state.access_token.get_custom_claims().session_id, session.id);
        assert_eq!(
            session.state.access_token.get_custom_claims().refresh_token_id,
            *session.state.refresh_token.get_id()
        );

        // Check if refresh token has correct properties
        assert_eq!(session.state.refresh_token.get_custom_claims().user_id, user_id.0);
        assert_eq!(session.state.refresh_token.get_custom_claims().session_id, session.id);
        assert!(session.state.refresh_token.get_custom_claims().parent_id.is_none());
    }

    #[test]
    fn test_refresh_should_succeed_with_good_refresh_token() {
        let session = UserSession::<NewlyCreated>::new(Uuid::new_v4().into());
        let refresh_token = session.state.refresh_token.clone();
        let session: UserSession<Active> = UserSession {
            id: session.id,
            user_id: session.user_id,
            created_at: session.created_at,
            state: Active {
                latest_refresh_token: session.state.refresh_token
            },
        };

        let id = session.id.clone();
        let user_id = session.user_id.clone();
        let created_at = session.created_at.clone();

        if let Ok(refresh_session) = session.refresh(refresh_token.clone()) {
            // Check if refresh_session has correct properties
            assert_eq!(id, refresh_session.id);
            assert_eq!(user_id, refresh_session.user_id);
            assert_eq!(created_at, refresh_session.created_at);

            // Check if access token has correct properties
            assert_eq!(refresh_session.state.new_access_token.get_custom_claims().user_id, user_id.0);
            assert_eq!(refresh_session.state.new_access_token.get_custom_claims().session_id, refresh_session.id);
            assert_eq!(
                refresh_session.state.new_access_token.get_custom_claims().refresh_token_id,
                *refresh_session.state.old_refresh_token.get_id()
            );

            // Check if refresh token has correct properties
            assert_eq!(refresh_session.state.new_refresh_token.get_custom_claims().user_id, user_id.0);
            assert_eq!(refresh_session.state.new_refresh_token.get_custom_claims().session_id, refresh_session.id);
            assert_eq!(refresh_session.state.new_refresh_token.get_custom_claims().parent_id.unwrap(), *refresh_token.get_id());

            // Check if old_refresh_token has correct properties
            assert_eq!(refresh_session.state.old_refresh_token.get_id(), refresh_token.get_id())
        } else {
            panic!("Failed to refresh session")
        }
    }

    #[test]
    fn test_refresh_should_end_session_with_previous_used_refresh_token() {
        let session = UserSession::<NewlyCreated>::new(Uuid::new_v4().into());
        let invalid_refresh_token: UserSessionToken<RefreshToken> = RefreshToken {
            user_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            parent_id: None,
        }.into();

        let session_token_id = session.state.refresh_token.get_id().clone();
        let session: UserSession<Active> = UserSession {
            id: session.id,
            user_id: session.user_id,
            created_at: session.created_at,
            state: Active {
                latest_refresh_token: session.state.refresh_token
            },
        };

        let id = session.id.clone();
        let user_id = session.user_id.clone();
        let created_at = session.created_at.clone();

        if let Err(ended_session) = session.refresh(invalid_refresh_token.clone()) {
            // Check if ended_session has correct properties
            assert_eq!(id, ended_session.id);
            assert_eq!(user_id, ended_session.user_id);
            assert_eq!(created_at, ended_session.created_at);

            assert!(ended_session.created_at < ended_session.state.session_end_time);
            assert_eq!(session_token_id, ended_session.state.latest_refresh_token.get_id().clone());

            let _ = if let SessionEndReason::AttemptedToReuseRefreshToken { caused_by } = ended_session.state.reason_for_ending {
                assert_eq!(caused_by.get_id(), invalid_refresh_token.get_id());
            } else {
                panic!("Failed to refresh for the wrong reason")
            };

        } else {
            panic!("Succeeded to refresh session")
        }
    }

    #[test]
    fn test_refresh_should_end_session_with_expired_used_refresh_token() {
        let session = UserSession::<NewlyCreated>::new(Uuid::new_v4().into());
        let now = Utc::now();
        let invalid_refresh_token = UserSessionToken::new(
            Uuid::new_v4(),
            Uuid::new_v4().to_string().to_string(),
            Uuid::new_v4().to_string().to_string(),
            Uuid::new_v4().to_string().to_string(),
            now - Duration::hours(2),
            now.clone(),
            now,
            RefreshToken {
                user_id: Uuid::new_v4(),
                session_id: Uuid::new_v4(),
                parent_id: None,
            }
        );

        let session_token_id = invalid_refresh_token.get_id().clone();
        let session: UserSession<Active> = UserSession {
            id: session.id,
            user_id: session.user_id,
            created_at: session.created_at,
            state: Active {
                latest_refresh_token: invalid_refresh_token.clone()
            },
        };

        let id = session.id.clone();
        let user_id = session.user_id.clone();
        let created_at = session.created_at.clone();

        if let Err(ended_session) = session.refresh(invalid_refresh_token.clone()) {
            // Check if ended_session has correct properties
            assert_eq!(id, ended_session.id);
            assert_eq!(user_id, ended_session.user_id);
            assert_eq!(created_at, ended_session.created_at);

            assert!(ended_session.created_at < ended_session.state.session_end_time);
            assert_eq!(session_token_id, ended_session.state.latest_refresh_token.get_id().clone());

            let _ = if let SessionEndReason::LatestRefreshTokenExpired { } = ended_session.state.reason_for_ending {

            } else {
                panic!("Failed to refresh for the wrong reason")
            };

        } else {
            panic!("Succeeded to refresh session")
        }
    }
    
    #[test]
    fn test_end_by_logout() {
        let user_id = Uuid::new_v4();
        let session = UserSession::<NewlyCreated>::new(user_id.into());
        let session: UserSession<Active> = UserSession {
            id: session.id,
            user_id: session.user_id,
            created_at: session.created_at,
            state: Active {
                latest_refresh_token: session.state.refresh_token.clone()
            },
        };
        
        let expected = UserSession {
            id: session.id.clone(),
            user_id: session.user_id.clone(),
            created_at: session.created_at.clone(),
            state: JustEnded {
                latest_refresh_token: session.state.latest_refresh_token.clone(),
                reason_for_ending: SessionEndReason::UserLogout,
                session_end_time: Utc::now(),
            }
        };
        
        let got = session.end_by_user_logout();
        
        assert_eq!(expected.id, got.id);
        assert_eq!(expected.user_id, got.user_id);
        assert_eq!(expected.created_at, got.created_at);
        assert_eq!(expected.state.latest_refresh_token.id, got.state.latest_refresh_token.id);
        assert_eq!(expected.state.reason_for_ending.to_string(), got.state.reason_for_ending.to_string());
        assert!(within_second(expected.state.session_end_time, got.state.session_end_time))
    }
    
    fn within_second(expected: DateTime<Utc>, got: DateTime<Utc>) -> bool {
        (got.timestamp_millis() - expected.timestamp_millis()) < Duration::seconds(1).num_milliseconds()
    }
}