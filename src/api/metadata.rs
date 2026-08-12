use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};
use serde_with::json::JsonString;
use serde_with::{DisplayFromStr, PickFirst, serde_as, skip_serializing_none};

use crate::client::{Client, HttpMethod};
use crate::error::Error;

use super::{FieldValue, List, empty_as_none, flexible_bool};

#[derive(Debug)]
pub struct MetadataApi<'a> {
    client: &'a Client,
}

/// ResourceSpace's `ajax_response_ok`/`ajax_response_fail` envelope.
///
/// `create_resource_type_field` wraps its reply in
/// `{"status": ..., "data": ...}` rather than returning the value directly.
/// Kept private: only the unwrapped value is exposed.
#[derive(Debug, Deserialize)]
struct AjaxEnvelope<T> {
    #[allow(dead_code)]
    status: String,
    data: T,
}

/// The `data` payload of a successful `create_resource_type_field` call.
#[derive(Debug, Deserialize)]
struct CreatedField {
    #[serde(rename = "ref")]
    field_id: u32,
}

/// A node — one selectable option of a fixed-list metadata field.
///
/// Returned by [`MetadataApi::get_nodes`] and, when
/// [`GetFieldOptionsRequest::nodeinfo`] is set, by
/// [`MetadataApi::get_field_options`]. The latter omits
/// [`Self::resource_type_field`], since the caller already supplied it.
///
/// Unlike [`ResourceTypeField`], these values arrive as real JSON numbers.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Node {
    /// The node's own ID.
    #[serde(rename = "ref")]
    pub node_id: u32,
    /// The option's value as stored.
    pub name: String,
    /// The option's value in the active language, which may equal
    /// [`Self::name`].
    pub translated_name: Option<String>,
    /// Parent node, for a category tree.
    #[serde(deserialize_with = "empty_as_none")]
    pub parent: Option<u32>,
    /// Position among its siblings.
    pub order_by: u32,
    /// Whether the option is selectable.
    #[serde(deserialize_with = "flexible_bool")]
    pub active: bool,
    /// The field this node belongs to. Absent from
    /// [`MetadataApi::get_field_options`].
    #[serde(deserialize_with = "empty_as_none")]
    pub resource_type_field: Option<u32>,
}

/// The options of a fixed-list field, from
/// [`MetadataApi::get_field_options`].
///
/// Which variant you get is decided by the request:
/// [`GetFieldOptionsRequest::nodeinfo`] returns [`Self::Nodes`], and omitting
/// it returns just the option text.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum FieldOptions {
    /// Full node records, when `nodeinfo` was set.
    Nodes(Vec<Node>),
    /// Option text only.
    Names(Vec<String>),
}

