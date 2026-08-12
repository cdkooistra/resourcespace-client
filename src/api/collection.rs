use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::BoolFromInt;
use serde_with::json::JsonString;
use serde_with::{serde_as, skip_serializing_none};
use validator::Validate;

use crate::client::{Client, HttpMethod};
use crate::error::Error;

use super::{List, SortOrder, bool_as_u8, empty_as_none, opt_bool_as_u8};

/// Sub-API for collection endpoints.
#[derive(Debug)]
pub struct CollectionApi<'a> {
    client: &'a Client,
}

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

impl<'a> CollectionApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of the user's collections.
    ///
    /// ## Arguments
    /// `None`
    ///
    /// ## Returns
    /// Every collection the user owns or has been given access to, including
    /// the built-in "Default Collection". This is the only endpoint that
    /// populates [`Collection::count`].
    ///
    /// A user holding the restrictive `b` permission gets an empty list rather
    /// than an error.
    ///
    /// ## Errors
    /// Returns [`Error::Transport`] if the request could not be sent, and
    /// [`Error::Deserialize`] if the response does not match [`Collection`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// for collection in client.collection().get_user_collections().await? {
    ///     println!("{} has {:?} resources", collection.collection_id, collection.count);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_collections(&self) -> Result<Vec<Collection>, Error> {
        self.client
            .send_request("get_user_collections", HttpMethod::Get, ())
            .await
    }

    /// Add a resource to a collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`AddResourceToCollectionRequest`]
    ///
    /// ## Returns
    /// Always `true`. ResourceSpace's `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection is not writable by
    /// the user or either ID does not exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::AddResourceToCollectionRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .add_resource_to_collection(AddResourceToCollectionRequest::new(1234, 7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_resource_to_collection(
        &self,
        request: AddResourceToCollectionRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("add_resource_to_collection", HttpMethod::Post, request)
            .await
    }

    /// Remove a resource from a collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`RemoveResourceFromCollectionRequest`]
    ///
    /// ## Returns
    /// Always `true`. ResourceSpace's `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection is not writable by
    /// the user or either ID does not exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::RemoveResourceFromCollectionRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .remove_resource_from_collection(RemoveResourceFromCollectionRequest::new(1234, 7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_resource_from_collection(
        &self,
        request: RemoveResourceFromCollectionRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("remove_resource_from_collection", HttpMethod::Post, request)
            .await
    }

    /// Create a new collection for the user.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CreateCollectionRequest`]
    ///
    /// ## Returns
    /// The ID of the newly created collection.
    ///
    /// Note [`CreateCollectionRequest::forupload`] only has an effect when the
    /// name is blank, in which case RS generates a timestamped one; it does
    /// not otherwise mark the collection.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if collection creation is not
    /// permitted for the user, which includes holding the restrictive `b`
    /// permission.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::CreateCollectionRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let collection_id = client
    ///     .collection()
    ///     .create_collection(CreateCollectionRequest::new("Trees"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_collection(&self, request: CreateCollectionRequest) -> Result<u32, Error> {
        self.client
            .send_request("create_collection", HttpMethod::Post, request)
            .await
    }

    /// Deletes a collection. The user must have write access to this collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteCollectionRequest`]
    ///
    /// ## Returns
    /// Nothing. Unlike its siblings this endpoint returns `null` rather than
    /// `true` on success, despite the knowledge base documenting it as
    /// "true or false depending on operation success". Failure still arrives
    /// as `false`, and so as [`Error::OperationFailed`].
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection is not writable by
    /// the user, if the user holds the restrictive `b` permission, or if the
    /// collection is a featured collection category that still has children.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::DeleteCollectionRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .delete_collection(DeleteCollectionRequest::new(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_collection(&self, request: DeleteCollectionRequest) -> Result<(), Error> {
        self.client
            .send_request("delete_collection", HttpMethod::Post, request)
            .await
    }

    /// Search public and featured collections.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SearchPublicCollectionsRequest`]
    ///
    /// ## Returns
    /// Matching public and featured collections, or an empty list when none
    /// match. This is the only endpoint that populates
    /// [`Collection::is_featured_collection_category`].
    ///
    /// ## Errors
    /// Returns [`Error::Deserialize`] if the response does not match
    /// [`Collection`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::SearchPublicCollectionsRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let found = client
    ///     .collection()
    ///     .search_public_collections(SearchPublicCollectionsRequest::new().search("trees"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_public_collections(
        &self,
        request: SearchPublicCollectionsRequest,
    ) -> Result<Vec<Collection>, Error> {
        self.client
            .send_request("search_public_collections", HttpMethod::Get, request)
            .await
    }

    /// Get collection details.
    ///
    /// This requires administrator access ("a" permission).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetCollectionRequest`]
    ///
    /// ## Returns
    /// All available columns for the collection. This is the only endpoint
    /// that populates [`Collection::groups`], [`Collection::users`] and
    /// [`Collection::request_feedback`].
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection does not exist or
    /// the user lacks the "a" permission.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::GetCollectionRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let collection = client
    ///     .collection()
    ///     .get_collection(GetCollectionRequest::new(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_collection(&self, request: GetCollectionRequest) -> Result<Collection, Error> {
        self.client
            .send_request("get_collection", HttpMethod::Get, request)
            .await
    }

    /// Save collection data.
    ///
    /// Replaces rather than patches: any field left unset in
    /// [`SaveCollectionColdata`] is *cleared* on the collection, so read the
    /// current values first and repeat the ones you intend to keep.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SaveCollectionRequest`]
    ///
    /// ## Returns
    /// Always `true`. ResourceSpace's `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the user holds the restrictive
    /// `b` permission, if [`SaveCollectionColdata`] is entirely empty, or if
    /// `type` is set to anything other than `0`, `3` or `4` — the API rejects
    /// the other collection types outright.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::{SaveCollectionColdata, SaveCollectionRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .save_collection(SaveCollectionRequest::new(
    ///         7,
    ///         SaveCollectionColdata::new().name("Trees").public(true),
    ///     ))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save_collection(&self, request: SaveCollectionRequest) -> Result<bool, Error> {
        self.client
            .send_request("save_collection", HttpMethod::Post, request)
            .await
    }

    /// Shows or hides a collection from the user's drop-down list.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ShowHideCollectionRequest`]
    ///
    /// ## Returns
    /// Always `true`. ResourceSpace's `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection or user does not
    /// exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::ShowHideCollectionRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .show_hide_collection(ShowHideCollectionRequest::new(7, false, 1))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show_hide_collection(
        &self,
        request: ShowHideCollectionRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("show_hide_collection", HttpMethod::Post, request)
            .await
    }

    /// Sends a copy of the collection for admin review.
    ///
    /// Notifies the administrator, so this has an effect outside the instance.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SendCollectionToAdminRequest`]
    ///
    /// ## Returns
    /// Always `true`. ResourceSpace's `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection does not exist or
    /// is not readable by the user.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::SendCollectionToAdminRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .send_collection_to_admin(SendCollectionToAdminRequest::new(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_collection_to_admin(
        &self,
        request: SendCollectionToAdminRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("send_collection_to_admin", HttpMethod::Post, request)
            .await
    }

    /// Get ResourceSpace featured collections (category).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetFeaturedCollectionsRequest`]
    ///
    /// ## Returns
    /// The featured collections directly beneath the requested parent, or an
    /// empty list if the parent is invalid or has no children. Pass `0` for
    /// the root.
    ///
    /// Note this is a narrower row than [`Collection`] — see
    /// [`FeaturedCollection`].
    ///
    /// ## Errors
    /// Returns [`Error::Deserialize`] if the response does not match
    /// [`FeaturedCollection`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::GetFeaturedCollectionsRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let roots = client
    ///     .collection()
    ///     .get_featured_collections(GetFeaturedCollectionsRequest::new(0))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_featured_collections(
        &self,
        request: GetFeaturedCollectionsRequest,
    ) -> Result<Vec<FeaturedCollection>, Error> {
        self.client
            .send_request("get_featured_collections", HttpMethod::Get, request)
            .await
    }

    /// Deletes all resources in a collection.
    ///
    /// The user must have edit access to the resources, permission to delete resources and the collection must be writable.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteResourcesInCollectionRequest`]
    ///
    /// ## Returns
    /// Always `true`. ResourceSpace's `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned. Note this
    /// succeeds on an empty collection.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection is not writable or
    /// the user lacks permission to delete resources.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::DeleteResourcesInCollectionRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .delete_resources_in_collection(DeleteResourcesInCollectionRequest::new(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_resources_in_collection(
        &self,
        request: DeleteResourcesInCollectionRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("delete_resources_in_collection", HttpMethod::Post, request)
            .await
    }

    /// Get the total resource count for a list of collections.
    ///
    /// Collections must be readable by the user; unreadable IDs are filtered
    /// out server-side rather than causing an error.
    ///
    /// The `b` permission *restricts* collection access — a user who holds it
    /// gets an empty map back. The knowledge base describes this endpoint as
    /// requiring `b`, which is backwards.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetCollectionsResourceCountRequest`]
    ///
    /// ## Returns
    /// A map of collection ID to resource count. IDs that fail validation are
    /// simply absent from the map, so it may be smaller than the input.
    ///
    /// ## Errors
    /// Returns [`Error::Deserialize`] if the response does not match. Note a
    /// user holding the restrictive `b` permission gets an empty map rather
    /// than an error.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::GetCollectionsResourceCountRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let counts = client
    ///     .collection()
    ///     .get_collections_resource_count(GetCollectionsResourceCountRequest::new([7, 8]))
    ///     .await?;
    /// println!("{:?}", counts.get(&7));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_collections_resource_count(
        &self,
        request: GetCollectionsResourceCountRequest,
    ) -> Result<HashMap<u32, u32>, Error> {
        self.client
            .send_request("get_collections_resource_count", HttpMethod::Get, request)
            .await
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddResourceToCollectionRequest {
    /// The ID of the resource to add.
    pub resource: u32,
    /// The ID of the collection to add the resource to.
    pub collection: u32,
    /// Adds every resource matching this search query, rather than just
    /// `resource`.
    pub search: Option<String>,
}

impl AddResourceToCollectionRequest {
    pub fn new(resource: u32, collection: u32) -> Self {
        Self {
            resource,
            collection,
            search: None,
        }
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemoveResourceFromCollectionRequest {
    /// The ID of the resource to remove.
    pub resource: u32,
    /// The ID of the collection to remove the resource from.
    pub collection: u32,
}

impl RemoveResourceFromCollectionRequest {
    pub fn new(resource: u32, collection: u32) -> Self {
        Self {
            resource,
            collection,
        }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateCollectionRequest {
    /// The name of the new collection.
    pub name: String,
    /// If true, marks this collection as an upload collection.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub forupload: Option<bool>,
}

impl CreateCollectionRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            forupload: None,
        }
    }

    pub fn forupload(mut self, forupload: bool) -> Self {
        self.forupload = Some(forupload);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteCollectionRequest {
    /// The ID of the collection to delete.
    ///
    /// Sent as `ref`, not `collection`. The knowledge base documents this
    /// parameter as `$collection`, but RS 11.0 only accepts `ref` — sending
    /// `collection` leaves the parameter unbound and the call returns `false`.
    #[serde(rename = "ref")]
    pub collection: u32,
}

impl DeleteCollectionRequest {
    pub fn new(collection: u32) -> Self {
        Self { collection }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SearchPublicCollectionsRequest {
    /// Optional search string to filter collections by name.
    pub search: Option<String>,
    /// Field name to order results by.
    pub order_by: Option<String>,
    /// Sort direction for the results.
    pub sort: Option<SortOrder>,
    /// If true, excludes theme/featured collections from results.
    /// Defaults to `true` server-side when omitted.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_themes: Option<bool>,
    /// If true, excludes public collections from results.
    /// Defaults to `false` server-side when omitted.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_public: Option<bool>,
}

impl SearchPublicCollectionsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn sort(mut self, sort: SortOrder) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn exclude_themes(mut self, exclude_themes: bool) -> Self {
        self.exclude_themes = Some(exclude_themes);
        self
    }

    pub fn exclude_public(mut self, exclude_public: bool) -> Self {
        self.exclude_public = Some(exclude_public);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetCollectionRequest {
    /// The ID of the collection to retrieve.
    #[serde(rename = "ref")]
    pub collection_id: u32,
}

impl GetCollectionRequest {
    pub fn new(collection_id: u32) -> Self {
        Self { collection_id }
    }
}

#[non_exhaustive]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SaveCollectionRequest {
    /// The ID of the collection to save.
    #[serde(rename = "ref")]
    pub collection_id: u32,
    /// JSON object containing the collection fields to update (e.g. name, description, public).
    #[serde_as(as = "JsonString")]
    pub coldata: SaveCollectionColdata,
}

impl SaveCollectionRequest {
    pub fn new(collection_id: u32, coldata: SaveCollectionColdata) -> Self {
        Self {
            collection_id,
            coldata,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Validate)]
pub struct SaveCollectionColdata {
    /// Comma-separated value of keywords to be associated with this collection.
    pub keywords: Option<List<String>>,
    /// If true, other users are allowed to add/remove resources when collection is shared or is public.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_changes: Option<bool>,
    /// Comma-separated value of users to attach to the collection.
    pub users: Option<List<String>>,
    /// Collection name.
    pub name: Option<String>,
    /// If true, public. Otherwise private (legacy).
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub public: Option<bool>,
    /// 0 = standard, 3 = Featured collection, 4 = public. If 3 or 4 then public should be set to 1.
    #[serde(rename = "type")]
    pub r#type: Option<u8>,
    /// ID of parent featured collection. Set to 0 to create a new root level collection (see below). Applies to Featured collections only.
    pub parent: Option<u32>,
    /// If true, creates a root level featured collection (parent=0). Applies to Featured collections only.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub force_featured_collection_type: Option<bool>,
    /// 0 = no image, 1 = most popular image, 10 - most popular images, 100 - manually select image. Applies to Featured collections only.
    pub thumbnail_selection_method: Option<u32>,
    /// Resource ID to use as thumbnail. Only if thumbnail_selection_method =100. Applies to Featured collections only.
    pub bg_img_resource_ref: Option<u32>,
}

impl SaveCollectionColdata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keywords(mut self, keywords: impl Into<List<String>>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    pub fn allow_changes(mut self, allow_changes: bool) -> Self {
        self.allow_changes = Some(allow_changes);
        self
    }

    pub fn users(mut self, users: impl Into<List<String>>) -> Self {
        self.users = Some(users.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn public(mut self, public: bool) -> Self {
        self.public = Some(public);
        self
    }

    pub fn r#type(mut self, r#type: u8) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn parent(mut self, parent: u32) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn force_featured_collection_type(mut self, force: bool) -> Self {
        self.force_featured_collection_type = Some(force);
        self
    }

    pub fn thumbnail_selection_method(mut self, method: u32) -> Self {
        self.thumbnail_selection_method = Some(method);
        self
    }

    pub fn bg_img_resource_ref(mut self, resource_ref: u32) -> Self {
        self.bg_img_resource_ref = Some(resource_ref);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ShowHideCollectionRequest {
    /// The ID of the collection to show or hide.
    pub collection: u32,
    /// If true, shows the collection in the drop-down list. Otherwise, hides it.
    #[serde(serialize_with = "bool_as_u8")]
    pub show: bool,
    /// The ID of the user whose drop-down list is being updated.
    pub user: u32,
}

impl ShowHideCollectionRequest {
    pub fn new(collection: u32, show: bool, user: u32) -> Self {
        Self {
            collection,
            show,
            user,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SendCollectionToAdminRequest {
    /// The ID of the collection to send to the administrator for review.
    pub collection: u32,
}

impl SendCollectionToAdminRequest {
    pub fn new(collection: u32) -> Self {
        Self { collection }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetFeaturedCollectionsRequest {
    /// The ID of the parent featured collection (category) to retrieve children for. Use 0 for top-level.
    pub parent: u32,
}

impl GetFeaturedCollectionsRequest {
    pub fn new(parent: u32) -> Self {
        Self { parent }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteResourcesInCollectionRequest {
    /// The ID of the collection whose resources should all be deleted.
    pub collection: u32,
}

impl DeleteResourcesInCollectionRequest {
    pub fn new(collection: u32) -> Self {
        Self { collection }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetCollectionsResourceCountRequest {
    /// Comma-separated list of collection IDs to retrieve resource counts for.
    #[serde(rename = "refs")]
    pub collection_ids: List<u32>,
}

impl GetCollectionsResourceCountRequest {
    pub fn new(collection_ids: impl Into<List<u32>>) -> Self {
        Self {
            collection_ids: collection_ids.into(),
        }
    }
}
