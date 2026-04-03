use crate::common::enums::Sort;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CorporateActionType {
    #[default]
    Dividend,
    Merger,
    SpinOff,
    Split,
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
