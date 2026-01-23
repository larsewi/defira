use gpgme::{Context, Protocol};
use std::fmt;

#[derive(Debug)]
pub enum CryptoError {
    NoData,
    GpgError(gpgme::Error),
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::NoData => write!(f, "No data in encrypted file"),
            CryptoError::GpgError(e) => write!(f, "GPG error: {}", e),
            CryptoError::Utf8Error(e) => write!(f, "Invalid UTF-8 in decrypted content: {}", e),
        }
    }
}

impl From<gpgme::Error> for CryptoError {
    fn from(e: gpgme::Error) -> Self {
        CryptoError::GpgError(e)
    }
}

impl From<std::string::FromUtf8Error> for CryptoError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        CryptoError::Utf8Error(e)
    }
}

/// Decrypt GPG-encrypted data using the gpg-agent's cached passphrase.
///
/// This function attempts decryption without prompting for a password,
/// relying on the gpg-agent having the passphrase cached. If the passphrase
/// is not cached, this will fail with an error.
pub fn decrypt(encrypted_data: &[u8]) -> Result<String, CryptoError> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;

    let mut plaintext = Vec::new();
    ctx.decrypt(encrypted_data, &mut plaintext)?;

    if plaintext.is_empty() {
        return Err(CryptoError::NoData);
    }

    String::from_utf8(plaintext).map_err(CryptoError::from)
}

