use std::collections::HashMap;

use crate::{Error, transport::pagination::PaginatedResponse};

use super::{Bar, Orderbook, Quote, Snapshot, Trade};

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct BarsResponse {
    pub bars: HashMap<String, Vec<Bar>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct QuotesResponse {
    pub quotes: HashMap<String, Vec<Quote>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct TradesResponse {
    pub trades: HashMap<String, Vec<Trade>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestBarsResponse {
    pub bars: HashMap<String, Bar>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestQuotesResponse {
    pub quotes: HashMap<String, Quote>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestTradesResponse {
    pub trades: HashMap<String, Trade>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestOrderbooksResponse {
    pub orderbooks: HashMap<String, Orderbook>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SnapshotsResponse {
    pub snapshots: HashMap<String, Snapshot>,
}

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

impl PaginatedResponse for QuotesResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_batch_page(&mut self.quotes, next.quotes);
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

    use crate::transport::pagination::PaginatedResponse;

    use super::{Bar, BarsResponse, QuotesResponse, TradesResponse};

    #[test]
    fn historical_responses_deserialize_official_wrapper_shapes() {
        let bars: BarsResponse = serde_json::from_str(
            r#"{"bars":{"BTC/USD":[{"c":66926.1935,"h":66946.85,"l":66920.6075,"n":0,"o":66942.46,"t":"2026-04-04T00:00:00Z","v":0.02821505,"vw":66933.72875}]},"next_page_token":"page-2"}"#,
        )
        .expect("bars response should deserialize");
        assert_eq!(bars.next_page_token.as_deref(), Some("page-2"));
        assert_eq!(
            bars.bars.get("BTC/USD").map(Vec::len).unwrap_or_default(),
            1
        );

        let quotes: QuotesResponse = serde_json::from_str(
            r#"{"quotes":{"BTC/USD":[{"ap":67005.5,"as":1.26733,"bp":66894.8,"bs":2.56753,"t":"2026-04-04T00:00:04.184229364Z"}]},"next_page_token":"page-3"}"#,
        )
        .expect("quotes response should deserialize");
        assert_eq!(quotes.next_page_token.as_deref(), Some("page-3"));
        assert_eq!(
            quotes
                .quotes
                .get("BTC/USD")
                .map(Vec::len)
                .unwrap_or_default(),
            1
        );

        let trades: TradesResponse = serde_json::from_str(
            r#"{"trades":{"BTC/USD":[{"i":5536632919737126473,"p":66969.687,"s":0.000073,"t":"2026-04-04T00:01:02.445726428Z","tks":"B"}]},"next_page_token":"page-4"}"#,
        )
        .expect("trades response should deserialize");
        assert_eq!(trades.next_page_token.as_deref(), Some("page-4"));
        assert_eq!(
            trades
                .trades
                .get("BTC/USD")
                .map(Vec::len)
                .unwrap_or_default(),
            1
        );
    }

    #[test]
    fn historical_merge_combines_symbol_buckets_and_clears_next_page_token() {
        let mut first = BarsResponse {
            bars: HashMap::from([(
                "BTC/USD".into(),
                vec![Bar {
                    t: Some("2026-04-04T00:00:00Z".into()),
                    ..Bar::default()
                }],
            )]),
            next_page_token: Some("page-2".into()),
        };
        let second = BarsResponse {
            bars: HashMap::from([(
                "ETH/USD".into(),
                vec![Bar {
                    t: Some("2026-04-04T00:00:00Z".into()),
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
    fn quote_and_trade_merge_append_more_items() {
        let mut quotes = QuotesResponse {
            quotes: HashMap::from([(
                "BTC/USD".into(),
                vec![super::Quote {
                    t: Some("2026-04-04T00:00:04.184229364Z".into()),
                    ..super::Quote::default()
                }],
            )]),
            next_page_token: Some("page-2".into()),
        };
        quotes
            .merge_page(QuotesResponse {
                quotes: HashMap::from([(
                    "BTC/USD".into(),
                    vec![super::Quote {
                        t: Some("2026-04-04T00:00:05.184229364Z".into()),
                        ..super::Quote::default()
                    }],
                )]),
                next_page_token: None,
            })
            .expect("quote merge should append items");
        assert_eq!(
            quotes
                .quotes
                .get("BTC/USD")
                .map(Vec::len)
                .unwrap_or_default(),
            2
        );

        let mut trades = TradesResponse {
            trades: HashMap::from([(
                "BTC/USD".into(),
                vec![super::Trade {
                    t: Some("2026-04-04T00:01:02.445726428Z".into()),
                    ..super::Trade::default()
                }],
            )]),
            next_page_token: Some("page-3".into()),
        };
        trades
            .merge_page(TradesResponse {
                trades: HashMap::from([(
                    "BTC/USD".into(),
                    vec![super::Trade {
                        t: Some("2026-04-04T00:01:03.445726428Z".into()),
                        ..super::Trade::default()
                    }],
                )]),
                next_page_token: None,
            })
            .expect("trade merge should append items");
        assert_eq!(
            trades
                .trades
                .get("BTC/USD")
                .map(Vec::len)
                .unwrap_or_default(),
            2
        );
    }
}
