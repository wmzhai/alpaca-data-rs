use crate::Error;

#[derive(Clone, Debug, Default)]
pub(crate) struct Auth {
    pub(crate) api_key: Option<String>,
    pub(crate) secret_key: Option<String>,
}

impl Auth {
    pub(crate) fn new(api_key: Option<String>, secret_key: Option<String>) -> Result<Self, Error> {
        match (api_key, secret_key) {
            (Some(api_key), Some(secret_key)) => Ok(Self {
                api_key: Some(api_key),
                secret_key: Some(secret_key),
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
}
