use serde::Serialize;
use serde_with::json::JsonString;
use serde_with::{serde_as, skip_serializing_none};
use std::collections::HashMap;

use crate::client::Client;
use crate::error::RsError;

use super::{List, SortOrder};

#[derive(Debug)]
pub struct ResourceApi<'a> {
    client: &'a Client,
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
    /// The ID of the new alternative file, or false on failure.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn add_alternative_file(
        &self,
        request: AddAlternativeFileRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("add_alternative_file", reqwest::Method::POST, request)
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
    /// The ID of the newly created resource, or false if the operation failed.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn copy_resource(
        &self,
        request: CopyResourceRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("copy_resource", reqwest::Method::POST, request)
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
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn create_resource(
        &self,
        request: CreateResourceRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("create_resource", reqwest::Method::POST, request)
            .await
    }

    /// Deletes an alternative file.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteAlternativeFile`]
    ///
    /// ## Returns
    ///
    /// The success of the operation (true/false).
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn delete_alternative_file(
        &self,
        request: DeleteAlternativeFile,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("delete_alternative_file", reqwest::Method::POST, request)
            .await
    }

    pub async fn delete_comment() -> Result<serde_json::Value, RsError> {
        todo!("available from RS v11.0")
    }

    /// Delete a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteResourceRequest`]
    ///
    /// ## Returns
    ///
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn delete_resource(
        &self,
        request: DeleteResourceRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("delete_resource", reqwest::Method::POST, request)
            .await
    }

    /// Returns a list of alternative files for a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetAlternativeFilesRequest`]
    ///
    /// ## Returns
    ///
    /// A list of alternative files.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_alternative_files(
        &self,
        request: GetAlternativeFilesRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_alternative_files", reqwest::Method::GET, request)
            .await
    }

    /// Check if the current user has edit access to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetEditAccessRequest`]
    ///
    /// ## Returns
    ///
    /// True if the user has edit access, false otherwise.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_edit_access(
        &self,
        request: GetEditAccessRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_edit_access", reqwest::Method::GET, request)
            .await
    }

    /// Returns a list of resources related to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetRelatedResourcesRequest`]
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_related_resources(
        &self,
        request: GetRelatedResourcesRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_related_resources", reqwest::Method::GET, request)
            .await
    }

    /// Retrieves the access level for the current user for a specified resource.
    ///
    /// Returns 0 (full), 1 (restricted), 2 (confidential), 99 (not found), or false (invalid ID).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceAccessRequest`]
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_access(
        &self,
        request: GetResourceAccessRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_resource_access", reqwest::Method::GET, request)
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
    /// JSON containing the resource's available sizes.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_all_image_sizes(
        &self,
        request: GetResourceAllImageSizesRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request(
                "get_resource_all_image_sizes",
                reqwest::Method::GET,
                request,
            )
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
    /// Array of comments in tree view by default, or flat list if requested. Returns an empty
    /// array if the user lacks permission or commenting is disabled.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_comments(
        &self,
        _request: GetResourceCommentsRequest,
    ) -> Result<serde_json::Value, RsError> {
        todo!("available from RS v11.0");
        // self.client
        //     .send_request("get_resource_comments", reqwest::Method::GET, request)
        //     .await
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
    /// The resource properties. Actual values depend on system configuration.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_data(
        &self,
        request: GetResourceDataRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_resource_data", reqwest::Method::GET, request)
            .await
    }

    /// Return all field data for a given resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceFieldDataRequest`]
    ///
    /// ## Returns
    ///
    /// JSON containing the resource metadata.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_field_data(
        &self,
        request: GetResourceFieldDataRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_resource_field_data", reqwest::Method::GET, request)
            .await
    }

    /// Returns the full log for a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceLogRequest`]
    ///
    /// ## Returns
    ///
    /// The resource log entries.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_log(
        &self,
        request: GetResourceLogRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_resource_log", reqwest::Method::GET, request)
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
    /// A temporary URL for the requested resource file, or false on failure.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_path(
        &self,
        request: GetResourcePathRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_resource_path", reqwest::Method::GET, request)
            .await
    }

    /// Returns all configured resource types available to the current user.
    ///
    /// From RS v10.2, the associated resource type field IDs are also returned.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_types(&self) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_resource_types", reqwest::Method::GET, ())
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
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn put_resource_data(
        &self,
        request: PutResourceDataRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("put_resource_data", reqwest::Method::POST, request)
            .await
    }

    /// Relate all the provided resources with each other.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`RelateAllResourcesRequest`]
    ///
    /// ## Returns
    ///
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn relate_all_resources(
        &self,
        request: RelateAllResourcesRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("relate_all_resources", reqwest::Method::POST, request)
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
    /// A JSON encoded array with `Status` (SUCCESS/FAILED) and `Message`.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn replace_resource_file(
        &self,
        request: ReplaceResourceFileRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("replace_resource_file", reqwest::Method::POST, request)
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
    /// A 200 HTTP status will be returned with a payload detailing if successful or a 400 status otherwise.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn resource_file_readonly(
        &self,
        request: ResourceFileReadonlyRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("resource_file_readonly", reqwest::Method::GET, request)
            .await
    }

    /// Retrieve recent entries from the resource log
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ResourceLogLastRowsRequest`]
    ///
    /// ## Returns
    ///
    /// Log entries in JSON format, including date, ref, resource, type (type of log entry), resource_type_field, user ID, notes, diff and usageoption value
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn resource_log_last_rows(
        &self,
        request: ResourceLogLastRowsRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("resource_log_last_rows", reqwest::Method::GET, request)
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
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn upload_file(
        &self,
        request: UploadFileRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("upload_file", reqwest::Method::POST, request)
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
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn upload_file_by_url(
        &self,
        request: UploadFileByUrlRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("upload_file_by_url", reqwest::Method::POST, request)
            .await
    }

    /// Uploads files using HTTP multipart to an existing resource, replacing any file that is already attached.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UploadMultipartRequest`]
    ///
    /// ## Returns
    ///
    /// 204 if succesful other status codes (413, 400, 500) if not
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn upload_multipart(
        &self,
        request: UploadMultipartRequest,
        file: impl AsRef<std::path::Path>,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_multipart_request("upload_multipart", request, file.as_ref())
            .await
    }

    /// Add or remove resource relationships.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateRelatedResourceRequest`]
    ///
    /// ## Returns
    ///
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn update_related_resource(
        &self,
        request: UpdateRelatedResourceRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("update_related_resource", reqwest::Method::POST, request)
            .await
    }

    /// Change the resource type of a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateResourceTypeRequest`]
    ///
    /// ## Returns
    ///
    /// True or false depending on operation success.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn update_resource_type(
        &self,
        request: UpdateResourceTypeRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("update_resource_type", reqwest::Method::POST, request)
            .await
    }

    /// Retrieves a list of collections that a resource is used in for the specified resource reference.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceCollectionsRequest`]
    ///
    /// ## Returns
    ///
    /// Array of collections with the collection ID, name and description. False on failure.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_resource_collections(
        &self,
        request: GetResourceCollectionsRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_resource_collections", reqwest::Method::GET, request)
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
    /// Returns true if a valid URL is found, false otherwise.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn validate_upload_url(
        &self,
        request: ValidateUploadUrlRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("validate_upload_url", reqwest::Method::GET, request)
            .await
    }
}

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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CopyResourceRequest {
    /// The ID of the resource to copy.
    pub from: u32,
    /// Resource type ID to assign to the copy; defaults to the source resource type if omitted.
    pub resource_type: Option<u32>,
}

