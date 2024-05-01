use std::error::Error;

/// A trait representing an encryptor capable of encrypting data of type `T` and producing
/// encrypted data of type `U`.
pub trait Encryptor<'a, T, U>: Clone {
    /// The type representing potential errors that may occur during the encryption process.
    type EncryptionError: Error;

    /// Encrypts data of type `T` and produces encrypted data of type `U`.
    fn encrypt(&self, encrypt: &'a T) -> Result<U, Self::EncryptionError>;
}
