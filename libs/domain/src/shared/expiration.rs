use chrono::{DateTime, Duration, Utc};

/// Expiration represents a deadline/expiration for given `DateTime<Utc>`.
#[derive(Copy, Clone, Debug)]
pub struct Expiration(pub DateTime<Utc>);

impl Expiration {
    pub fn has_passed(&self) -> bool {
        self.has_passed_at(Utc::now())
    }

    pub fn has_passed_at(&self, time: DateTime<Utc>) -> bool {
        time > self.0
    }
}

impl From<Duration> for Expiration {
    fn from(duration: Duration) -> Self {
        Self(Utc::now() + duration)
    }
}

impl From<DateTime<Utc>> for Expiration {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl Into<DateTime<Utc>> for Expiration {
    fn into(self) -> DateTime<Utc> {
        self.0
    }
}

impl Into<Duration> for Expiration {

    /// Returns the duration until expiration
    fn into(self) -> Duration {
        if self.has_passed() {
            return Duration::nanoseconds(0)
        }

        (self.0 - Utc::now()).abs().into()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Duration, Utc};
    use crate::shared::expiration::Expiration;

    fn within_duration(date_time_1: DateTime<Utc>, date_time_2: DateTime<Utc>, duration: Duration) -> bool {
        let difference = (date_time_1 - date_time_2).abs();
        difference <= duration
    }

    #[test]
    fn test_new_expiration_from_duration() {
        let duration = Duration::hours(2);
        let expiration: Expiration = duration.clone().into();
        let expiration_time = Utc::now() + duration;
        
        assert!(within_duration(expiration.0, expiration_time, Duration::seconds(1)))
    }

    #[test]
    fn test_new_expiration_from_datetime() {
        let now = Utc::now();
        let expiration: Expiration = now.into();
        assert_eq!(expiration.0, now)
    }

    #[test]
    fn test_has_passed() {
        let expiration: Expiration = Duration::hours(2).into();

        assert!(!expiration.has_passed());
        assert!(expiration.has_passed_at(Utc::now() + Duration::hours(3)));

        let expiration: Expiration = Duration::hours(-2).into();
        assert!(expiration.has_passed())
    }

    #[test]
    fn test_expiration_into() {
        let now = Utc::now();
        let after = now + Duration::hours(2);
        let expiration: Expiration = after.into();
        assert_eq!(after, expiration.into());

        let difference: Duration = after - now;
        assert!(difference - expiration.into() <= Duration::seconds(1));

        // before
        let now = Utc::now();
        let before = now - Duration::hours(2);
        let expiration: Expiration = before.into();
        assert_eq!(before, expiration.into());

        let duration_until_expiration: Duration = expiration.into();
        assert_eq!(duration_until_expiration, Duration::nanoseconds(0))
    }
}