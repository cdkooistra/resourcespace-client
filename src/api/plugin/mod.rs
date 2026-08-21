use serde::Serialize;

use crate::client::{Client, HttpMethod};
use crate::error::Error;

pub mod request;
pub mod response;

use request::{
    ConsentManagerBatchLinkUnlink, ConsentManagerCreateConsent, ConsentManagerDeleteConsent,
    ConsentManagerGetAllConsents, ConsentManagerGetAllConsentsByCollection,
    ConsentManagerGetConsent, ConsentManagerGetConsents, ConsentManagerLinkConsent,
    ConsentManagerSaveFile, ConsentManagerUnlinkConsent, ConsentManagerUpdateConsent, FacesSetNode,
    LicenseManagerGetLicenses,
};
use response::{Consent, License};

/// Sub-API for plugin endpoints.
#[derive(Debug)]
pub struct PluginApi<'a> {
    client: &'a Client,
}

impl<'a> PluginApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Run a custom plugin endpoint.
    ///
    /// Use this for plugin functions that are not (yet) modelled by this crate.
    ///
    /// ## Arguments
    /// * `function` - `ResourceSpace` API function name.
    /// * `method` - HTTP method to use.
    /// * `request` - Serializable request parameters.
    ///
    /// ## Returns
    ///
    /// The raw JSON value returned by `ResourceSpace`.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Client`] when the HTTP method is invalid,
    /// [`Error::OperationFailed`] when `ResourceSpace` returns `false`,
    /// or [`Error::Deserialize`] if the response cannot be converted
    /// to JSON.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use serde::Serialize;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Clone, Serialize)]
    /// struct Params { resource: u32 }
    /// let value = client.plugin().custom("my_plugin_function", "GET", Params { resource: 123 }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn custom<T>(
        &self,
        function: impl Into<String>,
        method: impl Into<String>,
        request: T,
    ) -> Result<serde_json::Value, Error>
    where
        T: Serialize + Clone,
    {
        let method = method.into();
        let method = match method.to_ascii_uppercase().as_str() {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            _ => {
                return Err(Error::Client(
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!(
                            "ResourceSpace only accepts GET and POST methods: {method} is invalid",
                        ),
                    )
                    .into(),
                ));
            }
        };

        self.client
            .send_request(function.into().as_str(), method, request)
            .await
    }

    /// Return all Consent manager consent data for a given resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerGetConsents`]
    ///
    /// ## Returns
    ///
    /// Consent records linked to the resource.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] when the Consent manager plugin is
    /// unavailable or `ResourceSpace` rejects the request.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerGetConsents};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consents = client
    ///     .plugin()
    ///     .consentmanager_get_consents(ConsentManagerGetConsents::new(123))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_get_consents(
        &self,
        request: ConsentManagerGetConsents,
    ) -> Result<Vec<Consent>, Error> {
        self.client
            .send_request("consentmanager_get_consents", HttpMethod::Get, request)
            .await
    }

    /// Return all License manager licenses held for a given resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`LicenseManagerGetLicenses`]
    ///
    /// ## Returns
    ///
    /// License records linked to the resource.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] when the License manager plugin is
    /// unavailable or `ResourceSpace` rejects the request.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::request::LicenseManagerGetLicenses};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let licenses = client
    ///     .plugin()
    ///     .licensemanager_get_licenses(LicenseManagerGetLicenses::new(123))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn licensemanager_get_licenses(
        &self,
        request: LicenseManagerGetLicenses,
    ) -> Result<Vec<License>, Error> {
        self.client
            .send_request("licensemanager_get_licenses", HttpMethod::Get, request)
            .await
    }

    /// Return all data for a given Consent manager consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerGetConsent`]
    ///
    /// ## Returns
    ///
    /// The consent record.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the consent does not exist, the
    /// Consent manager plugin is unavailable, or `ResourceSpace` rejects the
    /// request.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerGetConsent};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consent = client
    ///     .plugin()
    ///     .consentmanager_get_consent(ConsentManagerGetConsent::new(42))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_get_consent(
        &self,
        request: ConsentManagerGetConsent,
    ) -> Result<Consent, Error> {
        self.client
            .send_request("consentmanager_get_consent", HttpMethod::Get, request)
            .await
    }

    /// Delete a Consent manager consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerDeleteConsent`]
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
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerDeleteConsent};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_delete_consent(ConsentManagerDeleteConsent::new(42))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_delete_consent(
        &self,
        request: ConsentManagerDeleteConsent,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_delete_consent", HttpMethod::Get, request)
            .await
    }

    /// Link or unlink all resources in a collection with a consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerBatchLinkUnlink`]
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
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerBatchLinkUnlink};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // Link consent 42 to every resource in collection 10.
    /// client
    ///     .plugin()
    ///     .consentmanager_batch_link_unlink(ConsentManagerBatchLinkUnlink::new(42, 10, false))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_batch_link_unlink(
        &self,
        request: ConsentManagerBatchLinkUnlink,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_batch_link_unlink", HttpMethod::Get, request)
            .await
    }

    /// Link a consent record with a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerLinkConsent`]
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
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerLinkConsent};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_link_consent(ConsentManagerLinkConsent::new(42, 123))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_link_consent(
        &self,
        request: ConsentManagerLinkConsent,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_link_consent", HttpMethod::Get, request)
            .await
    }

    /// Unlink a consent record from a resource.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerUnlinkConsent`]
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
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerUnlinkConsent};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_unlink_consent(ConsentManagerUnlinkConsent::new(42, 123))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_unlink_consent(
        &self,
        request: ConsentManagerUnlinkConsent,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_unlink_consent", HttpMethod::Get, request)
            .await
    }

    /// Create a new consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerCreateConsent`]
    ///
    /// ## Returns
    ///
    /// The new consent record's ID.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller cannot manage consent
    /// records, the Consent manager plugin is unavailable, or `ResourceSpace`
    /// rejects the submitted values.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerCreateConsent};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consent_id = client
    ///     .plugin()
    ///     .consentmanager_create_consent(
    ///         ConsentManagerCreateConsent::new("Joe Smith", "joe@example.com", "", "Website")
    ///             .expires("2027-01-01"),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_create_consent(
        &self,
        request: ConsentManagerCreateConsent,
    ) -> Result<u32, Error> {
        self.client
            .send_request("consentmanager_create_consent", HttpMethod::Get, request)
            .await
    }

    /// Update a consent record.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerUpdateConsent`]
    ///
    /// ## Returns
    ///
    /// Always `true`; a failure arrives as [`Error::OperationFailed`].
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the consent does not exist, the
    /// caller cannot manage consent records, the Consent manager plugin is
    /// unavailable, or `ResourceSpace` rejects the submitted values.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerUpdateConsent};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_update_consent(
    ///         ConsentManagerUpdateConsent::new(42, "Joe Smith", "joe@example.com", "", "Website")
    ///             .notes("Updated by API"),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_update_consent(
        &self,
        request: ConsentManagerUpdateConsent,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_update_consent", HttpMethod::Get, request)
            .await
    }

    /// Fetch all consent records, optionally filtered by search text.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerGetAllConsents`]
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
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerGetAllConsents};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consents = client
    ///     .plugin()
    ///     .consentmanager_get_all_consents(ConsentManagerGetAllConsents::new().findtext("Smith"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_get_all_consents(
        &self,
        request: ConsentManagerGetAllConsents,
    ) -> Result<Vec<Consent>, Error> {
        self.client
            .send_request("consentmanager_get_all_consents", HttpMethod::Get, request)
            .await
    }

    /// Fetch all consent records linked to resources in a collection.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerGetAllConsentsByCollection`]
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
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerGetAllConsentsByCollection};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let consents = client
    ///     .plugin()
    ///     .consentmanager_get_all_consents_by_collection(
    ///         ConsentManagerGetAllConsentsByCollection::new(10),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_get_all_consents_by_collection(
        &self,
        request: ConsentManagerGetAllConsentsByCollection,
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
    /// The `ResourceSpace` KB states `filedata` must be posted rather than placed
    /// in the URL query string. This is not multipart upload; `filedata` is
    /// sent as a regular signed API parameter.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`ConsentManagerSaveFile`]
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
    /// # use resourcespace_client::{Client, api::plugin::request::ConsentManagerSaveFile};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .consentmanager_save_file(ConsentManagerSaveFile::new(42, "consent.txt", b"signed".to_vec()))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn consentmanager_save_file(
        &self,
        request: ConsentManagerSaveFile,
    ) -> Result<bool, Error> {
        self.client
            .send_request("consentmanager_save_file", HttpMethod::Post, request)
            .await
    }

    /// Update the named-person tag for a detected face using a metadata node.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`FacesSetNode`]
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
    /// # use resourcespace_client::{Client, api::plugin::request::FacesSetNode};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .plugin()
    ///     .faces_set_node(FacesSetNode::new(123, 5, 87))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn faces_set_node(&self, request: FacesSetNode) -> Result<bool, Error> {
        self.client
            .send_request("faces_set_node", HttpMethod::Get, request)
            .await
    }
}
