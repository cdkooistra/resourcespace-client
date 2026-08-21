use std::collections::HashMap;

use crate::client::{Client, HttpMethod};
use crate::error::Error;

use super::shared::AjaxEnvelope;
use response::CreatedField;

mod request;
mod response;
mod shared;

pub use request::{
    AddResourceNodesMultiRequest, AddResourceNodesRequest, CreateResourceTypeFieldRequest,
    FieldIdentifier, GetFieldOptionsRequest, GetNodeIdRequest, GetNodesRequest,
    GetResourceTypeFieldsRequest, SetNodeRequest, ToggleActiveStatesForNodesRequest,
    UpdateFieldRequest,
};
pub use response::{FieldOptions, Node, ResourceTypeField};

#[derive(Debug)]
pub struct MetadataApi<'a> {
    client: &'a Client,
}

/// Sub-API for metadata endpoints.
impl<'a> MetadataApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// For a given field, return all the available tags (nodes) or selectable options.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetFieldOptionsRequest`]
    ///
    /// ## Returns
    ///
    /// [`FieldOptions::Nodes`] when [`GetFieldOptionsRequest::nodeinfo`] is
    /// set, otherwise [`FieldOptions::Names`]. The node records here omit
    /// `resource_type_field`, unlike [`Self::get_nodes`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller cannot view the
    /// field.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::GetFieldOptionsRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let options = client
    ///     .metadata()
    ///     .get_field_options(GetFieldOptionsRequest::new("keywords").nodeinfo(true))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_field_options(
        &self,
        request: GetFieldOptionsRequest,
    ) -> Result<FieldOptions, Error> {
        self.client
            .send_request("get_field_options", HttpMethod::Get, request)
            .await
    }

    /// Find a node ID (entry in a fixed tag field) given the name of the node.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetNodeIdRequest`]
    ///
    /// ## Returns
    ///
    /// The node's ID.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller cannot view the
    /// field, or if no node with that name exists on it.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::GetNodeIdRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let node_id = client
    ///     .metadata()
    ///     .get_node_id(GetNodeIdRequest::new("Landscape", 12))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_node_id(&self, request: GetNodeIdRequest) -> Result<u32, Error> {
        self.client
            .send_request("get_node_id", HttpMethod::Get, request)
            .await
    }

    /// Get all nodes (fixed keywords) from database for a specific metadata field or parent.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetNodesRequest`]
    ///
    /// ## Returns
    ///
    /// Every node on the field, or only the children of
    /// [`GetNodesRequest::parent`] when that is set.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller cannot view the
    /// field.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::GetNodesRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// for node in client.metadata().get_nodes(GetNodesRequest::new(12)).await? {
    ///     println!("{} = {}", node.node_id, node.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_nodes(&self, request: GetNodesRequest) -> Result<Vec<Node>, Error> {
        self.client
            .send_request("get_nodes", HttpMethod::Get, request)
            .await
    }

    /// Add all node IDs (field options) in the list to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`AddResourceNodesRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] unless the caller holds the `a`
    /// permission — this endpoint is super-admin only — or if any node ID
    /// does not exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::AddResourceNodesRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .metadata()
    ///     .add_resource_nodes(AddResourceNodesRequest::new(1234, [87, 88]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_resource_nodes(
        &self,
        request: AddResourceNodesRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("add_resource_nodes", HttpMethod::Post, request)
            .await
    }

    /// Add all node IDs (field options) in the list to the resources specified.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`AddResourceNodesMultiRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] unless the caller holds the `a`
    /// permission — this endpoint is super-admin only.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::AddResourceNodesMultiRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .metadata()
    ///     .add_resource_nodes_multi(AddResourceNodesMultiRequest::new([1234, 1235], [87]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_resource_nodes_multi(
        &self,
        request: AddResourceNodesMultiRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("add_resource_nodes_multi", HttpMethod::Post, request)
            .await
    }

    /// Create a new node (option for a fixed list field).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SetNodeRequest`]
    ///
    /// ## Returns
    ///
    /// The node's ID — the new one when creating, or the existing one when
    /// updating. `ResourceSpace` deduplicates by name on non-tree fields, so
    /// creating a node that already exists returns the original's ID rather
    /// than making a second one.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the field is not a fixed-list
    /// type, or if the caller holds neither `a` nor `k` (nor, for a dynamic
    /// keywords field, the per-field permission).
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::SetNodeRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // Create
    /// let node_id = client
    ///     .metadata()
    ///     .set_node(SetNodeRequest::new(None, 12, "Landscape"))
    ///     .await?;
    ///
    /// // Rename that node
    /// client
    ///     .metadata()
    ///     .set_node(SetNodeRequest::new(node_id, 12, "Landscapes"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_node(&self, request: SetNodeRequest) -> Result<u32, Error> {
        self.client
            .send_request("set_node", HttpMethod::Post, request)
            .await
    }

    /// Get metadata field information for all (matching) fields.
    ///
    /// Available from RS version 10.3+ and requires permission `a`.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceTypeFieldsRequest`]
    ///
    /// ## Returns
    ///
    /// Matching field definitions, ordered by ID.
    ///
    /// ## Errors
    ///
    /// Returns an empty list, with HTTP 403 suppressed, when the caller
    /// lacks the `a` permission.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::GetResourceTypeFieldsRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // Every fixed-list (type 9) field
    /// let fields = client
    ///     .metadata()
    ///     .get_resource_type_fields(GetResourceTypeFieldsRequest::new().field_type_ids([9]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_type_fields(
        &self,
        request: GetResourceTypeFieldsRequest,
    ) -> Result<Vec<ResourceTypeField>, Error> {
        self.client
            .send_request("get_resource_type_fields", HttpMethod::Get, request)
            .await
    }

    /// Create a metadata field.
    ///
    /// Available from RS version 10.3+ and requires permission `a`.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CreateResourceTypeFieldRequest`]
    ///
    /// ## Returns
    ///
    /// The new field's ID, unwrapped from the
    /// `{"status": ..., "data": {"ref": N}}` envelope this endpoint returns.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Deserialize`] when the save is rejected — the
    /// failure envelope carries a message rather than a `ref`, so there is
    /// no ID to return. Callers lacking the `a` permission get HTTP 403 and
    /// so [`Error::Http`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::CreateResourceTypeFieldRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // Type 3 is a dropdown list; pass 0 as the resource type for a global field.
    /// let field_id = client
    ///     .metadata()
    ///     .create_resource_type_field(CreateResourceTypeFieldRequest::new("Region", [0], "3"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_resource_type_field(
        &self,
        request: CreateResourceTypeFieldRequest,
    ) -> Result<u32, Error> {
        let envelope: AjaxEnvelope<CreatedField> = self
            .client
            .send_request("create_resource_type_field", HttpMethod::Post, request)
            .await?;
        Ok(envelope.data.field_id)
    }

    /// Toggle nodes' active state.
    ///
    /// Available from RS version 10.4+ and requires permission `k`.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ToggleActiveStatesForNodesRequest`]
    ///
    /// ## Returns
    ///
    /// Each node ID mapped to its *new* active state, `1` active and `0`
    /// inactive. Nodes that could not be toggled are absent from the map.
    ///
    /// ## Errors
    ///
    /// Returns an empty map, with HTTP 403 suppressed, when the caller lacks
    /// the `k` permission.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::metadata::ToggleActiveStatesForNodesRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let states = client
    ///     .metadata()
    ///     .toggle_active_state_for_nodes(ToggleActiveStatesForNodesRequest::new([87]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn toggle_active_state_for_nodes(
        &self,
        request: ToggleActiveStatesForNodesRequest,
    ) -> Result<HashMap<u32, u8>, Error> {
        self.client
            .send_request("toggle_active_state_for_nodes", HttpMethod::Post, request)
            .await
    }

    /// Set the value of a metadata field.
    ///
    /// When constructing `FieldValue` from node IDs, the `nodevalues` parameter is
    /// automatically set to `true`.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateFieldRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource does not exist,
    /// the caller lacks edit access to it, the field does not exist, or the
    /// caller lacks edit access to that field.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::metadata::UpdateFieldRequest;
    /// # use resourcespace_client::api::FieldValue;
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder().base_url("https://example.com").user_key("user", "key").build().await?;
    /// # let rs_id = 11u32;
    ///
    /// // single text value
    /// client.metadata().update_field(
    ///     UpdateFieldRequest::new(rs_id, "name", FieldValue::from("Doe, John"))
    /// ).await?;
    ///
    /// // node IDs
    /// client.metadata().update_field(
    ///     UpdateFieldRequest::new(rs_id, "nodes", FieldValue::from([1u32, 2]))
    /// ).await?;
    ///
    /// // multiple keywords, auto-quoted if containing commas
    /// client.metadata().update_field(
    ///     UpdateFieldRequest::new(rs_id, "name_keywords", FieldValue::from(["Doe, John", "Smith, Jane"]))
    /// ).await?;
    /// # Ok(()) }
    /// ```
    pub async fn update_field(&self, request: UpdateFieldRequest) -> Result<bool, Error> {
        self.client
            .send_request("update_field", HttpMethod::Post, request)
            .await
    }
}
