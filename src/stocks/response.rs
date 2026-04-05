use std::collections::HashMap;

use crate::{Error, transport::pagination::PaginatedResponse};

use super::{Bar, Currency, DailyAuction, Quote, Snapshot, Trade};

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct BarsResponse {
    pub bars: HashMap<String, Vec<Bar>>,
    pub next_page_token: Option<String>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct BarsSingleResponse {
    pub symbol: String,
    pub bars: Vec<Bar>,
    pub next_page_token: Option<String>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct AuctionsResponse {
    pub auctions: HashMap<String, Vec<DailyAuction>>,
    pub next_page_token: Option<String>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct AuctionsSingleResponse {
    pub symbol: String,
    pub auctions: Vec<DailyAuction>,
    pub next_page_token: Option<String>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct QuotesResponse {
    pub quotes: HashMap<String, Vec<Quote>>,
    pub next_page_token: Option<String>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct TradesResponse {
    pub trades: HashMap<String, Vec<Trade>>,
    pub next_page_token: Option<String>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct QuotesSingleResponse {
    pub symbol: String,
    pub quotes: Vec<Quote>,
    pub next_page_token: Option<String>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct TradesSingleResponse {
    pub symbol: String,
    pub trades: Vec<Trade>,
    pub next_page_token: Option<String>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestBarsResponse {
    pub bars: HashMap<String, Bar>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestBarResponse {
    pub symbol: String,
    pub bar: Bar,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestQuotesResponse {
    pub quotes: HashMap<String, Quote>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestQuoteResponse {
    pub symbol: String,
    pub quote: Quote,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestTradesResponse {
    pub trades: HashMap<String, Trade>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct LatestTradeResponse {
    pub symbol: String,
    pub trade: Trade,
    pub currency: Option<Currency>,
}

pub type SnapshotsResponse = HashMap<String, Snapshot>;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct SnapshotResponse {
    pub symbol: String,
    pub currency: Option<Currency>,
    pub latestTrade: Option<Trade>,
    pub latestQuote: Option<Quote>,
    pub minuteBar: Option<Bar>,
    pub dailyBar: Option<Bar>,
    pub prevDailyBar: Option<Bar>,
}

pub type ConditionCodesResponse = HashMap<String, String>;

pub type ExchangeCodesResponse = HashMap<String, String>;

fn merge_batch_currency(
    operation: &'static str,
    currency: &mut Option<Currency>,
    next_currency: Option<Currency>,
) -> Result<(), Error> {
    match (currency.as_ref(), next_currency) {
        (Some(current), Some(next)) if current != &next => Err(Error::Pagination(format!(
            "{operation} received mismatched currency across pages: expected {}, got {}",
            current.as_str(),
            next.as_str()
        ))),
        (None, Some(next)) => {
            *currency = Some(next);
            Ok(())
        }
        _ => Ok(()),
    }
}

fn merge_batch_page<Item>(
    current: &mut HashMap<String, Vec<Item>>,
    next: HashMap<String, Vec<Item>>,
) {
    for (symbol, mut items) in next {
        current.entry(symbol).or_default().append(&mut items);
    }
}

fn merge_single_metadata(
    operation: &'static str,
    symbol: &mut String,
    currency: &mut Option<Currency>,
    next_symbol: String,
    next_currency: Option<Currency>,
) -> Result<(), Error> {
    if !symbol.is_empty() && *symbol != next_symbol {
        return Err(Error::Pagination(format!(
            "{operation} received mismatched symbol across pages: expected {}, got {}",
            symbol, next_symbol
        )));
    }

    if symbol.is_empty() {
        *symbol = next_symbol;
    }

    match (currency.as_ref(), next_currency) {
        (Some(current), Some(next)) if current != &next => Err(Error::Pagination(format!(
            "{operation} received mismatched currency across pages: expected {}, got {}",
            current.as_str(),
            next.as_str()
        ))),
        (None, Some(next)) => {
            *currency = Some(next);
            Ok(())
        }
        _ => Ok(()),
    }
}

impl PaginatedResponse for BarsSingleResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_single_metadata(
            "stocks.bars_single_all",
            &mut self.symbol,
            &mut self.currency,
            next.symbol,
            next.currency,
        )?;
        self.bars.extend(next.bars);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

impl PaginatedResponse for BarsResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_batch_currency("stocks.bars_all", &mut self.currency, next.currency)?;
        merge_batch_page(&mut self.bars, next.bars);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

impl PaginatedResponse for AuctionsSingleResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_single_metadata(
            "stocks.auctions_single_all",
            &mut self.symbol,
            &mut self.currency,
            next.symbol,
            next.currency,
        )?;
        self.auctions.extend(next.auctions);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

impl PaginatedResponse for AuctionsResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_batch_currency("stocks.auctions_all", &mut self.currency, next.currency)?;
        merge_batch_page(&mut self.auctions, next.auctions);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

impl PaginatedResponse for QuotesSingleResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_single_metadata(
            "stocks.quotes_single_all",
            &mut self.symbol,
            &mut self.currency,
            next.symbol,
            next.currency,
        )?;
        self.quotes.extend(next.quotes);
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
        merge_batch_currency("stocks.quotes_all", &mut self.currency, next.currency)?;
        merge_batch_page(&mut self.quotes, next.quotes);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

impl PaginatedResponse for TradesSingleResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        merge_single_metadata(
            "stocks.trades_single_all",
            &mut self.symbol,
            &mut self.currency,
            next.symbol,
            next.currency,
        )?;
        self.trades.extend(next.trades);
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
        merge_batch_currency("stocks.trades_all", &mut self.currency, next.currency)?;
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

    use super::{
        AuctionsResponse, AuctionsSingleResponse, BarsResponse, BarsSingleResponse,
        ConditionCodesResponse, ExchangeCodesResponse, LatestBarResponse, LatestBarsResponse,
        LatestQuoteResponse, LatestQuotesResponse, LatestTradeResponse, LatestTradesResponse,
        QuotesSingleResponse, SnapshotResponse, SnapshotsResponse, TradesSingleResponse,
    };
    use crate::{Decimal, Error, transport::pagination::PaginatedResponse};

    #[test]
    fn single_historical_responses_deserialize_official_wrapper_fields() {
        let auctions: AuctionsSingleResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","auctions":[],"next_page_token":"page-1","currency":"USD"}"#,
        )
        .expect("auctions single response should deserialize");
        assert_eq!(auctions.symbol, "AAPL");
        assert_eq!(auctions.next_page_token.as_deref(), Some("page-1"));
        assert_eq!(
            auctions.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );

        let bars: BarsSingleResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","bars":[],"next_page_token":"page-2","currency":"USD"}"#,
        )
        .expect("bars single response should deserialize");
        assert_eq!(bars.symbol, "AAPL");
        assert_eq!(bars.next_page_token.as_deref(), Some("page-2"));
        assert_eq!(
            bars.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );

        let quotes: QuotesSingleResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","quotes":[],"next_page_token":"page-3","currency":"USD"}"#,
        )
        .expect("quotes single response should deserialize");
        assert_eq!(quotes.symbol, "AAPL");
        assert_eq!(quotes.next_page_token.as_deref(), Some("page-3"));
        assert_eq!(
            quotes.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );

        let trades: TradesSingleResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","trades":[],"next_page_token":"page-4","currency":"USD"}"#,
        )
        .expect("trades single response should deserialize");
        assert_eq!(trades.symbol, "AAPL");
        assert_eq!(trades.next_page_token.as_deref(), Some("page-4"));
        assert_eq!(
            trades.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );
    }

    #[test]
    fn single_historical_merge_preserves_symbol_and_currency() {
        let mut auctions = AuctionsSingleResponse {
            symbol: "AAPL".into(),
            auctions: vec![],
            next_page_token: Some("page-2".into()),
            currency: Some("USD".into()),
        };

        auctions
            .merge_page(AuctionsSingleResponse {
                symbol: "AAPL".into(),
                auctions: vec![],
                next_page_token: None,
                currency: Some("USD".into()),
            })
            .expect("matching auction pages should merge");

        assert_eq!(auctions.symbol, "AAPL");
        assert_eq!(
            auctions.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );
        assert_eq!(auctions.next_page_token, None);

        let mut first = BarsSingleResponse {
            symbol: "AAPL".into(),
            bars: vec![],
            next_page_token: Some("page-2".into()),
            currency: Some("USD".into()),
        };

        first
            .merge_page(BarsSingleResponse {
                symbol: "AAPL".into(),
                bars: vec![],
                next_page_token: None,
                currency: Some("USD".into()),
            })
            .expect("matching pages should merge");

        assert_eq!(first.symbol, "AAPL");
        assert_eq!(
            first.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );
        assert_eq!(first.next_page_token, None);
    }

    #[test]
    fn single_historical_merge_rejects_mismatched_symbol_or_currency() {
        let mut auctions_symbol_mismatch = AuctionsSingleResponse {
            symbol: "AAPL".into(),
            auctions: vec![],
            next_page_token: Some("page-2".into()),
            currency: Some("USD".into()),
        };

        let auctions_symbol_error = auctions_symbol_mismatch
            .merge_page(AuctionsSingleResponse {
                symbol: "MSFT".into(),
                auctions: vec![],
                next_page_token: None,
                currency: Some("USD".into()),
            })
            .expect_err("mismatched auction symbols should fail");
        assert!(matches!(auctions_symbol_error, Error::Pagination(_)));

        let mut symbol_mismatch = BarsSingleResponse {
            symbol: "AAPL".into(),
            bars: vec![],
            next_page_token: Some("page-2".into()),
            currency: Some("USD".into()),
        };

        let symbol_error = symbol_mismatch
            .merge_page(BarsSingleResponse {
                symbol: "MSFT".into(),
                bars: vec![],
                next_page_token: None,
                currency: Some("USD".into()),
            })
            .expect_err("mismatched symbols should fail");
        assert!(matches!(symbol_error, Error::Pagination(_)));

        let mut currency_mismatch = BarsSingleResponse {
            symbol: "AAPL".into(),
            bars: vec![],
            next_page_token: Some("page-2".into()),
            currency: Some("USD".into()),
        };

        let currency_error = currency_mismatch
            .merge_page(BarsSingleResponse {
                symbol: "AAPL".into(),
                bars: vec![],
                next_page_token: None,
                currency: Some("CAD".into()),
            })
            .expect_err("mismatched currencies should fail");
        assert!(matches!(currency_error, Error::Pagination(_)));
    }

    #[test]
    fn latest_responses_deserialize_official_wrapper_shapes() {
        let batch_bars: LatestBarsResponse = serde_json::from_str(
            r#"{"bars":{"AAPL":{"t":"2024-03-01T20:00:00Z","c":179.66}},"currency":"USD"}"#,
        )
        .expect("latest bars response should deserialize");
        assert!(batch_bars.bars.contains_key("AAPL"));
        assert_eq!(
            batch_bars
                .bars
                .get("AAPL")
                .and_then(|bar| bar.c.as_ref())
                .cloned(),
            Some(Decimal::from_str("179.66").expect("decimal literal should parse"))
        );
        assert_eq!(
            batch_bars.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );

        let single_bar: LatestBarResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","bar":{"t":"2024-03-01T20:00:00Z","c":179.66},"currency":"USD"}"#,
        )
        .expect("latest bar response should deserialize");
        assert_eq!(single_bar.symbol, "AAPL");
        assert_eq!(
            single_bar.bar.c,
            Some(Decimal::from_str("179.66").expect("decimal literal should parse"))
        );

        let batch_quotes: LatestQuotesResponse = serde_json::from_str(
            r#"{"quotes":{"AAPL":{"t":"2024-03-01T20:00:00Z","bp":179.65}},"currency":"USD"}"#,
        )
        .expect("latest quotes response should deserialize");
        assert!(batch_quotes.quotes.contains_key("AAPL"));
        assert_eq!(
            batch_quotes
                .quotes
                .get("AAPL")
                .and_then(|quote| quote.bp.as_ref())
                .cloned(),
            Some(Decimal::from_str("179.65").expect("decimal literal should parse"))
        );

        let single_quote: LatestQuoteResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","quote":{"t":"2024-03-01T20:00:00Z","bp":179.65},"currency":"USD"}"#,
        )
        .expect("latest quote response should deserialize");
        assert_eq!(single_quote.symbol, "AAPL");
        assert_eq!(
            single_quote.quote.bp,
            Some(Decimal::from_str("179.65").expect("decimal literal should parse"))
        );

        let batch_trades: LatestTradesResponse = serde_json::from_str(
            r#"{"trades":{"AAPL":{"t":"2024-03-01T20:00:00Z","p":179.64}},"currency":"USD"}"#,
        )
        .expect("latest trades response should deserialize");
        assert!(batch_trades.trades.contains_key("AAPL"));
        assert_eq!(
            batch_trades
                .trades
                .get("AAPL")
                .and_then(|trade| trade.p.as_ref())
                .cloned(),
            Some(Decimal::from_str("179.64").expect("decimal literal should parse"))
        );

        let single_trade: LatestTradeResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","trade":{"t":"2024-03-01T20:00:00Z","p":179.64},"currency":"USD"}"#,
        )
        .expect("latest trade response should deserialize");
        assert_eq!(single_trade.symbol, "AAPL");
        assert_eq!(
            single_trade.trade.p,
            Some(Decimal::from_str("179.64").expect("decimal literal should parse"))
        );
    }

    #[test]
    fn snapshot_responses_deserialize_official_batch_and_single_shapes() {
        let batch: SnapshotsResponse = serde_json::from_str(
            r#"{
                "AAPL":{
                    "latestTrade":{"t":"2024-03-01T20:00:00Z","p":179.64},
                    "latestQuote":{"t":"2024-03-01T20:00:00Z","bp":179.65},
                    "minuteBar":{"t":"2024-03-01T20:00:00Z","c":179.66},
                    "dailyBar":{"t":"2024-03-01T20:00:00Z","c":179.66},
                    "prevDailyBar":{"t":"2024-02-29T20:00:00Z","c":180.75}
                }
            }"#,
        )
        .expect("batch snapshots response should deserialize");
        let aapl = batch
            .get("AAPL")
            .expect("batch snapshots response should keep the symbol as the top-level key");
        assert!(aapl.latestTrade.is_some());
        assert!(aapl.latestQuote.is_some());
        assert!(aapl.minuteBar.is_some());
        assert!(aapl.dailyBar.is_some());
        assert!(aapl.prevDailyBar.is_some());
        assert_eq!(
            aapl.latestQuote
                .as_ref()
                .and_then(|quote| quote.bp.as_ref())
                .cloned(),
            Some(Decimal::from_str("179.65").expect("decimal literal should parse"))
        );
        assert_eq!(
            aapl.latestTrade
                .as_ref()
                .and_then(|trade| trade.p.as_ref())
                .cloned(),
            Some(Decimal::from_str("179.64").expect("decimal literal should parse"))
        );
        assert_eq!(
            aapl.minuteBar
                .as_ref()
                .and_then(|bar| bar.c.as_ref())
                .cloned(),
            Some(Decimal::from_str("179.66").expect("decimal literal should parse"))
        );

        let single: SnapshotResponse = serde_json::from_str(
            r#"{
                "symbol":"AAPL",
                "currency":"USD",
                "latestTrade":{"t":"2024-03-01T20:00:00Z","p":179.64},
                "latestQuote":{"t":"2024-03-01T20:00:00Z","bp":179.65},
                "minuteBar":{"t":"2024-03-01T20:00:00Z","c":179.66},
                "dailyBar":{"t":"2024-03-01T20:00:00Z","c":179.66},
                "prevDailyBar":{"t":"2024-02-29T20:00:00Z","c":180.75}
            }"#,
        )
        .expect("single snapshot response should deserialize");
        assert_eq!(single.symbol, "AAPL");
        assert_eq!(
            single.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );
        assert!(single.latestTrade.is_some());
        assert!(single.latestQuote.is_some());
        assert!(single.minuteBar.is_some());
        assert!(single.dailyBar.is_some());
        assert!(single.prevDailyBar.is_some());
        assert_eq!(
            single
                .latestQuote
                .as_ref()
                .and_then(|quote| quote.bp.as_ref())
                .cloned(),
            Some(Decimal::from_str("179.65").expect("decimal literal should parse"))
        );
    }

    #[test]
    fn metadata_responses_deserialize_official_map_shapes() {
        let condition_codes: ConditionCodesResponse =
            serde_json::from_str(r#"{" ":"Regular Sale","4":"Derivatively Priced"}"#)
                .expect("condition codes should deserialize as a top-level map");
        assert_eq!(
            condition_codes.get(" ").map(String::as_str),
            Some("Regular Sale")
        );
        assert_eq!(
            condition_codes.get("4").map(String::as_str),
            Some("Derivatively Priced")
        );

        let exchange_codes: ExchangeCodesResponse =
            serde_json::from_str(r#"{"V":"IEX","N":"New York Stock Exchange"}"#)
                .expect("exchange codes should deserialize as a top-level map");
        assert_eq!(exchange_codes.get("V").map(String::as_str), Some("IEX"));
        assert_eq!(
            exchange_codes.get("N").map(String::as_str),
            Some("New York Stock Exchange")
        );
    }

    #[test]
    fn auctions_responses_deserialize_official_wrapper_shapes() {
        let batch: AuctionsResponse = serde_json::from_str(
            r#"{"auctions":{"AAPL":[{"d":"2024-03-01","o":[{"c":"Q","p":179.55,"s":8,"t":"2024-03-01T14:30:00.092366196Z","x":"P"}],"c":[{"c":"M","p":179.64,"s":2008,"t":"2024-03-01T21:00:00.071062102Z","x":"P"}]}]},"next_page_token":"page-2","currency":"USD"}"#,
        )
        .expect("batch auctions response should deserialize");
        assert_eq!(batch.next_page_token.as_deref(), Some("page-2"));
        assert_eq!(
            batch.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );
        assert_eq!(batch.auctions.get("AAPL").map(Vec::len), Some(1));

        let single: AuctionsSingleResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","auctions":[{"d":"2024-03-01","o":[{"c":"Q","p":179.55,"s":8,"t":"2024-03-01T14:30:00.092366196Z","x":"P"}],"c":[{"c":"M","p":179.64,"s":2008,"t":"2024-03-01T21:00:00.071062102Z","x":"P"}]}],"next_page_token":null,"currency":"USD"}"#,
        )
        .expect("single auctions response should deserialize");
        assert_eq!(single.symbol, "AAPL");
        assert_eq!(single.auctions.len(), 1);
    }

    #[test]
    fn auctions_batch_merge_combines_symbol_buckets_and_clears_next_page_token() {
        let mut first = AuctionsResponse {
            auctions: HashMap::from([(
                "AAPL".into(),
                vec![serde_json::from_str(
                    r#"{"d":"2024-03-01","o":[{"c":"Q","p":179.55,"t":"2024-03-01T14:30:00.092366196Z","x":"P"}],"c":[{"c":"M","p":179.64,"t":"2024-03-01T21:00:00.071062102Z","x":"P"}]}"#,
                )
                .expect("daily auction should deserialize")],
            )]),
            next_page_token: Some("page-2".into()),
            currency: Some("USD".into()),
        };

        first
            .merge_page(AuctionsResponse {
                auctions: HashMap::from([(
                    "MSFT".into(),
                    vec![serde_json::from_str(
                        r#"{"d":"2024-03-01","o":[{"c":"Q","p":415.10,"t":"2024-03-01T14:30:00.100000000Z","x":"P"}],"c":[{"c":"M","p":415.20,"t":"2024-03-01T21:00:00.100000000Z","x":"P"}]}"#,
                    )
                    .expect("daily auction should deserialize")],
                )]),
                next_page_token: None,
                currency: Some("USD".into()),
            })
            .expect("matching auction currencies should merge");

        assert_eq!(first.auctions.get("AAPL").map(Vec::len), Some(1));
        assert_eq!(first.auctions.get("MSFT").map(Vec::len), Some(1));
        assert_eq!(first.next_page_token, None);
    }

    #[test]
    fn batch_historical_merge_combines_symbol_buckets_and_clears_next_page_token() {
        let mut first = BarsResponse {
            bars: HashMap::from([
                (
                    "AAPL".into(),
                    vec![
                        serde_json::from_str(r#"{"t":"2024-03-01T20:00:00Z","c":179.66}"#)
                            .expect("bar should deserialize"),
                    ],
                ),
                (
                    "MSFT".into(),
                    vec![
                        serde_json::from_str(r#"{"t":"2024-03-01T20:00:00Z","c":415.32}"#)
                            .expect("bar should deserialize"),
                    ],
                ),
            ]),
            next_page_token: Some("page-2".into()),
            currency: Some("USD".into()),
        };

        first
            .merge_page(BarsResponse {
                bars: HashMap::from([
                    (
                        "AAPL".into(),
                        vec![
                            serde_json::from_str(r#"{"t":"2024-03-04T20:00:00Z","c":175.10}"#)
                                .expect("bar should deserialize"),
                        ],
                    ),
                    (
                        "NVDA".into(),
                        vec![
                            serde_json::from_str(r#"{"t":"2024-03-04T20:00:00Z","c":852.37}"#)
                                .expect("bar should deserialize"),
                        ],
                    ),
                ]),
                next_page_token: None,
                currency: Some("USD".into()),
            })
            .expect("matching currencies should merge");

        assert_eq!(first.bars.get("AAPL").map(Vec::len), Some(2));
        assert_eq!(first.bars.get("MSFT").map(Vec::len), Some(1));
        assert_eq!(first.bars.get("NVDA").map(Vec::len), Some(1));
        assert_eq!(
            first
                .bars
                .get("AAPL")
                .and_then(|bars| bars.last())
                .and_then(|bar| bar.c.as_ref())
                .map(ToString::to_string),
            Some(
                Decimal::from_str("175.10")
                    .expect("decimal literal should parse")
                    .to_string()
            )
        );
        assert_eq!(first.next_page_token, None);
    }

    #[test]
    fn batch_historical_merge_rejects_mismatched_currency() {
        let mut first = BarsResponse {
            bars: HashMap::new(),
            next_page_token: Some("page-2".into()),
            currency: Some("USD".into()),
        };

        let error = first
            .merge_page(BarsResponse {
                bars: HashMap::new(),
                next_page_token: None,
                currency: Some("CAD".into()),
            })
            .expect_err("mismatched currencies should fail");

        assert!(matches!(error, Error::Pagination(_)));
    }
}
