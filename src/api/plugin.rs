use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{bool_as_u8, empty_as_none};
use crate::client::{Client, HttpMethod};
use crate::error::Error;

/// Sub-API for plugin endpoints.
#[derive(Debug)]
pub struct PluginApi<'a> {
    client: &'a Client,
}

/// A consent record from the Consent manager plugin.
///
/// ResourceSpace's Consent manager fields vary by version and configuration,
/// so commonly documented fields are typed and any additional fields are kept
/// in [`Self::extra`].
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Consent {
    /// Consent record ID.
    #[serde(rename = "ref")]
    pub consent_id: u32,
    /// Name of the individual giving consent.
    #[serde(deserialize_with = "empty_as_none")]
    pub name: Option<String>,
    /// Email address of the individual.
    #[serde(deserialize_with = "empty_as_none")]
    pub email: Option<String>,
    /// Telephone number of the individual.
    #[serde(deserialize_with = "empty_as_none")]
    pub telephone: Option<String>,
    /// Description of the intended usage for which consent is given.
    #[serde(deserialize_with = "empty_as_none")]
    pub consent_usage: Option<String>,
    /// Additional notes.
    #[serde(deserialize_with = "empty_as_none")]
    pub notes: Option<String>,
    /// Expiry date, when set.
    #[serde(deserialize_with = "empty_as_none")]
    pub expires: Option<String>,
    /// Any other consent metadata ResourceSpace returns.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A license record from the License manager plugin.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct License {
    /// License record ID.
    #[serde(rename = "ref")]
    pub license_id: u32,
    /// Whether this is an outbound license.
    pub outbound: u8,
    /// License holder.
    #[serde(deserialize_with = "empty_as_none")]
    pub holder: Option<String>,
    /// License usage description.
    #[serde(deserialize_with = "empty_as_none")]
    pub license_usage: Option<String>,
    /// License description.
    #[serde(deserialize_with = "empty_as_none")]
    pub description: Option<String>,
    /// Expiry date, when set.
    #[serde(deserialize_with = "empty_as_none")]
    pub expires: Option<String>,
    /// Any other license metadata ResourceSpace returns.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl<'a> PluginApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Return all Consent manager consent data for a given resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerGetConsentsRequest`]
    ///
    /// ## Returns
    ///
    /// Consent records linked to the resource.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] when the Consent manager plugin is
    /// unavailable or ResourceSpace rejects the request.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerGetConsentsRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consents = client
    ///     .plugin()
    ///     .consentmanager_get_consents(ConsentManagerGetConsentsRequest::new(123))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_get_consents(
        &self,
        request: ConsentManagerGetConsentsRequest,
    ) -> Result<Vec<Consent>, Error> {
        self.client
            .send_request("consentmanager_get_consents", HttpMethod::Get, request)
            .await
    }

    /// Return all License manager licenses held for a given resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`LicenseManagerGetLicensesRequest`]
    ///
    /// ## Returns
    ///
    /// License records linked to the resource.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] when the License manager plugin is
    /// unavailable or ResourceSpace rejects the request.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::LicenseManagerGetLicensesRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let licenses = client
    ///     .plugin()
    ///     .licensemanager_get_licenses(LicenseManagerGetLicensesRequest::new(123))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn licensemanager_get_licenses(
        &self,
        request: LicenseManagerGetLicensesRequest,
    ) -> Result<Vec<License>, Error> {
        self.client
            .send_request("licensemanager_get_licenses", HttpMethod::Get, request)
            .await
    }

    /// Return all data for a given Consent manager consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerGetConsentRequest`]
    ///
    /// ## Returns
    ///
    /// The consent record.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the consent does not exist, the
    /// Consent manager plugin is unavailable, or ResourceSpace rejects the
    /// request.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerGetConsentRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consent = client
    ///     .plugin()
    ///     .consentmanager_get_consent(ConsentManagerGetConsentRequest::new(42))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_get_consent(
        &self,
        request: ConsentManagerGetConsentRequest,
    ) -> Result<Consent, Error> {
        self.client
            .send_request("consentmanager_get_consent", HttpMethod::Get, request)
            .await
    }

    /// Delete a Consent manager consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerDeleteConsentRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the consent does not exist, the
    /// caller cannot manage consent records, or the Consent manager plugin is
    /// unavailable.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerDeleteConsentRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_delete_consent(ConsentManagerDeleteConsentRequest::new(42))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_delete_consent(
        &self,
        request: ConsentManagerDeleteConsentRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_delete_consent", HttpMethod::Get, request)
            .await
    }

    /// Link or unlink all resources in a collection with a consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerBatchLinkUnlinkRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the consent or collection does
    /// not exist, the caller cannot manage consent records, or the Consent
    /// manager plugin is unavailable.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerBatchLinkUnlinkRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // Link consent 42 to every resource in collection 10.
    /// client
    ///     .plugin()
    ///     .consentmanager_batch_link_unlink(ConsentManagerBatchLinkUnlinkRequest::new(42, 10, false))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_batch_link_unlink(
        &self,
        request: ConsentManagerBatchLinkUnlinkRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_batch_link_unlink", HttpMethod::Get, request)
            .await
    }

    /// Link a consent record with a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerLinkConsentRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the consent or resource does not
    /// exist, the caller cannot manage consent records, or the Consent manager
    /// plugin is unavailable.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerLinkConsentRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_link_consent(ConsentManagerLinkConsentRequest::new(42, 123))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_link_consent(
        &self,
        request: ConsentManagerLinkConsentRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_link_consent", HttpMethod::Get, request)
            .await
    }

    /// Unlink a consent record from a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerUnlinkConsentRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the consent or resource does not
    /// exist, the caller cannot manage consent records, or the Consent manager
    /// plugin is unavailable.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerUnlinkConsentRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_unlink_consent(ConsentManagerUnlinkConsentRequest::new(42, 123))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_unlink_consent(
        &self,
        request: ConsentManagerUnlinkConsentRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_unlink_consent", HttpMethod::Get, request)
            .await
    }

    /// Create a new consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerCreateConsentRequest`]
    ///
    /// ## Returns
    ///
    /// The new consent record's ID.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller cannot manage consent
    /// records, the Consent manager plugin is unavailable, or ResourceSpace
    /// rejects the submitted values.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerCreateConsentRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consent_id = client
    ///     .plugin()
    ///     .consentmanager_create_consent(
    ///         ConsentManagerCreateConsentRequest::new("Joe Smith", "joe@example.com", "", "Website")
    ///             .expires("2027-01-01"),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_create_consent(
        &self,
        request: ConsentManagerCreateConsentRequest,
    ) -> Result<u32, Error> {
        self.client
            .send_request("consentmanager_create_consent", HttpMethod::Get, request)
            .await
    }

    /// Update a consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerUpdateConsentRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the consent does not exist, the
    /// caller cannot manage consent records, the Consent manager plugin is
    /// unavailable, or ResourceSpace rejects the submitted values.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerUpdateConsentRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_update_consent(
    ///         ConsentManagerUpdateConsentRequest::new(42, "Joe Smith", "joe@example.com", "", "Website")
    ///             .notes("Updated by API"),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_update_consent(
        &self,
        request: ConsentManagerUpdateConsentRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_update_consent", HttpMethod::Get, request)
            .await
    }

    /// Fetch all consent records, optionally filtered by search text.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerGetAllConsentsRequest`]
    ///
    /// ## Returns
    ///
    /// Matching consent records.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller cannot view consent
    /// records or the Consent manager plugin is unavailable.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerGetAllConsentsRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consents = client
    ///     .plugin()
    ///     .consentmanager_get_all_consents(ConsentManagerGetAllConsentsRequest::new().findtext("Smith"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_get_all_consents(
        &self,
        request: ConsentManagerGetAllConsentsRequest,
    ) -> Result<Vec<Consent>, Error> {
        self.client
            .send_request("consentmanager_get_all_consents", HttpMethod::Get, request)
            .await
    }

    /// Fetch all consent records linked to resources in a collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerGetAllConsentsByCollectionRequest`]
    ///
    /// ## Returns
    ///
    /// Consent records linked to any resource in the collection.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the collection does not exist,
    /// the caller cannot view consent records, or the Consent manager plugin
    /// is unavailable.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerGetAllConsentsByCollectionRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consents = client
    ///     .plugin()
    ///     .consentmanager_get_all_consents_by_collection(
    ///         ConsentManagerGetAllConsentsByCollectionRequest::new(10),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_get_all_consents_by_collection(
        &self,
        request: ConsentManagerGetAllConsentsByCollectionRequest,
    ) -> Result<Vec<Consent>, Error> {
        self.client
            .send_request(
                "consentmanager_get_all_consents_by_collection",
                HttpMethod::Get,
                request,
            )
            .await
    }

    /// Add a file to a consent record or replace the existing one if present.
    ///
    /// The ResourceSpace KB states `filedata` must be posted rather than placed
    /// in the URL query string. This is not multipart upload; `filedata` is
    /// sent as a regular signed API parameter.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerSaveFileRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the extension is banned, the
    /// caller cannot manage consent records, the consent does not exist, or
    /// the Consent manager plugin is unavailable.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::ConsentManagerSaveFileRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_save_file(ConsentManagerSaveFileRequest::new(42, "consent.txt", b"signed".to_vec()))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_save_file(
        &self,
        request: ConsentManagerSaveFileRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_save_file", HttpMethod::Post, request)
            .await
    }

    /// Update the named-person tag for a detected face using a metadata node.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`FacesSetNodeRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller lacks edit access to
    /// the resource, the face record does not exist, or the Faces plugin is
    /// unavailable.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::FacesSetNodeRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .faces_set_node(FacesSetNodeRequest::new(123, 5, 87))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn faces_set_node(&self, request: FacesSetNodeRequest) -> Result<bool, Error> {
        self.client
            .send_request("faces_set_node", HttpMethod::Get, request)
            .await
    }
}

