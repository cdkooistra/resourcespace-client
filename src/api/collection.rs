use serde::Serialize;
use serde_with::json::JsonString;
use serde_with::{serde_as, skip_serializing_none};
use validator::Validate;

use crate::client::Client;
use crate::error::Error;

use super::{List, SortOrder};

/// Sub-API for collection endpoints.
#[derive(Debug)]
pub struct CollectionApi<'a> {
    client: &'a Client,
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
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let status = client.collection().get_user_collections().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_collections(&self) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_user_collections", reqwest::Method::GET, ())
            .await
    }

    /// Add a resource to a collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`AddResourceToCollectionRequest`]
    ///
    /// ## Returns
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn add_resource_to_collection(
        &self,
        request: AddResourceToCollectionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("add_resource_to_collection", reqwest::Method::POST, request)
            .await
    }

    /// Remove a resource from a collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`RemoveResourceFromCollectionRequest`]
    ///
    /// ## Returns
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn remove_resource_from_collection(
        &self,
        request: RemoveResourceFromCollectionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request(
                "remove_resource_from_collection",
                reqwest::Method::POST,
                request,
            )
            .await
    }

    /// Create a new collection for the user.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CreateCollectionRequest`]
    ///
    /// ## Returns
    /// Integer|bool - ID of the collection created, false if collection creation is not permitted
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn create_collection(
        &self,
        request: CreateCollectionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("create_collection", reqwest::Method::POST, request)
            .await
    }

    /// Deletes a collection. The user must have write access to this collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteCollectionRequest`]
    ///
    /// ## Returns
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn delete_collection(
        &self,
        request: DeleteCollectionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("delete_collection", reqwest::Method::POST, request)
            .await
    }

    /// Search public and featured collections.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SearchPublicCollectionsRequest`]
    ///
    /// ## Returns
    /// A list of matching public or featured collections.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn search_public_collections(
        &self,
        request: SearchPublicCollectionsRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("search_public_collections", reqwest::Method::GET, request)
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
    /// The collection details including owner name, description, public/private status, thumbnail image reference. All available columns are returned.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_collection(
        &self,
        request: GetCollectionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_collection", reqwest::Method::GET, request)
            .await
    }

    /// Save collection data.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SaveCollectionRequest`]
    ///
    /// ## Returns
    /// Returns false if access control fails or invalid arguments have been received (e.g ref not a number), true otherwise.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn save_collection(
        &self,
        request: SaveCollectionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("save_collection", reqwest::Method::POST, request)
            .await
    }

    /// Shows or hides a collection from the user's drop-down list.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ShowHideCollectionRequest`]
    ///
    /// ## Returns
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn show_hide_collection(
        &self,
        request: ShowHideCollectionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("show_hide_collection", reqwest::Method::POST, request)
            .await
    }

    /// Sends a copy of the collection for admin review.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SendCollectionToAdminRequest`]
    ///
    /// ## Returns
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn send_collection_to_admin(
        &self,
        request: SendCollectionToAdminRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("send_collection_to_admin", reqwest::Method::POST, request)
            .await
    }

    /// Get ResourceSpace featured collections (category).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetFeaturedCollectionsRequest`]
    ///
    /// ## Returns
    /// If successful, a 200 HTTP status will be returned with the body containing an array. If the parent is invalid an empty array will be returned instead.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_featured_collections(
        &self,
        request: GetFeaturedCollectionsRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_featured_collections", reqwest::Method::GET, request)
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
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn delete_resources_in_collection(
        &self,
        request: DeleteResourcesInCollectionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request(
                "delete_resources_in_collection",
                reqwest::Method::POST,
                request,
            )
            .await
    }

    /// Get the total resource count for a list of collections.
    ///
    /// Requires permission `b` and the collections must be readable by the user.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetCollectionsResourceCountRequest`]
    ///
    /// ## Returns
    /// Array of collections and their total resource count. Note the returned array may
    /// not contain keys for all input IDs if validation fails for some.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_collections_resource_count(
        &self,
        request: GetCollectionsResourceCountRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request(
                "get_collections_resource_count",
                reqwest::Method::GET,
                request,
            )
            .await
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddResourceToCollectionRequest {
    /// The ID of the resource to add.
    pub resource: u32,
    /// The ID of the collection to add the resource to.
    pub collection: u32,
}

