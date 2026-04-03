use std::collections::HashMap;

use crate::{Error, transport::pagination::PaginatedResponse};

use super::{Bar, ConditionCode, Currency, ExchangeCode, Quote, Snapshot, Trade};

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestBarsResponse {
    pub bars: HashMap<String, Bar>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestQuotesResponse {
    pub quotes: HashMap<String, Quote>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestQuoteResponse {
    pub quote: Quote,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestTradesResponse {
    pub trades: HashMap<String, Trade>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SnapshotsResponse {
    pub snapshots: HashMap<String, Snapshot>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SnapshotResponse {
    pub snapshot: Snapshot,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConditionCodesResponse {
    pub condition_codes: Vec<ConditionCode>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeCodesResponse {
    pub exchange_codes: Vec<ExchangeCode>,
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
    use super::{BarsSingleResponse, QuotesSingleResponse, TradesSingleResponse};
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
}
