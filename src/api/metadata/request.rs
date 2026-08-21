use serde::{Serialize, Serializer};
use serde_with::json::JsonString;
use serde_with::{serde_as, skip_serializing_none};

use super::shared::node_id_or_null;
use crate::api::shared::{FieldValue, List};

// Referenced only from doc links below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::MetadataApi;

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

/// Parameters for [`MetadataApi::get_field_options`].
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

    #[must_use]
    pub fn nodeinfo(mut self, nodeinfo: bool) -> Self {
        self.nodeinfo = Some(nodeinfo);
        self
    }
}

/// Parameters for [`MetadataApi::get_node_id`].
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

/// Parameters for [`MetadataApi::get_nodes`].
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
    #[must_use]
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

    #[must_use]
    pub fn parent(mut self, parent: u32) -> Self {
        self.parent = Some(parent);
        self
    }

    #[must_use]
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = Some(recursive);
        self
    }

    #[must_use]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[must_use]
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = Some(rows);
        self
    }

    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[must_use]
    pub fn use_count(mut self, use_count: bool) -> Self {
        self.use_count = Some(use_count);
        self
    }

    #[must_use]
    pub fn order_by_translated_name(mut self, order_by_translated_name: bool) -> Self {
        self.order_by_translated_name = Some(order_by_translated_name);
        self
    }
}

/// Parameters for [`MetadataApi::add_resource_nodes`].
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

/// Parameters for [`MetadataApi::add_resource_nodes_multi`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddResourceNodesMultiRequest {
    /// Comma-separated list of resource IDs to add nodes to.
    ///
    /// Sent as `resources`; `ResourceSpace` silently substitutes an empty
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

/// Parameters for [`MetadataApi::set_node`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SetNodeRequest {
    /// The ID of an existing node to update, or `None` to create a new one.
    ///
    /// Serialized as the literal string `NULL` when `None`, which is what the
    /// API expects for a create; `ResourceSpace` converts that back to a real
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
    #[must_use]
    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    #[must_use]
    pub fn order_by(mut self, order_by: u32) -> Self {
        self.order_by = Some(order_by);
        self
    }

    #[must_use]
    pub fn returnexisting(mut self, returnexisting: bool) -> Self {
        self.returnexisting = Some(returnexisting);
        self
    }
}

/// Parameters for [`MetadataApi::get_resource_type_fields`].
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn resource_type_ids(mut self, resource_type_ids: impl Into<List<u32>>) -> Self {
        self.resource_type_ids = Some(resource_type_ids.into());
        self
    }

    #[must_use]
    pub fn find(mut self, find: impl Into<String>) -> Self {
        self.find = Some(find.into());
        self
    }

    #[must_use]
    pub fn field_type_ids(mut self, field_type_ids: impl Into<List<u32>>) -> Self {
        self.field_type_ids = Some(field_type_ids.into());
        self
    }
}

/// Parameters for [`MetadataApi::create_resource_type_field`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateResourceTypeFieldRequest {
    /// The name of the new metadata field.
    pub name: String,
    /// Comma-separated list of resource type IDs this field should apply to.
    #[serde(rename = "resource_types")]
    pub resource_type_ids: List<u32>,
    /// The field type, for values see the `FIELD_TYPE`_* constants.
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

/// Parameters for [`MetadataApi::toggle_active_state_for_nodes`].
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

/// Parameters for [`MetadataApi::update_field`].
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
