use serde::Serialize;

use crate::client::Client;
use crate::error::RsError;

use super::SortOrder;

pub struct ResourceApi<'a> {
    client: &'a Client,
}

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
            .send_request("get_resource_all_image_sizes", reqwest::Method::GET, request)
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

    // TODO: replace_resource_file — requires multipart support

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

    // TODO: upload_file            — requires multipart support
    // TODO: upload_file_by_url     — requires multipart support
    // TODO: upload_multipart       — requires multipart support

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
    // TODO: 
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

#[derive(Default, Serialize)]
/// Note that the file parameter here may be a physical path (to the server) or a remote URL.
pub struct AddAlternativeFileRequest {
    resource: u32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alt_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
}

impl AddAlternativeFileRequest {
    pub fn new(resource: u32, name: impl Into<String>) -> Self {
        Self { resource, name: name.into(), ..Default::default() }
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

#[derive(Default, Serialize)]
pub struct CopyResourceRequest {
    from: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<u32>,
}

impl CopyResourceRequest {
    pub fn new(from: u32) -> Self {
        Self { from, ..Default::default() }
    }

    pub fn resource_type(mut self, resource_type: u32) -> Self {
        self.resource_type = Some(resource_type);
        self
    }
}

#[derive(Default, Serialize)]
pub struct CreateResourceRequest {
    resource_type: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    archive: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_exif: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autorotate: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<String>,
}

impl CreateResourceRequest {
    pub fn new(resource_type: u32) -> Self {
        Self { resource_type, ..Default::default() }
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

    pub fn metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}

#[derive(Serialize)]
pub struct DeleteAlternativeFile {
    resource: u32,
    #[serde(rename = "ref")]
    r#ref: u32,
}

impl DeleteAlternativeFile {
    pub fn new(resource: u32, r#ref: u32) -> Self {
        Self { resource, r#ref }
    }
}

#[derive(Serialize)]
pub struct DeleteResourceRequest {
    resource: u32,
}

impl DeleteResourceRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[derive(Default, Serialize)]
pub struct GetAlternativeFilesRequest {
    resource: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    orderby: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl GetAlternativeFilesRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource, ..Default::default() }
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

#[derive(Serialize)]
pub struct GetEditAccessRequest {
    resource: u32,
}

impl GetEditAccessRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[derive(Serialize)]
pub struct GetRelatedResourcesRequest {
    #[serde(rename = "ref")]
    r#ref: u32,
}

impl GetRelatedResourcesRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref }
    }
}

#[derive(Serialize)]
pub struct GetResourceAccessRequest {
    resource: u32,
}

impl GetResourceAccessRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[derive(Serialize)]
pub struct GetResourceAllImageSizesRequest {
    resource: u32,
}

impl GetResourceAllImageSizesRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[derive(Default, Serialize)]
pub struct GetResourceCommentsRequest {
    resource_ref: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat_view: Option<bool>,
}

impl GetResourceCommentsRequest {
    pub fn new(resource_ref: u32) -> Self {
        Self { resource_ref, ..Default::default() }
    }

    pub fn flat_view(mut self, flat_view: bool) -> Self {
        self.flat_view = Some(flat_view);
        self
    }
}

#[derive(Serialize)]
pub struct GetResourceDataRequest {
    resource: u32,
}

impl GetResourceDataRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[derive(Serialize)]
pub struct GetResourceFieldDataRequest {
    resource: u32,
}

impl GetResourceFieldDataRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

#[derive(Default, Serialize)]
pub struct GetResourceLogRequest {
    resource: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    fetchrows: Option<u32>,
}

impl GetResourceLogRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource, ..Default::default() }
    }

    pub fn fetchrows(mut self, fetchrows: u32) -> Self {
        self.fetchrows = Some(fetchrows);
        self
    }
}

#[derive(Default, Serialize)]
pub struct GetResourcePathRequest {
    #[serde(rename = "ref")]
    r#ref: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generate: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    watermarked: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternative: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_metadata: Option<bool>,
}

impl GetResourcePathRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref, ..Default::default() }
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

#[derive(Serialize)]
pub struct PutResourceDataRequest {
    resource: u32,
    data: String,
}

impl PutResourceDataRequest {
    pub fn new(resource: u32, data: impl Into<String>) -> Self {
        Self { resource, data: data.into() }
    }
}

#[derive(Serialize)]
pub struct RelateAllResourcesRequest {
    related: String,
}

impl RelateAllResourcesRequest {
    pub fn new(related: impl Into<String>) -> Self {
        Self { related: related.into() }
    }
}

#[derive(Serialize)]
pub struct ResourceFileReadonlyRequest {
    #[serde(rename = "ref")]
    r#ref: u32,
}

impl ResourceFileReadonlyRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref }
    }
}

#[derive(Default, Serialize)]
pub struct ResourceLogLastRowsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    minref: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maxrecords: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_code: Option<String>,
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

    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn log_code(mut self, log_code: impl Into<String>) -> Self {
        self.log_code = Some(log_code.into());
        self
    }
}

#[derive(Default, Serialize)]
pub struct UpdateRelatedResourceRequest {
    #[serde(rename = "ref")]
    r#ref: u32,
    related: String, // TODO: CSV of T
    #[serde(skip_serializing_if = "Option::is_none")]
    add: Option<u8>,
}

impl UpdateRelatedResourceRequest {
    pub fn new(r#ref: u32, related: impl Into<String>) -> Self {
        Self { r#ref, related: related.into(), ..Default::default() }
    }

    pub fn add(mut self, add: bool) -> Self {
        self.add = Some(add as u8);
        self
    }
}

#[derive(Serialize)]
pub struct UpdateResourceTypeRequest {
    resource: u32,
    resourcetype: u32,
}

impl UpdateResourceTypeRequest {
    pub fn new(resource: u32, resourcetype: u32) -> Self {
        Self { resource, resourcetype }
    }
}

#[derive(Serialize)]
pub struct GetResourceCollectionsRequest {
    #[serde(rename = "ref")]
    r#ref: u32,
}

impl GetResourceCollectionsRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref }
    }
}

#[derive(Serialize)]
pub struct ValidateUploadUrlRequest {
    url: u32,
}

impl ValidateUploadUrlRequest {
    pub fn new(url: u32) -> Self {
        Self { url }
    }
}
