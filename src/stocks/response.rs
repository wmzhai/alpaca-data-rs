use std::collections::HashMap;

use crate::{Error, transport::pagination::PaginatedResponse};

use super::{Bar, Currency, Quote, Snapshot, Trade};

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

#[cfg(test)]
mod tests {
    use super::{
        BarsSingleResponse, ConditionCodesResponse, ExchangeCodesResponse, LatestBarResponse,
        LatestBarsResponse, LatestQuoteResponse, LatestQuotesResponse, LatestTradeResponse,
        LatestTradesResponse, QuotesSingleResponse, SnapshotResponse, SnapshotsResponse,
        TradesSingleResponse,
    };
    use crate::{Error, transport::pagination::PaginatedResponse};

    #[test]
    fn single_historical_responses_deserialize_official_wrapper_fields() {
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
            batch_bars.currency.as_ref().map(|value| value.as_str()),
            Some("USD")
        );

        let single_bar: LatestBarResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","bar":{"t":"2024-03-01T20:00:00Z","c":179.66},"currency":"USD"}"#,
        )
        .expect("latest bar response should deserialize");
        assert_eq!(single_bar.symbol, "AAPL");
        assert!(single_bar.bar.c.is_some());

        let batch_quotes: LatestQuotesResponse = serde_json::from_str(
            r#"{"quotes":{"AAPL":{"t":"2024-03-01T20:00:00Z","bp":179.65}},"currency":"USD"}"#,
        )
        .expect("latest quotes response should deserialize");
        assert!(batch_quotes.quotes.contains_key("AAPL"));

        let single_quote: LatestQuoteResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","quote":{"t":"2024-03-01T20:00:00Z","bp":179.65},"currency":"USD"}"#,
        )
        .expect("latest quote response should deserialize");
        assert_eq!(single_quote.symbol, "AAPL");
        assert!(single_quote.quote.bp.is_some());

        let batch_trades: LatestTradesResponse = serde_json::from_str(
            r#"{"trades":{"AAPL":{"t":"2024-03-01T20:00:00Z","p":179.64}},"currency":"USD"}"#,
        )
        .expect("latest trades response should deserialize");
        assert!(batch_trades.trades.contains_key("AAPL"));

        let single_trade: LatestTradeResponse = serde_json::from_str(
            r#"{"symbol":"AAPL","trade":{"t":"2024-03-01T20:00:00Z","p":179.64},"currency":"USD"}"#,
        )
        .expect("latest trade response should deserialize");
        assert_eq!(single_trade.symbol, "AAPL");
        assert!(single_trade.trade.p.is_some());
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
}
