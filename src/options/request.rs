use crate::Error;
use crate::common::query::QueryWriter;
use crate::common::validate::{validate_required_symbol, validate_required_symbols};
use crate::transport::pagination::PaginatedRequest;

use super::{ContractType, OptionsFeed, Sort, TickType, TimeFrame};

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

#[derive(Clone, Debug, Default)]
pub struct ConditionCodesRequest {
    pub ticktype: TickType,
}

impl BarsRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_option_symbols(&self.symbols)?;
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

impl TradesRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_option_symbols(&self.symbols)?;
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

impl LatestQuotesRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_option_symbols(&self.symbols)
    }

    #[allow(dead_code)]
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols, self.feed)
    }
}

impl LatestTradesRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_option_symbols(&self.symbols)
    }

    #[allow(dead_code)]
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_query(self.symbols, self.feed)
    }
}

impl SnapshotsRequest {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_option_symbols(&self.symbols)?;
        validate_limit(self.limit, 1, 1_000)
    }

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
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_required_symbol(&self.underlying_symbol, "underlying_symbol")?;
        validate_limit(self.limit, 1, 1_000)
    }

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

impl ConditionCodesRequest {
    pub(crate) fn ticktype(&self) -> &'static str {
        self.ticktype.as_str()
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

fn validate_option_symbols(symbols: &[String]) -> Result<(), Error> {
    if symbols.is_empty() {
        return validate_required_symbols(symbols);
    }

    if symbols.len() > 100 {
        return Err(Error::InvalidRequest(
            "symbols must contain at most 100 contract symbols".into(),
        ));
    }

    validate_required_symbols(symbols)
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

    use super::{
        BarsRequest, ChainRequest, ConditionCodesRequest, ContractType, LatestQuotesRequest,
        LatestTradesRequest, OptionsFeed, SnapshotsRequest, Sort, TickType, TimeFrame,
        TradesRequest,
    };

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

    #[test]
    fn latest_requests_serialize_official_query_words() {
        let quotes_query = LatestQuotesRequest {
            symbols: vec!["AAPL260406C00180000".into()],
            feed: Some(OptionsFeed::Indicative),
        }
        .to_query();
        assert_eq!(
            quotes_query,
            vec![
                ("symbols".to_string(), "AAPL260406C00180000".to_string()),
                ("feed".to_string(), "indicative".to_string()),
            ]
        );

        let trades_query = LatestTradesRequest {
            symbols: vec!["AAPL260406C00180000".into(), "AAPL260406C00185000".into()],
            feed: Some(OptionsFeed::Opra),
        }
        .to_query();
        assert_eq!(
            trades_query,
            vec![
                (
                    "symbols".to_string(),
                    "AAPL260406C00180000,AAPL260406C00185000".to_string(),
                ),
                ("feed".to_string(), "opra".to_string()),
            ]
        );
    }

    #[test]
    fn snapshot_requests_serialize_official_query_words() {
        let query = SnapshotsRequest {
            symbols: vec!["AAPL260406C00180000".into(), "AAPL260406C00185000".into()],
            feed: Some(OptionsFeed::Indicative),
            limit: Some(2),
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
                ("feed".to_string(), "indicative".to_string()),
                ("limit".to_string(), "2".to_string()),
                ("page_token".to_string(), "page-2".to_string()),
            ]
        );
    }

    #[test]
    fn chain_request_serializes_official_query_words() {
        let query = ChainRequest {
            underlying_symbol: "AAPL".into(),
            feed: Some(OptionsFeed::Indicative),
            r#type: Some(ContractType::Call),
            strike_price_gte: Some(180.0),
            strike_price_lte: Some(200.0),
            expiration_date: Some("2026-04-06".into()),
            expiration_date_gte: Some("2026-04-06".into()),
            expiration_date_lte: Some("2026-04-13".into()),
            root_symbol: Some("AAPL".into()),
            updated_since: Some("2026-04-02T19:30:00Z".into()),
            limit: Some(3),
            page_token: Some("page-3".into()),
        }
        .to_query();

        assert_eq!(
            query,
            vec![
                ("feed".to_string(), "indicative".to_string()),
                ("type".to_string(), "call".to_string()),
                ("strike_price_gte".to_string(), "180".to_string()),
                ("strike_price_lte".to_string(), "200".to_string()),
                ("expiration_date".to_string(), "2026-04-06".to_string()),
                ("expiration_date_gte".to_string(), "2026-04-06".to_string()),
                ("expiration_date_lte".to_string(), "2026-04-13".to_string()),
                ("root_symbol".to_string(), "AAPL".to_string()),
                (
                    "updated_since".to_string(),
                    "2026-04-02T19:30:00Z".to_string()
                ),
                ("limit".to_string(), "3".to_string()),
                ("page_token".to_string(), "page-3".to_string()),
            ]
        );
    }