impl CopyResourceRequest {
    pub fn new(from: u32) -> Self {
        Self {
            from,
            resource_type: None,
        }
    }

    pub fn resource_type(mut self, resource_type: u32) -> Self {
        self.resource_type = Some(resource_type);
        self
    }
}

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
    /// If 1, skips reading EXIF data from the attached file.
    pub no_exif: Option<u8>,
    /// If 1, automatically rotates the image based on EXIF orientation.
    pub autorotate: Option<u8>,
    /// JSON-encoded metadata fields to set on the resource at creation time.
    #[serde_as(as = "JsonString")]
    pub metadata: Option<HashMap<u32, String>>,
}

impl CreateResourceRequest {
    pub fn new(resource_type: u32) -> Self {
        Self {
            resource_type,
            archive: None,
            url: None,
            no_exif: None,
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
        self.no_exif = Some(no_exif as u8);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate as u8);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<u32, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteAlternativeFile {
    /// The ID of the resource the alternative file belongs to.
    pub resource: u32,
    /// The ID of the alternative file to delete.
    #[serde(rename = "ref")]
    pub r#ref: u32,
}

impl DeleteAlternativeFile {
    pub fn new(resource: u32, r#ref: u32) -> Self {
        Self { resource, r#ref }
    }
}

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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetAlternativeFilesRequest {
    /// The ID of the resource whose alternative files should be returned.
    pub resource: u32,
    /// Field name to order the alternative files by.
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetRelatedResourcesRequest {
    /// The ID of the resource whose related resources should be returned.
    #[serde(rename = "ref")]
    pub r#ref: u32,
}

impl GetRelatedResourcesRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref }
    }
}

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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceCommentsRequest {
    /// The ID of the resource to retrieve comments for.
    pub resource_ref: u32,
    /// If set, returns comments as a flat list rather than a threaded tree.
    pub flat_view: Option<bool>,
}

impl GetResourceCommentsRequest {
    pub fn new(resource_ref: u32) -> Self {
        Self {
            resource_ref,
            flat_view: None,
        }
    }

