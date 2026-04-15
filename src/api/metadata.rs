use serde::Serialize;

use crate::client::RsClient;
use crate::RsError;

pub struct MetadataApi<'a> {
    client: &'a RsClient,
}

impl<'a> MetadataApi<'a> {
    pub(crate) fn new(client: &'a RsClient) -> Self {
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
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_field_options", reqwest::Method::GET, request)
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
    pub async fn get_node_id(
        &self,
        request: GetNodeIdRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_node_id", reqwest::Method::GET, request)
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
    pub async fn get_nodes(
        &self,
        request: GetNodesRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_nodes", reqwest::Method::GET, request)
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
        request: AddResourceNodesRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("add_resource_nodes", reqwest::Method::POST, request)
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
        request: AddResourceNodesMultiRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("add_resource_nodes_multi", reqwest::Method::POST, request)
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
    pub async fn set_node(
        &self,
        request: SetNodeRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("set_node", reqwest::Method::POST, request)
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
        request: GetResourceTypeFieldsRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_resource_type_fields", reqwest::Method::GET, request)
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
        request: CreateResourceTypeFieldRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("create_resource_type_field", reqwest::Method::POST, request)
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
        request: ToggleActiveStatesForNodesRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("toggle_active_state_for_nodes", reqwest::Method::POST, request)
            .await
    }

    /// Set the value of a metadata field.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateFieldRequest`]
    ///
    /// ## TODO: Returns
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn update_field(
        &self,
        request: UpdateFieldRequest
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("update_field", reqwest::Method::POST, request)
            .await
    }
}

#[derive(Default, Serialize)]
pub struct GetFieldOptionsRequest {
    #[serde(rename = "ref")]
    r#ref: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodeinfo: Option<bool>
}

impl GetFieldOptionsRequest {
    pub fn new(r#ref: u32) -> Self {
        Self {
            r#ref,
            ..Default::default()
        }
    }

    pub fn nodeinfo(mut self, nodeinfo: bool) -> Self {
        self.nodeinfo = Some(nodeinfo);
        self
    }
}

#[derive(Serialize)]
pub struct GetNodeIdRequest {
    value: String,
    resource_type_field: u32
}

impl GetNodeIdRequest {
    pub fn new(
        value: impl Into<String>,
        resource_type_field: u32
    ) -> Self {
        Self {
            value: value.into(), resource_type_field
        }
    }
}

#[derive(Default, Serialize)]
pub struct GetNodesRequest {
    #[serde(rename = "ref")]
    r#ref: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recursive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_count: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by_translated_name: Option<bool>,
}

impl GetNodesRequest {
    pub fn new(r#ref: u32) -> Self {
        Self {
            r#ref,
            ..Default::default()
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

#[derive(Serialize)]
pub struct AddResourceNodesRequest {
    resource: u32,      // Resource ID to add nodes to
    nodestring: String  // List of node IDs to add (comma separated)
}

impl AddResourceNodesRequest {
    pub fn new(
        resource: u32,
        nodestring: impl Into<String>,
    ) -> Self {
        Self { resource, nodestring: nodestring.into() }
    }
}

#[derive(Serialize)]
pub struct AddResourceNodesMultiRequest {
    resourceid: String,
    nodes: String
}

impl AddResourceNodesMultiRequest {
    pub fn new(
        resourceid: impl Into<String>,  // List of resource IDs to add nodes to (comma separated)
        nodes: impl Into<String>,       // List of node IDs to add (comma separated)
    ) -> Self {
        Self { resourceid: resourceid.into(), nodes: nodes.into() }
    }
}

#[derive(Default, Serialize)]
pub struct SetNodeRequest {
    #[serde(rename = "ref")]
    r#ref: u32,
    resource_type_field: u32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returnexisting: Option<bool>,
}

impl SetNodeRequest {
    pub fn new(
        r#ref: u32,
        resource_type_field: u32,
        name: impl Into<String>,
    ) -> Self {
        Self {
            r#ref,
            resource_type_field,
            name: name.into(),
            ..Default::default()            
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

#[derive(Default, Serialize)]
pub struct GetResourceTypeFieldsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    by_resource_types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    find: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    by_types: Option<String>,
}

impl GetResourceTypeFieldsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn by_resource_types(mut self, by_resource_types: impl Into<String>) -> Self {
        self.by_resource_types = Some(by_resource_types.into());
        self
    }

    pub fn find(mut self, find: impl Into<String>) -> Self {
        self.find = Some(find.into());
        self
    }

    pub fn by_types(mut self, by_types: impl Into<String>) -> Self {
        self.by_types = Some(by_types.into());
        self
    }
}

#[derive(Serialize)]
pub struct CreateResourceTypeFieldRequest {
    name: String,
    resource_types: String,
    r#type: String,
}

impl CreateResourceTypeFieldRequest {
    pub fn new(
        name: impl Into<String>,
        resource_types: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        Self { 
            name: name.into(),
            resource_types: resource_types.into(),
            r#type: r#type.into()
        }
    }
}

#[derive(Serialize)]
pub struct ToggleActiveStatesForNodesRequest {
    refs: String, // TODO: API docs say this is a json encoded array
}

impl ToggleActiveStatesForNodesRequest {
    pub fn new(
        refs: impl Into<String>,
    ) -> Self {
        Self { 
            refs: refs.into(),
        }
    }
}

#[derive(Default, Serialize)]
pub struct UpdateFieldRequest {
    resource: u32,
    field: u32,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodevalues: Option<bool>,
}

impl UpdateFieldRequest {
    pub fn new(
        resource: u32,
        field: u32,
        value: impl Into<String>,
    ) -> Self {
        Self { 
            resource: resource.into(),
            field: field.into(),
            value: value.into(),
            ..Default::default()
        }
    }

    pub fn nodevalues(mut self, nodevalues: bool) -> Self {
        self.nodevalues = Some(nodevalues);
        self
    }
}
