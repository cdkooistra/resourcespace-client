use serde::Deserialize;
use serde_with::{BoolFromInt, serde_as};

use crate::api::shared::empty_as_none;

// Referenced only from doc links below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::CollectionApi;

/// A collection row, as returned by [`CollectionApi::get_collection`],
/// [`CollectionApi::get_user_collections`] and
/// [`CollectionApi::search_public_collections`].
///
/// Those three endpoints return the same row with small differences, so the
/// fields only some of them include are `Option` and default to `None`:
/// `count` on `get_user_collections`, `groups`/`users`/`request_feedback` on
/// `get_collection`, and `is_featured_collection_category` on
/// `search_public_collections`.
///
/// Absent values arrive as `null` from the first two and as `""` from
/// `search_public_collections`; both deserialize to `None`.
#[serde_as]
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Collection {
    /// The collection's own ID.
    #[serde(rename = "ref")]
    pub collection_id: u32,
    /// Collection name. Cleared to empty by a
    /// [`CollectionApi::save_collection`] that omits it.
    #[serde(deserialize_with = "empty_as_none")]
    pub name: Option<String>,
    /// ID of the user who owns the collection.
    pub user: u32,
    /// Username of the owner.
    pub username: Option<String>,
    /// Full name of the owner.
    pub fullname: Option<String>,
    /// When the collection was created, as `YYYY-MM-DD HH:MM:SS`.
    pub created: Option<String>,
    /// `0` = standard, `3` = featured, `4` = public.
    pub r#type: u8,
    /// Whether the collection is public.
    #[serde_as(as = "BoolFromInt")]
    pub public: bool,
    /// Whether other users may add or remove resources.
    #[serde_as(as = "BoolFromInt")]
    pub allow_changes: bool,
    /// Whether the collection is protected from deletion.
    #[serde_as(as = "BoolFromInt")]
    pub cant_delete: bool,
    /// Number of resources in the collection. Only returned by
    /// [`CollectionApi::get_user_collections`].
    #[serde(deserialize_with = "empty_as_none")]
    pub count: Option<u32>,
    /// Keywords associated with the collection.
    #[serde(deserialize_with = "empty_as_none")]
    pub keywords: Option<String>,
    /// Free-text description.
    #[serde(deserialize_with = "empty_as_none")]
    pub description: Option<String>,
    /// Parent featured collection, if any.
    #[serde(deserialize_with = "empty_as_none")]
    pub parent: Option<u32>,
    /// Saved search backing this collection, if any.
    #[serde(deserialize_with = "empty_as_none")]
    pub savedsearch: Option<u32>,
    /// Resource used as the background image.
    #[serde(deserialize_with = "empty_as_none")]
    pub bg_img_resource_ref: Option<u32>,
    /// How the collection thumbnail is chosen.
    #[serde(deserialize_with = "empty_as_none")]
    pub thumbnail_selection_method: Option<u32>,
    /// Sort order.
    pub order_by: u32,
    /// Session that owns the collection, for anonymous users.
    #[serde(deserialize_with = "empty_as_none")]
    pub session_id: Option<u32>,
    /// Home page image, if the collection is featured.
    #[serde(deserialize_with = "empty_as_none")]
    pub home_page_image: Option<u32>,
    /// Whether the collection is published to the home page.
    #[serde(deserialize_with = "empty_as_none")]
    pub home_page_publish: Option<u32>,
    /// Home page caption.
    #[serde(deserialize_with = "empty_as_none")]
    pub home_page_text: Option<String>,
    /// Groups the collection is shared with. Only returned by
    /// [`CollectionApi::get_collection`].
    #[serde(deserialize_with = "empty_as_none")]
    pub groups: Option<String>,
    /// Users the collection is shared with. Only returned by
    /// [`CollectionApi::get_collection`].
    #[serde(deserialize_with = "empty_as_none")]
    pub users: Option<String>,
    /// Whether feedback has been requested. Only returned by
    /// [`CollectionApi::get_collection`].
    #[serde(deserialize_with = "empty_as_none")]
    pub request_feedback: Option<u32>,
    /// Whether this is a featured collection *category* — a featured
    /// collection with children rather than resources. Only returned by
    /// [`CollectionApi::search_public_collections`].
    #[serde(deserialize_with = "empty_as_none")]
    pub is_featured_collection_category: Option<u32>,
}

/// A featured collection, as returned by
/// [`CollectionApi::get_featured_collections`].
#[serde_as]
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct FeaturedCollection {
    /// The collection's own ID.
    #[serde(rename = "ref")]
    pub collection_id: u32,
    /// Collection name.
    #[serde(deserialize_with = "empty_as_none")]
    pub name: Option<String>,
    /// When the collection was created, as `YYYY-MM-DD HH:MM:SS`.
    pub created: Option<String>,
    /// `3` for a featured collection.
    pub r#type: u8,
    /// Parent featured collection, or `None` at the root.
    #[serde(deserialize_with = "empty_as_none")]
    pub parent: Option<u32>,
    /// Whether this collection has child featured collections.
    #[serde_as(as = "BoolFromInt")]
    pub has_children: bool,
    /// Whether this collection directly contains resources.
    #[serde_as(as = "BoolFromInt")]
    pub has_resources: bool,
    /// Sort order.
    pub order_by: u32,
    /// Saved search backing this collection, if any.
    #[serde(deserialize_with = "empty_as_none")]
    pub savedsearch: Option<u32>,
    /// Resource used as the background image.
    #[serde(deserialize_with = "empty_as_none")]
    pub bg_img_resource_ref: Option<u32>,
    /// How the collection thumbnail is chosen.
    #[serde(deserialize_with = "empty_as_none")]
    pub thumbnail_selection_method: Option<u32>,
}
