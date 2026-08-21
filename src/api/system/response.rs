use std::collections::HashMap;

use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use super::shared::scalar_as_string;
use crate::api::shared::empty_as_none;

// Referenced only from doc links below; the imports keep them resolvable.
#[allow(unused_imports)]
use super::{DoReportRequest, GetSystemStatusRequest, SystemApi};

/// The outcome of [`SystemApi::get_system_status`].
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct SystemStatus {
    /// `"OK"` if every check passed, `"FAIL"` otherwise.
    pub status: String,
    /// One entry per healthcheck, keyed by check name. Empty in
    /// [`GetSystemStatusRequest::basic`] mode, and when the database
    /// connectivity check itself fails only `database_connection` is present.
    pub results: HashMap<String, SystemCheck>,
}

/// A single healthcheck within [`SystemStatus::results`].
///
/// Only [`Self::status`] is common to every check. The rest of each check's
/// payload varies by check and by plugin, so anything not named here is
/// collected into [`Self::extra`] rather than being dropped — on a stock
/// v11 instance that includes `total` (an integer for
/// `download_bandwidth_last_30_days_gb`, an array of objects for
/// `files_by_extension`), `active`, `non_ingested`, `total_approved` and
/// `within_year`.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct SystemCheck {
    /// `"OK"` or `"FAIL"`.
    pub status: String,
    /// Human-readable detail about the result.
    ///
    /// `ResourceSpace` sends this as a bare number for some checks
    /// (`recent_user_count`) and a string for others, so numbers are
    /// stringified here rather than exposing the inconsistency.
    #[serde(deserialize_with = "scalar_as_string")]
    pub info: Option<String>,
    /// `0` critical, `1` warning, `2` notice. Absent for checks that pass and
    /// for plugin checks that omit it.
    #[serde(deserialize_with = "empty_as_none")]
    pub severity: Option<u8>,
    /// Localised text for [`Self::severity`].
    pub severity_text: Option<String>,
    /// Any other keys this particular check reported. See the type-level docs
    /// for what a stock instance puts here.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// One report available to the current user, from
/// [`SystemApi::get_reports`].
#[serde_as]
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Report {
    /// The report's ID, for [`DoReportRequest::new`].
    ///
    /// Arrives as a quoted string over the wire and is parsed here.
    #[serde(rename = "ref")]
    #[serde_as(as = "DisplayFromStr")]
    pub report_id: u32,
    /// Display name of the report.
    pub name: String,
}

/// One day's activity total, from
/// [`SystemApi::get_daily_stat_summary`].
#[serde_as]
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct DailyStat {
    /// The kind of activity counted, e.g. `"Create resource"`.
    pub activity_type: String,
    /// How many times it occurred over the requested window.
    ///
    /// Arrives as a quoted string over the wire and is parsed here.
    #[serde_as(as = "DisplayFromStr")]
    pub count: u64,
}
