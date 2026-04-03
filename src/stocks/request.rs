use crate::common::query::QueryWriter;
use crate::transport::pagination::PaginatedRequest;

use super::{Adjustment, Currency, DataFeed, Sort, TimeFrame};

#[derive(Clone, Debug, Default)]
pub struct BarsRequest {
    pub symbols: Vec<String>,
    pub timeframe: TimeFrame,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub adjustment: Option<Adjustment>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BarsSingleRequest {
    pub symbol: String,
    pub timeframe: TimeFrame,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub adjustment: Option<Adjustment>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct QuotesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct QuotesSingleRequest {
    pub symbol: String,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TradesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TradesSingleRequest {
    pub symbol: String,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestBarsRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestBarRequest {
    pub symbol: String,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestQuotesRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestQuoteRequest {
    pub symbol: String,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestTradesRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestTradeRequest {
    pub symbol: String,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct SnapshotsRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct SnapshotRequest {
    pub symbol: String,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct ConditionCodesRequest {
    pub ticktype: TickType,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TickType {
    #[default]
    Trade,
    Quote,
}

impl BarsRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_csv("symbols", self.symbols);
        query.push_opt("timeframe", Some(self.timeframe));
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        query.push_opt("limit", self.limit);
        query.push_opt("adjustment", self.adjustment);
        query.push_opt("feed", self.feed);
        query.push_opt("currency", self.currency);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.push_opt("asof", self.asof);
        query.finish()
    }
}

impl BarsSingleRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_opt("timeframe", Some(self.timeframe));
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        query.push_opt("limit", self.limit);
        query.push_opt("adjustment", self.adjustment);
        query.push_opt("feed", self.feed);
        query.push_opt("currency", self.currency);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.push_opt("asof", self.asof);
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
        query.push_opt("feed", self.feed);
        query.push_opt("currency", self.currency);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.push_opt("asof", self.asof);
        query.finish()
    }
}

impl QuotesSingleRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        query.push_opt("limit", self.limit);
        query.push_opt("feed", self.feed);
        query.push_opt("currency", self.currency);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.push_opt("asof", self.asof);
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
        query.push_opt("feed", self.feed);
        query.push_opt("currency", self.currency);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.push_opt("asof", self.asof);
        query.finish()
    }
}

impl TradesSingleRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        query.push_opt("limit", self.limit);
        query.push_opt("feed", self.feed);
        query.push_opt("currency", self.currency);
        query.push_opt("page_token", self.page_token);
        query.push_opt("sort", self.sort);
        query.push_opt("asof", self.asof);
        query.finish()
    }
}

impl LatestBarsRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_batch_query(self.symbols, self.feed, self.currency)
    }
}

impl LatestBarRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_single_query(self.feed, self.currency)
    }
}

impl LatestQuotesRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_batch_query(self.symbols, self.feed, self.currency)
    }
}

impl LatestQuoteRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_single_query(self.feed, self.currency)
    }
}

impl LatestTradesRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_batch_query(self.symbols, self.feed, self.currency)
    }
}

impl LatestTradeRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_single_query(self.feed, self.currency)
    }
}

impl SnapshotsRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_batch_query(self.symbols, self.feed, self.currency)
    }
}

impl SnapshotRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        latest_single_query(self.feed, self.currency)
    }
}

impl PaginatedRequest for BarsSingleRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

impl PaginatedRequest for QuotesSingleRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

fn latest_batch_query(
    symbols: Vec<String>,
    feed: Option<DataFeed>,
    currency: Option<Currency>,
) -> Vec<(String, String)> {
    let mut query = QueryWriter::default();
    query.push_csv("symbols", symbols);
    query.push_opt("feed", feed);
    query.push_opt("currency", currency);
    query.finish()
}

fn latest_single_query(
    feed: Option<DataFeed>,
    currency: Option<Currency>,
) -> Vec<(String, String)> {
    let mut query = QueryWriter::default();
    query.push_opt("feed", feed);
    query.push_opt("currency", currency);
    query.finish()
}

