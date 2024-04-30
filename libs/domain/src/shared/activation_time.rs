use chrono::{DateTime, Duration, Utc};
use crate::shared::expiration::Expiration;

/// Activation is time attribute that determines if something is ready to be used as of
/// the current date and time.
///
#[derive(Clone, Copy, Debug)]
pub struct ActivationTime(pub Expiration);

impl ActivationTime {
    fn has_passed(&self) -> bool {
        self.0.has_passed()
    }

    fn has_not_passed(&self) -> bool {
        !self.has_passed()
    }
}

impl From<DateTime<Utc>> for ActivationTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value.into())
    }
}

impl From<Duration> for ActivationTime {
    fn from(value: Duration) -> Self {
        Self(value.into())
    }
}

impl Into<DateTime<Utc>> for ActivationTime {
    fn into(self) -> DateTime<Utc> {
        self.0.into()
    }
}

impl Into<Duration> for ActivationTime {

    /// Returns the duration until the activation time have passed
    fn into(self) -> Duration {
        self.0.into()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Duration, Utc};
    use crate::shared::activation_time::ActivationTime;

    fn within_duration(date_time_1: DateTime<Utc>, date_time_2: DateTime<Utc>, duration: Duration) -> bool {
        let difference = (date_time_1 - date_time_2).abs();
        difference <= duration
    }

    #[test]
    fn test_has_passed() {
        let now = Utc::now();
        let before = now - Duration::hours(2);

        let activation = ActivationTime::from(before);
        assert_eq!(before, activation.0.into());
        assert!(activation.has_passed());
        assert_ne!(activation.has_passed(), activation.has_not_passed());
    }

    #[test]
    fn test_has_not_passed() {
        let now = Utc::now();
        let soon = now + Duration::hours(2);

        let activation = ActivationTime::from(soon);
        assert_eq!(soon, activation.0.into());
        assert!(activation.has_not_passed());
        assert_ne!(activation.has_passed(), activation.has_not_passed());
    }

    #[test]
    fn test_type_conversions() {
        let now = Utc::now();
        let soon = now + Duration::hours(2);
        let activation_time = ActivationTime::from(Duration::hours(2));
        assert!(within_duration(soon, activation_time.clone().into(), Duration::seconds(1)));

        let duration: Duration = activation_time.into();
        assert!(within_duration(soon, now + duration, Duration::seconds(1)))
    }
}