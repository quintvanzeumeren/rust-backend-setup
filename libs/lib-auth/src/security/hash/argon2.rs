use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use argon2::password_hash::SaltString;
use password_hash::{PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use crate::security::hash::error::Error;
use crate::security::hash::schema::Scheme;


/// As of march 2024:
/// Use Argon2id with a minimum configuration of 19 MiB of memory, an iteration count of 2,
/// and 1 degree of parallelism.
/// Source:
/// https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#introduction
/// https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
/// 

#[derive(Debug)]
pub struct Argon2Scheme;

/// M_COST equals the minimum memory size that should be used
const M_COST: u32 = 19456; // 19MB

/// T_COST equals the number of iterations
const T_COST: u32 = 2;

/// P_COST equals the degree of parallelism that should be used.
const P_COST: u32 = 1;

impl Scheme for Argon2Scheme {
    fn hash<'a>(&self, to_hash: Secret<String>, salt_string: &'a SaltString) -> password_hash::Result<PasswordHash<'a>> {
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(M_COST, T_COST, P_COST, None).unwrap(),
        )
            .hash_password(to_hash.expose_secret().as_bytes(), salt_string)?;

        Ok(password_hash)
    }

    fn validate(&self, to_hash: &Secret<String>, password_hash: &PasswordHash) -> Result<(), Error> {
		Argon2::default()
			.verify_password(to_hash.expose_secret().as_bytes(), &password_hash)
			.map_err(Error::from)?;
		
        Ok(())
    }
}

/* TODO:

    - Continue the implementation of the Argon2Schema
    - Figure out what you should return as possible errors
    - After which, we need to figure out how to save it properly has a PHC string format.
    - Add the database to the tests, and create a test user to add a user within the test database.
    - After which we can test if we can authenticate a existing user.

 */

#[cfg(test)]
mod tests {
	use password_hash::SaltString;
	use secrecy::{ExposeSecret, Secret};
	use uuid::Uuid;
	use crate::security::hash::argon2::Argon2Scheme;
	use crate::security::hash::schema::Scheme;


	fn password() -> Secret<String> {
		Secret::new(Uuid::new_v4().to_string())
	}
	
	fn salt() -> SaltString {
		SaltString::generate(&mut rand::thread_rng())
	}

	#[test]
	fn test_argon2_id_schema() {
		let pw1 = password();
		let pw1_salt = salt();

		let hashed_pw1 = Argon2Scheme.hash(pw1.clone(), &pw1_salt)
			.unwrap();

		// Test pw1 equals its own hash
		let result = Argon2Scheme.validate(&pw1, &hashed_pw1);
		assert!(result.is_ok());

		// Test if pw2 does not equals pw1 hash
		let pw2 = password();
		assert_ne!(pw1.expose_secret(), pw2.expose_secret(), "pw1 and pw2 should not be equal");

		let result = Argon2Scheme.validate(&pw2, &hashed_pw1);
		assert!(result.is_err());
		// assert_eq!(result.err().unwrap(), PasswordInvalid);

		// Test if pw2 equals its own hash
		let pw2_salt = salt();
		let hashed_pw2 = Argon2Scheme.hash(pw2.clone(), &pw2_salt)
			.unwrap();

		let result = Argon2Scheme.validate(&pw2, &hashed_pw2);
		assert!(result.is_ok());

		// Test if pw1 does not equal pw2 hash
		let result = Argon2Scheme.validate(&pw1, &hashed_pw2);
		assert!(result.is_err());
		// assert_eq!(result.err().unwrap(), PasswordInvalid);
	}
}