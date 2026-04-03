use std::borrow::Cow;

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

    pub(crate) fn path(&self) -> Cow<'_, str> {
        match self {
            Self::CryptoLatestQuotes { loc: Loc::Us } => {
                Cow::Borrowed("/v1beta3/crypto/us/latest/quotes")
            }
            Self::CryptoLatestQuotes { loc: Loc::Us1 } => {
                Cow::Borrowed("/v1beta3/crypto/us1/latest/quotes")
            }
            Self::StocksBars => Cow::Borrowed("/v2/stocks/bars"),
            Self::StocksQuotes => Cow::Borrowed("/v2/stocks/quotes"),
            Self::StocksTrades => Cow::Borrowed("/v2/stocks/trades"),
            Self::StocksBarsSingle { symbol } => Cow::Owned(format!("/v2/stocks/{symbol}/bars")),
            Self::StocksQuotesSingle { symbol } => {
                Cow::Owned(format!("/v2/stocks/{symbol}/quotes"))
            }
            Self::StocksTradesSingle { symbol } => {
                Cow::Owned(format!("/v2/stocks/{symbol}/trades"))
            }
            Self::StocksLatestBars => Cow::Borrowed("/v2/stocks/bars/latest"),
            Self::StocksLatestQuotes => Cow::Borrowed("/v2/stocks/quotes/latest"),
            Self::StocksLatestTrades => Cow::Borrowed("/v2/stocks/trades/latest"),
            Self::StocksLatestBar { symbol } => {
                Cow::Owned(format!("/v2/stocks/{symbol}/bars/latest"))
            }
            Self::StocksLatestQuote { symbol } => {
                Cow::Owned(format!("/v2/stocks/{symbol}/quotes/latest"))
            }
            Self::StocksLatestTrade { symbol } => {
                Cow::Owned(format!("/v2/stocks/{symbol}/trades/latest"))
            }
            Self::StocksSnapshots => Cow::Borrowed("/v2/stocks/snapshots"),
            Self::StocksSnapshot { symbol } => Cow::Owned(format!("/v2/stocks/{symbol}/snapshot")),
            Self::StocksConditionCodes { ticktype } => {
                Cow::Owned(format!("/v2/stocks/meta/conditions/{ticktype}"))
            }
            Self::StocksExchangeCodes => Cow::Borrowed("/v2/stocks/meta/exchanges"),
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
    use std::borrow::Cow;

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

    #[test]
    fn endpoint_keeps_static_paths_borrowed() {
        assert!(matches!(
            Endpoint::crypto_latest_quotes(Loc::Us).path(),
            Cow::Borrowed("/v1beta3/crypto/us/latest/quotes")
        ));
        assert!(matches!(
            Endpoint::StocksBars.path(),
            Cow::Borrowed("/v2/stocks/bars")
        ));
        assert!(matches!(
            Endpoint::StocksQuotes.path(),
            Cow::Borrowed("/v2/stocks/quotes")
        ));
        assert!(matches!(
            Endpoint::StocksTrades.path(),
            Cow::Borrowed("/v2/stocks/trades")
        ));
        assert!(matches!(
            Endpoint::StocksLatestBars.path(),
            Cow::Borrowed("/v2/stocks/bars/latest")
        ));
        assert!(matches!(
            Endpoint::StocksLatestQuotes.path(),
            Cow::Borrowed("/v2/stocks/quotes/latest")
        ));
        assert!(matches!(
            Endpoint::StocksLatestTrades.path(),
            Cow::Borrowed("/v2/stocks/trades/latest")
        ));
        assert!(matches!(
            Endpoint::StocksSnapshots.path(),
            Cow::Borrowed("/v2/stocks/snapshots")
        ));
        assert!(matches!(
            Endpoint::StocksExchangeCodes.path(),
            Cow::Borrowed("/v2/stocks/meta/exchanges")
        ));
    }

    #[test]
    fn endpoint_routes_all_stocks_dynamic_paths_and_marks_them_authenticated() {
        let cases = [
            (
                Endpoint::StocksBarsSingle {
                    symbol: "AAPL".into(),
                },
                "/v2/stocks/AAPL/bars",
            ),
            (
                Endpoint::StocksQuotesSingle {
                    symbol: "AAPL".into(),
                },
                "/v2/stocks/AAPL/quotes",
            ),
            (
                Endpoint::StocksTradesSingle {
                    symbol: "AAPL".into(),
                },
                "/v2/stocks/AAPL/trades",
            ),
            (
                Endpoint::StocksLatestBar {
                    symbol: "AAPL".into(),
                },
                "/v2/stocks/AAPL/bars/latest",
            ),
            (
                Endpoint::StocksLatestQuote {
                    symbol: "AAPL".into(),
                },
                "/v2/stocks/AAPL/quotes/latest",
            ),
            (
                Endpoint::StocksLatestTrade {
                    symbol: "AAPL".into(),
                },
                "/v2/stocks/AAPL/trades/latest",
            ),
            (
                Endpoint::StocksSnapshot {
                    symbol: "AAPL".into(),
                },
                "/v2/stocks/AAPL/snapshot",
            ),
            (
                Endpoint::StocksConditionCodes { ticktype: "trade" },
                "/v2/stocks/meta/conditions/trade",
            ),
        ];

        for (endpoint, expected_path) in cases {
            assert!(matches!(endpoint.path(), Cow::Owned(_)));
            assert_eq!(endpoint.path(), expected_path);
            assert!(endpoint.requires_auth());
        }
    }

    #[test]
    fn endpoint_requires_auth_for_all_stocks_batch_routes() {
        let cases = [
            Endpoint::StocksBars,
            Endpoint::StocksQuotes,
            Endpoint::StocksTrades,
            Endpoint::StocksLatestBars,
            Endpoint::StocksLatestQuotes,
            Endpoint::StocksLatestTrades,
            Endpoint::StocksSnapshots,
            Endpoint::StocksExchangeCodes,
        ];

        for endpoint in cases {
            assert!(endpoint.requires_auth());
        }

        assert!(!Endpoint::crypto_latest_quotes(Loc::Us1).requires_auth());
    }
}
