use std::sync::Arc;

use crate::{
    Error,
    client::Inner,
    common::response::ResponseStream,
    transport::endpoint::Endpoint,
    transport::pagination::{collect_all, stream_pages},
};

use super::{ListRequest, ListResponse};

#[derive(Clone, Debug)]
pub struct CorporateActionsClient {
    inner: Arc<Inner>,
}

impl CorporateActionsClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub async fn list(&self, request: ListRequest) -> Result<ListResponse, Error> {
        self.ensure_credentials()?;
        request.validate()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::CorporateActionsList,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn list_all(&self, request: ListRequest) -> Result<ListResponse, Error> {
        self.ensure_credentials()?;
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.list(request).await }
        })
        .await
    }

    pub fn list_stream(&self, request: ListRequest) -> ResponseStream<Result<ListResponse, Error>> {
        if let Err(error) = self.ensure_credentials() {
            return Self::error_stream(error);
        }

        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.list(request).await }
        })
    }

    fn ensure_credentials(&self) -> Result<(), Error> {
        if self.inner.auth.has_credentials() {
            Ok(())
        } else {
            Err(Error::MissingCredentials)
        }
    }

    fn error_stream<Response>(error: Error) -> ResponseStream<Result<Response, Error>>
    where
        Response: Send + 'static,
    {
        Box::pin(futures_util::stream::once(async move { Err(error) }))
    }
}