/// Parameters for [`PluginApi::consentmanager_get_consents`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerGetConsentsRequest {
    /// The resource ID.
    #[serde(rename = "ref")]
    pub resource: u32,
}

impl ConsentManagerGetConsentsRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

/// Parameters for [`PluginApi::licensemanager_get_licenses`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LicenseManagerGetLicensesRequest {
    /// The resource ID.
    pub resource: u32,
}

impl LicenseManagerGetLicensesRequest {
    pub fn new(resource: u32) -> Self {
        Self { resource }
    }
}

/// Parameters for [`PluginApi::consentmanager_get_consent`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerGetConsentRequest {
    /// The consent record ID.
    pub consent: u32,
}

impl ConsentManagerGetConsentRequest {
    pub fn new(consent: u32) -> Self {
        Self { consent }
    }
}

/// Parameters for [`PluginApi::consentmanager_delete_consent`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerDeleteConsentRequest {
    /// The consent record ID.
    #[serde(rename = "ref")]
    pub consent: u32,
}

impl ConsentManagerDeleteConsentRequest {
    pub fn new(consent: u32) -> Self {
        Self { consent }
    }
}

/// Parameters for [`PluginApi::consentmanager_batch_link_unlink`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerBatchLinkUnlinkRequest {
    /// The consent record ID to link or unlink.
    pub consent: u32,
    /// The collection containing resources to process.
    pub collection: u32,
    /// `true` to unlink; `false` to link.
    #[serde(serialize_with = "bool_as_u8")]
    pub unlink: bool,
}

