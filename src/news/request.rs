use crate::common::enums::Sort;
use crate::common::query::QueryWriter;
use crate::transport::pagination::PaginatedRequest;

#[derive(Clone, Debug, Default)]
pub struct ListRequest {
    pub start: Option<String>,
    pub end: Option<String>,
    pub sort: Option<Sort>,
    pub symbols: Option<Vec<String>>,
    pub limit: Option<u32>,
    pub include_content: Option<bool>,
    pub exclude_contentless: Option<bool>,
    pub page_token: Option<String>,
}

impl ListRequest {
    pub(crate) fn to_query(self) -> Vec<(String, String)> {
        let mut query = QueryWriter::default();
        query.push_opt("start", self.start);
        query.push_opt("end", self.end);
        query.push_opt("sort", self.sort);
        if let Some(symbols) = self.symbols {
            query.push_csv("symbols", symbols);
        }
        query.push_opt("limit", self.limit);
        query.push_opt("include_content", self.include_content);
        query.push_opt("exclude_contentless", self.exclude_contentless);
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
    use super::ListRequest;
    use crate::common::enums::Sort;

    #[test]
    fn list_request_serializes_official_query_words() {
        let query = ListRequest {
            start: Some("2026-04-01T00:00:00Z".into()),
            end: Some("2026-04-04T00:00:00Z".into()),
            sort: Some(Sort::Desc),
            symbols: Some(vec!["AAPL".into(), "BTCUSD".into()]),
            limit: Some(2),
            include_content: Some(false),
            exclude_contentless: Some(true),
            page_token: Some("page-2".into()),
        }
        .to_query();

        assert_eq!(
            query,
            vec![
                ("start".to_string(), "2026-04-01T00:00:00Z".to_string()),
                ("end".to_string(), "2026-04-04T00:00:00Z".to_string()),
                ("sort".to_string(), "desc".to_string()),
                ("symbols".to_string(), "AAPL,BTCUSD".to_string()),
                ("limit".to_string(), "2".to_string()),
                ("include_content".to_string(), "false".to_string()),
                ("exclude_contentless".to_string(), "true".to_string()),
                ("page_token".to_string(), "page-2".to_string()),
            ]
        );
    }
}
