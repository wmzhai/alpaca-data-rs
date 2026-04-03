use std::sync::Arc;

use crate::{
    Error,
    client::Inner,
    common::response::{ResponseStream, empty_stream},
};

use super::{
    BarsRequest, BarsResponse, BarsSingleRequest, BarsSingleResponse, ConditionCodesRequest,
    ConditionCodesResponse, ExchangeCodesResponse, LatestBarsRequest, LatestBarsResponse,
    LatestQuoteRequest, LatestQuoteResponse, LatestQuotesRequest, LatestQuotesResponse,
    LatestTradesRequest, LatestTradesResponse, QuotesRequest, QuotesResponse, SnapshotRequest,
    SnapshotResponse, SnapshotsRequest, SnapshotsResponse, TradesRequest, TradesResponse,
};

#[derive(Clone, Debug)]
pub struct StocksClient {
    inner: Arc<Inner>,
}

impl StocksClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub async fn bars(&self, _request: BarsRequest) -> Result<BarsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.bars",
        })
    }

    pub async fn bars_all(&self, _request: BarsRequest) -> Result<BarsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.bars_all",
        })
    }

    pub async fn bars_single(
        &self,
        _request: BarsSingleRequest,
    ) -> Result<BarsSingleResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.bars_single",
        })
    }

    pub fn bars_stream(
        &self,
        _request: BarsRequest,
    ) -> ResponseStream<Result<BarsResponse, Error>> {
        empty_stream()
    }

    pub async fn quotes(&self, _request: QuotesRequest) -> Result<QuotesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.quotes",
        })
    }

    pub async fn quotes_all(&self, _request: QuotesRequest) -> Result<QuotesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.quotes_all",
        })
    }

    pub fn quotes_stream(
        &self,
        _request: QuotesRequest,
    ) -> ResponseStream<Result<QuotesResponse, Error>> {
        empty_stream()
    }

    pub async fn trades(&self, _request: TradesRequest) -> Result<TradesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.trades",
        })
    }

    pub async fn trades_all(&self, _request: TradesRequest) -> Result<TradesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.trades_all",
        })
    }

    pub fn trades_stream(
        &self,
        _request: TradesRequest,
    ) -> ResponseStream<Result<TradesResponse, Error>> {
        empty_stream()
    }

    pub async fn latest_bars(
        &self,
        _request: LatestBarsRequest,
    ) -> Result<LatestBarsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.latest_bars",
        })
    }

    pub async fn latest_quotes(
        &self,
        _request: LatestQuotesRequest,
    ) -> Result<LatestQuotesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.latest_quotes",
        })
    }

    pub async fn latest_quote(
        &self,
        _request: LatestQuoteRequest,
    ) -> Result<LatestQuoteResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.latest_quote",
        })
    }

    pub async fn latest_trades(
        &self,
        _request: LatestTradesRequest,
    ) -> Result<LatestTradesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.latest_trades",
        })
    }

    pub async fn snapshots(&self, _request: SnapshotsRequest) -> Result<SnapshotsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.snapshots",
        })
    }

    pub async fn snapshot(&self, _request: SnapshotRequest) -> Result<SnapshotResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "stocks.snapshot",
        })
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
}
