use crate::crypto::Loc;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Endpoint {
    CryptoLatestQuotes { loc: Loc },
    StocksBars,
    StocksQuotes,
    StocksTrades,
    StocksBarsSingle { symbol: String },
    StocksQuotesSingle { symbol: String },
    StocksTradesSingle { symbol: String },
    StocksLatestBars,
    StocksLatestQuotes,
    StocksLatestTrades,
    StocksLatestBar { symbol: String },
    StocksLatestQuote { symbol: String },
    StocksLatestTrade { symbol: String },
    StocksSnapshots,
    StocksSnapshot { symbol: String },
    StocksConditionCodes { ticktype: &'static str },
    StocksExchangeCodes,
}

#[allow(dead_code)]
impl Endpoint {
    pub(crate) fn crypto_latest_quotes(loc: Loc) -> Self {
        Self::CryptoLatestQuotes { loc }
    }

    pub(crate) fn stocks_bars() -> Self {
        Self::StocksBars
    }

    pub(crate) fn stocks_bars_single(symbol: impl Into<String>) -> Self {
        Self::StocksBarsSingle {
            symbol: symbol.into(),
        }
    }

    pub(crate) fn stocks_latest_quote(symbol: impl Into<String>) -> Self {
        Self::StocksLatestQuote {
            symbol: symbol.into(),
        }
    }

    pub(crate) fn stocks_snapshot(symbol: impl Into<String>) -> Self {
        Self::StocksSnapshot {
            symbol: symbol.into(),
        }
    }

    pub(crate) fn path(&self) -> String {
        match self {
            Self::CryptoLatestQuotes { loc: Loc::Us } => "/v1beta3/crypto/us/latest/quotes".into(),
            Self::CryptoLatestQuotes { loc: Loc::Us1 } => {
                "/v1beta3/crypto/us1/latest/quotes".into()
            }
            Self::StocksBars => "/v2/stocks/bars".into(),
            Self::StocksQuotes => "/v2/stocks/quotes".into(),
            Self::StocksTrades => "/v2/stocks/trades".into(),
            Self::StocksBarsSingle { symbol } => format!("/v2/stocks/{symbol}/bars"),
            Self::StocksQuotesSingle { symbol } => format!("/v2/stocks/{symbol}/quotes"),
            Self::StocksTradesSingle { symbol } => format!("/v2/stocks/{symbol}/trades"),
            Self::StocksLatestBars => "/v2/stocks/bars/latest".into(),
            Self::StocksLatestQuotes => "/v2/stocks/quotes/latest".into(),
            Self::StocksLatestTrades => "/v2/stocks/trades/latest".into(),
            Self::StocksLatestBar { symbol } => format!("/v2/stocks/{symbol}/bars/latest"),
            Self::StocksLatestQuote { symbol } => format!("/v2/stocks/{symbol}/quotes/latest"),
            Self::StocksLatestTrade { symbol } => format!("/v2/stocks/{symbol}/trades/latest"),
            Self::StocksSnapshots => "/v2/stocks/snapshots".into(),
            Self::StocksSnapshot { symbol } => format!("/v2/stocks/{symbol}/snapshot"),
            Self::StocksConditionCodes { ticktype } => {
                format!("/v2/stocks/meta/conditions/{ticktype}")
            }
            Self::StocksExchangeCodes => "/v2/stocks/meta/exchanges".into(),
        }
    }

    pub(crate) fn requires_auth(&self) -> bool {
        match self {
            Self::CryptoLatestQuotes { .. } => false,
            Self::StocksBars
            | Self::StocksQuotes
            | Self::StocksTrades
            | Self::StocksBarsSingle { .. }
            | Self::StocksQuotesSingle { .. }
            | Self::StocksTradesSingle { .. }
            | Self::StocksLatestBars
            | Self::StocksLatestQuotes
            | Self::StocksLatestTrades
            | Self::StocksLatestBar { .. }
            | Self::StocksLatestQuote { .. }
            | Self::StocksLatestTrade { .. }
            | Self::StocksSnapshots
            | Self::StocksSnapshot { .. }
            | Self::StocksConditionCodes { .. }
            | Self::StocksExchangeCodes => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Endpoint;
    use crate::crypto::Loc;

    #[test]
    fn endpoint_routes_crypto_latest_quotes_to_official_path() {
        let endpoint = Endpoint::crypto_latest_quotes(Loc::Us);

        assert_eq!(endpoint.path(), "/v1beta3/crypto/us/latest/quotes");
        assert!(!endpoint.requires_auth());
    }

    #[test]
    fn endpoint_routes_stocks_batch_and_single_paths() {
        assert_eq!(Endpoint::stocks_bars().path(), "/v2/stocks/bars");
        assert_eq!(
            Endpoint::stocks_bars_single("AAPL").path(),
            "/v2/stocks/AAPL/bars"
        );
        assert_eq!(
            Endpoint::stocks_latest_quote("AAPL").path(),
            "/v2/stocks/AAPL/quotes/latest"
        );
        assert_eq!(
            Endpoint::stocks_snapshot("AAPL").path(),
            "/v2/stocks/AAPL/snapshot"
        );
    }
}
