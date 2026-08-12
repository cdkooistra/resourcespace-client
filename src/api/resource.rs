use serde::{Deserialize, Serialize, Serializer};
use serde_with::json::JsonString;
use serde_with::{DisplayFromStr, PickFirst, Same, SerializeAs, serde_as, skip_serializing_none};
use std::collections::HashMap;

use crate::client::{Client, HttpMethod};
use crate::error::Error;

use super::{
    FieldValue, List, SortOrder, bool_as_u8, empty_as_none, flexible_bool, opt_bool_as_u8,
};

#[derive(Debug)]
pub struct ResourceApi<'a> {
    client: &'a Client,
}

/// ResourceSpace's `ajax_response_ok`/`ajax_response_fail` envelope.
///
/// `resource_file_readonly` wraps its reply in
/// `{"status": ..., "data": ...}`. Kept private; only the inner value is
/// exposed.
#[derive(Debug, Deserialize)]
struct AjaxEnvelope<T> {
    #[allow(dead_code)]
    status: String,
    data: T,
}

/// The `data` payload of `resource_file_readonly`.
#[derive(Debug, Deserialize)]
struct ReadonlyData {
    readonly: bool,
}

/// A resource's own properties, from [`ResourceApi::get_resource_data`].
///
/// The denormalised metadata columns ResourceSpace keeps on the resource row
/// — `field8`, `field12` and friends, one per field with a
/// `resource_column` — land in [`Self::extra`], since which of them exist
/// depends entirely on how the instance is configured.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
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
    /// Field type; see ResourceSpace's `FIELD_TYPE_*` constants.
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
    /// Everything else ResourceSpace reports for the field.
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
    /// ResourceSpace formats it for display rather than reporting bytes.
    #[serde(deserialize_with = "empty_as_none")]
    pub filesize: Option<String>,
}

