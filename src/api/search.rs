use serde::Serialize;

use crate::client::Client;
use crate::error::RsError;

use super::SortOrder;

/// Sub-API for search endpoints.
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
    /// # use resourcespace_client::{RsClient, api::search::{DoSearchRequest}};
    /// # use resourcespace_client::api::SortOrder;
    /// # async fn example(client: RsClient) -> Result<(), Box<dyn std::error::Error>> {
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
    pub async fn do_search(
        &self,
        request: DoSearchRequest,
    ) -> Result<serde_json::Value, RsError> {
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
    /// # use resourcespace_client::{RsClient, api::search::{SearchGetPreviewsRequest, SearchSort}};
    /// # async fn example(client: RsClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let results = client.search()
    ///     .search_get_previews(SearchGetPreviewsRequest::new("cat").getsizes("thm,scr"))
    ///     .await?;
    ///
    /// let specific_results = client.search()
    ///     .search_get_previews(
    ///         SearchGetPreviewsRequest::new("cat")
    ///             .getsizes("thm,scr,pre")
    ///             .previewext("jpg")
    ///             .sort(SearchSort::Desc)
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

#[derive(Default, Serialize)]
pub struct DoSearchRequest {
    search: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    restypes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archive: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fetchrows: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

impl DoSearchRequest {
    pub fn new(search: impl Into<String>) -> Self {
        Self { 
            search: search.into(),
            ..Default::default()
        }
    }

    pub fn restypes(mut self, restypes: impl Into<String>) -> Self {
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

    pub fn fetchrows(mut self, fetchrows: impl Into<String>) -> Self {
        self.fetchrows = Some(fetchrows.into());
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
    
#[derive(Default, Serialize)]
pub struct SearchGetPreviewsRequest {
    search: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    restypes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archive: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fetchrows: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recent_search_daylimit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    getsizes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previewext: Option<String>,
}

impl SearchGetPreviewsRequest {
    pub fn new(search: impl Into<String>) -> Self {
        Self { 
            search: search.into(),
            ..Default::default()
        }
    }

    pub fn restypes(mut self, restypes: impl Into<String>) -> Self {
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

    pub fn fetchrows(mut self, fetchrows: impl Into<String>) -> Self {
        self.fetchrows = Some(fetchrows.into());
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

    pub fn getsizes(mut self, getsizes: impl Into<String>) -> Self {
        self.getsizes = Some(getsizes.into());
        self
    }

    pub fn previewext(mut self, previewext: impl Into<String>) -> Self {
        self.previewext = Some(previewext.into());
        self
    }
}
