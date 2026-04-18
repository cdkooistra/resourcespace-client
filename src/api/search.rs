use serde::{Serialize, Serializer};

use crate::client::Client;
use crate::error::RsError;

use super::{SortOrder, List};

/// Sub-API for search endpoints.
#[derive(Debug)]
pub struct SearchApi<'a> {
    client: &'a Client,
}

impl<'a> SearchApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Performs a search and returns matching resources.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DoSearchRequest`]
    ///
    /// ## TODO: Errors
    /// Returns [`RsError::OperationFailed`] if the search returns no results
    /// or the user lacks search permissions.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::search::{DoSearchRequest}};
    /// # use resourcespace_client::api::SortOrder;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let results = client.search()
    ///     .do_search(DoSearchRequest::new("cat").sort(SortOrder::Desc))
    ///     .await?;
    ///
    /// let specific_results = client.search()
    ///     .do_search(
    ///         DoSearchRequest::new("cat")
    ///             .fetchrows("100")
    ///             .offset(50)
    ///             .archive(0)
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn do_search(&self, request: DoSearchRequest) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("do_search", reqwest::Method::GET, request)
            .await
    }

    /// Performs a search and returns matching resources including URLs for requested preview sizes.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SearchGetPreviewsRequest`]
    ///
    /// ## TODO: Errors
    /// Returns [`RsError::OperationFailed`] if the search returns no results
    /// or the user lacks search permissions.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::search::SearchGetPreviewsRequest};
    /// # use resourcespace_client::api::SortOrder;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let results = client.search()
    ///     .search_get_previews(SearchGetPreviewsRequest::new("cat").getsizes("thm,scr"))
    ///     .await?;
    ///
    /// let specific_results = client.search()
    ///     .search_get_previews(
    ///         SearchGetPreviewsRequest::new("cat")
    ///             .getsizes("thm,scr,pre")
    ///             .previewext("jpg")
    ///             .sort(SortOrder::Desc)
    ///             .fetchrows("0,50")
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_get_previews(
        &self,
        request: SearchGetPreviewsRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("search_get_previews", reqwest::Method::GET, request)
            .await
    }
}

/// The row fetch mode for a search request.
///
/// Use [`FetchRows::limit`] to cap the number of results, or
/// [`FetchRows::page`] to fetch a specific window with offset and limit.
/// Note that these two modes return different response shapes from
/// ResourceSpace — `page` returns a structured response with a `total`
/// count alongside the results.
///
/// ```no_run
/// FetchRows::limit(100)         // return up to 100 results
/// FetchRows::page(0, 50)        // return results 0–50
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum FetchRows {
    /// Return up to N rows
    Limit(u32),
    /// Return rows with explicit offset and limit, enables paginated response
    Page {offset: u32, limit: u32 },
}

impl FetchRows {
    pub fn limit(n: u32) -> Self {
        Self::Limit(n)
    }

    pub fn page(offset: u32, limit: u32) -> Self {
        Self::Page { offset, limit }
    }
}

impl Serialize for FetchRows {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Limit(n) => n.serialize(serializer),
            Self::Page { offset, limit } => {
                format!("{},{}", offset, limit).serialize(serializer)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DoSearchRequest {
    /// The search string to match resources against.
    pub search: String,
    /// Comma-separated list of resource type IDs to restrict results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restypes: Option<List<u32>>,
    /// Field name to order results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Archive status filter: 0 = live, 1 = archived, 2 = deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<i8>,
    /// Number of rows to return, or `"offset,rows"` for paginated fetching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetchrows: Option<FetchRows>,
    /// Sort direction for the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortOrder>,
    /// Number of results to skip, used for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl DoSearchRequest {
    pub fn new(search: impl Into<String>) -> Self {
        Self {
            search: search.into(),
            restypes: None,
            order_by: None,
            archive: None,
            fetchrows: None,
            sort: None,
            offset: None,
        }
    }

    pub fn restypes(mut self, restypes: impl Into<List<u32>>) -> Self {
        self.restypes = Some(restypes.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn archive(mut self, archive: i8) -> Self {
        self.archive = Some(archive);
        self
    }

    pub fn fetchrows(mut self, fetchrows: FetchRows) -> Self {
        self.fetchrows = Some(fetchrows);
        self
    }

    pub fn sort(mut self, sort: SortOrder) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchGetPreviewsRequest {
    /// The search string to match resources against.
    pub search: String,
    /// Comma-separated list of resource type IDs to restrict results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restypes: Option<List<u32>>,
    /// Field name to order results by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Archive status filter: 0 = live, 1 = archived, 2 = deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<i8>,
    /// Number of rows to return, or `"offset,rows"` for paginated fetching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetchrows: Option<FetchRows>,
    /// Sort direction for the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortOrder>,
    /// Only return resources modified within this many days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_search_daylimit: Option<String>,
    /// Comma-separated list of preview sizes to include URLs for (e.g. `"thm,scr,pre"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getsizes: Option<List<u32>>,
    /// Override the preview file extension returned (e.g. `"jpg"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previewext: Option<String>,
}

impl SearchGetPreviewsRequest {
    pub fn new(search: impl Into<String>) -> Self {
        Self {
            search: search.into(),
            restypes: None,
            order_by: None,
            archive: None,
            fetchrows: None,
            sort: None,
            recent_search_daylimit: None,
            getsizes: None,
            previewext: None,
        }
    }

    pub fn restypes(mut self, restypes: impl Into<List<u32>>) -> Self {
        self.restypes = Some(restypes.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn archive(mut self, archive: i8) -> Self {
        self.archive = Some(archive);
        self
    }

    pub fn fetchrows(mut self, fetchrows: FetchRows) -> Self {
        self.fetchrows = Some(fetchrows);
        self
    }

    pub fn sort(mut self, sort: SortOrder) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn recent_search_daylimit(mut self, recent_search_daylimit: impl Into<String>) -> Self {
        self.recent_search_daylimit = Some(recent_search_daylimit.into());
        self
    }

    pub fn getsizes(mut self, getsizes: impl Into<List<u32>>) -> Self {
        self.getsizes = Some(getsizes.into());
        self
    }

    pub fn previewext(mut self, previewext: impl Into<String>) -> Self {
        self.previewext = Some(previewext.into());
        self
    }
}
