use crate::crypto::Loc;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Endpoint {
    CryptoLatestQuotes { loc: Loc },
}

#[allow(dead_code)]
impl Endpoint {
    pub(crate) fn crypto_latest_quotes(loc: Loc) -> Self {
        Self::CryptoLatestQuotes { loc }
    }

    pub(crate) fn path(&self) -> &'static str {
        match self {
            Self::CryptoLatestQuotes { loc: Loc::Us } => "/v1beta3/crypto/us/latest/quotes",
            Self::CryptoLatestQuotes { loc: Loc::Us1 } => "/v1beta3/crypto/us1/latest/quotes",
        }
    }

    pub(crate) fn requires_auth(&self) -> bool {
        match self {
            Self::CryptoLatestQuotes { .. } => false,
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
}
