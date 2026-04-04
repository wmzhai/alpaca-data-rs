use std::sync::Arc;

use crate::{
    Error,
    client::Inner,
    common::response::ResponseStream,
    transport::endpoint::Endpoint,
    transport::pagination::{collect_all, stream_pages},
};

use super::{
    BarsRequest, BarsResponse, ChainRequest, ChainResponse, ConditionCodesRequest,
    ConditionCodesResponse, ExchangeCodesResponse, LatestQuotesRequest, LatestQuotesResponse,
    LatestTradesRequest, LatestTradesResponse, SnapshotsRequest, SnapshotsResponse, TradesRequest,
    TradesResponse,
};

#[derive(Clone, Debug)]
pub struct OptionsClient {
    inner: Arc<Inner>,
}

impl OptionsClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub async fn bars(&self, request: BarsRequest) -> Result<BarsResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::OptionsBars,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn bars_all(&self, request: BarsRequest) -> Result<BarsResponse, Error> {
        self.ensure_credentials()?;
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.bars(request).await }
        })
        .await
    }

    pub fn bars_stream(&self, request: BarsRequest) -> ResponseStream<Result<BarsResponse, Error>> {
        if let Err(error) = self.ensure_credentials() {
            return Self::error_stream(error);
        }

        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.bars(request).await }
        })
    }

    pub async fn trades(&self, request: TradesRequest) -> Result<TradesResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::OptionsTrades,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn trades_all(&self, request: TradesRequest) -> Result<TradesResponse, Error> {
        self.ensure_credentials()?;
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
        if let Err(error) = self.ensure_credentials() {
            return Self::error_stream(error);
        }

        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.trades(request).await }
        })
    }

    pub async fn latest_quotes(
        &self,
        request: LatestQuotesRequest,
    ) -> Result<LatestQuotesResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::OptionsLatestQuotes,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn latest_trades(
        &self,
        request: LatestTradesRequest,
    ) -> Result<LatestTradesResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::OptionsLatestTrades,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn snapshots(&self, request: SnapshotsRequest) -> Result<SnapshotsResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::OptionsSnapshots,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn snapshots_all(
        &self,
        request: SnapshotsRequest,
    ) -> Result<SnapshotsResponse, Error> {
        self.ensure_credentials()?;
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.snapshots(request).await }
        })
        .await
    }

    pub fn snapshots_stream(
        &self,
        request: SnapshotsRequest,
    ) -> ResponseStream<Result<SnapshotsResponse, Error>> {
        if let Err(error) = self.ensure_credentials() {
            return Self::error_stream(error);
        }

        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.snapshots(request).await }
        })
    }

    pub async fn chain(&self, request: ChainRequest) -> Result<ChainResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::OptionsChain {
            underlying_symbol: request.underlying_symbol.clone(),
        };
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

    pub async fn chain_all(&self, request: ChainRequest) -> Result<ChainResponse, Error> {
        self.ensure_credentials()?;
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.chain(request).await }
        })
        .await
    }

    pub fn chain_stream(
        &self,
        request: ChainRequest,
    ) -> ResponseStream<Result<ChainResponse, Error>> {
        if let Err(error) = self.ensure_credentials() {
            return Self::error_stream(error);
        }

        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.chain(request).await }
        })
    }

    pub async fn exchange_codes(&self) -> Result<ExchangeCodesResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::OptionsExchangeCodes,
                &self.inner.auth,
                Vec::new(),
            )
            .await
    }

    pub async fn condition_codes(
        &self,
        request: ConditionCodesRequest,
    ) -> Result<ConditionCodesResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::OptionsConditionCodes {
            ticktype: request.ticktype(),
        };
        self.inner
            .http
            .get_json(&self.inner.base_url, endpoint, &self.inner.auth, Vec::new())
            .await
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
