use gpgme::{Context, Protocol};
use std::fmt;

#[derive(Debug)]
pub enum CryptoError {
    WrongPassword,
    NoData,
    GpgError(gpgme::Error),
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::WrongPassword => write!(f, "Incorrect password"),
            CryptoError::NoData => write!(f, "No data in encrypted file"),
            CryptoError::GpgError(e) => write!(f, "GPG error: {}", e),
            CryptoError::Utf8Error(e) => write!(f, "Invalid UTF-8 in decrypted content: {}", e),
        }
    }
}

impl From<gpgme::Error> for CryptoError {
    fn from(e: gpgme::Error) -> Self {
        // Check for common error codes that indicate wrong password
        // GPG_ERR_BAD_PASSPHRASE = 11, GPG_ERR_DECRYPT_FAILED = 152
        const GPG_ERR_BAD_PASSPHRASE: gpgme::error::ErrorCode = 11;
        const GPG_ERR_DECRYPT_FAILED: gpgme::error::ErrorCode = 152;

        if e.code() == GPG_ERR_BAD_PASSPHRASE || e.code() == GPG_ERR_DECRYPT_FAILED {
            CryptoError::WrongPassword
        } else {
            CryptoError::GpgError(e)
        }
    }
}

impl From<std::string::FromUtf8Error> for CryptoError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        CryptoError::Utf8Error(e)
    }
}

/// Decrypt GPG-encrypted data using a password/passphrase.
///
/// This function uses symmetric (password-based) decryption only.
/// Key-based decryption is not supported.
pub fn decrypt_with_password(encrypted_data: &[u8], password: &str) -> Result<String, CryptoError> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;

    // Set the passphrase provider
    ctx.set_pinentry_mode(gpgme::PinentryMode::Loopback)?;
    ctx.with_passphrase_provider(|_req: gpgme::PassphraseRequest, out: &mut dyn std::io::Write| {
        out.write_all(password.as_bytes())?;
        Ok(())
    }, |ctx| {
        let mut plaintext = Vec::new();
        ctx.decrypt(encrypted_data, &mut plaintext)?;

        if plaintext.is_empty() {
            return Err(CryptoError::NoData);
        }

        String::from_utf8(plaintext).map_err(CryptoError::from)
    })
}
