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

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct SnapshotsResponse {
    pub snapshots: HashMap<String, Snapshot>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
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

fn merge_snapshot_page(
    operation: &'static str,
    current: &mut HashMap<String, Snapshot>,
    next: HashMap<String, Snapshot>,
) -> Result<(), Error> {
    for (symbol, snapshot) in next {
        if current.insert(symbol.clone(), snapshot).is_some() {
            return Err(Error::Pagination(format!(
                "{operation} received duplicate symbol across pages: {symbol}"
            )));
        }
    }

    Ok(())
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

impl PaginatedResponse for SnapshotsResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_snapshot_page("options.snapshots", &mut self.snapshots, next.snapshots)?;
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

impl PaginatedResponse for ChainResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_snapshot_page("options.chain", &mut self.snapshots, next.snapshots)?;
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
        Bar, BarsResponse, ChainResponse, ExchangeCodesResponse, LatestQuotesResponse,
        LatestTradesResponse, SnapshotsResponse, Trade, TradesResponse,
    };
    use crate::{Error, transport::pagination::PaginatedResponse};

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

    #[test]
    fn snapshot_responses_deserialize_official_wrapper_shapes() {
        let snapshots: SnapshotsResponse = serde_json::from_str(
            r#"{"snapshots":{"AAPL260406C00180000":{"latestQuote":{"ap":77.75,"as":5,"ax":"A","bp":73.95,"bs":3,"bx":"N","c":" ","t":"2026-04-02T19:59:59.792862244Z"},"latestTrade":{"c":"n","p":70.97,"s":1,"t":"2026-04-02T13:39:38.883488197Z","x":"I"},"minuteBar":{"c":70.97,"h":71.21,"l":70.97,"n":2,"o":71.21,"t":"2026-04-02T13:39:00Z","v":2,"vw":71.09},"dailyBar":{"c":70.97,"h":71.21,"l":70.97,"n":2,"o":71.21,"t":"2026-04-02T04:00:00Z","v":2,"vw":71.09},"prevDailyBar":{"c":72.32,"h":72.32,"l":72.32,"n":1,"o":72.32,"t":"2026-04-01T04:00:00Z","v":1,"vw":72.32},"greeks":{"delta":0.0232,"gamma":0.0118,"rho":0.0005,"theta":-0.043,"vega":0.0127},"impliedVolatility":0.2006}},"next_page_token":"page-2"}"#,
        )
        .expect("snapshots response should deserialize");
        assert_eq!(snapshots.next_page_token.as_deref(), Some("page-2"));
        let snapshot = snapshots
            .snapshots
            .get("AAPL260406C00180000")
            .expect("snapshots response should include the symbol");
        assert!(snapshot.latestQuote.is_some());
        assert!(snapshot.latestTrade.is_some());
        assert!(snapshot.greeks.is_some());
        assert_eq!(snapshot.impliedVolatility, Some(0.2006));

        let chain: ChainResponse = serde_json::from_str(
            r#"{"snapshots":{"AAPL260406C00185000":{"latestQuote":{"ap":72.85,"as":5,"ax":"X","bp":69.1,"bs":4,"bx":"S","c":" ","t":"2026-04-02T19:59:59.792862244Z"}}},"next_page_token":null}"#,
        )
        .expect("chain response should deserialize");
        assert!(chain.snapshots.contains_key("AAPL260406C00185000"));
        assert_eq!(chain.next_page_token, None);
    }

    #[test]
    fn snapshot_merge_combines_distinct_symbol_pages_and_clears_next_page_token() {
        let mut first = SnapshotsResponse {
            snapshots: HashMap::from([(
                "AAPL260406C00180000".into(),
                serde_json::from_str(
                    r#"{"latestQuote":{"ap":77.75,"as":5,"ax":"A","bp":73.95,"bs":3,"bx":"N","c":" ","t":"2026-04-02T19:59:59.792862244Z"}}"#,
                )
                .expect("first snapshot should deserialize"),
            )]),
            next_page_token: Some("page-2".into()),
        };
        let second = SnapshotsResponse {
            snapshots: HashMap::from([(
                "AAPL260406C00185000".into(),
                serde_json::from_str(
                    r#"{"latestQuote":{"ap":72.85,"as":5,"ax":"X","bp":69.1,"bs":4,"bx":"S","c":" ","t":"2026-04-02T19:59:59.792862244Z"}}"#,
                )
                .expect("second snapshot should deserialize"),
            )]),
            next_page_token: None,
        };

        first
            .merge_page(second)
            .expect("snapshots merge should combine distinct symbols");
        first.clear_next_page_token();

        assert_eq!(first.next_page_token, None);
        assert_eq!(first.snapshots.len(), 2);
    }

    #[test]
    fn snapshot_merge_rejects_duplicate_symbols_across_pages() {
        let mut first = ChainResponse {
            snapshots: HashMap::from([(
                "AAPL260406C00180000".into(),
                serde_json::from_str(
                    r#"{"latestQuote":{"ap":77.75,"as":5,"ax":"A","bp":73.95,"bs":3,"bx":"N","c":" ","t":"2026-04-02T19:59:59.792862244Z"}}"#,
                )
                .expect("first snapshot should deserialize"),
            )]),
            next_page_token: Some("page-2".into()),
        };
        let second = ChainResponse {
            snapshots: HashMap::from([(
                "AAPL260406C00180000".into(),
                serde_json::from_str(
                    r#"{"latestQuote":{"ap":78.00,"as":1,"ax":"B","bp":74.00,"bs":2,"bx":"A","c":" ","t":"2026-04-02T20:00:00Z"}}"#,
                )
                .expect("duplicate snapshot should deserialize"),
            )]),
            next_page_token: None,
        };

        let error = first
            .merge_page(second)
            .expect_err("duplicate symbols should be rejected");
        assert!(matches!(error, Error::Pagination(_)));
    }
}
