use serde::{Serialize, Serializer};
use serde_with::skip_serializing_none;

use crate::api::shared::{List, SortOrder};

// Referenced only from doc links below; the import keeps them resolvable.
#[allow(unused_imports)]
use super::SearchApi;

/// The row fetch mode for a search request.
///
/// Use [`FetchRows::limit`] to cap the number of results, or
/// [`FetchRows::page`] to fetch a specific window with offset and limit.
/// Note that these two modes return different response shapes from
/// ResourceSpace — `page` returns a structured response with a `total`
/// count alongside the results.
///
/// ```no_run
/// # use resourcespace_client::api::search::FetchRows;
/// let _ = FetchRows::limit(100);         // return up to 100 results
/// let _ = FetchRows::page(0, 50);        // return results 0–50
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum FetchRows {
    Limit(u32),
    Page { offset: u32, limit: u32 },
}

impl FetchRows {
    /// Return up to N rows
    pub fn limit(n: u32) -> Self {
        Self::Limit(n)
    }

    /// Return rows with explicit offset and limit, enables paginated response
    pub fn page(offset: u32, limit: u32) -> Self {
        Self::Page { offset, limit }
    }
}

impl Serialize for FetchRows {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Limit(n) => n.serialize(serializer),
            Self::Page { offset, limit } => format!("{},{}", offset, limit).serialize(serializer),
        }
    }
}

/// Parameters for [`SearchApi::do_search`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DoSearchRequest {
    /// The search string to match resources against.
    pub search: String,
    /// Comma-separated list of resource type IDs to restrict results to.
    pub restypes: Option<List<u32>>,
    /// Field name to order results by.
    pub order_by: Option<String>,
    /// Archive status filter: 0 = live, 1 = archived, 2 = deleted.
    pub archive: Option<i8>,
    /// Number of rows to return, or `"offset,rows"` for paginated fetching.
    pub fetchrows: Option<FetchRows>,
    /// Sort direction for the results.
    pub sort: Option<SortOrder>,
    /// Number of results to skip, used for pagination.
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

/// Parameters for [`SearchApi::search_get_previews`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchGetPreviewsRequest {
    /// The search string to match resources against.
    pub search: String,
    /// Comma-separated list of resource type IDs to restrict results to.
    pub restypes: Option<List<u32>>,
    /// Field name to order results by.
    pub order_by: Option<String>,
    /// Archive status filter: 0 = live, 1 = archived, 2 = deleted.
    pub archive: Option<i8>,
    /// Number of rows to return, or `"offset,rows"` for paginated fetching.
    pub fetchrows: Option<FetchRows>,
    /// Sort direction for the results.
    pub sort: Option<SortOrder>,
    /// Only return resources modified within this many days.
    pub recent_search_daylimit: Option<String>,
    /// Comma-separated list of preview sizes to include URLs for (e.g. `"thm,scr,pre"`).
    pub getsizes: Option<List<String>>,
    /// Override the preview file extension returned (e.g. `"jpg"`).
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

    pub fn getsizes(mut self, getsizes: impl Into<List<String>>) -> Self {
        self.getsizes = Some(getsizes.into());
        self
    }

    pub fn previewext(mut self, previewext: impl Into<String>) -> Self {
        self.previewext = Some(previewext.into());
        self
    }
}
