use serde::{Serialize, Serializer};
use serde_with::json::JsonString;
use serde_with::{serde_as, skip_serializing_none};

use crate::client::{Client, HttpMethod};
use crate::error::Error;

use super::{FieldValue, List};

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
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_field_options(
        &self,
        request: GetFieldOptionsRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_field_options", HttpMethod::Get, request)
            .await
    }

    /// Find a node ID (entry in a fixed tag field) given the name of the node.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetNodeIdRequest`]
    ///
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_node_id(&self, request: GetNodeIdRequest) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_node_id", HttpMethod::Get, request)
            .await
    }

    /// Get all nodes (fixed keywords) from database for a specific metadata field or parent.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetNodesRequest`]
    ///
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_nodes(&self, request: GetNodesRequest) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_nodes", HttpMethod::Get, request)
            .await
    }

    /// Add all node IDs (field options) in the list to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`AddResourceNodesRequest`]
    ///
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn add_resource_nodes(
        &self,
        request: AddResourceNodesRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("add_resource_nodes", HttpMethod::Post, request)
            .await
    }

    /// Add all node IDs (field options) in the list to the resources specified.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`AddResourceNodesMultiRequest`]
    ///
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn add_resource_nodes_multi(
        &self,
        request: AddResourceNodesMultiRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("add_resource_nodes_multi", HttpMethod::Post, request)
            .await
    }

    /// Create a new node (option for a fixed list field).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SetNodeRequest`]
    ///
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn set_node(&self, request: SetNodeRequest) -> Result<serde_json::Value, Error> {
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
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_type_fields(
        &self,
        request: GetResourceTypeFieldsRequest,
    ) -> Result<serde_json::Value, Error> {
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
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn create_resource_type_field(
        &self,
        request: CreateResourceTypeFieldRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("create_resource_type_field", HttpMethod::Post, request)
            .await
    }

    /// Toggle nodes' active state.
    ///
    /// Available from RS version 10.4+ and requires permission `k`.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ToggleActiveStatesForNodesRequest`]
    ///
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn toggle_active_state_for_nodes(
        &self,
        request: ToggleActiveStatesForNodesRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("toggle_active_state_for_nodes", HttpMethod::Post, request)
            .await
    }

    /// Set the value of a metadata field.
    ///
    /// When constructing FieldValue from node IDs, the `nodevalues` parameter is
    /// automatically set to `true`.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateFieldRequest`]
    ///
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
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
    pub async fn update_field(
        &self,
        request: UpdateFieldRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("update_field", HttpMethod::Post, request)
            .await
    }
}

/// A metadata field identifier, either a numeric ID or a shortname.
///
/// Accepts a `u32` field ID or a string shortname via [`Into`] conversions,
/// making it ergonomic to reference fields at call sites:
///
/// ```no_run
/// # use resourcespace_client::api::metadata::FieldIdentifier;
/// let _ = FieldIdentifier::from(72u32);       // numeric ID
/// let _ = FieldIdentifier::from("title");     // shortname
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum FieldIdentifier {
    Id(u32),
    Shortname(String),
}

impl From<u32> for FieldIdentifier {
    fn from(id: u32) -> Self {
        Self::Id(id)
    }
}

impl From<String> for FieldIdentifier {
    fn from(name: String) -> Self {
        Self::Shortname(name)
    }
}

impl From<&str> for FieldIdentifier {
    fn from(name: &str) -> Self {
        Self::Shortname(name.to_string())
    }
}

impl Serialize for FieldIdentifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Id(id) => id.serialize(serializer),
            Self::Shortname(name) => name.serialize(serializer),
        }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetFieldOptionsRequest {
    /// The ID or shortname of the metadata field to retrieve options for.
    #[serde(rename = "ref")]
    pub field: FieldIdentifier,
    /// If set, returns additional node information alongside each option.
    pub nodeinfo: Option<bool>,
}

impl GetFieldOptionsRequest {
    pub fn new(field: impl Into<FieldIdentifier>) -> Self {
        Self {
            field: field.into(),
            nodeinfo: None,
        }
    }

