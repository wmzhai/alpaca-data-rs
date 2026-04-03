#[derive(Clone, Debug, Default)]
pub(crate) struct Auth {
    pub(crate) api_key: Option<String>,
    pub(crate) secret_key: Option<String>,
}

impl Auth {
    pub(crate) fn has_credentials(&self) -> bool {
        self.api_key.is_some() && self.secret_key.is_some()
    }
}
