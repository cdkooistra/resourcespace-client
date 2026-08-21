use crate::client::{Client, HttpMethod};
use crate::error::Error;

pub mod request;
pub mod response;

use request::{DoSearch, FetchRows, SearchGetPreviews};
use response::SearchResults;

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
    /// * `request` - Parameters built via [`DoSearch`]
    ///
    /// ## Returns
    ///
    /// [`SearchResults::Paged`] when [`DoSearch::fetchrows`] is
    /// [`FetchRows::page`], otherwise [`SearchResults::Flat`].
    ///
    /// ## Errors
    ///
    /// Does not error on "no results" or missing permissions — the user
    /// lacking the `s` permission and a search matching nothing both return
    /// an empty result rather than [`Error::OperationFailed`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::search::request::{DoSearch, FetchRows}};
    /// # use resourcespace_client::api::SortOrder;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let results = client.search()
    ///     .do_search(DoSearch::new("cat").sort(SortOrder::Desc))
    ///     .await?;
    ///
    /// let specific_results = client.search()
    ///     .do_search(
    ///         DoSearch::new("cat")
    ///             .fetchrows(FetchRows::limit(100))
    ///             .offset(50)
    ///             .archive(0)
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn do_search(&self, request: DoSearch) -> Result<SearchResults, Error> {
        self.client
            .send_request("do_search", HttpMethod::Get, request)
            .await
    }

    /// Performs a search and returns matching resources including URLs for requested preview sizes.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SearchGetPreviews`]
    ///
    /// ## Returns
    ///
    /// [`SearchResults::Paged`] when [`SearchGetPreviews::fetchrows`]
    /// is [`FetchRows::page`], otherwise [`SearchResults::Flat`]. Each row
    /// has a `url_<size>` key per requested size in
    /// [`SearchGetPreviews::getsizes`].
    ///
    /// ## Errors
    ///
    /// Does not error on "no results" or missing permissions — the user
    /// lacking the `s` permission and a search matching nothing both return
    /// an empty result rather than [`Error::OperationFailed`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::search::request::{SearchGetPreviews, FetchRows}};
    /// # use resourcespace_client::api::SortOrder;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let results = client.search()
    ///     .search_get_previews(SearchGetPreviews::new("cat").getsizes("thm,scr"))
    ///     .await?;
    ///
    /// let specific_results = client.search()
    ///     .search_get_previews(
    ///         SearchGetPreviews::new("cat")
    ///             .getsizes("thm,scr,pre")
    ///             .previewext("jpg")
    ///             .sort(SortOrder::Desc)
    ///             .fetchrows(FetchRows::page(0, 50))
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_get_previews(
        &self,
        request: SearchGetPreviews,
    ) -> Result<SearchResults, Error> {
        self.client
            .send_request("search_get_previews", HttpMethod::Get, request)
            .await
    }
}