    pub fn nodeinfo(mut self, nodeinfo: bool) -> Self {
        self.nodeinfo = Some(nodeinfo);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetNodeIdRequest {
    /// The name of the node to look up.
    pub value: String,
    /// The ID of the resource type field the node belongs to.
    pub resource_type_field: u32,
}

impl GetNodeIdRequest {
    pub fn new(value: impl Into<String>, resource_type_field: u32) -> Self {
        Self {
            value: value.into(),
            resource_type_field,
        }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetNodesRequest {
    /// The ID of the metadata field to retrieve nodes from.
    #[serde(rename = "ref")]
    pub field_id: u32,
    /// Restrict results to children of this parent node ID.
    pub parent: Option<u32>,
    /// If true, retrieves all descendant nodes recursively.
    pub recursive: Option<bool>,
    /// Number of nodes to skip, used for pagination.
    pub offset: Option<u32>,
    /// Maximum number of nodes to return.
    pub rows: Option<u32>,
    /// Filter nodes by name (partial match).
    pub name: Option<String>,
    /// If true, includes the number of resources using each node.
    pub use_count: Option<bool>,
    /// If true, orders results by the translated node name.
    pub order_by_translated_name: Option<bool>,
}

impl GetNodesRequest {
    pub fn new(field_id: u32) -> Self {
        Self {
            field_id,
            parent: None,
            recursive: None,
            offset: None,
            rows: None,
            name: None,
            use_count: None,
            order_by_translated_name: None,
        }
    }

    pub fn parent(mut self, parent: u32) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = Some(recursive);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn use_count(mut self, use_count: bool) -> Self {
        self.use_count = Some(use_count);
        self
    }

    pub fn order_by_translated_name(mut self, order_by_translated_name: bool) -> Self {
        self.order_by_translated_name = Some(order_by_translated_name);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddResourceNodesRequest {
    /// The ID of the resource to add nodes to.
    pub resource: u32,
    /// Comma-separated list of node IDs to add to the resource.
    pub nodestring: List<u32>,
}

impl AddResourceNodesRequest {
    pub fn new(resource: u32, nodestring: impl Into<List<u32>>) -> Self {
        Self {
            resource,
            nodestring: nodestring.into(),
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddResourceNodesMultiRequest {
    /// Comma-separated list of resource IDs to add nodes to.
    #[serde(rename = "resourceid")]
    pub resource_id: List<u32>,
    /// Comma-separated list of node IDs to add to each resource.
    #[serde(rename = "nodes")]
    pub node_ids: List<u32>,
}

impl AddResourceNodesMultiRequest {
    pub fn new(resource_id: impl Into<List<u32>>, node_ids: impl Into<List<u32>>) -> Self {
        Self {
            resource_id: resource_id.into(),
            node_ids: node_ids.into(),
        }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SetNodeRequest {
    /// The ID of an existing node to update, or 0 to create a new one.
    #[serde(rename = "ref")]
    pub node_id: u32,
    /// The ID of the resource type field this node belongs to.
    pub resource_type_field: u32,
    /// The name of the node.
    pub name: String,
    /// The ID of the parent node, if this is a child node.
    pub parent: Option<String>,
    /// Position used to order this node relative to siblings.
    pub order_by: Option<u32>,
    /// If set, returns the existing node instead of creating a duplicate.
    pub returnexisting: Option<bool>,
}

impl SetNodeRequest {
    pub fn new(node_id: u32, resource_type_field: u32, name: impl Into<String>) -> Self {
        Self {
            node_id,
            resource_type_field,
            name: name.into(),
            parent: None,
            order_by: None,
            returnexisting: None,
        }
    }
    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    pub fn order_by(mut self, order_by: u32) -> Self {
        self.order_by = Some(order_by);
        self
    }

    pub fn returnexisting(mut self, returnexisting: bool) -> Self {
        self.returnexisting = Some(returnexisting);
        self
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GetResourceTypeFieldsRequest {
    /// Comma-separated list of resource type IDs to filter fields by.
    #[serde(rename = "by_resource_types")]
    pub resource_type_ids: Option<List<u32>>,
    /// Search string to filter fields by name.
    pub find: Option<String>,
    /// Comma-separated list of field type IDs to filter by.
    #[serde(rename = "by_types")]
    pub field_type_ids: Option<List<u32>>,
}

impl GetResourceTypeFieldsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_type_ids(mut self, resource_type_ids: impl Into<List<u32>>) -> Self {
        self.resource_type_ids = Some(resource_type_ids.into());
        self
    }

    pub fn find(mut self, find: impl Into<String>) -> Self {
        self.find = Some(find.into());
        self
    }

    pub fn field_type_ids(mut self, field_type_ids: impl Into<List<u32>>) -> Self {
        self.field_type_ids = Some(field_type_ids.into());
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateResourceTypeFieldRequest {
    /// The name of the new metadata field.
    pub name: String,
    /// Comma-separated list of resource type IDs this field should apply to.
    #[serde(rename = "resource_types")]
    pub resource_type_ids: List<u32>,
    /// The field type, for values see the FIELD_TYPE_* constants.
    pub r#type: String,
}

impl CreateResourceTypeFieldRequest {
    pub fn new(
        name: impl Into<String>,
        resource_type_ids: impl Into<List<u32>>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            resource_type_ids: resource_type_ids.into(),
            r#type: r#type.into(),
        }
    }
}

#[non_exhaustive]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ToggleActiveStatesForNodesRequest {
    /// JSON-encoded array of node IDs whose active states should be toggled.
    #[serde_as(as = "JsonString")]
    #[serde(rename = "refs")]
    pub node_ids: Vec<u32>,
}

impl ToggleActiveStatesForNodesRequest {
    pub fn new(node_ids: impl Into<List<u32>>) -> Self {
        Self {
            node_ids: node_ids.into().into_inner(),
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateFieldRequest {
    /// The ID of the resource to update.
    pub resource: u32,
    /// The ID or shortname of the metadata field to set a value on.
    pub field: FieldIdentifier,
    /// The new value to assign to the field.
    /// This can be a comma separated list for fixed list option fields.
    pub value: FieldValue,
}

// Serializes FieldValue::Nodes with an extra `nodevalues = true` entry.
impl Serialize for UpdateFieldRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("resource", &self.resource)?;
        map.serialize_entry("field", &self.field)?;
        map.serialize_entry("value", &self.value.to_wire_string())?;

        if matches!(self.value, FieldValue::Nodes(_)) {
            map.serialize_entry("nodevalues", &true)?;
        }
        map.end()
    }
}

impl UpdateFieldRequest {
    pub fn new(
        resource: u32,
        field: impl Into<FieldIdentifier>,
        value: impl Into<FieldValue>,
    ) -> Self {
        Self {
            resource,
            field: field.into(),
            value: value.into(),
        }
    }
}
