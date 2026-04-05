use crate::Error;
use crate::common::query::QueryWriter;
use crate::transport::pagination::PaginatedRequest;

use super::{Loc, Sort, TimeFrame};

#[derive(Clone, Debug, Default)]
pub struct BarsRequest {
    pub symbols: Vec<String>,
    pub timeframe: TimeFrame,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub loc: Option<Loc>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct QuotesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub loc: Option<Loc>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TradesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub loc: Option<Loc>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestBarsRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestQuotesRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestTradesRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestOrderbooksRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, Default)]
pub struct SnapshotsRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

impl BarsRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbols(&self.symbols)?;
        validate_limit(self.limit, 1, 10_000)
    }

    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_csv("symbols", self.symbols);
        query.push_opt("timeframe", Some(self.timeframe));
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        query.push_opt("limit", self.limit);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.finish()
    }
}

impl QuotesRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbols(&self.symbols)?;
        validate_limit(self.limit, 1, 10_000)
    }

    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_csv("symbols", self.symbols);
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        query.push_opt("limit", self.limit);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.finish()
    }
}

impl TradesRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbols(&self.symbols)?;
        validate_limit(self.limit, 1, 10_000)
    }

    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_csv("symbols", self.symbols);
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        query.push_opt("limit", self.limit);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.finish()
    }
}

impl LatestBarsRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbols(&self.symbols)
    }

    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols)
    }
}

impl LatestQuotesRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbols(&self.symbols)
    }

    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols)
    }
}

impl LatestTradesRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbols(&self.symbols)
    }

    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols)
    }
}

impl LatestOrderbooksRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbols(&self.symbols)
    }

    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols)
    }
}

impl SnapshotsRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbols(&self.symbols)
    }

    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols)
    }
}

impl PaginatedRequest for BarsRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

impl PaginatedRequest for QuotesRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

impl PaginatedRequest for TradesRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

fn latest_query(symbols: Vec<String>) -> Vec<(String, String)> {
    let mut query = QueryWriter::default();
    query.push_csv("symbols", symbols);
    query.finish()
}

fn validate_required_symbols(symbols: &[String]) -> Result<(), Error> {
    if symbols.is_empty() {
        return Err(Error::InvalidRequest("symbols must not be empty".into()));
    }

    Ok(())
}