impl ConsentManagerBatchLinkUnlinkRequest {
    pub fn new(consent: u32, collection: u32, unlink: bool) -> Self {
        Self {
            consent,
            collection,
            unlink,
        }
    }
}

/// Parameters for [`PluginApi::consentmanager_link_consent`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerLinkConsentRequest {
    /// The consent record ID.
    pub consent: u32,
    /// The resource ID.
    pub resource: u32,
}

impl ConsentManagerLinkConsentRequest {
    pub fn new(consent: u32, resource: u32) -> Self {
        Self { consent, resource }
    }
}

/// Parameters for [`PluginApi::consentmanager_unlink_consent`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerUnlinkConsentRequest {
    /// The consent record ID.
    pub consent: u32,
    /// The resource ID.
    pub resource: u32,
}

impl ConsentManagerUnlinkConsentRequest {
    pub fn new(consent: u32, resource: u32) -> Self {
        Self { consent, resource }
    }
}

/// Parameters for [`PluginApi::consentmanager_create_consent`].
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerCreateConsentRequest {
    /// The name of the individual giving consent.
    pub name: String,
    /// The email address of the individual.
    pub email: String,
    /// The telephone number of the individual.
    pub telephone: String,
    /// Description of the intended usage for which consent is given.
    pub consent_usage: String,
    /// Any additional notes related to the consent record.
    pub notes: Option<String>,
    /// The expiry date of the consent, formatted as a string.
    pub expires: Option<String>,
}

