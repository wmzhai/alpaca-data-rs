use crate::common::query::QueryWriter;

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
pub struct LatestBarsRequest {
    pub symbols: Vec<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stocks_data_feed_serializes_to_official_strings() {
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
}
