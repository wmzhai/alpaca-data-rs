use super::NewsItem;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListResponse {
    pub news: Vec<NewsItem>,
    pub next_page_token: Option<String>,
}
