use std::collections::HashMap;

use serde::Serialize;
use serde_with::json::JsonString;
use serde_with::{Same, serde_as, skip_serializing_none};

use super::shared::FieldValueAsString;
use crate::api::shared::{FieldValue, List, SortOrder, bool_as_u8, opt_bool_as_u8};

// Referenced only from doc links below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::ResourceApi;

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
