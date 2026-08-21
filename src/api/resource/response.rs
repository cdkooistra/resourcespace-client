use std::collections::HashMap;

use serde::Deserialize;
use serde_with::{DisplayFromStr, PickFirst, serde_as};

use crate::api::shared::{empty_as_none, flexible_bool};

// Referenced only from doc links below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::ResourceApi;

/// A resource's own properties, from [`ResourceApi::get_resource_data`].
///
/// The denormalised metadata columns `ResourceSpace` keeps on the resource row
/// — `field8`, `field12` and friends, one per field with a
/// `resource_column` — land in [`Self::extra`], since which of them exist
/// depends entirely on how the instance is configured.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Resource {
    /// The resource's own ID.
    #[serde(rename = "ref")]
    pub resource_id: u32,
    /// Resource type ID.
    pub resource_type: u32,
    /// Archive state: `0` live, `-2` pending review, `1` archived, `2` deleted.
    pub archive: i16,
    /// Access level: `0` open, `1` restricted, `2` confidential.
    pub access: u8,
    /// ID of the user who created the resource.
    #[serde(deserialize_with = "empty_as_none")]
    pub created_by: Option<u32>,
    /// When the resource was created, as `YYYY-MM-DD HH:MM:SS`.
    #[serde(deserialize_with = "empty_as_none")]
    pub creation_date: Option<String>,
    /// When the resource was last modified.
    #[serde(deserialize_with = "empty_as_none")]
    pub modified: Option<String>,
    /// Extension of the attached file, empty when there is none.
    #[serde(deserialize_with = "empty_as_none")]
    pub file_extension: Option<String>,
    /// Size of the attached file in bytes.
    #[serde(deserialize_with = "empty_as_none")]
    pub file_size: Option<u64>,
    /// MD5 checksum of the attached file.
    #[serde(deserialize_with = "empty_as_none")]
    pub file_checksum: Option<String>,
    /// Path on disk, usually `None` unless the instance exposes it.
    #[serde(deserialize_with = "empty_as_none")]
    pub file_path: Option<String>,
    /// Whether a preview image exists.
    #[serde(deserialize_with = "flexible_bool")]
    pub has_image: bool,
    /// Whether the resource has no file attached.
    #[serde(deserialize_with = "flexible_bool")]
    pub no_file: bool,
    /// Extension of the generated preview.
    #[serde(deserialize_with = "empty_as_none")]
    pub preview_extension: Option<String>,
    /// Thumbnail width in pixels.
    #[serde(deserialize_with = "empty_as_none")]
    pub thumb_width: Option<u32>,
    /// Thumbnail height in pixels.
    #[serde(deserialize_with = "empty_as_none")]
    pub thumb_height: Option<u32>,
    /// Bytes this resource occupies including all generated sizes.
    #[serde(deserialize_with = "empty_as_none")]
    pub disk_usage: Option<u64>,
    /// Number of times the resource has been viewed.
    #[serde(deserialize_with = "empty_as_none")]
    pub hit_count: Option<u64>,
    /// Title, when the instance maps one onto the resource row.
    #[serde(deserialize_with = "empty_as_none")]
    pub title: Option<String>,
    /// User currently holding an edit lock, if any.
    #[serde(deserialize_with = "empty_as_none")]
    pub lock_user: Option<u32>,
    /// Whether an integrity check has failed for the file.
    #[serde(deserialize_with = "flexible_bool")]
    pub integrity_fail: bool,
    /// Whether the file is currently being transcoded.
    #[serde(deserialize_with = "flexible_bool")]
    pub is_transcoding: bool,
    /// Latitude, when geolocated.
    #[serde(deserialize_with = "empty_as_none")]
    pub geo_lat: Option<f64>,
    /// Longitude, when geolocated.
    #[serde(deserialize_with = "empty_as_none")]
    pub geo_long: Option<f64>,
    /// Everything else on the resource row, including the per-field
    /// `fieldN` metadata columns.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// One metadata field of a resource, with its value, from