impl ConsentManagerCreateConsentRequest {
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        telephone: impl Into<String>,
        consent_usage: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            telephone: telephone.into(),
            consent_usage: consent_usage.into(),
            notes: None,
            expires: None,
        }
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }
}

/// Parameters for [`PluginApi::consentmanager_update_consent`].
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerUpdateConsentRequest {
    /// The consent record ID.
    pub consent: u32,
    /// The name of the individual giving consent.
    pub name: String,
    /// The email address of the individual.
    pub email: String,
    /// The telephone number of the individual.
    pub telephone: String,
    /// Description of the intended usage for which consent is given.
    pub consent_usage: String,
    /// Any additional notes related to the consent record.
    pub notes: Option<String>,
    /// The expiry date of the consent, formatted as a string.
    pub expires: Option<String>,
}

impl ConsentManagerUpdateConsentRequest {
    pub fn new(
        consent: u32,
        name: impl Into<String>,
        email: impl Into<String>,
        telephone: impl Into<String>,
        consent_usage: impl Into<String>,
    ) -> Self {
        Self {
            consent,
            name: name.into(),
            email: email.into(),
            telephone: telephone.into(),
            consent_usage: consent_usage.into(),
            notes: None,
            expires: None,
        }
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }
}

/// Parameters for [`PluginApi::consentmanager_get_all_consents`].
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ConsentManagerGetAllConsentsRequest {
    /// Optional search text to filter by the name of the person giving consent.
    pub findtext: Option<String>,
}

impl ConsentManagerGetAllConsentsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn findtext(mut self, findtext: impl Into<String>) -> Self {
        self.findtext = Some(findtext.into());
        self
    }
}

/// Parameters for [`PluginApi::consentmanager_get_all_consents_by_collection`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerGetAllConsentsByCollectionRequest {
    /// The collection ID.
    pub collection: u32,
}

impl ConsentManagerGetAllConsentsByCollectionRequest {
    pub fn new(collection: u32) -> Self {
        Self { collection }
    }
}

/// Parameters for [`PluginApi::consentmanager_save_file`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsentManagerSaveFileRequest {
    /// The consent record ID.
    pub consent: u32,
    /// The filename of the file.
    pub filename: String,
    /// The binary contents of the file to upload.
    pub filedata: Vec<u8>,
}

impl ConsentManagerSaveFileRequest {
    pub fn new(consent: u32, filename: impl Into<String>, filedata: impl Into<Vec<u8>>) -> Self {
        Self {
            consent,
            filename: filename.into(),
            filedata: filedata.into(),
        }
    }
}

/// Parameters for [`PluginApi::faces_set_node`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FacesSetNodeRequest {
    /// The resource ID to update.
    pub resource: u32,
    /// The unique face reference ID from `resource_face.ref`.
    pub face: u32,
    /// The metadata node ID to assign to the face.
    pub node: u32,
}

impl FacesSetNodeRequest {
    pub fn new(resource: u32, face: u32, node: u32) -> Self {
        Self {
            resource,
            face,
            node,
        }
    }
}