impl PaginatedRequest for TradesSingleRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stocks_data_feed_serializes_to_official_strings() {
        assert_eq!(DataFeed::DelayedSip.to_string(), "delayed_sip");
        assert_eq!(DataFeed::Iex.to_string(), "iex");
        assert_eq!(DataFeed::Otc.to_string(), "otc");
        assert_eq!(DataFeed::Sip.to_string(), "sip");
        assert_eq!(DataFeed::Boats.to_string(), "boats");
        assert_eq!(DataFeed::Overnight.to_string(), "overnight");
    }

    #[test]
    fn stocks_adjustment_serializes_to_official_strings() {
        assert_eq!(Adjustment::raw().to_string(), "raw");
        assert_eq!(Adjustment::split().to_string(), "split");
        assert_eq!(Adjustment::dividend().to_string(), "dividend");
        assert_eq!(Adjustment::spin_off().to_string(), "spin-off");
        assert_eq!(Adjustment::all().to_string(), "all");
        assert_eq!(
            Adjustment::from("split,dividend").to_string(),
            "split,dividend"
        );
    }

    #[test]
    fn stocks_timeframe_serializes_to_official_strings() {
        assert_eq!(TimeFrame::from("1Min").to_string(), "1Min");
        assert_eq!(TimeFrame::from("5Min").to_string(), "5Min");
        assert_eq!(TimeFrame::from("1Day").to_string(), "1Day");
        assert_eq!(TimeFrame::from("1Week").to_string(), "1Week");
        assert_eq!(TimeFrame::from("3Month").to_string(), "3Month");
    }

    #[test]
    fn bars_request_serializes_official_query_words() {
        let request = BarsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            timeframe: TimeFrame::from("1Day"),
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-05T00:00:00Z".into()),
            limit: Some(50),
            adjustment: Some(Adjustment::from("split,dividend")),
            feed: Some(DataFeed::Boats),
            sort: Some(Sort::Desc),
            asof: Some("2024-03-04".into()),
            currency: Some(Currency::from("USD")),
            page_token: Some("page-123".into()),
        };

        assert_eq!(
            request.to_query(),
            vec![
                ("symbols".to_string(), "AAPL,MSFT".to_string()),
                ("timeframe".to_string(), "1Day".to_string()),
                ("start".to_string(), "2024-03-01T00:00:00Z".to_string()),
                ("end".to_string(), "2024-03-05T00:00:00Z".to_string()),
                ("limit".to_string(), "50".to_string()),
                ("adjustment".to_string(), "split,dividend".to_string()),
                ("feed".to_string(), "boats".to_string()),
                ("currency".to_string(), "USD".to_string()),
                ("page_token".to_string(), "page-123".to_string()),
                ("sort".to_string(), "desc".to_string()),
                ("asof".to_string(), "2024-03-04".to_string()),
            ]
        );
    }

    #[test]
    fn bars_single_request_serializes_official_query_words() {
        let request = BarsSingleRequest {
            symbol: "AAPL".into(),
            timeframe: TimeFrame::from("1Day"),
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-05T00:00:00Z".into()),
            limit: Some(50),
            adjustment: Some(Adjustment::from("split,dividend")),
            feed: Some(DataFeed::Boats),
            sort: Some(Sort::Desc),
            asof: Some("2024-03-04".into()),
            currency: Some(Currency::from("USD")),
            page_token: Some("page-123".into()),
        };

        assert_eq!(
            request.to_query(),
            vec![
                ("timeframe".to_string(), "1Day".to_string()),
                ("start".to_string(), "2024-03-01T00:00:00Z".to_string()),
                ("end".to_string(), "2024-03-05T00:00:00Z".to_string()),
                ("limit".to_string(), "50".to_string()),
                ("adjustment".to_string(), "split,dividend".to_string()),
                ("feed".to_string(), "boats".to_string()),
                ("currency".to_string(), "USD".to_string()),
                ("page_token".to_string(), "page-123".to_string()),
                ("sort".to_string(), "desc".to_string()),
                ("asof".to_string(), "2024-03-04".to_string()),
            ]
        );
    }

    #[test]
    fn quotes_request_serializes_official_query_words() {
        let request = QuotesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-05T00:00:00Z".into()),
            limit: Some(25),
            feed: Some(DataFeed::Iex),
            sort: Some(Sort::Asc),
            asof: Some("2024-03-04".into()),
            currency: Some(Currency::from("USD")),
            page_token: Some("page-456".into()),
        };

        assert_eq!(
            request.to_query(),
            vec![
                ("symbols".to_string(), "AAPL,MSFT".to_string()),
                ("start".to_string(), "2024-03-01T00:00:00Z".to_string()),
                ("end".to_string(), "2024-03-05T00:00:00Z".to_string()),
                ("limit".to_string(), "25".to_string()),
                ("feed".to_string(), "iex".to_string()),
                ("currency".to_string(), "USD".to_string()),
                ("page_token".to_string(), "page-456".to_string()),
                ("sort".to_string(), "asc".to_string()),
                ("asof".to_string(), "2024-03-04".to_string()),
            ]
        );
    }

    #[test]
    fn quotes_single_request_serializes_official_query_words() {
        let request = QuotesSingleRequest {
            symbol: "AAPL".into(),
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-05T00:00:00Z".into()),
            limit: Some(25),
            feed: Some(DataFeed::Iex),
            sort: Some(Sort::Asc),
            asof: Some("2024-03-04".into()),
            currency: Some(Currency::from("USD")),
            page_token: Some("page-456".into()),
        };

        assert_eq!(
            request.to_query(),
            vec![
                ("start".to_string(), "2024-03-01T00:00:00Z".to_string()),
                ("end".to_string(), "2024-03-05T00:00:00Z".to_string()),
                ("limit".to_string(), "25".to_string()),
                ("feed".to_string(), "iex".to_string()),
                ("currency".to_string(), "USD".to_string()),
                ("page_token".to_string(), "page-456".to_string()),
                ("sort".to_string(), "asc".to_string()),
                ("asof".to_string(), "2024-03-04".to_string()),
            ]
        );
    }

    #[test]
    fn trades_request_serializes_official_query_words() {
        let request = TradesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-05T00:00:00Z".into()),
            limit: Some(10),
            feed: Some(DataFeed::Sip),
            sort: Some(Sort::Desc),
            asof: Some("2024-03-04".into()),
            currency: Some(Currency::from("USD")),
            page_token: Some("page-789".into()),
        };

        assert_eq!(
            request.to_query(),
            vec![
                ("symbols".to_string(), "AAPL,MSFT".to_string()),
                ("start".to_string(), "2024-03-01T00:00:00Z".to_string()),
                ("end".to_string(), "2024-03-05T00:00:00Z".to_string()),
                ("limit".to_string(), "10".to_string()),
                ("feed".to_string(), "sip".to_string()),
                ("currency".to_string(), "USD".to_string()),
                ("page_token".to_string(), "page-789".to_string()),
                ("sort".to_string(), "desc".to_string()),
                ("asof".to_string(), "2024-03-04".to_string()),
            ]
        );
    }

    #[test]
    fn trades_single_request_serializes_official_query_words() {
        let request = TradesSingleRequest {
            symbol: "AAPL".into(),
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-05T00:00:00Z".into()),
            limit: Some(10),
            feed: Some(DataFeed::Sip),
            sort: Some(Sort::Desc),
            asof: Some("2024-03-04".into()),
            currency: Some(Currency::from("USD")),
            page_token: Some("page-789".into()),
        };

        assert_eq!(
            request.to_query(),
            vec![
                ("start".to_string(), "2024-03-01T00:00:00Z".to_string()),
                ("end".to_string(), "2024-03-05T00:00:00Z".to_string()),
                ("limit".to_string(), "10".to_string()),
                ("feed".to_string(), "sip".to_string()),
                ("currency".to_string(), "USD".to_string()),
                ("page_token".to_string(), "page-789".to_string()),
                ("sort".to_string(), "desc".to_string()),
                ("asof".to_string(), "2024-03-04".to_string()),
            ]
        );
    }

    #[test]
    fn latest_batch_requests_serialize_official_query_words() {
        let bars = LatestBarsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(DataFeed::DelayedSip),
            currency: Some(Currency::from("USD")),
        };

        assert_eq!(
            bars.to_query(),
            vec![
                ("symbols".to_string(), "AAPL,MSFT".to_string()),
                ("feed".to_string(), "delayed_sip".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ]
        );

        let trades = LatestTradesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(DataFeed::Iex),
            currency: Some(Currency::from("USD")),
        };

        assert_eq!(
            trades.to_query(),
            vec![
                ("symbols".to_string(), "AAPL,MSFT".to_string()),
                ("feed".to_string(), "iex".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ]
        );
    }

    #[test]
    fn latest_single_requests_serialize_official_query_words() {
        let bar = LatestBarRequest {
            symbol: "AAPL".into(),
            feed: Some(DataFeed::Sip),
            currency: Some(Currency::from("USD")),
        };

        assert_eq!(
            bar.to_query(),
            vec![
                ("feed".to_string(), "sip".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ]
        );

        let trade = LatestTradeRequest {
            symbol: "AAPL".into(),
            feed: Some(DataFeed::Boats),
            currency: Some(Currency::from("USD")),
        };

        assert_eq!(
            trade.to_query(),
            vec![
                ("feed".to_string(), "boats".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ]
        );
    }

    #[test]
    fn snapshot_requests_serialize_official_query_words() {
        let batch = SnapshotsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(DataFeed::Overnight),
            currency: Some(Currency::from("USD")),
        };

        assert_eq!(
            batch.to_query(),
            vec![
                ("symbols".to_string(), "AAPL,MSFT".to_string()),
                ("feed".to_string(), "overnight".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ]
        );

        let single = SnapshotRequest {
            symbol: "AAPL".into(),
            feed: Some(DataFeed::Otc),
            currency: Some(Currency::from("USD")),
        };

        assert_eq!(
            single.to_query(),
            vec![
                ("feed".to_string(), "otc".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ]
        );
    }
}
