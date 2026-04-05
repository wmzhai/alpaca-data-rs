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
