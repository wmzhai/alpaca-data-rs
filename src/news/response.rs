use crate::{Error, transport::pagination::PaginatedResponse};

use super::NewsItem;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct ListResponse {
    pub news: Vec<NewsItem>,
    pub next_page_token: Option<String>,
}

impl PaginatedResponse for ListResponse {
    fn next_page_token(&self) -> Option<&str> {
        self.next_page_token.as_deref()
    }

    fn merge_page(&mut self, mut next: Self) -> Result<(), Error> {
        self.news.append(&mut next.news);
        self.next_page_token = next.next_page_token;
        Ok(())
    }

    fn clear_next_page_token(&mut self) {
        self.next_page_token = None;
    }
}

#[cfg(test)]
mod tests {
    use super::ListResponse;
    use crate::transport::pagination::PaginatedResponse;

    #[test]
    fn list_response_deserializes_official_wrapper_shape() {
        let response: ListResponse = serde_json::from_str(
            r#"{"news":[{"id":24843171,"headline":"Apple headline","author":"Charles Gross","created_at":"2021-12-31T11:08:42Z","updated_at":"2021-12-31T11:08:43Z","summary":"Summary","content":"","url":"https://example.com/article","images":[{"size":"thumb","url":"https://example.com/image.jpg"}],"symbols":["AAPL"],"source":"benzinga"}],"next_page_token":"page-2"}"#,
        )
        .expect("response should deserialize");

        assert_eq!(response.news.len(), 1);
        assert_eq!(response.next_page_token.as_deref(), Some("page-2"));
    }

    #[test]
    fn list_response_merge_appends_news_and_clears_next_page_token() {
        let mut first: ListResponse = serde_json::from_str(
            r#"{"news":[{"id":1,"headline":"First","author":"Author","created_at":"2026-04-01T00:00:00Z","updated_at":"2026-04-01T00:00:01Z","summary":"Summary","content":"","url":"https://example.com/1","images":[],"symbols":["AAPL"],"source":"benzinga"}],"next_page_token":"page-2"}"#,
        )
        .expect("first response should deserialize");
        let second: ListResponse = serde_json::from_str(
            r#"{"news":[{"id":2,"headline":"Second","author":"Author","created_at":"2026-04-02T00:00:00Z","updated_at":"2026-04-02T00:00:01Z","summary":"Summary","content":"","url":"https://example.com/2","images":[],"symbols":["MSFT"],"source":"benzinga"}],"next_page_token":null}"#,
        )
        .expect("second response should deserialize");

        first
            .merge_page(second)
            .expect("merge should append later news page");
        first.clear_next_page_token();

        assert_eq!(first.news.len(), 2);
        assert_eq!(first.next_page_token, None);
    }
}