/// [`ResourceApi::get_resource_field_data`].
///
/// This is a field *definition* joined to the resource's value for it, so it
/// repeats much of [`crate::api::metadata::ResourceTypeField`] — but with
/// real JSON numbers rather than quoted strings, and with the extra
/// `value`/`fref`/`frequired` columns. Configuration columns not named here
/// are kept in [`Self::extra`].
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct ResourceFieldData {
    /// The field's own ID.
    #[serde(rename = "ref")]
    pub field_id: u32,
    /// Short name of the field.
    pub name: String,
    /// Display title.
    #[serde(deserialize_with = "empty_as_none")]
    pub title: Option<String>,
    /// The resource's value for this field, already translated.
    #[serde(deserialize_with = "empty_as_none")]
    pub value: Option<String>,
    /// Field type; see `ResourceSpace`'s `FIELD_TYPE_*` constants.
    pub r#type: u8,
    /// Position within its tab.
    pub order_by: u32,
    /// Whether a value is required.
    #[serde(deserialize_with = "flexible_bool")]
    pub required: bool,
    /// Whether the field is in use.
    #[serde(deserialize_with = "flexible_bool")]
    pub active: bool,
    /// Whether the field is shown on the resource view page.
    #[serde(deserialize_with = "flexible_bool")]
    pub display_field: bool,
    /// Denormalised column on the resource row backing this field, if any.
    #[serde(deserialize_with = "empty_as_none")]
    pub resource_column: Option<String>,
    /// Resource types this field applies to, as a comma-separated list.
    #[serde(deserialize_with = "empty_as_none")]
    pub resource_types: Option<String>,
    /// Everything else `ResourceSpace` reports for the field.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A resource type, from [`ResourceApi::get_resource_types`].
///
/// Unlike most of this sub-API, these values arrive quoted.
#[serde_as]
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct ResourceType {
    /// The resource type's own ID.
    #[serde(rename = "ref")]
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub resource_type_id: u32,
    /// Display name.
    pub name: String,
    /// Icon name used in the UI.
    #[serde(deserialize_with = "empty_as_none")]
    pub icon: Option<String>,
    /// IDs of the metadata fields belonging to this type.
    #[serde_as(as = "Vec<PickFirst<(_, DisplayFromStr)>>")]
    pub resource_type_fields: Vec<u32>,
    /// Sort order.
    #[serde(deserialize_with = "empty_as_none")]
    pub order_by: Option<u32>,
    /// File extensions permitted for this type.
    #[serde(deserialize_with = "empty_as_none")]
    pub allowed_extensions: Option<String>,
    /// Everything else reported for the type.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// An alternative file attached to a resource, from
/// [`ResourceApi::get_alternative_files`].
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct AlternativeFile {
    /// The alternative file's own ID.
    #[serde(rename = "ref")]
    pub alternative_id: u32,
    /// Display name.
    pub name: String,
    /// Free-text description.
    #[serde(deserialize_with = "empty_as_none")]
    pub description: Option<String>,
    /// Original file name.
    #[serde(deserialize_with = "empty_as_none")]
    pub file_name: Option<String>,
    /// File extension.
    #[serde(deserialize_with = "empty_as_none")]
    pub file_extension: Option<String>,
    /// Size in bytes; `0` when only a database record exists.
    #[serde(deserialize_with = "empty_as_none")]
    pub file_size: Option<u64>,
    /// Caller-defined category for the alternative.
    #[serde(deserialize_with = "empty_as_none")]
    pub alt_type: Option<String>,
    /// When the record was created.
    #[serde(deserialize_with = "empty_as_none")]
    pub creation_date: Option<String>,
}

/// A collection a resource belongs to, from
/// [`ResourceApi::get_resource_collections`].
///
/// Deliberately narrow — this endpoint reports only these three columns, not
/// the full [`crate::api::collection::Collection`] row.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct ResourceCollection {
    /// The collection's own ID.
    #[serde(rename = "ref")]
    pub collection_id: u32,
    /// Collection name.
    #[serde(deserialize_with = "empty_as_none")]
    pub name: Option<String>,
    /// Free-text description.
    #[serde(deserialize_with = "empty_as_none")]
    pub description: Option<String>,
}

/// One entry from a resource's history, returned by both
/// [`ResourceApi::get_resource_log`] and
/// [`ResourceApi::resource_log_last_rows`].
///
/// The two endpoints report overlapping but different columns:
/// `get_resource_log` joins the user's name and adds file-revert details,
/// while `resource_log_last_rows` reports a bare [`Self::user`] ID instead.
/// Fields only one of them provides are `Option`.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct LogEntry {
    /// The log entry's own ID.
    #[serde(rename = "ref")]
    pub log_id: u32,
    /// Resource the entry refers to.
    pub resource: u32,
    /// When it happened, as `YYYY-MM-DD HH:MM:SS`.
    #[serde(deserialize_with = "empty_as_none")]
    pub date: Option<String>,
    /// Single-character log code, e.g. `"c"` created, `"u"` uploaded.
    #[serde(deserialize_with = "empty_as_none")]
    pub r#type: Option<String>,
    /// Free-text note.
    #[serde(deserialize_with = "empty_as_none")]
    pub notes: Option<String>,
    /// Metadata field this entry concerns, when it was a field edit.
    #[serde(deserialize_with = "empty_as_none")]
    pub field: Option<u32>,
    /// The value before the change.
    #[serde(deserialize_with = "empty_as_none")]
    pub previous_value: Option<String>,
    /// Rendered difference for a field edit.
    #[serde(deserialize_with = "empty_as_none")]
    pub diff: Option<String>,
    /// Acting user's ID. [`ResourceApi::resource_log_last_rows`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub user: Option<u32>,
    /// Acting user's login name. [`ResourceApi::get_resource_log`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub username: Option<String>,
    /// Acting user's display name. [`ResourceApi::get_resource_log`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub fullname: Option<String>,
    /// Whether the logged file change can be reverted.
    /// [`ResourceApi::get_resource_log`] only.
    #[serde(deserialize_with = "flexible_bool")]
    pub revert_enabled: bool,
    /// Everything else reported for the entry.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// One generated size of a resource's image, from
/// [`ResourceApi::get_resource_all_image_sizes`].
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct ImageSize {
    /// Size identifier, e.g. `"original"`, `"thm"`, `"scr"`.
    pub size_code: String,
    /// Direct download URL. Carries a temporary `access_key` when the
    /// instance hides real file paths.
    pub url: String,
    /// Width in pixels.
    #[serde(deserialize_with = "empty_as_none")]
    pub width: Option<u32>,
    /// Height in pixels.
    #[serde(deserialize_with = "empty_as_none")]
    pub height: Option<u32>,
    /// File extension for this size.
    #[serde(deserialize_with = "empty_as_none")]
    pub extension: Option<String>,
    /// Human-readable size. Contains an HTML non-breaking space, as
    /// `ResourceSpace` formats it for display rather than reporting bytes.
    #[serde(deserialize_with = "empty_as_none")]
    pub filesize: Option<String>,
}

/// The `data` payload of `resource_file_readonly`.
#[derive(Debug, Deserialize)]
pub(crate) struct ReadonlyData {
    pub(crate) readonly: bool,
}