    pub fn flat_view(mut self, flat_view: bool) -> Self {
        self.flat_view = Some(flat_view);
        self
    }
}

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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourcePathRequest {
    /// The ID of the resource to generate a download URL for.
    #[serde(rename = "ref")]
    pub r#ref: u32,
    /// Preview size to retrieve (e.g. `"thm"`, `"scr"`, `"pre"`). Omit for the original file.
    pub size: Option<String>,
    /// If 1, generates the preview if it does not yet exist.
    pub generate: Option<u8>,
    /// Override the file extension of the returned URL.
    pub extension: Option<String>,
    /// Page number for multi-page resources (e.g. PDF).
    pub page: Option<u32>,
    /// If 1, returns a URL to the watermarked version of the file.
    pub watermarked: Option<u8>,
    /// ID of the alternative file to return a URL for, or -1 for the original.
    pub alternative: Option<i32>,
    /// If set, writes embedded metadata into the file before returning the URL.
    pub write_metadata: Option<bool>,
}

impl GetResourcePathRequest {
    pub fn new(r#ref: u32) -> Self {
        Self {
            r#ref,
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
        self.generate = Some(generate as u8);
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
        self.watermarked = Some(watermarked as u8);
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

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplaceResourceFileRequest {
    /// The ID of the resource whose file should be replaced.
    pub resource: u32,
    /// Local server path or publicly accessible URL of the replacement file.
    pub file_location: String,
    /// If 1, skips reading EXIF data from the replacement file.
    pub no_exif: Option<u8>,
    /// If 1, automatically rotates the image based on EXIF orientation.
    pub autorotate: Option<u8>,
    /// If 1, retains the previous file as an alternative file rather than deleting it.
    pub keep_original: Option<u8>,
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
        self.no_exif = Some(no_exif as u8);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate as u8);
        self
    }

    pub fn keep_original(mut self, keep_original: bool) -> Self {
        self.keep_original = Some(keep_original as u8);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceFileReadonlyRequest {
    /// The ID of the resource to check for read-only file status.
    #[serde(rename = "ref")]
    pub r#ref: u32,
}

impl ResourceFileReadonlyRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref }
    }
}

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
    pub field: Option<List<u32>>,
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

