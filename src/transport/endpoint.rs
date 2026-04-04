use std::borrow::Cow;

use crate::crypto::Loc;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Endpoint {
    CryptoBars { loc: Loc },
    CryptoQuotes { loc: Loc },
    CryptoTrades { loc: Loc },
    CryptoLatestBars { loc: Loc },
    CryptoLatestQuotes { loc: Loc },
    CryptoLatestTrades { loc: Loc },
    CryptoLatestOrderbooks { loc: Loc },
    CryptoSnapshots { loc: Loc },
    NewsList,
    OptionsBars,
    OptionsTrades,
    OptionsLatestQuotes,
    OptionsLatestTrades,
    OptionsSnapshots,
    OptionsChain { underlying_symbol: String },
    OptionsExchangeCodes,
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
    CorporateActionsList,
}

#[allow(dead_code)]
impl Endpoint {
    pub(crate) fn crypto_bars(loc: Loc) -> Self {
        Self::CryptoBars { loc }
    }

    pub(crate) fn crypto_quotes(loc: Loc) -> Self {
        Self::CryptoQuotes { loc }
    }

    pub(crate) fn crypto_trades(loc: Loc) -> Self {
        Self::CryptoTrades { loc }
    }

    pub(crate) fn crypto_latest_quotes(loc: Loc) -> Self {
        Self::CryptoLatestQuotes { loc }
    }

    pub(crate) fn crypto_latest_bars(loc: Loc) -> Self {
        Self::CryptoLatestBars { loc }
    }

    pub(crate) fn crypto_latest_trades(loc: Loc) -> Self {
        Self::CryptoLatestTrades { loc }
    }

    pub(crate) fn crypto_latest_orderbooks(loc: Loc) -> Self {
        Self::CryptoLatestOrderbooks { loc }
    }

    pub(crate) fn crypto_snapshots(loc: Loc) -> Self {
        Self::CryptoSnapshots { loc }
    }

    pub(crate) fn options_bars() -> Self {
        Self::OptionsBars
    }

    pub(crate) fn options_trades() -> Self {
        Self::OptionsTrades
    }

    pub(crate) fn options_latest_quotes() -> Self {
        Self::OptionsLatestQuotes
    }

    pub(crate) fn options_latest_trades() -> Self {
        Self::OptionsLatestTrades
    }

    pub(crate) fn options_snapshots() -> Self {
        Self::OptionsSnapshots
    }

    pub(crate) fn options_chain(underlying_symbol: impl Into<String>) -> Self {
        Self::OptionsChain {
            underlying_symbol: underlying_symbol.into(),
        }
    }

