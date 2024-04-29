use std::error::Error;
use secrecy::Secret;

/// A trait representing a decryptor capable of decrypting an encrypted token of type `Secret<String>`
/// and producing data of type `T`.
pub trait Decryptor<T: Sized>: Clone {

    /// The type representing potential errors that may occur during the decryption process.
    type DecryptionError: Error;

    /// Decrypts an encrypted token of type `Secret<String>` and produces data of type `T`.
    ///
    /// # Parameters
    ///
    /// - `encrypted_token`: The encrypted token to be decrypted.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decrypted data (`T`) if successful, or an error of type
    /// `Self::DecryptionError` if decryption fails.
    fn decrypt(&self, encrypted_token: &Secret<String>) -> Result<T, Self::DecryptionError>;
}