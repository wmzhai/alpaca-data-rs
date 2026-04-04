//! News endpoints.
//!
//! Mirror method:
//!
//! - `list`
//!
//! Convenience methods:
//!
//! - `list_all`
//! - `list_stream`

mod client;
mod model;
mod request;
mod response;

pub use crate::common::enums::Sort;

pub use client::NewsClient;
pub use model::{NewsImage, NewsItem};
pub use request::ListRequest;
pub use response::ListResponse;