impl AddResourceToCollectionRequest {
    pub fn new(resource: u32, collection: u32) -> Self {
        Self {
            resource,
            collection,
        }
    }
}

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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateCollectionRequest {
    /// The name of the new collection.
    pub name: String,
    /// If set, marks this collection as an upload collection.
    pub forupload: Option<u8>,
}

impl CreateCollectionRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            forupload: None,
        }
    }

    pub fn forupload(mut self, forupload: bool) -> Self {
        self.forupload = Some(forupload as u8);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteCollectionRequest {
    /// The ID of the collection to delete.
    pub collection: u32,
}

impl DeleteCollectionRequest {
    pub fn new(collection: u32) -> Self {
        Self { collection }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SearchPublicCollectionsRequest {
    /// Optional search string to filter collections by name.
    pub search: Option<String>,
    /// Field name to order results by.
    pub order_by: Option<String>,
    /// Sort direction for the results.
    pub sort: Option<SortOrder>,
    /// If set, excludes theme/featured collections from results.
    pub exclude_themes: Option<u8>,
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
        self.exclude_themes = Some(exclude_themes as u8);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetCollectionRequest {
    /// The ID of the collection to retrieve.
    #[serde(rename = "ref")]
    pub r#ref: u32,
}

impl GetCollectionRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref }
    }
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SaveCollectionRequest {
    /// The ID of the collection to save.
    #[serde(rename = "ref")]
    pub r#ref: u32,
    /// JSON object containing the collection fields to update (e.g. name, description, public).
    #[serde_as(as = "JsonString")]
    pub coldata: SaveCollectionColdata,
}

impl SaveCollectionRequest {
    pub fn new(r#ref: u32, coldata: SaveCollectionColdata) -> Self {
        Self { r#ref, coldata }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Validate)]
pub struct SaveCollectionColdata {
    /// Comma-separated value of keywords to be associated with this collection.
    pub keywords: Option<List<String>>,
    /// To set whether other users are allowed to add/remove resources when collection is shared or is public. The allowed value is 0 or 1.
    #[validate(range(min = 0, max = 1))]
    pub allow_changes: Option<u8>,
    /// Comma-separated value of users to attach to the collection.
    pub users: Option<List<String>>,
    /// Collection name.
    pub name: Option<String>,
    /// 0 for private, 1 for public (legacy).
    #[validate(range(min = 0, max = 1))]
    pub public: Option<u8>,
    /// 0 = standard, 3 = Featured collection, 4 = public. If 3 or 4 then public should be set to 1.
    #[serde(rename = "type")]
    pub r#type: Option<u8>,
    /// ID of parent featured collection. Set to 0 to create a new root level collection (see below). Applies to Featured collections only.
    pub parent: Option<u32>,
    /// Required to be set to 1 if creating a root level featured collection (parent=0). Applies to Featured collections only.
    #[validate(range(min = 0, max = 1))]
    pub force_featured_collection_type: Option<u8>,
    /// 0 = no image, 1 = most popular image, 10 - most popular images, 100 - manually select image. Applies to Featured collections only.
    pub thumbnail_selection_method: Option<u32>,
    /// Resource ID to use as thumbnail. Only if thumbnail_selection_method =100. Applies to Featured collections only.
    pub bg_img_resource_ref: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ShowHideCollectionRequest {
    /// The ID of the collection to show or hide.
    pub collection: u32,
    /// If set, shows the collection in the drop-down list.
    pub show: u8,
    /// The ID of the user whose drop-down list is being updated.
    pub user: u32,
}

impl ShowHideCollectionRequest {
    pub fn new(collection: u32, show: bool, user: u32) -> Self {
        Self {
            collection,
            show: show as u8,
            user,
        }
    }
}

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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetCollectionsResourceCountRequest {
    /// Comma-separated list of collection IDs to retrieve resource counts for.
    pub refs: List<u32>,
}

impl GetCollectionsResourceCountRequest {
    pub fn new(refs: impl Into<List<u32>>) -> Self {
        Self { refs: refs.into() }
    }
}
