use crate::Error;

pub(crate) const DEFAULT_API_KEY_ENV: &str = "APCA_API_KEY_ID";
pub(crate) const DEFAULT_SECRET_KEY_ENV: &str = "APCA_API_SECRET_KEY";

pub(crate) fn credentials_from_env_names(
    api_key_var: &str,
    secret_key_var: &str,
) -> Result<Option<(String, String)>, Error> {
    let api_key = std::env::var(api_key_var).ok();
    let secret_key = std::env::var(secret_key_var).ok();

    match (api_key, secret_key) {
        (Some(api_key), Some(secret_key)) => Ok(Some((api_key, secret_key))),
        (None, None) => Ok(None),
        _ => Err(Error::InvalidConfiguration(format!(
            "{api_key_var} and {secret_key_var} must be paired"
        ))),
    }
}
