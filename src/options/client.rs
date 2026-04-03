use std::sync::Arc;

use crate::{
    Error,
    client::Inner,
    common::response::{ResponseStream, empty_stream},
};

use super::{
    BarsRequest, BarsResponse, ChainRequest, ChainResponse, ExchangeCodesResponse,
    LatestQuotesRequest, LatestQuotesResponse, LatestTradesRequest, LatestTradesResponse,
    SnapshotsRequest, SnapshotsResponse, TradesRequest, TradesResponse,
};

#[derive(Clone, Debug)]
pub struct OptionsClient {
    inner: Arc<Inner>,
}

impl OptionsClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub async fn bars(&self, _request: BarsRequest) -> Result<BarsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.bars",
        })
    }

    pub async fn bars_all(&self, _request: BarsRequest) -> Result<BarsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.bars_all",
        })
    }

    pub fn bars_stream(
        &self,
        _request: BarsRequest,
    ) -> ResponseStream<Result<BarsResponse, Error>> {
        empty_stream()
    }

    pub async fn trades(&self, _request: TradesRequest) -> Result<TradesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.trades",
        })
    }

    pub async fn trades_all(&self, _request: TradesRequest) -> Result<TradesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.trades_all",
        })
    }

    pub fn trades_stream(
        &self,
        _request: TradesRequest,
    ) -> ResponseStream<Result<TradesResponse, Error>> {
        empty_stream()
    }

    pub async fn latest_quotes(
        &self,
        _request: LatestQuotesRequest,
    ) -> Result<LatestQuotesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.latest_quotes",
        })
    }

    pub async fn latest_trades(
        &self,
        _request: LatestTradesRequest,
    ) -> Result<LatestTradesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.latest_trades",
        })
    }

    pub async fn snapshots(&self, _request: SnapshotsRequest) -> Result<SnapshotsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.snapshots",
        })
    }

    pub async fn snapshots_all(
        &self,
        _request: SnapshotsRequest,
    ) -> Result<SnapshotsResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.snapshots_all",
        })
    }

    pub fn snapshots_stream(
        &self,
        _request: SnapshotsRequest,
    ) -> ResponseStream<Result<SnapshotsResponse, Error>> {
        empty_stream()
    }

    pub async fn chain(&self, _request: ChainRequest) -> Result<ChainResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.chain",
        })
    }

    pub async fn chain_all(&self, _request: ChainRequest) -> Result<ChainResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.chain_all",
        })
    }

    pub fn chain_stream(
        &self,
        _request: ChainRequest,
    ) -> ResponseStream<Result<ChainResponse, Error>> {
        empty_stream()
    }

    pub async fn exchange_codes(&self) -> Result<ExchangeCodesResponse, Error> {
        self.ensure_credentials()?;
        Err(Error::NotImplemented {
            operation: "options.exchange_codes",
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
