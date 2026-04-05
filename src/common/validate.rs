use crate::Error;

pub(crate) fn validate_required_symbol(symbol: &str, field_name: &str) -> Result<(), Error> {
    if symbol.trim().is_empty() {
        return Err(Error::InvalidRequest(format!(
            "{field_name} is invalid: must not be empty or whitespace-only"
        )));
    }

    Ok(())
}

pub(crate) fn validate_required_symbols(symbols: &[String]) -> Result<(), Error> {
    if symbols.is_empty() {
        return Err(Error::InvalidRequest(
            "symbols are invalid: must not be empty".into(),
        ));
    }

    if symbols.iter().any(|symbol| symbol.trim().is_empty()) {
        return Err(Error::InvalidRequest(
            "symbols are invalid: must not contain empty or whitespace-only entries".into(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::{validate_required_symbol, validate_required_symbols};

    #[test]
    fn validate_required_symbol_accepts_non_blank_values() {
        assert_eq!(validate_required_symbol("AAPL", "symbol"), Ok(()));
    }

    #[test]
    fn validate_required_symbol_rejects_empty_symbol_with_stable_template() {
        assert_eq!(
            validate_required_symbol("", "symbol"),
            Err(Error::InvalidRequest(
                "symbol is invalid: must not be empty or whitespace-only".into(),
            ))
        );
    }

    #[test]
    fn validate_required_symbol_rejects_blank_underlying_symbol_with_stable_template() {
        assert_eq!(
            validate_required_symbol("   ", "underlying_symbol"),
            Err(Error::InvalidRequest(
                "underlying_symbol is invalid: must not be empty or whitespace-only".into(),
            ))
        );
    }

    #[test]
    fn validate_required_symbols_rejects_empty_lists_with_stable_template() {
        assert_eq!(
            validate_required_symbols(&[]),
            Err(Error::InvalidRequest(
                "symbols are invalid: must not be empty".into(),
            ))
        );
    }

    #[test]
    fn validate_required_symbols_rejects_blank_entries_with_stable_template() {
        assert_eq!(
            validate_required_symbols(&["AAPL".into(), " ".into()]),
            Err(Error::InvalidRequest(
                "symbols are invalid: must not contain empty or whitespace-only entries".into(),
            ))
        );
    }

    #[test]
    fn validate_required_symbols_accepts_non_blank_lists() {
        assert_eq!(
            validate_required_symbols(&["AAPL".into(), "MSFT".into()]),
            Ok(())
        );
    }
}