    #[test]
    fn condition_codes_request_uses_official_ticktype_word() {
        let trade = ConditionCodesRequest {
            ticktype: TickType::Trade,
        };
        assert_eq!(trade.ticktype(), "trade");

        let quote = ConditionCodesRequest {
            ticktype: TickType::Quote,
        };
        assert_eq!(quote.ticktype(), "quote");
    }

    #[test]
    fn requests_reject_empty_or_oversized_symbol_lists() {
        let empty_errors = [
            BarsRequest::default()
                .validate()
                .expect_err("bars symbols must be required"),
            TradesRequest::default()
                .validate()
                .expect_err("trades symbols must be required"),
            LatestQuotesRequest::default()
                .validate()
                .expect_err("latest quotes symbols must be required"),
            LatestTradesRequest::default()
                .validate()
                .expect_err("latest trades symbols must be required"),
            SnapshotsRequest::default()
                .validate()
                .expect_err("snapshots symbols must be required"),
        ];

        for error in empty_errors {
            assert!(matches!(
                error,
                Error::InvalidRequest(message)
                    if message.contains("symbols") && message.contains("empty")
            ));
        }

        let symbols = (0..101)
            .map(|index| format!("AAPL260406C{:08}", index))
            .collect::<Vec<_>>();

        let oversized_errors = [
            BarsRequest {
                symbols: symbols.clone(),
                ..BarsRequest::default()
            }
            .validate()
            .expect_err("bars symbols over one hundred must fail"),
            LatestQuotesRequest {
                symbols: symbols.clone(),
                ..LatestQuotesRequest::default()
            }
            .validate()
            .expect_err("latest quotes symbols over one hundred must fail"),
            SnapshotsRequest {
                symbols,
                ..SnapshotsRequest::default()
            }
            .validate()
            .expect_err("snapshots symbols over one hundred must fail"),
        ];

        for error in oversized_errors {
            assert!(matches!(
                error,
                Error::InvalidRequest(message)
                    if message.contains("symbols") && message.contains("100")
            ));
        }
    }

    #[test]
    fn oversized_symbol_lists_still_win_before_blank_entry_errors() {
        let mut symbols = (0..101)
            .map(|index| format!("AAPL260406C{:08}", index))
            .collect::<Vec<_>>();
        symbols[100] = "   ".into();

        let error = LatestQuotesRequest {
            symbols,
            ..LatestQuotesRequest::default()
        }
        .validate()
        .expect_err("mixed invalid symbol lists should still report the options cap first");

        assert!(matches!(
            error,
            Error::InvalidRequest(message)
                if message.contains("symbols") && message.contains("100")
        ));
    }

    #[test]
    fn chain_request_rejects_blank_underlying_symbols() {
        let errors = [
            ChainRequest::default()
                .validate()
                .expect_err("chain underlying symbol must be required"),
            ChainRequest {
                underlying_symbol: "   ".into(),
                ..ChainRequest::default()
            }
            .validate()
            .expect_err("chain underlying symbol must reject whitespace-only input"),
        ];

        for error in errors {
            assert!(matches!(
                error,
                Error::InvalidRequest(message)
                    if message.contains("underlying_symbol") && message.contains("invalid")
            ));
        }
    }

    #[test]
    fn requests_reject_limits_outside_documented_ranges() {
        let errors = [
            BarsRequest {
                symbols: vec!["AAPL260406C00180000".into()],
                limit: Some(0),
                ..BarsRequest::default()
            }
            .validate()
            .expect_err("bars limit below one must fail"),
            TradesRequest {
                symbols: vec!["AAPL260406C00180000".into()],
                limit: Some(10_001),
                ..TradesRequest::default()
            }
            .validate()
            .expect_err("trades limit above ten thousand must fail"),
            SnapshotsRequest {
                symbols: vec!["AAPL260406C00180000".into()],
                limit: Some(0),
                ..SnapshotsRequest::default()
            }
            .validate()
            .expect_err("snapshots limit below one must fail"),
            ChainRequest {
                underlying_symbol: "AAPL".into(),
                limit: Some(1_001),
                ..ChainRequest::default()
            }
            .validate()
            .expect_err("chain limit above one thousand must fail"),
        ];

        let expected_maxima = ["10000", "10000", "1000", "1000"];
        for (error, expected_max) in errors.into_iter().zip(expected_maxima) {
            assert!(matches!(
                error,
                Error::InvalidRequest(message)
                    if message.contains("limit") && message.contains(expected_max)
            ));
        }
    }
}
