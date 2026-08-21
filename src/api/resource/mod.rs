use crate::client::{Client, HttpMethod};
use crate::error::Error;

use super::shared::AjaxEnvelope;
use response::ReadonlyData;

pub mod request;
pub mod response;
mod shared;

use request::{
    AddAlternativeFile, CopyResource, CreateResource, DeleteAlternativeFile, DeleteComment,
    DeleteResource, GetAlternativeFiles, GetEditAccess, GetRelatedResources, GetResourceAccess,
    GetResourceAllImageSizes, GetResourceCollections, GetResourceComments, GetResourceData,
    GetResourceFieldData, GetResourceLog, GetResourcePath, PutResourceData, RelateAllResources,
    ReplaceResourceFile, ResourceFileReadonly, ResourceLogLastRows, UpdateRelatedResource,
    UpdateResourceType, UploadFile, UploadFileByUrl, UploadMultipart, UploadSource,
    ValidateUploadUrl,
};
use response::{
    AlternativeFile, ImageSize, LogEntry, Resource, ResourceCollection, ResourceFieldData,
    ResourceType,
};

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
    /// * `request` - Parameters built via [`AddAlternativeFile`]
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
    /// use resourcespace_client::api::resource::request::AddAlternativeFile;
    /// let alt_id = client.resource()
    ///     .add_alternative_file(AddAlternativeFile::new(1234, "Print master"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_alternative_file(&self, request: AddAlternativeFile) -> Result<u32, Error> {
        self.client
            .send_request("add_alternative_file", HttpMethod::Post, request)
            .await
    }

    /// Copy a resource. Note that attached files are not copied — this is a metadata
    /// and property copy only.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CopyResource`]
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
    /// use resourcespace_client::api::resource::request::CopyResource;
    /// let new_id = client.resource()
    ///     .copy_resource(CopyResource::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy_resource(&self, request: CopyResource) -> Result<u32, Error> {
        self.client
            .send_request("copy_resource", HttpMethod::Post, request)
            .await
    }

    /// Create a new resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CreateResource`]
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
    /// use resourcespace_client::api::resource::request::CreateResource;
    ///
    /// client.resource().create_resource(
    ///     CreateResource::new(1)
    ///         .metadata(HashMap::from([
    ///             (90u32, FieldValue::from("A plain text description")),       // Text field
    ///             (91u32, FieldValue::from(["Doe, John", "Smith, Jane"])),     // Keywords, auto-quoted
    ///             (92u32, FieldValue::from([1u32, 2, 3])),                     // Node IDs
    ///         ]))
    /// ).await?;
    /// # Ok(()) }
    /// ```
    pub async fn create_resource(&self, request: CreateResource) -> Result<u32, Error> {
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
    /// use resourcespace_client::api::resource::request::DeleteAlternativeFile;
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
    /// * `request` - Parameters built via [`DeleteComment`]
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
    /// # use resourcespace_client::api::resource::request::DeleteComment;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .resource()
    ///     .delete_comment(DeleteComment::new(12))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_comment(&self, request: DeleteComment) -> Result<bool, Error> {
        self.client
            .send_request("delete_comment", HttpMethod::Post, request)
            .await
    }

    /// Delete a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DeleteResource`]
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
    /// use resourcespace_client::api::resource::request::DeleteResource;
    /// client.resource()
    ///     .delete_resource(DeleteResource::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_resource(&self, request: DeleteResource) -> Result<bool, Error> {
        self.client
            .send_request("delete_resource", HttpMethod::Post, request)
            .await
    }

    /// Returns a list of alternative files for a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetAlternativeFiles`]
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
    /// use resourcespace_client::api::resource::request::GetAlternativeFiles;
    /// let files = client.resource()
    ///     .get_alternative_files(GetAlternativeFiles::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_alternative_files(
        &self,
        request: GetAlternativeFiles,
    ) -> Result<Vec<AlternativeFile>, Error> {
        self.client
            .send_request("get_alternative_files", HttpMethod::Get, request)
            .await
    }

    /// Check if the current user has edit access to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetEditAccess`]
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
    /// use resourcespace_client::api::resource::request::GetEditAccess;
    /// let can_edit = client.resource()
    ///     .get_edit_access(GetEditAccess::new(1234))
    ///     .await
    ///     .unwrap_or(false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_edit_access(&self, request: GetEditAccess) -> Result<bool, Error> {
        self.client
            .send_request("get_edit_access", HttpMethod::Get, request)
            .await
    }

    /// Returns a list of resources related to a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetRelatedResources`]
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
    /// use resourcespace_client::api::resource::request::GetRelatedResources;
    /// let related = client.resource()
    ///     .get_related_resources(GetRelatedResources::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_related_resources(
        &self,
        request: GetRelatedResources,
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
    /// * `request` - Parameters built via [`GetResourceAccess`]
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
    /// use resourcespace_client::api::resource::request::GetResourceAccess;
    /// let access = client.resource()
    ///     .get_resource_access(GetResourceAccess::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_access(&self, request: GetResourceAccess) -> Result<u8, Error> {
        self.client
            .send_request("get_resource_access", HttpMethod::Get, request)
            .await
    }

    /// Get all preview sizes available for a specific resource.
    ///
    /// Multi-page resources will include each page size in the response.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceAllImageSizes`]
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
    /// use resourcespace_client::api::resource::request::GetResourceAllImageSizes;
    /// let sizes = client.resource()
    ///     .get_resource_all_image_sizes(GetResourceAllImageSizes::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_all_image_sizes(
        &self,
        request: GetResourceAllImageSizes,
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
    /// * `request` - Parameters built via [`GetResourceComments`]
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
        request: GetResourceComments,
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
    /// * `request` - Parameters built via [`GetResourceData`]
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
    /// use resourcespace_client::api::resource::request::GetResourceData;
    /// let resource = client.resource()
    ///     .get_resource_data(GetResourceData::new(1234))
    ///     .await?;
    /// println!("{:?}", resource.file_extension);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_data(&self, request: GetResourceData) -> Result<Resource, Error> {
        self.client
            .send_request("get_resource_data", HttpMethod::Get, request)
            .await
    }

    /// Return all field data for a given resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceFieldData`]
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
    /// use resourcespace_client::api::resource::request::GetResourceFieldData;
    /// for field in client.resource()
    ///     .get_resource_field_data(GetResourceFieldData::new(1234))
    ///     .await? {
    ///     println!("{} = {:?}", field.name, field.value);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_field_data(
        &self,
        request: GetResourceFieldData,
    ) -> Result<Vec<ResourceFieldData>, Error> {
        self.client
            .send_request("get_resource_field_data", HttpMethod::Get, request)
            .await
    }

    /// Returns the full log for a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceLog`]
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
    /// use resourcespace_client::api::resource::request::GetResourceLog;
    /// let log = client.resource()
    ///     .get_resource_log(GetResourceLog::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_log(&self, request: GetResourceLog) -> Result<Vec<LogEntry>, Error> {
        self.client
            .send_request("get_resource_log", HttpMethod::Get, request)
            .await
    }

    /// Returns a temporary URL for downloading a resource file.
    ///
    /// The URL is valid for 24 hours by default (configurable via `$api_resource_path_expiry_hours`).
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourcePath`]
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
    /// use resourcespace_client::api::resource::request::GetResourcePath;
    /// let url = client.resource()
    ///     .get_resource_path(GetResourcePath::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_path(&self, request: GetResourcePath) -> Result<String, Error> {
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
    /// * `request` - Parameters built via [`PutResourceData`]
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
    /// use resourcespace_client::api::resource::request::PutResourceData;
    /// use std::collections::HashMap;
    /// client.resource()
    ///     .put_resource_data(PutResourceData::new(
    ///         1234,
    ///         HashMap::from([("archive".to_string(), "0".to_string())]),
    ///     ))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn put_resource_data(&self, request: PutResourceData) -> Result<bool, Error> {
        self.client
            .send_request("put_resource_data", HttpMethod::Post, request)
            .await
    }

    /// Relate all the provided resources with each other.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`RelateAllResources`]
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
    /// use resourcespace_client::api::resource::request::RelateAllResources;
    /// client.resource()
    ///     .relate_all_resources(RelateAllResources::new([1234, 1235]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn relate_all_resources(&self, request: RelateAllResources) -> Result<bool, Error> {
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
    /// * `request` - Parameters built via [`ReplaceResourceFile`]
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
    /// # use resourcespace_client::api::resource::request::ReplaceResourceFile;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let result = client
    ///     .resource()
    ///     .replace_resource_file(ReplaceResourceFile::new(
    ///         1234,
    ///         "https://example.com/replacement.jpg",
    ///     ))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn replace_resource_file(
        &self,
        request: ReplaceResourceFile,
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
    /// * `request` - Parameters built via [`ResourceFileReadonly`]
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
    /// use resourcespace_client::api::resource::request::ResourceFileReadonly;
    /// let readonly = client.resource()
    ///     .resource_file_readonly(ResourceFileReadonly::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resource_file_readonly(
        &self,
        request: ResourceFileReadonly,
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
    /// * `request` - Parameters built via [`ResourceLogLastRows`]
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
    /// use resourcespace_client::api::resource::request::ResourceLogLastRows;
    /// let rows = client.resource()
    ///     .resource_log_last_rows(ResourceLogLastRows::new().days(1))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resource_log_last_rows(
        &self,
        request: ResourceLogLastRows,
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
    /// * `request` - Parameters built via [`UploadFile`]
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
    /// use resourcespace_client::api::resource::request::UploadFile;
    /// client.resource()
    ///     .upload_file(UploadFile::new(1234).file_path("/var/tmp/a.jpg"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_file(&self, request: UploadFile) -> Result<u32, Error> {
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
    /// * `request` - Parameters built via [`UploadFileByUrl`]
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
    /// use resourcespace_client::api::resource::request::UploadFileByUrl;
    /// client.resource()
    ///     .upload_file_by_url(UploadFileByUrl::new(1234).url("https://example.com/a.jpg"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_file_by_url(&self, request: UploadFileByUrl) -> Result<u32, Error> {
        self.client
            .send_request("upload_file_by_url", HttpMethod::Post, request)
            .await
    }

    /// Uploads files using HTTP multipart to an existing resource, replacing any file that is already attached.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UploadMultipart`]
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
    /// # use resourcespace_client::api::resource::request::{UploadMultipart, UploadSource};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .resource()
    ///     .upload_multipart(
    ///         UploadMultipart::new(1234, false, false),
    ///         UploadSource::from_file("photo.jpg"),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_multipart(
        &self,
        request: UploadMultipart,
        source: impl Into<UploadSource>,
    ) -> Result<(), Error> {
        self.client
            .send_multipart_request("upload_multipart", request, source.into())
            .await
    }

    /// Add or remove resource relationships.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateRelatedResource`]
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
    /// use resourcespace_client::api::resource::request::UpdateRelatedResource;
    /// client.resource()
    ///     .update_related_resource(
    ///         UpdateRelatedResource::new(1234, [1235]).add(true),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_related_resource(
        &self,
        request: UpdateRelatedResource,
    ) -> Result<bool, Error> {
        self.client
            .send_request("update_related_resource", HttpMethod::Post, request)
            .await
    }

    /// Change the resource type of a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`UpdateResourceType`]
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
    /// use resourcespace_client::api::resource::request::UpdateResourceType;
    /// client.resource()
    ///     .update_resource_type(UpdateResourceType::new(1234, 2))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_resource_type(&self, request: UpdateResourceType) -> Result<bool, Error> {
        self.client
            .send_request("update_resource_type", HttpMethod::Post, request)
            .await
    }

    /// Retrieves a list of collections that a resource is used in for the specified resource reference.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetResourceCollections`]
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
    /// use resourcespace_client::api::resource::request::GetResourceCollections;
    /// let collections = client.resource()
    ///     .get_resource_collections(GetResourceCollections::new(1234))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_resource_collections(
        &self,
        request: GetResourceCollections,
    ) -> Result<Vec<ResourceCollection>, Error> {
        self.client
            .send_request("get_resource_collections", HttpMethod::Get, request)
            .await
    }

    /// Validate a URL supplied in the `create_resource` or `upload_file_by_url` API calls.
    ///
    /// Requires the URL hostname to be added in the config option $`api_upload_urls`, for example:
    /// `$api_upload_urls = array('resourcespace.com', 'localhost');`
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ValidateUploadUrl`]
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
    /// use resourcespace_client::api::resource::request::ValidateUploadUrl;
    /// let allowed = client.resource()
    ///     .validate_upload_url(ValidateUploadUrl::new("https://example.com/a.jpg"))
    ///     .await
    ///     .unwrap_or(false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn validate_upload_url(&self, request: ValidateUploadUrl) -> Result<bool, Error> {
        self.client
            .send_request("validate_upload_url", HttpMethod::Get, request)
            .await
    }
}
