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

#[cfg(test)]
mod tests {
    use crate::transport::pagination::PaginatedRequest;

    use super::{BarsRequest, Loc, QuotesRequest, Sort, TimeFrame, TradesRequest};

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
}
