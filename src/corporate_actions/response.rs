use super::CorporateAction;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListResponse {
    pub corporate_actions: Vec<CorporateAction>,
    pub next_page_token: Option<String>,
}
