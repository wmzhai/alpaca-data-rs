use crate::common::query::QueryWriter;
use crate::transport::pagination::PaginatedRequest;

use super::{ContractType, OptionsFeed, Sort, TimeFrame};

#[derive(Clone, Debug, Default)]
pub struct BarsRequest {
    pub symbols: Vec<String>,
    pub timeframe: TimeFrame,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TradesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestQuotesRequest {
    pub symbols: Vec<String>,
    pub feed: Option<OptionsFeed>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestTradesRequest {
    pub symbols: Vec<String>,
    pub feed: Option<OptionsFeed>,
}

#[derive(Clone, Debug, Default)]
pub struct SnapshotsRequest {
    pub symbols: Vec<String>,
    pub feed: Option<OptionsFeed>,
    pub limit: Option<u32>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ChainRequest {
    pub underlying_symbol: String,
    pub feed: Option<OptionsFeed>,
    pub r#type: Option<ContractType>,
    pub strike_price_gte: Option<f64>,
    pub strike_price_lte: Option<f64>,
    pub expiration_date: Option<String>,
    pub expiration_date_gte: Option<String>,
    pub expiration_date_lte: Option<String>,
    pub root_symbol: Option<String>,
    pub updated_since: Option<String>,
    pub limit: Option<u32>,
    pub page_token: Option<String>,
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

impl LatestQuotesRequest {
    #[allow(dead_code)]
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols, self.feed)
    }
}

impl LatestTradesRequest {
    #[allow(dead_code)]
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols, self.feed)
    }
}

impl SnapshotsRequest {
    #[allow(dead_code)]
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_csv("symbols", self.symbols);
        query.push_opt("feed", self.feed);
        query.push_opt("limit", self.limit);
        query.push_opt("page_token", self.page_token);
        query.finish()
    }
}

impl ChainRequest {
    #[allow(dead_code)]
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_opt("feed", self.feed);
        query.push_opt("type", self.r#type);
        query.push_opt("strike_price_gte", self.strike_price_gte);
        query.push_opt("strike_price_lte", self.strike_price_lte);
        query.push_opt("expiration_date", self.expiration_date);
        query.push_opt("expiration_date_gte", self.expiration_date_gte);
        query.push_opt("expiration_date_lte", self.expiration_date_lte);
        query.push_opt("root_symbol", self.root_symbol);
        query.push_opt("updated_since", self.updated_since);
        query.push_opt("limit", self.limit);
        query.push_opt("page_token", self.page_token);
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

impl PaginatedRequest for TradesRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

impl PaginatedRequest for SnapshotsRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

impl PaginatedRequest for ChainRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

#[allow(dead_code)]
fn latest_query(symbols: Vec<String>, feed: Option<OptionsFeed>) -> Vec<(String, String)> {
    let mut query = QueryWriter::default();
    query.push_csv("symbols", symbols);
    query.push_opt("feed", feed);
    query.finish()
}

#[cfg(test)]
mod tests {
    use super::{BarsRequest, Sort, TimeFrame, TradesRequest};

    #[test]
    fn bars_request_serializes_official_query_words() {
        let query = BarsRequest {
            symbols: vec!["AAPL260406C00180000".into(), "AAPL260406C00185000".into()],
            timeframe: TimeFrame::from("1Day"),
            start: Some("2026-04-01T00:00:00Z".into()),
            end: Some("2026-04-03T00:00:00Z".into()),
            limit: Some(2),
            sort: Some(Sort::Asc),
            page_token: Some("page-2".into()),
        }
        .to_query();

        assert_eq!(
            query,
            vec![
                (
                    "symbols".to_string(),
                    "AAPL260406C00180000,AAPL260406C00185000".to_string(),
                ),
                ("timeframe".to_string(), "1Day".to_string()),
                ("start".to_string(), "2026-04-01T00:00:00Z".to_string()),
                ("end".to_string(), "2026-04-03T00:00:00Z".to_string()),
                ("limit".to_string(), "2".to_string()),
                ("page_token".to_string(), "page-2".to_string()),
                ("sort".to_string(), "asc".to_string()),
            ]
        );
    }

    #[test]
    fn trades_request_serializes_official_query_words() {
        let query = TradesRequest {
            symbols: vec!["AAPL260406C00180000".into()],
            start: Some("2026-04-02T13:39:00Z".into()),
            end: Some("2026-04-02T13:40:00Z".into()),
            limit: Some(1),
            sort: Some(Sort::Desc),
            page_token: Some("page-3".into()),
        }
        .to_query();

        assert_eq!(
            query,
            vec![
                ("symbols".to_string(), "AAPL260406C00180000".to_string()),
                ("start".to_string(), "2026-04-02T13:39:00Z".to_string()),
                ("end".to_string(), "2026-04-02T13:40:00Z".to_string()),
                ("limit".to_string(), "1".to_string()),
                ("page_token".to_string(), "page-3".to_string()),
                ("sort".to_string(), "desc".to_string()),
            ]
        );
    }
}
