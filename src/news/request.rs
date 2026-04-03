use crate::common::enums::Sort;

#[derive(Clone, Debug, Default)]
pub struct ListRequest {
    pub start: Option<String>,
    pub end: Option<String>,
    pub sort: Option<Sort>,
    pub symbols: Option<String>,
    pub limit: Option<u32>,
    pub include_content: Option<bool>,
    pub exclude_contentless: Option<bool>,
    pub page_token: Option<String>,
}
