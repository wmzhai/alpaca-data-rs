#[allow(dead_code)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct QueryWriter {
    pairs: Vec<(String, String)>,
}

#[allow(dead_code)]
impl QueryWriter {
    pub(crate) fn push_csv<I, S>(&mut self, key: &'static str, values: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = values
            .into_iter()
            .map(|value| value.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(",");

        if !joined.is_empty() {
            self.pairs.push((key.to_string(), joined));
        }
    }

    pub(crate) fn push_opt<T>(&mut self, key: &'static str, value: Option<T>)
    where
        T: ToString,
    {
        if let Some(value) = value {
            self.pairs.push((key.to_string(), value.to_string()));
        }
    }

    pub(crate) fn finish(self) -> Vec<(String, String)> {
        self.pairs
    }
}

#[cfg(test)]
mod tests {
    use super::QueryWriter;

    #[test]
    fn query_writer_joins_symbols_and_skips_none() {
        let mut query = QueryWriter::default();
        query.push_csv("symbols", ["AAPL", "MSFT"]);
        query.push_opt("limit", Some(100u32));
        query.push_opt::<u32>("page_token", None);

        assert_eq!(
            query.finish(),
            vec![
                ("symbols".to_string(), "AAPL,MSFT".to_string()),
                ("limit".to_string(), "100".to_string()),
            ]
        );
    }

    #[test]
    fn query_writer_keeps_decimal_scale() {
        use std::str::FromStr;

        let mut query = QueryWriter::default();
        query.push_opt(
            "strike_price_gte",
            Some(crate::Decimal::from_str("180.0").expect("decimal literal should parse")),
        );

        assert_eq!(
            query.finish(),
            vec![("strike_price_gte".to_string(), "180.0".to_string())]
        );
    }
}