    pub fn field(mut self, field: impl Into<List<u32>>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn log_code(mut self, log_code: impl Into<List<String>>) -> Self {
        self.log_code = Some(log_code.into());
        self
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UploadFileRequest {
    /// The ID of the resource to upload the file to.
    #[serde(rename = "ref")]
    pub r#ref: u32,
    /// If 1, skips reading EXIF data from the uploaded file.
    pub no_exif: Option<u8>,
    /// If 1, reverts to the original file instead of uploading a new one.
    pub revert: Option<u8>,
    /// If 1, automatically rotates the image based on EXIF orientation.
    pub autorotate: Option<u8>,
    /// Local server path of the file to upload (must be within `$valid_upload_paths`).
    pub file_path: Option<String>,
}

impl UploadFileRequest {
    pub fn new(r#ref: u32) -> Self {
        Self {
            r#ref,
            no_exif: None,
            revert: None,
            autorotate: None,
            file_path: None,
        }
    }

    pub fn no_exif(mut self, no_exif: bool) -> Self {
        self.no_exif = Some(no_exif as u8);
        self
    }

    pub fn revert(mut self, revert: bool) -> Self {
        self.revert = Some(revert as u8);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate as u8);
        self
    }

    pub fn file_path(mut self, file_path: impl Into<String>) -> Self {
        self.file_path = Some(file_path.into());
        self
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UploadFileByUrlRequest {
    /// The ID of the resource to upload the file to.
    #[serde(rename = "ref")]
    pub r#ref: u32,
    /// If 1, skips reading EXIF data from the downloaded file.
    pub no_exif: Option<u8>,
    /// If 1, reverts to the original file instead of uploading a new one.
    pub revert: Option<u8>,
    /// If 1, automatically rotates the image based on EXIF orientation.
    pub autorotate: Option<u8>,
    /// Publicly accessible URL for the RS server to fetch and attach (hostname must be in `$api_upload_urls`).
    pub url: Option<String>,
}

impl UploadFileByUrlRequest {
    pub fn new(r#ref: u32) -> Self {
        Self {
            r#ref,
            no_exif: None,
            revert: None,
            autorotate: None,
            url: None,
        }
    }

    pub fn no_exif(mut self, no_exif: bool) -> Self {
        self.no_exif = Some(no_exif as u8);
        self
    }

    pub fn revert(mut self, revert: bool) -> Self {
        self.revert = Some(revert as u8);
        self
    }

    pub fn autorotate(mut self, autorotate: bool) -> Self {
        self.autorotate = Some(autorotate as u8);
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UploadMultipartRequest {
    /// The ID of the resource to upload the file to.
    #[serde(rename = "ref")]
    pub r#ref: u32,
    /// If 1, skips reading EXIF data from the uploaded file.
    pub no_exif: u8,
    /// If 1, reverts to the original file instead of uploading a new one.
    pub revert: u8,
    /// If set, only generates a preview without replacing the stored file.
    pub previewonly: Option<bool>,
    /// ID of an alternative file slot to upload into instead of the primary file.
    pub alternative: Option<u32>,
}

impl UploadMultipartRequest {
    pub fn new(r#ref: u32, no_exif: bool, revert: bool) -> Self {
        Self {
            r#ref,
            no_exif: no_exif as u8,
            revert: revert as u8,
            previewonly: None,
            alternative: None,
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
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateRelatedResourceRequest {
    /// The ID of the resource to update relationships for.
    #[serde(rename = "ref")]
    pub r#ref: u32,
    /// Comma-separated list of resource IDs to add or remove as related resources.
    pub related: List<u32>,
    /// If 1, adds the related resources; if 0, removes them.
    pub add: Option<u8>,
}

impl UpdateRelatedResourceRequest {
    pub fn new(r#ref: u32, related: impl Into<List<u32>>) -> Self {
        Self {
            r#ref,
            related: related.into(),
            add: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, add: bool) -> Self {
        self.add = Some(add as u8);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateResourceTypeRequest {
    /// The ID of the resource to update.
    pub resource: u32,
    /// The new resource type ID to assign to the resource.
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetResourceCollectionsRequest {
    /// The ID of the resource to retrieve associated collections for.
    #[serde(rename = "ref")]
    pub r#ref: u32,
}

impl GetResourceCollectionsRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref }
    }
}

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
