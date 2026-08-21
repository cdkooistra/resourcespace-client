use serde::Deserialize;

// Referenced only from doc links below; the import keeps them resolvable.
#[allow(unused_imports)]
use super::{FetchRows, SearchApi};

/// The result of [`SearchApi::do_search`] or [`SearchApi::search_get_previews`].
///
/// ResourceSpace returns one of two shapes depending on which
/// [`FetchRows`] mode the request used: [`FetchRows::page`] gets a
/// structured [`Self::Paged`] response with a total count, anything else
/// gets a bare array of results.
///
/// Individual rows are left as [`serde_json::Value`] rather than a resource
/// struct — full resource typing is a separate, larger pass and would only
/// need doing twice.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum SearchResults {
    Paged {
        total: u32,
        data: Vec<serde_json::Value>,
    },
    Flat(Vec<serde_json::Value>),
}
