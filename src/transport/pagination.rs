use crate::{Error, common::response::ResponseStream};

#[allow(dead_code)]
pub(crate) trait PaginatedRequest: Clone {
    fn with_page_token(&self, page_token: Option<String>) -> Self;
}

#[allow(dead_code)]
pub(crate) trait PaginatedResponse: Sized {
    fn next_page_token(&self) -> Option<&str>;
    fn merge_page(&mut self, next: Self) -> Result<(), Error>;
    fn clear_next_page_token(&mut self);
}

#[allow(dead_code)]
pub(crate) async fn collect_all<Request, Response, Fetch, Fut>(
    request: Request,
    mut fetch: Fetch,
) -> Result<Response, Error>
where
    Request: PaginatedRequest,
    Response: PaginatedResponse,
    Fetch: FnMut(Request) -> Fut,
    Fut: std::future::Future<Output = Result<Response, Error>>,
{
    let mut current_request = request;
    let mut combined = fetch(current_request.clone()).await?;

    while let Some(next_page_token) = combined.next_page_token().map(str::to_owned) {
        current_request = current_request.with_page_token(Some(next_page_token));
        let page = fetch(current_request.clone()).await?;
        combined.merge_page(page)?;
    }

    combined.clear_next_page_token();
    Ok(combined)
}

#[allow(dead_code)]
pub(crate) fn stream_pages<Request, Response, Fetch, Fut>(
    request: Request,
    fetch: Fetch,
) -> ResponseStream<Result<Response, Error>>
where
    Request: PaginatedRequest + Send + 'static,
    Response: PaginatedResponse + Clone + Send + 'static,
    Fetch: FnMut(Request) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<Response, Error>> + Send + 'static,
{
    struct StreamState<Request, Response, Fetch> {
        next_request: Option<Request>,
        previous_page: Option<Response>,
        fetch: Fetch,
    }

    let state: StreamState<Request, Response, Fetch> = StreamState {
        next_request: Some(request),
        previous_page: None,
        fetch,
    };

    Box::pin(futures_util::stream::unfold(
        state,
        |mut state| async move {
            let request = state.next_request.take()?;
            let response = (state.fetch)(request.clone()).await;

            match response {
                Ok(page) => {
                    if let Some(mut previous_page) = state.previous_page.take() {
                        if let Err(error) = previous_page.merge_page(page.clone()) {
                            state.next_request = None;
                            return Some((Err(error), state));
                        }
                    }

                    state.next_request = page
                        .next_page_token()
                        .map(str::to_owned)
                        .map(|page_token| request.with_page_token(Some(page_token)));
                    state.previous_page = Some(page.clone());
                    Some((Ok(page), state))
                }
                Err(error) => Some((Err(error), state)),
            }
        },
    ))
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;

    use super::{PaginatedRequest, PaginatedResponse, collect_all, stream_pages};
    use crate::Error;

    #[derive(Clone, Debug, Default, PartialEq)]
    struct FakeRequest {
        page_token: Option<String>,
    }

    impl PaginatedRequest for FakeRequest {
        fn with_page_token(&self, page_token: Option<String>) -> Self {
            Self { page_token }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct FakeResponse {
        values: Vec<u32>,
        next_page_token: Option<String>,
        group: Option<&'static str>,
    }

    impl PaginatedResponse for FakeResponse {
        fn next_page_token(&self) -> Option<&str> {
            self.next_page_token.as_deref()
        }

        fn merge_page(&mut self, next: Self) -> Result<(), Error> {
            if self.group.is_some() && self.group != next.group {
                return Err(Error::Pagination("group mismatch".into()));
            }
            self.values.extend(next.values);
            self.next_page_token = next.next_page_token;
            Ok(())
        }

        fn clear_next_page_token(&mut self) {
            self.next_page_token = None;
        }
    }

    #[tokio::test]
    async fn collect_all_merges_pages_and_clears_next_page_token() {
        let first = FakeResponse {
            values: vec![1, 2],
            next_page_token: Some("page-2".into()),
            group: Some("A"),
        };
        let second = FakeResponse {
            values: vec![3],
            next_page_token: None,
            group: Some("A"),
        };

        let response = collect_all(FakeRequest::default(), |request| {
            let first = first.clone();
            let second = second.clone();
            async move {
                if request.page_token.as_deref() == Some("page-2") {
                    Ok(second)
                } else {
                    Ok(first)
                }
            }
        })
        .await
        .expect("pagination should succeed");

        assert_eq!(response.values, vec![1, 2, 3]);
        assert_eq!(response.next_page_token, None);
    }

    #[tokio::test]
    async fn stream_pages_yields_each_page() {
        let first = FakeResponse {
            values: vec![1, 2],
            next_page_token: Some("page-2".into()),
            group: Some("A"),
        };
        let second = FakeResponse {
            values: vec![3],
            next_page_token: None,
            group: Some("A"),
        };

        let pages = stream_pages(FakeRequest::default(), move |request| {
            let first = first.clone();
            let second = second.clone();
            async move {
                if request.page_token.as_deref() == Some("page-2") {
                    Ok(second)
                } else {
                    Ok(first)
                }
            }
        })
        .collect::<Vec<_>>()
        .await;

        assert_eq!(pages.len(), 2);
        assert_eq!(
            pages[0].as_ref().expect("first page should succeed").values,
            vec![1, 2]
        );
        assert_eq!(
            pages[1]
                .as_ref()
                .expect("second page should succeed")
                .values,
            vec![3]
        );
    }

    #[tokio::test]
    async fn stream_pages_yields_error_when_page_merge_validation_fails() {
        let first = FakeResponse {
            values: vec![1, 2],
            next_page_token: Some("page-2".into()),
            group: Some("A"),
        };
        let second = FakeResponse {
            values: vec![3],
            next_page_token: None,
            group: Some("B"),
        };

        let pages = stream_pages(FakeRequest::default(), move |request| {
            let first = first.clone();
            let second = second.clone();
            async move {
                if request.page_token.as_deref() == Some("page-2") {
                    Ok(second)
                } else {
                    Ok(first)
                }
            }
        })
        .collect::<Vec<_>>()
        .await;

        assert_eq!(pages.len(), 2);
        assert_eq!(
            pages[0].as_ref().expect("first page should succeed").values,
            vec![1, 2]
        );
        assert!(matches!(pages[1], Err(Error::Pagination(_))));
    }
}