/// A metadata field definition, from
/// [`MetadataApi::get_resource_type_fields`].
///
/// **The JSON types depend on the request.** Called without a filter this
/// endpoint quotes every value (`"ref": "1"`); called with
/// [`GetResourceTypeFieldsRequest::find`] it returns real numbers
/// (`"ref": 93`) for the same columns. Every numeric and boolean field below
/// therefore accepts either form. The long tail of
/// integration and macro columns — `exiftool_field`, `onchange_macro`,
/// `display_condition`, `regexp_filter` and similar — is kept in
/// [`Self::extra`] rather than enumerated.
#[serde_as]
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct ResourceTypeField {
    /// The field's own ID.
    #[serde(rename = "ref")]
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub field_id: u32,
    /// Short name, used wherever a field can be named instead of numbered.
    pub name: String,
    /// Display title.
    #[serde(deserialize_with = "empty_as_none")]
    pub title: Option<String>,
    /// Field type; see ResourceSpace's `FIELD_TYPE_*` constants. `9` is a
    /// dynamic keywords list, `3` a dropdown, `0` a single-line text box.
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub r#type: u8,
    /// Resource types this field applies to, or `None` when it is global.
    #[serde(deserialize_with = "empty_as_none")]
    pub resource_types: Option<String>,
    /// Whether the field applies to every resource type.
    #[serde(deserialize_with = "flexible_bool")]
    pub global: bool,
    /// Position within its tab.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    pub order_by: Option<u32>,
    /// Whether the field is in use.
    #[serde(deserialize_with = "flexible_bool")]
    pub active: bool,
    /// Whether a value must be supplied.
    #[serde(deserialize_with = "flexible_bool")]
    pub required: bool,
    /// Whether the field is shown on the resource view page.
    #[serde(deserialize_with = "flexible_bool")]
    pub display_field: bool,
    /// Whether the field appears in advanced search.
    #[serde(deserialize_with = "flexible_bool")]
    pub advanced_search: bool,
    /// Whether the field appears in simple search.
    #[serde(deserialize_with = "flexible_bool")]
    pub simple_search: bool,
    /// Whether the field's values feed the keyword index.
    #[serde(deserialize_with = "flexible_bool")]
    pub keywords_index: bool,
    /// Whether the field cannot be edited.
    #[serde(deserialize_with = "flexible_bool")]
    pub read_only: bool,
    /// Whether a fixed list renders as a dropdown.
    #[serde(deserialize_with = "flexible_bool")]
    pub display_as_dropdown: bool,
    /// Whether the field spans the full width of the form.
    #[serde(deserialize_with = "flexible_bool")]
    pub full_width: bool,
    /// Whether external (non-logged-in) users can see the field.
    #[serde(deserialize_with = "flexible_bool")]
    pub external_user_access: bool,
    /// Whether the field is hidden from restricted-access users.
    #[serde(deserialize_with = "flexible_bool")]
    pub hide_when_restricted: bool,
    /// Whether the field is hidden on the upload form.
    #[serde(deserialize_with = "flexible_bool")]
    pub hide_when_uploading: bool,
    /// Whether the field is included in CSV exports.
    #[serde(deserialize_with = "flexible_bool")]
    pub include_in_csv_export: bool,
    /// Whether the field is skipped when copying a resource.
    #[serde(deserialize_with = "flexible_bool")]
    pub omit_when_copying: bool,
    /// Tab this field is grouped under.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    pub tab: Option<u32>,
    /// Name of that tab.
    #[serde(deserialize_with = "empty_as_none")]
    pub tab_name: Option<String>,
    /// Help text shown alongside the field.
    #[serde(deserialize_with = "empty_as_none")]
    pub help_text: Option<String>,
    /// Tooltip shown on hover.
    #[serde(deserialize_with = "empty_as_none")]
    pub tooltip_text: Option<String>,
    /// Denormalised column on the resource table backing this field, if any.
    #[serde(deserialize_with = "empty_as_none")]
    pub resource_column: Option<String>,
    /// Everything else ResourceSpace reports for the field — integration and
    /// macro configuration, mostly `null` on a stock instance.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
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
    /// updating. ResourceSpace deduplicates by name on non-tree fields, so
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
    /// When constructing FieldValue from node IDs, the `nodevalues` parameter is
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
    ///
    /// Sent as `resources`; ResourceSpace silently substitutes an empty
    /// string for any parameter it cannot match by name, so a wrong name
    /// here fails quietly rather than erroring.
    #[serde(rename = "resources")]
    pub resource_id: List<u32>,
    /// Comma-separated list of node IDs to add to each resource.
    #[serde(rename = "nodestring")]
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
    /// The ID of an existing node to update, or `None` to create a new one.
    ///
    /// Serialized as the literal string `NULL` when `None`, which is what the
    /// API expects for a create; ResourceSpace converts that back to a real
    /// null before inserting, which also lets it pick the next `order_by`.
    #[serde(rename = "ref", serialize_with = "node_id_or_null")]
    pub node_id: Option<u32>,
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

/// Serializes a node ID as the literal `NULL` when absent.
///
/// `api_set_node` matches on the uppercase string `NULL` to decide between
/// creating and updating, so an omitted or numeric value will not do.
fn node_id_or_null<S: Serializer>(id: &Option<u32>, s: S) -> Result<S::Ok, S::Error> {
    match id {
        Some(id) => s.serialize_str(&id.to_string()),
        None => s.serialize_str("NULL"),
    }
}

impl SetNodeRequest {
    /// Pass `None` as `node_id` to create a node, or `Some(id)` to update one.
    pub fn new(
        node_id: impl Into<Option<u32>>,
        resource_type_field: u32,
        name: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
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
