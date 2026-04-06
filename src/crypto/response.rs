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

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestBarsResponse {
    pub bars: HashMap<String, Bar>,
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
pub struct LatestOrderbooksResponse {
    pub orderbooks: HashMap<String, Orderbook>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
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
    use std::{collections::HashMap, str::FromStr};

    use crate::transport::pagination::PaginatedResponse;
    use rust_decimal::Decimal;

    use super::{
        Bar, BarsResponse, LatestBarsResponse, LatestOrderbooksResponse, LatestQuotesResponse,
        LatestTradesResponse, QuotesResponse, SnapshotsResponse, TradesResponse,
    };

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
        assert_eq!(
            quotes
                .quotes
                .get("BTC/USD")
                .and_then(|quotes| quotes.first())
                .and_then(|quote| quote.ap.as_ref())
                .cloned(),
            Some(Decimal::from_str("67005.5").expect("decimal literal should parse"))
        );
        assert_eq!(
            quotes
                .quotes
                .get("BTC/USD")
                .and_then(|quotes| quotes.first())
                .and_then(|quote| quote.r#as.as_ref())
                .cloned(),
            Some(Decimal::from_str("1.26733").expect("decimal literal should parse"))
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

    #[test]
    fn latest_responses_deserialize_official_wrapper_shapes() {
        let latest_bars: LatestBarsResponse = serde_json::from_str(
            r#"{"bars":{"BTC/USD":{"c":66800.79,"h":66817.1675,"l":66800.79,"n":0,"o":66812.172,"t":"2026-04-04T04:13:00Z","v":0.0,"vw":66808.97875}}}"#,
        )
        .expect("latest bars response should deserialize");
        assert!(latest_bars.bars.contains_key("BTC/USD"));
        assert_eq!(
            latest_bars
                .bars
                .get("BTC/USD")
                .and_then(|bar| bar.c.as_ref())
                .cloned(),
            Some(Decimal::from_str("66800.79").expect("decimal literal should parse"))
        );
        assert_eq!(
            latest_bars
                .bars
                .get("BTC/USD")
                .and_then(|bar| bar.v.as_ref())
                .map(ToString::to_string),
            Some(
                Decimal::from_str("0.0")
                    .expect("decimal literal should parse")
                    .to_string()
            )
        );
        assert_eq!(
            latest_bars
                .bars
                .get("BTC/USD")
                .and_then(|bar| bar.v.as_ref())
                .map(Decimal::scale),
            Some(1)
        );

        let latest_quotes: LatestQuotesResponse = serde_json::from_str(
            r#"{"quotes":{"BTC/USD":{"ap":66819.4,"as":1.28052,"bp":66763.431,"bs":1.272,"t":"2026-04-04T04:14:35.580241652Z"}}}"#,
        )
        .expect("latest quotes response should deserialize");
        assert!(latest_quotes.quotes.contains_key("BTC/USD"));
        assert_eq!(
            latest_quotes
                .quotes
                .get("BTC/USD")
                .and_then(|quote| quote.bp.as_ref())
                .cloned(),
            Some(Decimal::from_str("66763.431").expect("decimal literal should parse"))
        );
        assert_eq!(
            latest_quotes
                .quotes
                .get("BTC/USD")
                .and_then(|quote| quote.bs.as_ref())
                .map(Decimal::scale),
            Some(3)
        );

        let latest_trades: LatestTradesResponse = serde_json::from_str(
            r#"{"trades":{"BTC/USD":{"i":519366231866950988,"p":66842.8,"s":0.000828,"t":"2026-04-04T04:12:55.361347989Z","tks":"B"}}}"#,
        )
        .expect("latest trades response should deserialize");
        assert!(latest_trades.trades.contains_key("BTC/USD"));
        assert_eq!(
            latest_trades
                .trades
                .get("BTC/USD")
                .and_then(|trade| trade.s.as_ref())
                .cloned(),
            Some(Decimal::from_str("0.000828").expect("decimal literal should parse"))
        );

        let latest_orderbooks: LatestOrderbooksResponse = serde_json::from_str(
            r#"{"orderbooks":{"BTC/USD":{"a":[{"p":66819.4,"s":1.28052},{"p":66847.8,"s":2.5525}],"b":[{"p":66763.431,"s":1.272},{"p":66743.135,"s":2.5795}],"t":"2026-04-04T04:14:35.581059122Z"}}}"#,
        )
        .expect("latest orderbooks response should deserialize");
        assert!(latest_orderbooks.orderbooks.contains_key("BTC/USD"));
        assert_eq!(
            latest_orderbooks
                .orderbooks
                .get("BTC/USD")
                .and_then(|orderbook| orderbook.a.as_ref())
                .and_then(|asks| asks.first())
                .and_then(|level| level.s.as_ref())
                .cloned(),
            Some(Decimal::from_str("1.28052").expect("decimal literal should parse"))
        );
    }

    #[test]
    fn snapshots_response_deserializes_official_wrapper_shape() {
        let snapshots: SnapshotsResponse = serde_json::from_str(
            r#"{"snapshots":{"BTC/USD":{"dailyBar":{"c":66800.79,"h":66975.1,"l":66772.66,"n":87,"o":66942.46,"t":"2026-04-04T00:00:00Z","v":0.029938953,"vw":66854.9651939408},"latestQuote":{"ap":66819.4,"as":1.28052,"bp":66763.431,"bs":1.272,"t":"2026-04-04T04:14:35.580241652Z"},"latestTrade":{"i":7456836641300628344,"p":66832.6,"s":0.000946,"t":"2026-04-04T04:14:32.161121311Z","tks":"B"},"minuteBar":{"c":66800.79,"h":66817.1675,"l":66800.79,"n":0,"o":66812.172,"t":"2026-04-04T04:13:00Z","v":0.0,"vw":66808.97875},"prevDailyBar":{"c":66961.45,"h":67293.2523,"l":66252.479,"n":549,"o":66887.805,"t":"2026-04-03T00:00:00Z","v":1.117036142,"vw":66779.3688392417}}}}"#,
        )
        .expect("snapshots response should deserialize");

        let snapshot = snapshots
            .snapshots
            .get("BTC/USD")
            .expect("snapshot response should include the symbol");
        assert!(snapshot.latestTrade.is_some());
        assert!(snapshot.latestQuote.is_some());
        assert!(snapshot.minuteBar.is_some());
        assert!(snapshot.dailyBar.is_some());
        assert!(snapshot.prevDailyBar.is_some());
    }
}
