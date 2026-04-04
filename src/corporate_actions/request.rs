use std::fmt::{self, Display, Formatter};

use crate::common::enums::Sort;
use crate::common::query::QueryWriter;
use crate::transport::pagination::PaginatedRequest;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CorporateActionType {
    #[default]
    ForwardSplit,
    ReverseSplit,
    UnitSplit,
    StockDividend,
    CashDividend,
    SpinOff,
    CashMerger,
    StockMerger,
    StockAndCashMerger,
    Redemption,
    NameChange,
    WorthlessRemoval,
    RightsDistribution,
    ContractAdjustment,
    PartialCall,
}

impl Display for CorporateActionType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ForwardSplit => "forward_split",
            Self::ReverseSplit => "reverse_split",
            Self::UnitSplit => "unit_split",
            Self::StockDividend => "stock_dividend",
            Self::CashDividend => "cash_dividend",
            Self::SpinOff => "spin_off",
            Self::CashMerger => "cash_merger",
            Self::StockMerger => "stock_merger",
            Self::StockAndCashMerger => "stock_and_cash_merger",
            Self::Redemption => "redemption",
            Self::NameChange => "name_change",
            Self::WorthlessRemoval => "worthless_removal",
            Self::RightsDistribution => "rights_distribution",
            Self::ContractAdjustment => "contract_adjustment",
            Self::PartialCall => "partial_call",
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListRequest {
    pub symbols: Option<Vec<String>>,
    pub cusips: Option<Vec<String>>,
    pub types: Option<Vec<CorporateActionType>>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub ids: Option<Vec<String>>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub page_token: Option<String>,
}

impl ListRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        if let Some(symbols) = self.symbols {
            query.push_csv("symbols", symbols);
        }
        if let Some(cusips) = self.cusips {
            query.push_csv("cusips", cusips);
        }
        if let Some(types) = self.types {
            query.push_csv("types", types.into_iter().map(|value| value.to_string()));
        }
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        if let Some(ids) = self.ids {
            query.push_csv("ids", ids);
        }
        query.push_opt("limit", self.limit);
        query.push_opt("sort", self.sort);
        query.push_opt("page_token", self.page_token);
        query.finish()
    }
}

impl PaginatedRequest for ListRequest {
    fn with_page_token(&self, page_token: Option<String>) -> Self {
        let mut next = self.clone();
        next.page_token = page_token;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::{CorporateActionType, ListRequest};
    use crate::common::enums::Sort;

    #[test]
    fn corporate_action_type_serializes_to_official_query_words() {
        assert_eq!(
            CorporateActionType::ForwardSplit.to_string(),
            "forward_split"
        );
        assert_eq!(
            CorporateActionType::ReverseSplit.to_string(),
            "reverse_split"
        );
        assert_eq!(CorporateActionType::UnitSplit.to_string(), "unit_split");
        assert_eq!(
            CorporateActionType::StockDividend.to_string(),
            "stock_dividend"
        );
        assert_eq!(
            CorporateActionType::CashDividend.to_string(),
            "cash_dividend"
        );
        assert_eq!(CorporateActionType::SpinOff.to_string(), "spin_off");
        assert_eq!(CorporateActionType::CashMerger.to_string(), "cash_merger");
        assert_eq!(CorporateActionType::StockMerger.to_string(), "stock_merger");
        assert_eq!(
            CorporateActionType::StockAndCashMerger.to_string(),
            "stock_and_cash_merger"
        );
        assert_eq!(CorporateActionType::Redemption.to_string(), "redemption");
        assert_eq!(CorporateActionType::NameChange.to_string(), "name_change");
        assert_eq!(
            CorporateActionType::WorthlessRemoval.to_string(),
            "worthless_removal"
        );
        assert_eq!(
            CorporateActionType::RightsDistribution.to_string(),
            "rights_distribution"
        );
        assert_eq!(
            CorporateActionType::ContractAdjustment.to_string(),
            "contract_adjustment"
        );
        assert_eq!(CorporateActionType::PartialCall.to_string(), "partial_call");
    }

    #[test]
    fn list_request_serializes_official_query_words() {
        let query = ListRequest {
            symbols: Some(vec!["AAPL".into(), "TSLA".into()]),
            cusips: Some(vec!["037833100".into(), "88160R101".into()]),
            types: Some(vec![
                CorporateActionType::CashDividend,
                CorporateActionType::NameChange,
            ]),
            start: Some("2024-08-01".into()),
            end: Some("2024-08-20".into()),
            ids: Some(vec!["ca-1".into(), "ca-2".into()]),
            limit: Some(2),
            sort: Some(Sort::Desc),
            page_token: Some("page-2".into()),
        }
        .to_query();

        assert_eq!(
            query,
            vec![
                ("symbols".to_string(), "AAPL,TSLA".to_string()),
                ("cusips".to_string(), "037833100,88160R101".to_string()),
                ("types".to_string(), "cash_dividend,name_change".to_string(),),
                ("start".to_string(), "2024-08-01".to_string()),
                ("end".to_string(), "2024-08-20".to_string()),
                ("ids".to_string(), "ca-1,ca-2".to_string()),
                ("limit".to_string(), "2".to_string()),
                ("sort".to_string(), "desc".to_string()),
                ("page_token".to_string(), "page-2".to_string()),
            ]
        );
    }
}