    pub(crate) fn options_exchange_codes() -> Self {
        Self::OptionsExchangeCodes
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
            Self::CryptoBars { loc } => Cow::Owned(format!("/v1beta3/crypto/{loc}/bars")),
            Self::CryptoQuotes { loc } => Cow::Owned(format!("/v1beta3/crypto/{loc}/quotes")),
            Self::CryptoTrades { loc } => Cow::Owned(format!("/v1beta3/crypto/{loc}/trades")),
            Self::CryptoLatestBars { loc } => {
                Cow::Owned(format!("/v1beta3/crypto/{loc}/latest/bars"))
            }
            Self::CryptoLatestQuotes { loc: Loc::Us } => {
                Cow::Borrowed("/v1beta3/crypto/us/latest/quotes")
            }
            Self::CryptoLatestQuotes { loc: Loc::Us1 } => {
                Cow::Borrowed("/v1beta3/crypto/us-1/latest/quotes")
            }
            Self::CryptoLatestQuotes { loc: Loc::Eu1 } => {
                Cow::Borrowed("/v1beta3/crypto/eu-1/latest/quotes")
            }
            Self::CryptoLatestTrades { loc } => {
                Cow::Owned(format!("/v1beta3/crypto/{loc}/latest/trades"))
            }
            Self::CryptoLatestOrderbooks { loc } => {
                Cow::Owned(format!("/v1beta3/crypto/{loc}/latest/orderbooks"))
            }
            Self::CryptoSnapshots { loc } => Cow::Owned(format!("/v1beta3/crypto/{loc}/snapshots")),
            Self::NewsList => Cow::Borrowed("/v1beta1/news"),
            Self::OptionsBars => Cow::Borrowed("/v1beta1/options/bars"),
            Self::OptionsTrades => Cow::Borrowed("/v1beta1/options/trades"),
            Self::OptionsLatestQuotes => Cow::Borrowed("/v1beta1/options/quotes/latest"),
            Self::OptionsLatestTrades => Cow::Borrowed("/v1beta1/options/trades/latest"),
            Self::OptionsSnapshots => Cow::Borrowed("/v1beta1/options/snapshots"),
            Self::OptionsChain { underlying_symbol } => {
                Cow::Owned(format!("/v1beta1/options/snapshots/{underlying_symbol}"))
            }
            Self::OptionsExchangeCodes => Cow::Borrowed("/v1beta1/options/meta/exchanges"),
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
            Self::CorporateActionsList => Cow::Borrowed("/v1/corporate-actions"),
        }
    }

    pub(crate) fn requires_auth(&self) -> bool {
        match self {
            Self::CryptoBars { .. }
            | Self::CryptoQuotes { .. }
            | Self::CryptoTrades { .. }
            | Self::CryptoLatestBars { .. }
            | Self::CryptoLatestQuotes { .. } => false,
            Self::CryptoLatestTrades { .. }
            | Self::CryptoLatestOrderbooks { .. }
            | Self::CryptoSnapshots { .. } => false,
            Self::OptionsBars
            | Self::NewsList
            | Self::OptionsTrades
            | Self::OptionsLatestQuotes
            | Self::OptionsLatestTrades
            | Self::OptionsSnapshots
            | Self::OptionsChain { .. }
            | Self::OptionsExchangeCodes
            | Self::StocksBars
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
            | Self::StocksExchangeCodes
            | Self::CorporateActionsList => true,
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
    fn endpoint_routes_crypto_historical_paths_with_official_loc_words() {
        assert_eq!(
            Endpoint::crypto_bars(Loc::Us).path(),
            "/v1beta3/crypto/us/bars"
        );
        assert_eq!(
            Endpoint::crypto_quotes(Loc::Us1).path(),
            "/v1beta3/crypto/us-1/quotes"
        );
        assert_eq!(
            Endpoint::crypto_trades(Loc::Eu1).path(),
            "/v1beta3/crypto/eu-1/trades"
        );
        assert!(!Endpoint::crypto_bars(Loc::Eu1).requires_auth());
    }

    #[test]
    fn endpoint_routes_news_and_corporate_actions_to_official_paths() {
        assert_eq!(Endpoint::NewsList.path(), "/v1beta1/news");
        assert!(Endpoint::NewsList.requires_auth());
        assert_eq!(
            Endpoint::CorporateActionsList.path(),
            "/v1/corporate-actions"
        );
        assert!(Endpoint::CorporateActionsList.requires_auth());
    }

    #[test]
    fn endpoint_routes_crypto_latest_paths_with_official_loc_words() {
        assert_eq!(
            Endpoint::crypto_latest_bars(Loc::Us).path(),
            "/v1beta3/crypto/us/latest/bars"
        );
        assert_eq!(
            Endpoint::crypto_latest_quotes(Loc::Us1).path(),
            "/v1beta3/crypto/us-1/latest/quotes"
        );
        assert_eq!(
            Endpoint::crypto_latest_trades(Loc::Eu1).path(),
            "/v1beta3/crypto/eu-1/latest/trades"
        );
        assert_eq!(
            Endpoint::crypto_latest_orderbooks(Loc::Us).path(),
            "/v1beta3/crypto/us/latest/orderbooks"
        );
        assert!(!Endpoint::crypto_latest_orderbooks(Loc::Eu1).requires_auth());
    }

    #[test]
    fn endpoint_routes_crypto_snapshots_with_official_loc_words() {
        assert_eq!(
            Endpoint::crypto_snapshots(Loc::Us).path(),
            "/v1beta3/crypto/us/snapshots"
        );
        assert_eq!(
            Endpoint::crypto_snapshots(Loc::Us1).path(),
            "/v1beta3/crypto/us-1/snapshots"
        );
        assert_eq!(
            Endpoint::crypto_snapshots(Loc::Eu1).path(),
            "/v1beta3/crypto/eu-1/snapshots"
        );
        assert!(!Endpoint::crypto_snapshots(Loc::Eu1).requires_auth());
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
    fn endpoint_routes_options_historical_paths() {
        assert_eq!(Endpoint::options_bars().path(), "/v1beta1/options/bars");
        assert_eq!(Endpoint::options_trades().path(), "/v1beta1/options/trades");
        assert_eq!(
            Endpoint::options_latest_quotes().path(),
            "/v1beta1/options/quotes/latest"
        );
        assert_eq!(
            Endpoint::options_latest_trades().path(),
            "/v1beta1/options/trades/latest"
        );
        assert_eq!(
            Endpoint::options_snapshots().path(),
            "/v1beta1/options/snapshots"
        );
        assert_eq!(
            Endpoint::options_chain("AAPL").path(),
            "/v1beta1/options/snapshots/AAPL"
        );
        assert_eq!(
            Endpoint::options_exchange_codes().path(),
            "/v1beta1/options/meta/exchanges"
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

    #[test]
    fn endpoint_requires_auth_for_options_historical_routes() {
        let cases = [
            Endpoint::OptionsBars,
            Endpoint::OptionsTrades,
            Endpoint::OptionsLatestQuotes,
            Endpoint::OptionsLatestTrades,
            Endpoint::OptionsSnapshots,
            Endpoint::OptionsExchangeCodes,
        ];

        for endpoint in cases {
            assert!(endpoint.requires_auth());
            assert!(matches!(endpoint.path(), Cow::Borrowed(_)));
        }

        let endpoint = Endpoint::options_chain("AAPL");
        assert!(endpoint.requires_auth());
        assert!(matches!(endpoint.path(), Cow::Owned(_)));
    }
}
