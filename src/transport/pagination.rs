#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub(crate) struct PaginationState {
    pub(crate) page_token: Option<String>,
}
