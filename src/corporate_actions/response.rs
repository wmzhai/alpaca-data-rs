use crate::{Error, transport::pagination::PaginatedResponse};

use super::CorporateActions;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct ListResponse {
    pub corporate_actions: CorporateActions,
    pub next_page_token: Option<String>,
}

impl PaginatedResponse for ListResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, next: Self) -> Result<(), Error> {
        self.corporate_actions.merge(next.corporate_actions);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::ListResponse;
    use crate::{Decimal, transport::pagination::PaginatedResponse};

    #[test]
    fn list_response_deserializes_official_bucketed_wrapper_shape() {
        let response: ListResponse = serde_json::from_str(
            r#"{"corporate_actions":{"cash_dividends":[{"id":"e2b597ca-c2cb-47af-9315-cafb8708766d","symbol":"640CVR031","cusip":"640CVR031","rate":0.055284,"special":false,"foreign":false,"process_date":"2024-08-14","ex_date":"2024-08-07","record_date":"2024-08-07","payable_date":"2024-08-15"}],"name_changes":[{"id":"564620f3-dac4-4558-a227-5c8dd6f4d82e","old_symbol":"007975113","old_cusip":"007975113","new_symbol":"22112H119","new_cusip":"22112H119","process_date":"2024-08-13"}],"contract_adjustments":[{"id":"ca-undocumented","memo":"undocumented family"}],"mystery_bucket":[{"id":"mystery-1","field":"value"}]},"next_page_token":"page-2"}"#,
        )
        .expect("response should deserialize");

        assert_eq!(response.corporate_actions.cash_dividends.len(), 1);
        assert_eq!(
            response.corporate_actions.cash_dividends[0].rate,
            Decimal::from_str("0.055284").expect("decimal literal should parse")
        );
        assert_eq!(
            response.corporate_actions.name_changes[0].new_symbol,
            "22112H119"
        );
        assert_eq!(response.corporate_actions.name_changes.len(), 1);
        assert_eq!(response.corporate_actions.contract_adjustments.len(), 1);
        assert_eq!(
            response
                .corporate_actions
                .other
                .get("mystery_bucket")
                .map(Vec::len),
            Some(1)
        );
        assert_eq!(response.next_page_token.as_deref(), Some("page-2"));
    }

    #[test]
    fn list_response_merge_appends_bucketed_pages_and_clears_next_page_token() {
        let mut first: ListResponse = serde_json::from_str(
            r#"{"corporate_actions":{"cash_dividends":[{"id":"ca-1","symbol":"AAA","cusip":"111111111","rate":0.1,"special":false,"foreign":false,"process_date":"2024-08-01","ex_date":"2024-08-01"}],"contract_adjustments":[{"id":"undoc-1"}],"mystery_bucket":[{"id":"mystery-1"}]},"next_page_token":"page-2"}"#,
        )
        .expect("first response should deserialize");
        let second: ListResponse = serde_json::from_str(
            r#"{"corporate_actions":{"cash_dividends":[{"id":"ca-2","symbol":"BBB","cusip":"222222222","rate":0.2,"special":false,"foreign":false,"process_date":"2024-08-02","ex_date":"2024-08-02"}],"name_changes":[{"id":"name-1","old_symbol":"OLD","old_cusip":"333333333","new_symbol":"NEW","new_cusip":"444444444","process_date":"2024-08-02"}],"contract_adjustments":[{"id":"undoc-2"}],"mystery_bucket":[{"id":"mystery-2"}]},"next_page_token":null}"#,
        )
        .expect("second response should deserialize");

        first
            .merge_page(second)
            .expect("merge should append bucketed corporate action pages");
        first.clear_next_page_token();

        assert_eq!(first.corporate_actions.cash_dividends.len(), 2);
        assert_eq!(
            first.corporate_actions.cash_dividends[1].rate,
            Decimal::from_str("0.2").expect("decimal literal should parse")
        );
        assert_eq!(first.corporate_actions.name_changes.len(), 1);
        assert_eq!(first.corporate_actions.contract_adjustments.len(), 2);
        assert_eq!(
            first
                .corporate_actions
                .other
                .get("mystery_bucket")
                .map(Vec::len),
            Some(2)
        );
        assert_eq!(first.next_page_token, None);
    }

    #[test]
    fn list_response_deserializes_non_dividend_decimal_rate_fields() {
        let response: ListResponse = serde_json::from_str(
            r#"{"corporate_actions":{"forward_splits":[{"id":"fs-1","symbol":"ABC","cusip":"000000001","new_rate":3.0,"old_rate":2.0,"process_date":"2024-08-14","ex_date":"2024-08-07","record_date":"2024-08-07","payable_date":"2024-08-15"}],"stock_and_cash_mergers":[{"id":"scm-1","acquirer_symbol":"AAA","acquirer_cusip":"111111111","acquirer_rate":0.75,"acquiree_symbol":"BBB","acquiree_cusip":"222222222","acquiree_rate":1.0,"cash_rate":4.25,"process_date":"2024-08-14","effective_date":"2024-08-15","payable_date":"2024-08-16"}]},"next_page_token":null}"#,
        )
        .expect("response should deserialize");

        assert_eq!(
            response.corporate_actions.forward_splits[0].new_rate,
            Decimal::from_str("3.0").expect("decimal literal should parse")
        );
        assert_eq!(
            response.corporate_actions.stock_and_cash_mergers[0].cash_rate,
            Decimal::from_str("4.25").expect("decimal literal should parse")
        );
    }
}
