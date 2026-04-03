use std::sync::Arc;

use crate::{
    Error,
    client::Inner,
    common::response::{ResponseStream, empty_stream},
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

    pub async fn bars(&self, _request: BarsRequest) -> Result<BarsResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.bars",
        })
    }

    pub async fn bars_all(&self, _request: BarsRequest) -> Result<BarsResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.bars_all",
        })
    }

    pub fn bars_stream(
        &self,
        _request: BarsRequest,
    ) -> ResponseStream<Result<BarsResponse, Error>> {
        empty_stream()
    }

    pub async fn quotes(&self, _request: QuotesRequest) -> Result<QuotesResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.quotes",
        })
    }

    pub async fn quotes_all(&self, _request: QuotesRequest) -> Result<QuotesResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.quotes_all",
        })
    }

    pub fn quotes_stream(
        &self,
        _request: QuotesRequest,
    ) -> ResponseStream<Result<QuotesResponse, Error>> {
        empty_stream()
    }

    pub async fn trades(&self, _request: TradesRequest) -> Result<TradesResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.trades",
        })
    }

    pub async fn trades_all(&self, _request: TradesRequest) -> Result<TradesResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.trades_all",
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
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.latest_bars",
        })
    }

    pub async fn latest_quotes(
        &self,
        _request: LatestQuotesRequest,
    ) -> Result<LatestQuotesResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.latest_quotes",
        })
    }

    pub async fn latest_trades(
        &self,
        _request: LatestTradesRequest,
    ) -> Result<LatestTradesResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.latest_trades",
        })
    }

    pub async fn latest_orderbooks(
        &self,
        _request: LatestOrderbooksRequest,
    ) -> Result<LatestOrderbooksResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.latest_orderbooks",
        })
    }

    pub async fn snapshots(&self, _request: SnapshotsRequest) -> Result<SnapshotsResponse, Error> {
        let _ = &self.inner;
        Err(Error::NotImplemented {
            operation: "crypto.snapshots",
        })
    }
}