fn validate_limit(limit: Option<u32>, min: u32, max: u32) -> Result<(), Error> {
    if let Some(limit) = limit {
        if !(min..=max).contains(&limit) {
            return Err(Error::InvalidRequest(format!(
                "limit must be between {min} and {max}"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::transport::pagination::PaginatedRequest;

    use super::{
        BarsRequest, LatestBarsRequest, LatestOrderbooksRequest, LatestQuotesRequest,
        LatestTradesRequest, Loc, QuotesRequest, SnapshotsRequest, Sort, TimeFrame, TradesRequest,
    };

    #[test]
    fn bars_request_serializes_official_query_words_without_loc() {
        let query = BarsRequest {
            symbols: vec!["BTC/USD".into(), "ETH/USD".into()],
            timeframe: TimeFrame::from("1Min"),
            start: Some("2026-04-04T00:00:00Z".into()),
            end: Some("2026-04-04T00:02:00Z".into()),
            limit: Some(2),
            sort: Some(Sort::Desc),
            loc: Some(Loc::Eu1),
            page_token: Some("page-2".into()),
        }
        .to_query();

        assert_eq!(
            query,
            vec![
                ("symbols".to_string(), "BTC/USD,ETH/USD".to_string()),
                ("timeframe".to_string(), "1Min".to_string()),
                ("start".to_string(), "2026-04-04T00:00:00Z".to_string()),
                ("end".to_string(), "2026-04-04T00:02:00Z".to_string()),
                ("limit".to_string(), "2".to_string()),
                ("page_token".to_string(), "page-2".to_string()),
                ("sort".to_string(), "desc".to_string()),
            ]
        );
    }

    #[test]
    fn quotes_and_trades_requests_serialize_official_query_words_without_loc() {
        let quotes_query = QuotesRequest {
            symbols: vec!["BTC/USD".into()],
            start: Some("2026-04-04T00:00:00Z".into()),
            end: Some("2026-04-04T00:00:05Z".into()),
            limit: Some(1),
            sort: Some(Sort::Asc),
            loc: Some(Loc::Us1),
            page_token: Some("page-3".into()),
        }
        .to_query();
        assert_eq!(
            quotes_query,
            vec![
                ("symbols".to_string(), "BTC/USD".to_string()),
                ("start".to_string(), "2026-04-04T00:00:00Z".to_string()),
                ("end".to_string(), "2026-04-04T00:00:05Z".to_string()),
                ("limit".to_string(), "1".to_string()),
                ("page_token".to_string(), "page-3".to_string()),
                ("sort".to_string(), "asc".to_string()),
            ]
        );

        let trades_query = TradesRequest {
            symbols: vec!["BTC/USD".into()],
            start: Some("2026-04-04T00:01:00Z".into()),
            end: Some("2026-04-04T00:01:03Z".into()),
            limit: Some(1),
            sort: Some(Sort::Desc),
            loc: Some(Loc::Us),
            page_token: Some("page-4".into()),
        }
        .to_query();
        assert_eq!(
            trades_query,
            vec![
                ("symbols".to_string(), "BTC/USD".to_string()),
                ("start".to_string(), "2026-04-04T00:01:00Z".to_string()),
                ("end".to_string(), "2026-04-04T00:01:03Z".to_string()),
                ("limit".to_string(), "1".to_string()),
                ("page_token".to_string(), "page-4".to_string()),
                ("sort".to_string(), "desc".to_string()),
            ]
        );
    }

    #[test]
    fn historical_requests_replace_page_token_through_shared_pagination_trait() {
        let bars = BarsRequest {
            page_token: Some("page-2".into()),
            ..BarsRequest::default()
        };
        let quotes = QuotesRequest {
            page_token: Some("page-3".into()),
            ..QuotesRequest::default()
        };
        let trades = TradesRequest {
            page_token: Some("page-4".into()),
            ..TradesRequest::default()
        };

        assert_eq!(
            bars.with_page_token(Some("page-9".into()))
                .page_token
                .as_deref(),
            Some("page-9")
        );
        assert_eq!(
            quotes
                .with_page_token(Some("page-8".into()))
                .page_token
                .as_deref(),
            Some("page-8")
        );
        assert_eq!(
            trades
                .with_page_token(Some("page-7".into()))
                .page_token
                .as_deref(),
            Some("page-7")
        );
    }

    #[test]
    fn latest_requests_serialize_symbols_only_without_loc() {
        let bars_query = LatestBarsRequest {
            symbols: vec!["BTC/USD".into(), "ETH/USD".into()],
            loc: Some(Loc::Us1),
        }
        .to_query();
        assert_eq!(
            bars_query,
            vec![("symbols".to_string(), "BTC/USD,ETH/USD".to_string())]
        );

        let quotes_query = LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(Loc::Eu1),
        }
        .to_query();
        assert_eq!(
            quotes_query,
            vec![("symbols".to_string(), "BTC/USD".to_string())]
        );

        let trades_query = LatestTradesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(Loc::Us),
        }
        .to_query();
        assert_eq!(
            trades_query,
            vec![("symbols".to_string(), "BTC/USD".to_string())]
        );

        let orderbooks_query = LatestOrderbooksRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(Loc::Us1),
        }
        .to_query();
        assert_eq!(
            orderbooks_query,
            vec![("symbols".to_string(), "BTC/USD".to_string())]
        );
    }

    #[test]
    fn snapshots_request_serializes_symbols_only_without_loc() {
        let query = SnapshotsRequest {
            symbols: vec!["BTC/USD".into(), "ETH/USD".into()],
            loc: Some(Loc::Eu1),
        }
        .to_query();

        assert_eq!(
            query,
            vec![("symbols".to_string(), "BTC/USD,ETH/USD".to_string())]
        );
    }

    #[test]
    fn requests_reject_empty_symbols_for_required_symbol_endpoints() {
        let errors = [
            BarsRequest::default()
                .validate()
                .expect_err("bars symbols must be required"),
            QuotesRequest::default()
                .validate()
                .expect_err("quotes symbols must be required"),
            TradesRequest::default()
                .validate()
                .expect_err("trades symbols must be required"),
            LatestBarsRequest::default()
                .validate()
                .expect_err("latest bars symbols must be required"),
            LatestQuotesRequest::default()
                .validate()
                .expect_err("latest quotes symbols must be required"),
            LatestTradesRequest::default()
                .validate()
                .expect_err("latest trades symbols must be required"),
            LatestOrderbooksRequest::default()
                .validate()
                .expect_err("latest orderbooks symbols must be required"),
            SnapshotsRequest::default()
                .validate()
                .expect_err("snapshots symbols must be required"),
        ];

        for error in errors {
            assert!(matches!(
                error,
                Error::InvalidRequest(message)
                    if message.contains("symbols") && message.contains("empty")
            ));
        }
    }

    #[test]
    fn historical_requests_reject_limits_outside_documented_range() {
        let errors = [
            BarsRequest {
                symbols: vec!["BTC/USD".into()],
                limit: Some(0),
                ..BarsRequest::default()
            }
            .validate()
            .expect_err("bars limit below one must fail"),
            QuotesRequest {
                symbols: vec!["BTC/USD".into()],
                limit: Some(10_001),
                ..QuotesRequest::default()
            }
            .validate()
            .expect_err("quotes limit above ten thousand must fail"),
            TradesRequest {
                symbols: vec!["BTC/USD".into()],
                limit: Some(0),
                ..TradesRequest::default()
            }
            .validate()
            .expect_err("trades limit below one must fail"),
        ];

        for error in errors {
            assert!(matches!(
                error,
                Error::InvalidRequest(message)
                    if message.contains("limit") && message.contains("10000")
            ));
        }
    }
}