/// Sub-API for resource endpoints.
impl<'a> ResourceApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Add a new alternative file to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`AddAlternativeFileRequest`]
    ///
    /// ## Returns
    ///
    /// The ID of the new alternative file.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller lacks edit access to the
    /// resource and the `A` permission, or if the file extension is banned.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::AddAlternativeFileRequest;
    /// let alt_id = client.resource()
    ///     .add_alternative_file(AddAlternativeFileRequest::new(1234, "Print master"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_alternative_file(
        &self,
        request: AddAlternativeFileRequest,
    ) -> Result<u32, Error> {
        self.client
            .send_request("add_alternative_file", HttpMethod::Post, request)
            .await
    }

    /// Copy a resource. Note that attached files are not copied — this is a metadata
    /// and property copy only.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CopyResourceRequest`]
    ///
    /// ## Returns
    ///
    /// The ID of the newly created resource. Files are not copied — this is a
    /// metadata and property copy only.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the source resource does not exist
    /// or the caller cannot create resources.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::CopyResourceRequest;
    /// let new_id = client.resource()
    ///     .copy_resource(CopyResourceRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy_resource(&self, request: CopyResourceRequest) -> Result<u32, Error> {
        self.client
            .send_request("copy_resource", HttpMethod::Post, request)
            .await
    }

    /// Create a new resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CreateResourceRequest`]
    ///
    /// ## Returns
    ///
    /// The ID of the newly created resource.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller holds neither the
    /// `c` nor `d` permission, is barred from this resource type, requested
    /// an archive state they may not set, or supplied a `url` that
    /// [`Self::validate_upload_url`] would reject.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use std::collections::HashMap;
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder().base_url("https://example.com").user_key("user", "key").build().await?;
    /// use resourcespace_client::api::FieldValue;
    /// use resourcespace_client::api::resource::CreateResourceRequest;
    ///
    /// client.resource().create_resource(
    ///     CreateResourceRequest::new(1)
    ///         .metadata(HashMap::from([
    ///             (90u32, FieldValue::from("A plain text description")),       // Text field
    ///             (91u32, FieldValue::from(["Doe, John", "Smith, Jane"])),     // Keywords, auto-quoted
    ///             (92u32, FieldValue::from([1u32, 2, 3])),                     // Node IDs
    ///         ]))
    /// ).await?;
    /// # Ok(()) }
    /// ```
    pub async fn create_resource(&self, request: CreateResourceRequest) -> Result<u32, Error> {
        self.client
            .send_request("create_resource", HttpMethod::Post, request)
            .await
    }

    /// Deletes an alternative file.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteAlternativeFile`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller lacks edit access to the
    /// resource and the `A` permission.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::DeleteAlternativeFile;
    /// client.resource()
    ///     .delete_alternative_file(DeleteAlternativeFile::new(1234, 7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_alternative_file(
        &self,
        request: DeleteAlternativeFile,
    ) -> Result<bool, Error> {
        self.client
            .send_request("delete_alternative_file", HttpMethod::Post, request)
            .await
    }

    /// Delete a comment.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteCommentRequest`]
    ///
    /// ## Returns
    ///
    /// The success of the operation (true/false).
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if resource commenting is disabled
    /// system-wide (`$comments_resource_enable`), or if the comment does not
    /// exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::resource::DeleteCommentRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .resource()
    ///     .delete_comment(DeleteCommentRequest::new(12))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_comment(&self, request: DeleteCommentRequest) -> Result<bool, Error> {
        self.client
            .send_request("delete_comment", HttpMethod::Post, request)
            .await
    }

    /// Delete a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteResourceRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource does not exist or the
    /// caller lacks permission to delete it.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::DeleteResourceRequest;
    /// client.resource()
    ///     .delete_resource(DeleteResourceRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_resource(&self, request: DeleteResourceRequest) -> Result<bool, Error> {
        self.client
            .send_request("delete_resource", HttpMethod::Post, request)
            .await
    }

    /// Returns a list of alternative files for a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetAlternativeFilesRequest`]
    ///
    /// ## Returns
    ///
    /// Every alternative file on the resource, or an empty list when it has none.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource ID is invalid, or if
    /// the caller's access to it is restricted and
    /// `$alt_files_visible_when_restricted` is off.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetAlternativeFilesRequest;
    /// let files = client.resource()
    ///     .get_alternative_files(GetAlternativeFilesRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_alternative_files(
        &self,
        request: GetAlternativeFilesRequest,
    ) -> Result<Vec<AlternativeFile>, Error> {
        self.client
            .send_request("get_alternative_files", HttpMethod::Get, request)
            .await
    }

    /// Check if the current user has edit access to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetEditAccessRequest`]
    ///
    /// ## Returns
    ///
    /// `true` when the caller may edit the resource.
    ///
    /// **A negative answer is not currently representable.** RS returns bare
    /// `false` for "no", which becomes [`Error::OperationFailed`] before it
    /// reaches here, so `Ok` is always `true`.
    ///
    /// ## Errors
    ///
    /// Treat [`Error::OperationFailed`] as "no edit access" — see above.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetEditAccessRequest;
    /// let can_edit = client.resource()
    ///     .get_edit_access(GetEditAccessRequest::new(1234))
    ///     .await
    ///     .unwrap_or(false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_edit_access(&self, request: GetEditAccessRequest) -> Result<bool, Error> {
        self.client
            .send_request("get_edit_access", HttpMethod::Get, request)
            .await
    }

    /// Returns a list of resources related to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetRelatedResourcesRequest`]
    ///
    /// ## Returns
    ///
    /// IDs of the resources related to this one.
    ///
    /// ## Errors
    ///
    /// Returns an empty list, not an error, when the resource ID is invalid,
    /// when related resources are disabled system-wide, or when the caller's
    /// access to the resource is confidential.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetRelatedResourcesRequest;
    /// let related = client.resource()
    ///     .get_related_resources(GetRelatedResourcesRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_related_resources(
        &self,
        request: GetRelatedResourcesRequest,
    ) -> Result<Vec<u32>, Error> {
        self.client
            .send_request("get_related_resources", HttpMethod::Get, request)
            .await
    }

    /// Retrieves the access level for the current user for a specified resource.
    ///
    /// Returns 0 (full), 1 (restricted), 2 (confidential), 99 (not found), or false (invalid ID).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceAccessRequest`]
    ///
    /// ## Returns
    ///
    /// `0` open, `1` restricted, `2` confidential, `99` not found.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource ID is not numeric.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetResourceAccessRequest;
    /// let access = client.resource()
    ///     .get_resource_access(GetResourceAccessRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_access(
        &self,
        request: GetResourceAccessRequest,
    ) -> Result<u8, Error> {
        self.client
            .send_request("get_resource_access", HttpMethod::Get, request)
            .await
    }

    /// Get all preview sizes available for a specific resource.
    ///
    /// Multi-page resources will include each page size in the response.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceAllImageSizesRequest`]
    ///
    /// ## Returns
    ///
    /// One entry per generated size, including `original`. URLs carry a
    /// temporary `access_key` when the instance hides real file paths.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Deserialize`] if the response does not match
    /// [`ImageSize`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetResourceAllImageSizesRequest;
    /// let sizes = client.resource()
    ///     .get_resource_all_image_sizes(GetResourceAllImageSizesRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_all_image_sizes(
        &self,
        request: GetResourceAllImageSizesRequest,
    ) -> Result<Vec<ImageSize>, Error> {
        self.client
            .send_request("get_resource_all_image_sizes", HttpMethod::Get, request)
            .await
    }

    /// Retrieve comments for a resource.
    ///
    /// Available from **RS 11.0+**.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceCommentsRequest`]
    ///
    /// ## Returns
    ///
    /// The resource's comments, threaded by default. Left as
    /// [`serde_json::Value`]: commenting is disabled on the instance this was
    /// verified against, so only the empty shape could be observed and the
    /// populated one would be a guess.
    ///
    /// ## Errors
    ///
    /// Returns an empty list rather than an error when resource commenting is
    /// disabled system-wide (`$comments_resource_enable`).
    ///
    pub async fn get_resource_comments(
        &self,
        request: GetResourceCommentsRequest,
    ) -> Result<Vec<serde_json::Value>, Error> {
        self.client
            .send_request("get_resource_comments", HttpMethod::Get, request)
            .await
    }

    /// Returns the top level property data for a resource, including truncated summary metadata.
    ///
    /// For full non-truncated metadata use [`get_resource_field_data`](Self::get_resource_field_data).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceDataRequest`]
    ///
    /// ## Returns
    ///
    /// The resource row. Which columns exist beyond those named on
    /// [`Resource`] depends on the instance's field configuration; they are
    /// available via [`Resource::extra`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource does not exist or the
    /// caller's access to it is confidential.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetResourceDataRequest;
    /// let resource = client.resource()
    ///     .get_resource_data(GetResourceDataRequest::new(1234))
    ///     .await?;
    /// println!("{:?}", resource.file_extension);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_data(
        &self,
        request: GetResourceDataRequest,
    ) -> Result<Resource, Error> {
        self.client
            .send_request("get_resource_data", HttpMethod::Get, request)
            .await
    }

    /// Return all field data for a given resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceFieldDataRequest`]
    ///
    /// ## Returns
    ///
    /// Every metadata field visible to the caller, each with the resource's
    /// value for it.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Deserialize`] if the response does not match
    /// [`ResourceFieldData`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetResourceFieldDataRequest;
    /// for field in client.resource()
    ///     .get_resource_field_data(GetResourceFieldDataRequest::new(1234))
    ///     .await? {
    ///     println!("{} = {:?}", field.name, field.value);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_field_data(
        &self,
        request: GetResourceFieldDataRequest,
    ) -> Result<Vec<ResourceFieldData>, Error> {
        self.client
            .send_request("get_resource_field_data", HttpMethod::Get, request)
            .await
    }

    /// Returns the full log for a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceLogRequest`]
    ///
    /// ## Returns
    ///
    /// The resource's history, newest first.
    ///
    /// ## Errors
    ///
    /// Returns an empty list rather than an error when the resource has no log.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetResourceLogRequest;
    /// let log = client.resource()
    ///     .get_resource_log(GetResourceLogRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_log(
        &self,
        request: GetResourceLogRequest,
    ) -> Result<Vec<LogEntry>, Error> {
        self.client
            .send_request("get_resource_log", HttpMethod::Get, request)
            .await
    }

    /// Returns a temporary URL for downloading a resource file.
    ///
    /// The URL is valid for 24 hours by default (configurable via `$api_resource_path_expiry_hours`).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourcePathRequest`]
    ///
    /// ## Returns
    ///
    /// A download URL for the requested size. Carries a temporary `access_key`
    /// when the instance hides real file paths, so treat it as a secret.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource does not exist.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetResourcePathRequest;
    /// let url = client.resource()
    ///     .get_resource_path(GetResourcePathRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_path(
        &self,
        request: GetResourcePathRequest,
    ) -> Result<String, Error> {
        self.client
            .send_request("get_resource_path", HttpMethod::Get, request)
            .await
    }

    /// Returns all configured resource types available to the current user.
    ///
    /// From RS v10.2, the associated resource type field IDs are also returned.
    ///
    /// ## Returns
    ///
    /// Every resource type, each with the IDs of its metadata fields.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Deserialize`] if the response does not match
    /// [`ResourceType`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let types = client.resource().get_resource_types().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_types(&self) -> Result<Vec<ResourceType>, Error> {
        self.client
            .send_request("get_resource_types", HttpMethod::Get, ())
            .await
    }

    /// Update resource properties (not metadata fields).
    ///
    /// Valid columns: `resource_type`, `creation_date`, `rating`, `user_rating`, `archive`,
    /// `access`, `created_by`, `mapzoom`, `modified`, `geo_lat`, `geo_long`.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`PutResourceDataRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource does not exist or the
    /// caller cannot edit it.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::PutResourceDataRequest;
    /// use std::collections::HashMap;
    /// client.resource()
    ///     .put_resource_data(PutResourceDataRequest::new(
    ///         1234,
    ///         HashMap::from([("archive".to_string(), "0".to_string())]),
    ///     ))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn put_resource_data(&self, request: PutResourceDataRequest) -> Result<bool, Error> {
        self.client
            .send_request("put_resource_data", HttpMethod::Post, request)
            .await
    }

    /// Relate all the provided resources with each other.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`RelateAllResourcesRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] when related resources are disabled
    /// system-wide.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::RelateAllResourcesRequest;
    /// client.resource()
    ///     .relate_all_resources(RelateAllResourcesRequest::new([1234, 1235]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn relate_all_resources(
        &self,
        request: RelateAllResourcesRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("relate_all_resources", HttpMethod::Post, request)
            .await
    }

    /// Replaces the primary resource file for a given resource.
    ///
    /// The file location must be accessible without authentication from the RS server —
    /// either a local path or a publicly accessible URL.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ReplaceResourceFileRequest`]
    ///
    /// ## Returns
    ///
    /// An object with `Status` (`SUCCESS`/`FAILED`) and `Message`. Left as
    /// [`serde_json::Value`]: replacing a file needs a path on the server or
    /// a whitelisted URL, neither of which was reachable from the dev
    /// instance, so only the documented shape is known and typing it would
    /// be a guess.
    ///
    /// Note this reports failure *inside* a 200 response, so a `FAILED`
    /// status does not produce an [`Error`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller cannot edit the
    /// resource; a rejected file is reported in the returned value instead.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::resource::ReplaceResourceFileRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let result = client
    ///     .resource()
    ///     .replace_resource_file(ReplaceResourceFileRequest::new(
    ///         1234,
    ///         "https://example.com/replacement.jpg",
    ///     ))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn replace_resource_file(
        &self,
        request: ReplaceResourceFileRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("replace_resource_file", HttpMethod::Post, request)
            .await
    }

    /// Check if a resource file is read-only due to filestore template threshold configuration.
    ///
    /// Available from **RS 10.5+**.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ResourceFileReadonlyRequest`]
    ///
    /// ## Returns
    ///
    /// Whether the resource's file is read-only, unwrapped from the
    /// `{"status": ..., "data": {"readonly": ...}}` envelope this endpoint
    /// returns.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Http`] with status 400 if the resource ID is not a
    /// positive integer; the envelope in `body` carries the message.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::ResourceFileReadonlyRequest;
    /// let readonly = client.resource()
    ///     .resource_file_readonly(ResourceFileReadonlyRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resource_file_readonly(
        &self,
        request: ResourceFileReadonlyRequest,
    ) -> Result<bool, Error> {
        let envelope: AjaxEnvelope<ReadonlyData> = self
            .client
            .send_request("resource_file_readonly", HttpMethod::Get, request)
            .await?;
        Ok(envelope.data.readonly)
    }

    /// Retrieve recent entries from the resource log
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ResourceLogLastRowsRequest`]
    ///
    /// ## Returns
    ///
    /// Recent log entries across all resources. Reports [`LogEntry::user`]
    /// rather than the joined username that [`Self::get_resource_log`] gives.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Deserialize`] if the response does not match
    /// [`LogEntry`].
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::ResourceLogLastRowsRequest;
    /// let rows = client.resource()
    ///     .resource_log_last_rows(ResourceLogLastRowsRequest::new().days(1))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resource_log_last_rows(
        &self,
        request: ResourceLogLastRowsRequest,
    ) -> Result<Vec<LogEntry>, Error> {
        self.client
            .send_request("resource_log_last_rows", HttpMethod::Get, request)
            .await
    }

    /// Uploads a file from a local server path to an existing resource.
    ///
    /// The path must be local to the RS server. See `$valid_upload_paths` in RS config
    /// if using a custom upload path.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UploadFileRequest`]
    ///
    /// ## Returns
    ///
    /// The resource ID the file was attached to.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller is over quota, cannot
    /// edit the resource, or the path is not under a permitted upload
    /// directory.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::UploadFileRequest;
    /// client.resource()
    ///     .upload_file(UploadFileRequest::new(1234).file_path("/var/tmp/a.jpg"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_file(&self, request: UploadFileRequest) -> Result<u32, Error> {
        self.client
            .send_request("upload_file", HttpMethod::Post, request)
            .await
    }

    /// Uploads a remote file to an existing resource by URL.
    ///
    /// RS fetches the file server-side. The URL hostname must be allowed via
    /// `$api_upload_urls` in RS config.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UploadFileByUrlRequest`]
    ///
    /// ## Returns
    ///
    /// The resource ID the file was attached to.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the URL is not permitted — see
    /// [`Self::validate_upload_url`] — or the caller cannot edit the resource.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::UploadFileByUrlRequest;
    /// client.resource()
    ///     .upload_file_by_url(UploadFileByUrlRequest::new(1234).url("https://example.com/a.jpg"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_file_by_url(&self, request: UploadFileByUrlRequest) -> Result<u32, Error> {
        self.client
            .send_request("upload_file_by_url", HttpMethod::Post, request)
            .await
    }

    /// Uploads files using HTTP multipart to an existing resource, replacing any file that is already attached.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UploadMultipartRequest`]
    /// * `source` - Source accepts a file path or a stream as per [`UploadSource`].
    ///
    /// ## Returns
    ///
    /// Nothing. A successful upload is an HTTP 204 with no body, so there is
    /// no value to return.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Http`] for any non-204 response — 413 when the file
    /// exceeds the server limit, 400 for a malformed request, 500 for a
    /// server-side failure. The response body is carried in `body`.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::resource::{UploadMultipartRequest, UploadSource};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .resource()
    ///     .upload_multipart(
    ///         UploadMultipartRequest::new(1234, false, false),
    ///         UploadSource::from_file("photo.jpg"),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_multipart(
        &self,
        request: UploadMultipartRequest,
        source: impl Into<UploadSource>,
    ) -> Result<(), Error> {
        self.client
            .send_multipart_request("upload_multipart", request, source.into())
            .await
    }

    /// Add or remove resource relationships.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateRelatedResourceRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] when related resources are disabled
    /// system-wide.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::UpdateRelatedResourceRequest;
    /// client.resource()
    ///     .update_related_resource(
    ///         UpdateRelatedResourceRequest::new(1234, [1235]).add(true),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_related_resource(
        &self,
        request: UpdateRelatedResourceRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("update_related_resource", HttpMethod::Post, request)
            .await
    }

    /// Change the resource type of a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateResourceTypeRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource does not exist or the
    /// caller cannot edit it.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::UpdateResourceTypeRequest;
    /// client.resource()
    ///     .update_resource_type(UpdateResourceTypeRequest::new(1234, 2))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_resource_type(
        &self,
        request: UpdateResourceTypeRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("update_resource_type", HttpMethod::Post, request)
            .await
    }

    /// Retrieves a list of collections that a resource is used in for the specified resource reference.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceCollectionsRequest`]
    ///
    /// ## Returns
    ///
    /// The collections the resource belongs to, with only ID, name and
    /// description — not the full collection row.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the resource ID is not numeric.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::GetResourceCollectionsRequest;
    /// let collections = client.resource()
    ///     .get_resource_collections(GetResourceCollectionsRequest::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_collections(
        &self,
        request: GetResourceCollectionsRequest,
    ) -> Result<Vec<ResourceCollection>, Error> {
        self.client
            .send_request("get_resource_collections", HttpMethod::Get, request)
            .await
    }

    /// Validate a URL supplied in the create_resource or upload_file_by_url API calls.
    ///
    /// Requires the URL hostname to be added in the config option $api_upload_urls, for example:
    /// `$api_upload_urls = array('resourcespace.com', 'localhost');`
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ValidateUploadUrlRequest`]
    ///
    /// ## Returns
    ///
    /// `true` when the URL may be used for an upload.
    ///
    /// **A negative answer is not currently representable.** RS returns bare
    /// `false` for a URL that is not permitted, and that becomes
    /// [`Error::OperationFailed`] before it reaches here.
    ///
    /// ## Errors
    ///
    /// Treat [`Error::OperationFailed`] as "not permitted". A URL is permitted
    /// only if its host appears in the instance's `$api_upload_urls`, or if
    /// that setting is absent entirely.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// use resourcespace_client::api::resource::ValidateUploadUrlRequest;
    /// let allowed = client.resource()
    ///     .validate_upload_url(ValidateUploadUrlRequest::new("https://example.com/a.jpg"))
    ///     .await
    ///     .unwrap_or(false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn validate_upload_url(
        &self,
        request: ValidateUploadUrlRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("validate_upload_url", HttpMethod::Get, request)
            .await
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddAlternativeFileRequest {
    /// The ID of the resource to attach the alternative file to.
    pub resource: u32,
    /// Display name for the alternative file.
    pub name: String,
    /// Optional description of the alternative file.
    pub description: Option<String>,
    /// Original file name of the alternative file.
    pub file_name: Option<String>,
    /// File extension of the alternative file (e.g. `"pdf"`).
    pub file_extension: Option<String>,
    /// Size of the file in bytes.
    pub file_size: Option<u64>,
    /// Alternative file type identifier used to categorise the file.
    pub alt_type: Option<String>,
    /// Local server path or publicly accessible URL of the file to attach.
    pub file: Option<String>,
}

impl AddAlternativeFileRequest {
    pub fn new(resource: u32, name: impl Into<String>) -> Self {
        Self {
            resource,
            name: name.into(),
            description: None,
            file_name: None,
            file_extension: None,
            file_size: None,
            alt_type: None,
            file: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub fn file_extension(mut self, file_extension: impl Into<String>) -> Self {
        self.file_extension = Some(file_extension.into());
        self
    }

    pub fn file_size(mut self, file_size: u64) -> Self {
        self.file_size = Some(file_size);
        self
    }

    pub fn alt_type(mut self, alt_type: impl Into<String>) -> Self {
        self.alt_type = Some(alt_type.into());
        self
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CopyResourceRequest {
    /// The ID of the resource to copy.
    #[serde(rename = "from")]
    pub resource_id: u32,
    /// Resource type ID to assign to the copy; defaults to the source resource type if omitted.
    pub resource_type: Option<u32>,
}

impl CopyResourceRequest {
    pub fn new(resource_id: u32) -> Self {
        Self {
            resource_id,
            resource_type: None,
        }
    }

    pub fn resource_type(mut self, resource_type: u32) -> Self {
        self.resource_type = Some(resource_type);
        self
    }
}

#[non_exhaustive]
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateResourceRequest {
    /// The resource type ID for the new resource.
    pub resource_type: u32,
    /// Initial archive state: 0 = live, 1 = archived, 2 = deleted.
    pub archive: Option<i16>,
    /// URL of a remote file to attach to the resource at creation time.
    pub url: Option<String>,
    /// If true, skips reading EXIF data from the attached file.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_exif: Option<bool>,
    /// If true, reverts to the original file rather than the processed one.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub revert: Option<bool>,
    /// If true, automatically rotates the image based on EXIF orientation.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub autorotate: Option<bool>,
    /// JSON-encoded metadata fields to set on the resource at creation time.
    #[serde_as(as = "Option<JsonString<HashMap<Same, FieldValueAsString>>>")]
    pub metadata: Option<HashMap<u32, FieldValue>>,
}

struct FieldValueAsString;

impl SerializeAs<FieldValue> for FieldValueAsString {
    fn serialize_as<S: Serializer>(val: &FieldValue, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&val.to_wire_string())
    }
}

impl CreateResourceRequest {
    pub fn new(resource_type: u32) -> Self {
        Self {
            resource_type,
            archive: None,
            url: None,
            no_exif: None,
            revert: None,
            autorotate: None,
            metadata: None,
        }
    }

    pub fn archive(mut self, archive: i16) -> Self {
        self.archive = Some(archive);
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn no_exif(mut self, no_exif: bool) -> Self {
        self.no_exif = Some(no_exif);
        self
    }

    pub fn revert(mut self, revert: bool) -> Self {
        self.revert = Some(revert);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<u32, FieldValue>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteAlternativeFile {
    /// The ID of the resource the alternative file belongs to.
    pub resource: u32,
    /// The ID of the alternative file to delete.
    #[serde(rename = "ref")]
    pub alternative_file_id: u32,
}

impl DeleteAlternativeFile {
    pub fn new(resource: u32, alternative_file_id: u32) -> Self {
        Self {
            resource,
            alternative_file_id,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteResourceRequest {
    /// The ID of the resource to delete.
    pub resource: u32,
}

impl DeleteResourceRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetAlternativeFilesRequest {
    /// The ID of the resource whose alternative files should be returned.
    pub resource: u32,
    /// Field name to order the alternative files by.
    #[serde(rename = "order_by")]
    pub orderby: Option<String>,
    /// Sort direction for the results.
    pub sort: Option<SortOrder>,
    /// Filter results to only alternative files of this type.
    pub r#type: Option<String>,
}

impl GetAlternativeFilesRequest {
    pub fn new(resource: u32) -> Self {
        Self {
            resource,
            orderby: None,
            sort: None,
            r#type: None,
        }
    }

    pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
        self.orderby = Some(orderby.into());
        self
    }

    pub fn sort(mut self, sort: SortOrder) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetEditAccessRequest {
    /// The ID of the resource to check edit access for.
    pub resource: u32,
}

impl GetEditAccessRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetRelatedResourcesRequest {
    /// The ID of the resource whose related resources should be returned.
    #[serde(rename = "ref")]
    pub resource_id: u32,
}

impl GetRelatedResourcesRequest {
    pub fn new(resource_id: u32) -> Self {
        Self { resource_id }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceAccessRequest {
    /// The ID of the resource to retrieve the access level for.
    pub resource: u32,
}

impl GetResourceAccessRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceAllImageSizesRequest {
    /// The ID of the resource to retrieve available preview sizes for.
    pub resource: u32,
}

impl GetResourceAllImageSizesRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteCommentRequest {
    /// The ID of the comment to delete.
    pub comment_ref: u32,
}

impl DeleteCommentRequest {
    pub fn new(comment_ref: u32) -> Self {
        Self { comment_ref }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceCommentsRequest {
    /// The ID of the resource to retrieve comments for.
    #[serde(rename = "resource_ref")]
    pub resource_id: u32,
    /// If true, returns comments as a flat list rather than a threaded tree.
    pub flat_view: Option<bool>,
}

impl GetResourceCommentsRequest {
    pub fn new(resource_id: u32) -> Self {
        Self {
            resource_id,
            flat_view: None,
        }
    }

    pub fn flat_view(mut self, flat_view: bool) -> Self {
        self.flat_view = Some(flat_view);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceDataRequest {
    /// The ID of the resource to retrieve top-level property data for.
    pub resource: u32,
}

impl GetResourceDataRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceFieldDataRequest {
    /// The ID of the resource to retrieve full metadata field data for.
    pub resource: u32,
}

impl GetResourceFieldDataRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceLogRequest {
    /// The ID of the resource whose log entries should be returned.
    pub resource: u32,
    /// Maximum number of log rows to return.
    pub fetchrows: Option<u32>,
}

impl GetResourceLogRequest {
    pub fn new(resource: u32) -> Self {
        Self {
            resource,
            fetchrows: None,
        }
    }

    pub fn fetchrows(mut self, fetchrows: u32) -> Self {
        self.fetchrows = Some(fetchrows);
        self
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourcePathRequest {
    /// The ID of the resource to generate a download URL for.
    #[serde(rename = "ref")]
    pub resource_id: u32,
    /// Preview size to retrieve (e.g. `"thm"`, `"scr"`, `"pre"`). Omit for the original file.
    pub size: Option<String>,
    /// If true, generates the preview if it does not yet exist.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate: Option<bool>,
    /// Override the file extension of the returned URL.
    pub extension: Option<String>,
    /// Page number for multi-page resources (e.g. PDF).
    pub page: Option<u32>,
    /// If true, returns a URL to the watermarked version of the file.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub watermarked: Option<bool>,
    /// ID of the alternative file to return a URL for, or -1 for the original.
    pub alternative: Option<i32>,
    /// If set, writes embedded metadata into the file before returning the URL.
    pub write_metadata: Option<bool>,
}

impl GetResourcePathRequest {
    pub fn new(resource_id: u32) -> Self {
        Self {
            resource_id,
            size: None,
            generate: None,
            extension: None,
            page: None,
            watermarked: None,
            alternative: None,
            write_metadata: None,
        }
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn generate(mut self, generate: bool) -> Self {
        self.generate = Some(generate);
        self
    }

    pub fn extension(mut self, extension: impl Into<String>) -> Self {
        self.extension = Some(extension.into());
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn watermarked(mut self, watermarked: bool) -> Self {
        self.watermarked = Some(watermarked);
        self
    }

    pub fn alternative(mut self, alternative: i32) -> Self {
        self.alternative = Some(alternative);
        self
    }

    pub fn write_metadata(mut self, write_metadata: bool) -> Self {
        self.write_metadata = Some(write_metadata);
        self
    }
}

#[non_exhaustive]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PutResourceDataRequest {
    /// The ID of the resource to update.
    pub resource: u32,
    /// JSON-encoded object mapping column names to new values. For valid columns/values view API docs.
    #[serde_as(as = "JsonString")]
    pub data: HashMap<String, String>,
}

impl PutResourceDataRequest {
    pub fn new(resource: u32, data: HashMap<String, String>) -> Self {
        Self { resource, data }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RelateAllResourcesRequest {
    /// Comma-separated list of resource IDs to relate with each other.
    pub related: List<u32>,
}

impl RelateAllResourcesRequest {
    pub fn new(related: impl Into<List<u32>>) -> Self {
        Self {
            related: related.into(),
        }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplaceResourceFileRequest {
    /// The ID of the resource whose file should be replaced.
    #[serde(rename = "ref")]
    pub resource: u32,
    /// Local server path or publicly accessible URL of the replacement file.
    pub file_location: String,
    /// If true, skips reading EXIF data from the replacement file.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_exif: Option<bool>,
    /// If true, automatically rotates the image based on EXIF orientation.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub autorotate: Option<bool>,
    /// If true, retains the previous file as an alternative file rather than deleting it.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_original: Option<bool>,
}

impl ReplaceResourceFileRequest {
    pub fn new(resource: u32, file_location: impl Into<String>) -> Self {
        Self {
            resource,
            file_location: file_location.into(),
            no_exif: None,
            autorotate: None,
            keep_original: None,
        }
    }

    pub fn no_exif(mut self, no_exif: bool) -> Self {
        self.no_exif = Some(no_exif);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate);
        self
    }

    pub fn keep_original(mut self, keep_original: bool) -> Self {
        self.keep_original = Some(keep_original);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceFileReadonlyRequest {
    /// The ID of the resource to check for read-only file status.
    #[serde(rename = "ref")]
    pub resource_id: u32,
}

impl ResourceFileReadonlyRequest {
    pub fn new(resource_id: u32) -> Self {
        Self { resource_id }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ResourceLogLastRowsRequest {
    /// Only return log entries with a ref greater than this value.
    pub minref: Option<u32>,
    /// Only return log entries from the last N days.
    pub days: Option<u32>,
    /// Maximum number of log entries to return.
    pub maxrecords: Option<u32>,
    /// Comma-separated list of field IDs to limit results to.
    #[serde(rename = "field")]
    pub field_ids: Option<List<u32>>,
    /// Comma-separated list of log codes to limit results to (e.g. `"FD"` for field data changes).
    pub log_code: Option<List<String>>,
}

impl ResourceLogLastRowsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn minref(mut self, minref: u32) -> Self {
        self.minref = Some(minref);
        self
    }

    pub fn days(mut self, days: u32) -> Self {
        self.days = Some(days);
        self
    }

    pub fn maxrecords(mut self, maxrecords: u32) -> Self {
        self.maxrecords = Some(maxrecords);
        self
    }

    pub fn field_ids(mut self, field_ids: impl Into<List<u32>>) -> Self {
        self.field_ids = Some(field_ids.into());
        self
    }

    pub fn log_code(mut self, log_code: impl Into<List<String>>) -> Self {
        self.log_code = Some(log_code.into());
        self
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UploadFileRequest {
    /// The ID of the resource to upload the file to.
    #[serde(rename = "ref")]
    pub resource_id: u32,
    /// If true, skips reading EXIF data from the uploaded file.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_exif: Option<bool>,
    /// If true, reverts to the original file instead of uploading a new one.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub revert: Option<bool>,
    /// If true, automatically rotates the image based on EXIF orientation.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub autorotate: Option<bool>,
    /// Local server path of the file to upload (must be within `$valid_upload_paths`).
    pub file_path: Option<String>,
}

impl UploadFileRequest {
    pub fn new(resource_id: u32) -> Self {
        Self {
            resource_id,
            no_exif: None,
            revert: None,
            autorotate: None,
            file_path: None,
        }
    }

    pub fn no_exif(mut self, no_exif: bool) -> Self {
        self.no_exif = Some(no_exif);
        self
    }

    pub fn revert(mut self, revert: bool) -> Self {
        self.revert = Some(revert);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate);
        self
    }

    pub fn file_path(mut self, file_path: impl Into<String>) -> Self {
        self.file_path = Some(file_path.into());
        self
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UploadFileByUrlRequest {
    /// The ID of the resource to upload the file to.
    #[serde(rename = "ref")]
    pub resource_id: u32,
    /// If true, skips reading EXIF data from the downloaded file.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_exif: Option<bool>,
    /// If true, reverts to the original file instead of uploading a new one.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub revert: Option<bool>,
    /// If true, automatically rotates the image based on EXIF orientation.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub autorotate: Option<bool>,
    /// Publicly accessible URL for the RS server to fetch and attach (hostname must be in `$api_upload_urls`).
    pub url: Option<String>,
}

impl UploadFileByUrlRequest {
    pub fn new(resource_id: u32) -> Self {
        Self {
            resource_id,
            no_exif: None,
            revert: None,
            autorotate: None,
            url: None,
        }
    }

    pub fn no_exif(mut self, no_exif: bool) -> Self {
        self.no_exif = Some(no_exif);
        self
    }

    pub fn revert(mut self, revert: bool) -> Self {
        self.revert = Some(revert);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate);
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// Data source for a multipart upload
#[non_exhaustive]
pub enum UploadSource {
    File(std::path::PathBuf),
    Stream {
        body: reqwest::Body,
        filename: String,
    },
}

impl UploadSource {
    /// Creates a file-based upload source.
    pub fn from_file(path: impl Into<std::path::PathBuf>) -> Self {
        UploadSource::File(path.into())
    }

    /// Creates a stream-based upload source.
    ///
    /// `body` accepts anything that converts into a [`reqwest::Body`]:
    /// `Bytes`, `Vec<u8>`, or a `Stream<Item = Result<Bytes, E>>` via
    /// `reqwest::Body::wrap_stream(ReaderStream::new(reader))`.
    pub fn from_stream(body: impl Into<reqwest::Body>, filename: impl Into<String>) -> Self {
        UploadSource::Stream {
            body: body.into(),
            filename: filename.into(),
        }
    }
}

impl From<std::path::PathBuf> for UploadSource {
    fn from(path: std::path::PathBuf) -> Self {
        UploadSource::File(path)
    }
}

impl From<&std::path::Path> for UploadSource {
    fn from(path: &std::path::Path) -> Self {
        UploadSource::File(path.to_path_buf())
    }
}

impl From<&str> for UploadSource {
    fn from(s: &str) -> Self {
        UploadSource::File(std::path::PathBuf::from(s))
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UploadMultipartRequest {
    /// The ID of the resource to upload the file to.
    #[serde(rename = "ref")]
    pub resource_id: u32,
    /// If true, skips reading EXIF data from the uploaded file.
    #[serde(serialize_with = "bool_as_u8")]
    pub no_exif: bool,
    /// If true, reverts to the original file instead of uploading a new one.
    #[serde(serialize_with = "bool_as_u8")]
    pub revert: bool,
    /// If set, only generates a preview without replacing the stored file.
    pub previewonly: Option<bool>,
    /// ID of an alternative file slot to upload into instead of the primary file.
    pub alternative: Option<u32>,
    /// If true, automatically rotates the image based on EXIF orientation.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub autorotate: Option<bool>,
}

impl UploadMultipartRequest {
    /// Creates a new `UploadMultipartRequest` with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `resource_id` - The ID of the resource to upload the file to.
    /// * `no_exif` - If true, skips reading EXIF data from the uploaded file.
    /// * `revert` - If true, reverts to the original file instead of uploading a new one.
    pub fn new(resource_id: u32, no_exif: bool, revert: bool) -> Self {
        Self {
            resource_id,
            no_exif,
            revert,
            previewonly: None,
            alternative: None,
            autorotate: None,
        }
    }
    pub fn previewonly(mut self, previewonly: bool) -> Self {
        self.previewonly = Some(previewonly);
        self
    }

    pub fn alternative(mut self, alternative: u32) -> Self {
        self.alternative = Some(alternative);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate);
        self
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateRelatedResourceRequest {
    /// The ID of the resource to update relationships for.
    #[serde(rename = "ref")]
    pub resource_id: u32,
    /// Comma-separated list of resource IDs to add or remove as related resources.
    pub related: List<u32>,
    /// If true, adds the related resources; if false, removes them.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub add: Option<bool>,
}

impl UpdateRelatedResourceRequest {
    pub fn new(resource_id: u32, related: impl Into<List<u32>>) -> Self {
        Self {
            resource_id,
            related: related.into(),
            add: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, add: bool) -> Self {
        self.add = Some(add);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateResourceTypeRequest {
    /// The ID of the resource to update.
    pub resource: u32,
    /// The new resource type ID to assign to the resource.
    #[serde(rename = "type")]
    pub resourcetype: u32,
}

impl UpdateResourceTypeRequest {
    pub fn new(resource: u32, resourcetype: u32) -> Self {
        Self {
            resource,
            resourcetype,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceCollectionsRequest {
    /// The ID of the resource to retrieve associated collections for.
    #[serde(rename = "ref")]
    pub resource_id: u32,
}

impl GetResourceCollectionsRequest {
    pub fn new(resource_id: u32) -> Self {
        Self { resource_id }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ValidateUploadUrlRequest {
    /// The URL to validate against the server's allowed `$api_upload_urls` list.
    pub url: String,
}

impl ValidateUploadUrlRequest {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}
