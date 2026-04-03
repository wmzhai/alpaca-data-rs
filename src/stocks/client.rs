use std::sync::Arc;

use crate::{
    Error,
    client::Inner,
    common::response::{ResponseStream, empty_stream},
    transport::endpoint::Endpoint,
    transport::pagination::{collect_all, stream_pages},
};

use super::{
    BarsRequest, BarsResponse, BarsSingleRequest, BarsSingleResponse, ConditionCodesRequest,
    ConditionCodesResponse, ExchangeCodesResponse, LatestBarRequest, LatestBarResponse,
    LatestBarsRequest, LatestBarsResponse, LatestQuoteRequest, LatestQuoteResponse,
    LatestQuotesRequest, LatestQuotesResponse, LatestTradeRequest, LatestTradeResponse,
    LatestTradesRequest, LatestTradesResponse, QuotesRequest, QuotesResponse, QuotesSingleRequest,
    QuotesSingleResponse, SnapshotRequest, SnapshotResponse, SnapshotsRequest, SnapshotsResponse,
    TradesRequest, TradesResponse, TradesSingleRequest, TradesSingleResponse,
};

#[derive(Clone, Debug)]
pub struct StocksClient {
    inner: Arc<Inner>,
}

impl StocksClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub async fn bars(&self, request: BarsRequest) -> Result<BarsResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::StocksBars,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn bars_all(&self, _request: BarsRequest) -> Result<BarsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.bars_all",
        })
    }

    pub async fn bars_single(
        &self,
        request: BarsSingleRequest,
    ) -> Result<BarsSingleResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::StocksBarsSingle {
            symbol: request.symbol.clone(),
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

    pub async fn bars_single_all(
        &self,
        request: BarsSingleRequest,
    ) -> Result<BarsSingleResponse, Error> {
        self.ensure_credentials()?;
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.bars_single(request).await }
        })
        .await
    }

    pub fn bars_stream(
        &self,
        _request: BarsRequest,
    ) -> ResponseStream<Result<BarsResponse, Error>> {
        empty_stream()
    }

    pub fn bars_single_stream(
        &self,
        request: BarsSingleRequest,
    ) -> ResponseStream<Result<BarsSingleResponse, Error>> {
        if let Err(error) = self.ensure_credentials() {
            return Self::error_stream(error);
        }

        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.bars_single(request).await }
        })
    }

    pub async fn quotes(&self, request: QuotesRequest) -> Result<QuotesResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::StocksQuotes,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn quotes_all(&self, _request: QuotesRequest) -> Result<QuotesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.quotes_all",
        })
    }

    pub async fn quotes_single(
        &self,
        request: QuotesSingleRequest,
    ) -> Result<QuotesSingleResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::StocksQuotesSingle {
            symbol: request.symbol.clone(),
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

    pub async fn quotes_single_all(
        &self,
        request: QuotesSingleRequest,
    ) -> Result<QuotesSingleResponse, Error> {
        self.ensure_credentials()?;
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.quotes_single(request).await }
        })
        .await
    }

    pub fn quotes_stream(
        &self,
        _request: QuotesRequest,
    ) -> ResponseStream<Result<QuotesResponse, Error>> {
        empty_stream()
    }

    pub fn quotes_single_stream(
        &self,
        request: QuotesSingleRequest,
    ) -> ResponseStream<Result<QuotesSingleResponse, Error>> {
        if let Err(error) = self.ensure_credentials() {
            return Self::error_stream(error);
        }

        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.quotes_single(request).await }
        })
    }

    pub async fn trades(&self, request: TradesRequest) -> Result<TradesResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::StocksTrades,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn trades_all(&self, _request: TradesRequest) -> Result<TradesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.trades_all",
        })
    }

    pub async fn trades_single(
        &self,
        request: TradesSingleRequest,
    ) -> Result<TradesSingleResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::StocksTradesSingle {
            symbol: request.symbol.clone(),
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

    pub async fn trades_single_all(
        &self,
        request: TradesSingleRequest,
    ) -> Result<TradesSingleResponse, Error> {
        self.ensure_credentials()?;
        let client = self.clone();

        collect_all(request, move |request| {
            let client = client.clone();
            async move { client.trades_single(request).await }
        })
        .await
    }

    pub fn trades_stream(
        &self,
        _request: TradesRequest,
    ) -> ResponseStream<Result<TradesResponse, Error>> {
        empty_stream()
    }

    pub fn trades_single_stream(
        &self,
        request: TradesSingleRequest,
    ) -> ResponseStream<Result<TradesSingleResponse, Error>> {
        if let Err(error) = self.ensure_credentials() {
            return Self::error_stream(error);
        }

        let client = self.clone();
        stream_pages(request, move |request| {
            let client = client.clone();
            async move { client.trades_single(request).await }
        })
    }

    pub async fn latest_bars(
        &self,
        request: LatestBarsRequest,
    ) -> Result<LatestBarsResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::StocksLatestBars,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn latest_bar(&self, request: LatestBarRequest) -> Result<LatestBarResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::StocksLatestBar {
            symbol: request.symbol.clone(),
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

    pub async fn latest_quotes(
        &self,
        request: LatestQuotesRequest,
    ) -> Result<LatestQuotesResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::StocksLatestQuotes,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn latest_quote(
        &self,
        request: LatestQuoteRequest,
    ) -> Result<LatestQuoteResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::StocksLatestQuote {
            symbol: request.symbol.clone(),
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

    pub async fn latest_trades(
        &self,
        request: LatestTradesRequest,
    ) -> Result<LatestTradesResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::StocksLatestTrades,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn latest_trade(
        &self,
        request: LatestTradeRequest,
    ) -> Result<LatestTradeResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::StocksLatestTrade {
            symbol: request.symbol.clone(),
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

    pub async fn snapshots(&self, request: SnapshotsRequest) -> Result<SnapshotsResponse, Error> {
        self.ensure_credentials()?;
        self.inner
            .http
            .get_json(
                &self.inner.base_url,
                Endpoint::StocksSnapshots,
                &self.inner.auth,
                request.to_query(),
            )
            .await
    }

    pub async fn snapshot(&self, request: SnapshotRequest) -> Result<SnapshotResponse, Error> {
        self.ensure_credentials()?;
        let endpoint = Endpoint::StocksSnapshot {
            symbol: request.symbol.clone(),
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

    pub async fn condition_codes(
        &self,
        _request: ConditionCodesRequest,
    ) -> Result<ConditionCodesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.condition_codes",
        })
    }

    pub async fn exchange_codes(&self) -> Result<ExchangeCodesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.exchange_codes",
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
