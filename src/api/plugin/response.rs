use std::collections::HashMap;

use serde::Deserialize;

use crate::api::shared::empty_as_none;

/// A consent record from the Consent manager plugin.
///
/// `ResourceSpace`'s Consent manager fields vary by version and configuration,
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
    /// Any other consent metadata `ResourceSpace` returns.
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
    /// Any other license metadata `ResourceSpace` returns.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
