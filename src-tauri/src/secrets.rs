use crate::lifecycle::SecretProbe;

const SERVICE_NAME: &str = "lazyme";
const API_KEY_ACCOUNT: &str = "api_key";

/// Windows Credential Manager adapter implementing `SecretProbe`.
pub struct KeyringSecretProbe;

impl SecretProbe for KeyringSecretProbe {
    fn has_api_key(&self) -> Result<bool, String> {
        let entry =
            keyring::Entry::new(SERVICE_NAME, API_KEY_ACCOUNT).map_err(|e| e.to_string())?;
        match entry.get_password() {
            Ok(secret) => Ok(!secret.trim().is_empty()),
            Err(keyring::Error::NoEntry) => Ok(false),
            Err(err) => Err(err.to_string()),
        }
    }
}
