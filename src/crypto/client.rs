use std::sync::Arc;

use crate::{
    Error,
    client::Inner,
    common::response::ResponseStream,
    transport::endpoint::Endpoint,
    transport::pagination::{collect_all, stream_pages},
};

use super::{
    BarsRequest, BarsResponse, LatestBarsRequest, LatestBarsResponse, LatestOrderbooksRequest,
    LatestOrderbooksResponse, LatestQuotesRequest, LatestQuotesResponse, LatestTradesRequest,
    LatestTradesResponse, QuotesRequest, QuotesResponse, SnapshotsRequest, SnapshotsResponse,
    TradesRequest, TradesResponse,
};

#[derive(Clone, Debug)]
pub struct CryptoClient {
    inner: Arc<Inner>,
}

impl CryptoClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub async fn bars(&self, request: BarsRequest) -> Result<BarsResponse, Error> {
        let endpoint = Endpoint::crypto_bars(request.loc.unwrap_or_default());
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                endpoint,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn bars_all(&self, request: BarsRequest) -> Result<BarsResponse, Error> {
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.bars(request).await }
        })
        .await
    }

    pub fn bars_stream(&self, request: BarsRequest) -> ResponseStream<Result<BarsResponse, Error>> {
        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.bars(request).await }
        })
    }

    pub async fn quotes(&self, request: QuotesRequest) -> Result<QuotesResponse, Error> {
        let endpoint = Endpoint::crypto_quotes(request.loc.unwrap_or_default());
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                endpoint,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn quotes_all(&self, request: QuotesRequest) -> Result<QuotesResponse, Error> {
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.quotes(request).await }
        })
        .await
    }

    pub fn quotes_stream(
        &self,
        request: QuotesRequest,
    ) -> ResponseStream<Result<QuotesResponse, Error>> {
        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.quotes(request).await }
        })
    }

    pub async fn trades(&self, request: TradesRequest) -> Result<TradesResponse, Error> {
        let endpoint = Endpoint::crypto_trades(request.loc.unwrap_or_default());
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                endpoint,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn trades_all(&self, request: TradesRequest) -> Result<TradesResponse, Error> {
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.trades(request).await }
        })
        .await
    }

    pub fn trades_stream(
        &self,
        request: TradesRequest,
    ) -> ResponseStream<Result<TradesResponse, Error>> {
        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.trades(request).await }
        })
    }

    pub async fn latest_bars(
        &self,
        request: LatestBarsRequest,
    ) -> Result<LatestBarsResponse, Error> {
        let endpoint = Endpoint::crypto_latest_bars(request.loc.unwrap_or_default());
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                endpoint,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn latest_quotes(
        &self,
        request: LatestQuotesRequest,
    ) -> Result<LatestQuotesResponse, Error> {
        let endpoint = Endpoint::crypto_latest_quotes(request.loc.unwrap_or_default());

        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                endpoint,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn latest_trades(
        &self,
        request: LatestTradesRequest,
    ) -> Result<LatestTradesResponse, Error> {
        let endpoint = Endpoint::crypto_latest_trades(request.loc.unwrap_or_default());
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                endpoint,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn latest_orderbooks(
        &self,
        request: LatestOrderbooksRequest,
    ) -> Result<LatestOrderbooksResponse, Error> {
        let endpoint = Endpoint::crypto_latest_orderbooks(request.loc.unwrap_or_default());
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                endpoint,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn snapshots(&self, _request: SnapshotsRequest) -> Result<SnapshotsResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.snapshots",
        })
    }
}
