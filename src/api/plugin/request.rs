use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::api::shared::bool_as_u8;

// Referenced only from doc links below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::PluginApi;

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
