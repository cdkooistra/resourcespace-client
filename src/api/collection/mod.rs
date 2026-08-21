use std::collections::HashMap;

use crate::client::{Client, HttpMethod};
use crate::error::Error;

pub mod request;
pub mod response;

// SaveCollectionColdata is referenced only from a doc link below; the import
// keeps it resolvable.
#[allow(unused_imports)]
use request::{
    AddResourceToCollection, CreateCollection, DeleteCollection, DeleteResourcesInCollection,
    GetCollection, GetCollectionsResourceCount, GetFeaturedCollections,
    RemoveResourceFromCollection, SaveCollection, SaveCollectionColdata, SearchPublicCollections,
    SendCollectionToAdmin, ShowHideCollection,
};
use response::{Collection, FeaturedCollection};

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
    /// * `request` - Parameters built via [`AddResourceToCollection`]
    ///
    /// ## Returns
    /// Always `true`. `ResourceSpace`'s `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection is not writable by
    /// the user or either ID does not exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::request::AddResourceToCollection;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .add_resource_to_collection(AddResourceToCollection::new(1234, 7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_resource_to_collection(
        &self,
        request: AddResourceToCollection,
    ) -> Result<bool, Error> {
        self.client
            .send_request("add_resource_to_collection", HttpMethod::Post, request)
            .await
    }

    /// Remove a resource from a collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`RemoveResourceFromCollection`]
    ///
    /// ## Returns
    /// Always `true`. `ResourceSpace`'s `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection is not writable by
    /// the user or either ID does not exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::request::RemoveResourceFromCollection;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .remove_resource_from_collection(RemoveResourceFromCollection::new(1234, 7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_resource_from_collection(
        &self,
        request: RemoveResourceFromCollection,
    ) -> Result<bool, Error> {
        self.client
            .send_request("remove_resource_from_collection", HttpMethod::Post, request)
            .await
    }

    /// Create a new collection for the user.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CreateCollection`]
    ///
    /// ## Returns
    /// The ID of the newly created collection.
    ///
    /// Note [`CreateCollection::forupload`] only has an effect when the
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
    /// # use resourcespace_client::api::collection::request::CreateCollection;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let collection_id = client
    ///     .collection()
    ///     .create_collection(CreateCollection::new("Trees"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_collection(&self, request: CreateCollection) -> Result<u32, Error> {
        self.client
            .send_request("create_collection", HttpMethod::Post, request)
            .await
    }

    /// Deletes a collection. The user must have write access to this collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteCollection`]
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
    /// # use resourcespace_client::api::collection::request::DeleteCollection;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .delete_collection(DeleteCollection::new(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_collection(&self, request: DeleteCollection) -> Result<(), Error> {
        self.client
            .send_request("delete_collection", HttpMethod::Post, request)
            .await
    }

    /// Search public and featured collections.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SearchPublicCollections`]
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
    /// # use resourcespace_client::api::collection::request::SearchPublicCollections;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let found = client
    ///     .collection()
    ///     .search_public_collections(SearchPublicCollections::new().search("trees"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_public_collections(
        &self,
        request: SearchPublicCollections,
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
    /// * `request` - Parameters built via [`GetCollection`]
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
    /// # use resourcespace_client::api::collection::request::GetCollection;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let collection = client
    ///     .collection()
    ///     .get_collection(GetCollection::new(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_collection(&self, request: GetCollection) -> Result<Collection, Error> {
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
    /// * `request` - Parameters built via [`SaveCollection`]
    ///
    /// ## Returns
    /// Always `true`. `ResourceSpace`'s `false` is surfaced as
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
    /// # use resourcespace_client::api::collection::request::{SaveCollectionColdata, SaveCollection};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .save_collection(SaveCollection::new(
    ///         7,
    ///         SaveCollectionColdata::new().name("Trees").public(true),
    ///     ))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save_collection(&self, request: SaveCollection) -> Result<bool, Error> {
        self.client
            .send_request("save_collection", HttpMethod::Post, request)
            .await
    }

    /// Shows or hides a collection from the user's drop-down list.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ShowHideCollection`]
    ///
    /// ## Returns
    /// Always `true`. `ResourceSpace`'s `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection or user does not
    /// exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::request::ShowHideCollection;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .show_hide_collection(ShowHideCollection::new(7, false, 1))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show_hide_collection(&self, request: ShowHideCollection) -> Result<bool, Error> {
        self.client
            .send_request("show_hide_collection", HttpMethod::Post, request)
            .await
    }

    /// Sends a copy of the collection for admin review.
    ///
    /// Notifies the administrator, so this has an effect outside the instance.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SendCollectionToAdmin`]
    ///
    /// ## Returns
    /// Always `true`. `ResourceSpace`'s `false` is surfaced as
    /// [`Error::OperationFailed`] instead of being returned.
    ///
    /// ## Errors
    /// Returns [`Error::OperationFailed`] if the collection does not exist or
    /// is not readable by the user.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::collection::request::SendCollectionToAdmin;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .send_collection_to_admin(SendCollectionToAdmin::new(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_collection_to_admin(
        &self,
        request: SendCollectionToAdmin,
    ) -> Result<bool, Error> {
        self.client
            .send_request("send_collection_to_admin", HttpMethod::Post, request)
            .await
    }

    /// Get `ResourceSpace` featured collections (category).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetFeaturedCollections`]
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
    /// # use resourcespace_client::api::collection::request::GetFeaturedCollections;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let roots = client
    ///     .collection()
    ///     .get_featured_collections(GetFeaturedCollections::new(0))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_featured_collections(
        &self,
        request: GetFeaturedCollections,
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
    /// * `request` - Parameters built via [`DeleteResourcesInCollection`]
    ///
    /// ## Returns
    /// Always `true`. `ResourceSpace`'s `false` is surfaced as
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
    /// # use resourcespace_client::api::collection::request::DeleteResourcesInCollection;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .collection()
    ///     .delete_resources_in_collection(DeleteResourcesInCollection::new(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_resources_in_collection(
        &self,
        request: DeleteResourcesInCollection,
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
    /// * `request` - Parameters built via [`GetCollectionsResourceCount`]
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
    /// # use resourcespace_client::api::collection::request::GetCollectionsResourceCount;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let counts = client
    ///     .collection()
    ///     .get_collections_resource_count(GetCollectionsResourceCount::new([7, 8]))
    ///     .await?;
    /// println!("{:?}", counts.get(&7));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_collections_resource_count(
        &self,
        request: GetCollectionsResourceCount,
    ) -> Result<HashMap<u32, u32>, Error> {
        self.client
            .send_request("get_collections_resource_count", HttpMethod::Get, request)
            .await
    }
}
