use std::fmt;

use crate::Error;
use reqwest::{RequestBuilder, header::HeaderValue};

#[derive(Clone, Default)]
pub(crate) struct Auth {
    pub(crate) api_key: Option<String>,
    pub(crate) secret_key: Option<String>,
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Auth")
            .field("api_key", &RedactedCredential(&self.api_key))
            .field("secret_key", &RedactedCredential(&self.secret_key))
            .finish()
    }
}

impl Auth {
    pub(crate) fn new(api_key: Option<String>, secret_key: Option<String>) -> Result<Self, Error> {
        match (api_key, secret_key) {
            (Some(api_key), Some(secret_key)) => Ok(Self {
                api_key: Some(validate_credential("api_key", api_key)?),
                secret_key: Some(validate_credential("secret_key", secret_key)?),
            }),
            (None, None) => Ok(Self::default()),
            _ => Err(Error::InvalidConfiguration(
                "api_key and secret_key must be paired".into(),
            )),
        }
    }

    pub(crate) fn has_credentials(&self) -> bool {
        self.api_key.is_some() && self.secret_key.is_some()
    }

    pub(crate) fn apply(
        &self,
        request: RequestBuilder,
        requires_auth: bool,
    ) -> Result<RequestBuilder, Error> {
        if !requires_auth {
            return Ok(request);
        }

        match (&self.api_key, &self.secret_key) {
            (Some(api_key), Some(secret_key)) => Ok(request
                .header("APCA-API-KEY-ID", api_key)
                .header("APCA-API-SECRET-KEY", secret_key)),
            _ => Err(Error::MissingCredentials),
        }
    }
}

fn validate_credential(name: &str, value: String) -> Result<String, Error> {
    if value.trim().is_empty() {
        return Err(Error::InvalidConfiguration(format!(
            "{name} must not be blank or whitespace-only"
        )));
    }

    HeaderValue::from_str(&value).map_err(|_| {
        Error::InvalidConfiguration(format!("{name} must be a valid HTTP header value"))
    })?;

    Ok(value)
}

struct RedactedCredential<'a>(&'a Option<String>);

impl fmt::Debug for RedactedCredential<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(_) => f.write_str("\"[REDACTED]\""),
            None => f.write_str("None"),
        }
    }
}
