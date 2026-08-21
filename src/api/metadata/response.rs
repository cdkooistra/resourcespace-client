use std::collections::HashMap;

use serde::Deserialize;
use serde_with::{DisplayFromStr, PickFirst, serde_as};

use crate::api::shared::{empty_as_none, flexible_bool};

// Referenced only from doc links below; the imports keep them resolvable.
#[allow(unused_imports)]
use super::{GetFieldOptions, GetResourceTypeFields, MetadataApi};

/// A node — one selectable option of a fixed-list metadata field.
///
/// Returned by [`MetadataApi::get_nodes`] and, when
/// [`GetFieldOptions::nodeinfo`] is set, by
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
/// [`GetFieldOptions::nodeinfo`] returns [`Self::Nodes`], and omitting
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
/// [`GetResourceTypeFields::find`] it returns real numbers
/// (`"ref": 93`) for the same columns. Every numeric and boolean field below
/// therefore accepts either form. The long tail of
/// integration and macro columns — `exiftool_field`, `onchange_macro`,
/// `display_condition`, `regexp_filter` and similar — is kept in
/// [`Self::extra`] rather than enumerated.
#[serde_as]
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
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
    /// Field type; see `ResourceSpace`'s `FIELD_TYPE_*` constants. `9` is a
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
    /// Everything else `ResourceSpace` reports for the field — integration and
    /// macro configuration, mostly `null` on a stock instance.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// The `data` payload of a successful `create_resource_type_field` call.
#[derive(Debug, Deserialize)]
pub(crate) struct CreatedField {
    #[serde(rename = "ref")]
    pub(crate) field_id: u32,
}
