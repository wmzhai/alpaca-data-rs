use std::sync::Arc;

use crate::{
    Error,
    client::Inner,
    common::response::{ResponseStream, empty_stream},
};

use super::{ListRequest, ListResponse};

#[derive(Clone, Debug)]
pub struct NewsClient {
    inner: Arc<Inner>,
}

impl NewsClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub async fn list(&self, _request: ListRequest) -> Result<ListResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "news.list",
        })
    }

    pub async fn list_all(&self, _request: ListRequest) -> Result<ListResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "news.list_all",
        })
    }

    pub fn list_stream(
        &self,
        _request: ListRequest,
    ) -> ResponseStream<Result<ListResponse, Error>> {
        empty_stream()
    }

    fn ensure_credentials(&self) -> Result<(), Error> {
        if self.inner.auth.has_credentials() {
            Ok(())
        } else {
            Err(Error::MissingCredentials)
        }
    }
}
