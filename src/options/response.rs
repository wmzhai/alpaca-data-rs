use std::collections::HashMap;

use crate::{Error, transport::pagination::PaginatedResponse};

use super::{Bar, Quote, Snapshot, Trade};

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct BarsResponse {
    pub bars: HashMap<String, Vec<Bar>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct TradesResponse {
    pub trades: HashMap<String, Vec<Trade>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestQuotesResponse {
    pub quotes: HashMap<String, Quote>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestTradesResponse {
    pub trades: HashMap<String, Trade>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SnapshotsResponse {
    pub snapshots: HashMap<String, Snapshot>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChainResponse {
    pub snapshots: HashMap<String, Snapshot>,
    pub next_page_token: Option<String>,
}

pub type ExchangeCodesResponse = HashMap<String, String>;

fn merge_batch_page<Item>(
    current: &mut HashMap<String, Vec<Item>>,
    next: HashMap<String, Vec<Item>>,
) {
    for (symbol, mut items) in next {
        current.entry(symbol).or_default().append(&mut items);
    }
}

impl PaginatedResponse for BarsResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_batch_page(&mut self.bars, next.bars);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

impl PaginatedResponse for TradesResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_batch_page(&mut self.trades, next.trades);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{
        Bar, BarsResponse, ExchangeCodesResponse, LatestQuotesResponse, LatestTradesResponse,
        Trade, TradesResponse,
    };
    use crate::transport::pagination::PaginatedResponse;

    #[test]
    fn historical_responses_deserialize_official_wrapper_shapes() {
        let bars: BarsResponse = serde_json::from_str(
            r#"{"bars":{"AAPL260406C00180000":[{"c":70.97,"h":71.21,"l":70.97,"n":2,"o":71.21,"t":"2026-04-02T04:00:00Z","v":2,"vw":71.09}]},"next_page_token":"page-2"}"#,
        )
        .expect("bars response should deserialize");
        assert_eq!(bars.next_page_token.as_deref(), Some("page-2"));
        assert_eq!(
            bars.bars
                .get("AAPL260406C00180000")
                .map(Vec::len)
                .unwrap_or_default(),
            1
        );

        let trades: TradesResponse = serde_json::from_str(
            r#"{"trades":{"AAPL260406C00180000":[{"c":"n","p":70.97,"s":1,"t":"2026-04-02T13:39:38.883488197Z","x":"I"}]},"next_page_token":"page-3"}"#,
        )
        .expect("trades response should deserialize");
        assert_eq!(trades.next_page_token.as_deref(), Some("page-3"));
        assert_eq!(
            trades
                .trades
                .get("AAPL260406C00180000")
                .map(Vec::len)
                .unwrap_or_default(),
            1
        );
    }

    #[test]
    fn historical_merge_combines_symbol_buckets_and_clears_next_page_token() {
        let mut first = BarsResponse {
            bars: HashMap::from([(
                "AAPL260406C00180000".into(),
                vec![Bar {
                    t: Some("2026-04-02T04:00:00Z".into()),
                    ..Bar::default()
                }],
            )]),
            next_page_token: Some("page-2".into()),
        };
        let second = BarsResponse {
            bars: HashMap::from([(
                "AAPL260406C00185000".into(),
                vec![Bar {
                    t: Some("2026-04-02T04:00:00Z".into()),
                    ..Bar::default()
                }],
            )]),
            next_page_token: None,
        };

        first
            .merge_page(second)
            .expect("merge should combine pages without error");
        first.clear_next_page_token();

        assert_eq!(first.next_page_token, None);
        assert_eq!(first.bars.len(), 2);
    }

    #[test]
    fn historical_merge_accepts_multiple_trade_pages() {
        let mut first = TradesResponse {
            trades: HashMap::from([(
                "AAPL260406C00180000".into(),
                vec![Trade {
                    t: Some("2026-04-02T13:39:17.488838508Z".into()),
                    ..Trade::default()
                }],
            )]),
            next_page_token: Some("page-2".into()),
        };
        let second = TradesResponse {
            trades: HashMap::from([(
                "AAPL260406C00180000".into(),
                vec![Trade {
                    t: Some("2026-04-02T13:39:38.883488197Z".into()),
                    ..Trade::default()
                }],
            )]),
            next_page_token: None,
        };

        first
            .merge_page(second)
            .expect("trade merge should append more items");

        assert_eq!(
            first
                .trades
                .get("AAPL260406C00180000")
                .map(Vec::len)
                .unwrap_or_default(),
            2
        );
    }

    #[test]
    fn latest_responses_deserialize_official_wrapper_shapes() {
        let quotes: LatestQuotesResponse = serde_json::from_str(
            r#"{"quotes":{"AAPL260406C00180000":{"ap":77.75,"as":5,"ax":"A","bp":73.95,"bs":3,"bx":"N","c":" ","t":"2026-04-02T19:59:59.792862244Z"}}}"#,
        )
        .expect("latest quotes response should deserialize");
        assert!(quotes.quotes.contains_key("AAPL260406C00180000"));

        let trades: LatestTradesResponse = serde_json::from_str(
            r#"{"trades":{"AAPL260406C00180000":{"c":"n","p":70.97,"s":1,"t":"2026-04-02T13:39:38.883488197Z","x":"I"}}}"#,
        )
        .expect("latest trades response should deserialize");
        assert!(trades.trades.contains_key("AAPL260406C00180000"));
    }

    #[test]
    fn exchange_codes_response_deserializes_official_map_shape() {
        let exchange_codes: ExchangeCodesResponse = serde_json::from_str(
            r#"{"A":"AMEX - NYSE American","O":"OPRA - Options Price Reporting Authority"}"#,
        )
        .expect("exchange codes response should deserialize");
        assert_eq!(
            exchange_codes.get("A").map(String::as_str),
            Some("AMEX - NYSE American")
        );
        assert_eq!(
            exchange_codes.get("O").map(String::as_str),
            Some("OPRA - Options Price Reporting Authority")
        );
    }
}
