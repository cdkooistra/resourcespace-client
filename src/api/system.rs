use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as, skip_serializing_none};
use validator::Validate;

use super::{empty_as_none, opt_bool_as_u8};
use crate::client::{Client, HttpMethod};
use crate::error::Error;

/// Sub-API for system endpoints.
#[derive(Debug)]
pub struct SystemApi<'a> {
    client: &'a Client,
}

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
    /// ResourceSpace sends this as a bare number for some checks
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

/// Deserializes a string or a bare number alike into `Option<String>`.
///
/// `SystemCheck::info` is the only place this is needed: ResourceSpace sends
/// it as a number for `recent_user_count` and a string everywhere else, and
/// no `serde_with` combinator covers "any scalar to String".
fn scalar_as_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(
        match Option::<serde_json::Value>::deserialize(deserializer)? {
            Some(serde_json::Value::String(s)) => Some(s),
            Some(serde_json::Value::Number(n)) => Some(n.to_string()),
            Some(serde_json::Value::Bool(b)) => Some(b.to_string()),
            _ => None,
        },
    )
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

impl<'a> SystemApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get system status - healthcheck information.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetSystemStatusRequest`]
    ///
    /// ## Returns
    ///
    /// [`SystemStatus::status`] is `"OK"` or `"FAIL"`, alongside one
    /// [`SystemCheck`] per healthcheck performed. Which checks run is
    /// configuration dependent (e.g. `mysql_log_transactions`).
    ///
    /// In [`GetSystemStatusRequest::basic`] mode ResourceSpace returns early
    /// after testing database connectivity only, so
    /// [`SystemStatus::results`] is empty — that is not the same as all
    /// checks having passed.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Deserialize`] if the response does not match
    /// [`SystemStatus`]. A failing healthcheck is reported in the returned
    /// value, not as an error.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::system::GetSystemStatusRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let status = client
    ///     .system()
    ///     .get_system_status(GetSystemStatusRequest::new())
    ///     .await?;
    /// for (name, check) in &status.results {
    ///     println!("{name}: {}", check.status);
    /// }
    ///
    /// // Rapid database-connectivity check only.
    /// let basic = client
    ///     .system()
    ///     .get_system_status(GetSystemStatusRequest::new().basic(true))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_system_status(
        &self,
        request: GetSystemStatusRequest,
    ) -> Result<SystemStatus, Error> {
        self.client
            .send_request("get_system_status", HttpMethod::Get, request)
            .await
    }

    /// Return a summary of daily statistics by activity type.
    ///
    /// Note max 365 days as only the current and previous year's data is accessed.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetDailyStatSummaryRequest`]
    ///
    /// ## Returns
    ///
    /// One [`DailyStat`] per activity type seen in the window. Activity types
    /// with no activity are absent rather than reported as zero.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Validation`] if `days` is outside 1–365, before any
    /// request is sent.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::system::GetDailyStatSummaryRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // Default — last 30 days
    /// let stats = client.system()
    ///     .get_daily_stat_summary(GetDailyStatSummaryRequest::new())
    ///     .await?;
    ///
    /// // Last 7 days
    /// let stats = client.system()
    ///     .get_daily_stat_summary(GetDailyStatSummaryRequest::new().days(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_daily_stat_summary(
        &self,
        request: GetDailyStatSummaryRequest,
    ) -> Result<Vec<DailyStat>, Error> {
        request
            .validate()
            .map_err(|e| Error::Validation(e.into()))?;
        self.client
            .send_request("get_daily_stat_summary", HttpMethod::Get, request)
            .await
    }

    /// List the reports available to the current user.
    ///
    /// ## Arguments
    /// `None`
    ///
    /// ## Returns
    ///
    /// The name and ID of each report. Reports that run against search results
    /// are internal-only and are omitted by ResourceSpace.
    ///
    /// ## Errors
    ///
    /// Returns an empty list rather than an error when the user lacks the `t`
    /// permission.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let reports = client.system().get_reports().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_reports(&self) -> Result<Vec<Report>, Error> {
        self.client
            .send_request("get_reports", HttpMethod::Get, ())
            .await
    }

    /// Run a report over a date range.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`DoReportRequest`]
    ///
    /// ## Returns
    ///
    /// One map per row, keyed by column name. Columns are defined by the
    /// report itself and differ completely between reports — "Most used
    /// keywords" returns `Count`/`Value` pairs, "Database statistics" returns
    /// a single row of named totals — so there is no fixed row type.
    ///
    /// Values are strings even when numeric (`"41"`), and `None` for a SQL
    /// `NULL`. A report matching nothing returns an empty list.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the user lacks the `t`
    /// permission, or if either date is not exactly `YYYY-MM-DD` —
    /// ResourceSpace validates the format itself and refuses anything else.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # use resourcespace_client::api::system::DoReportRequest;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let rows = client
    ///     .system()
    ///     .do_report(DoReportRequest::new(1).from_date("2026-01-01"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn do_report(
        &self,
        request: DoReportRequest,
    ) -> Result<Vec<HashMap<String, Option<String>>>, Error> {
        self.client
            .send_request("do_report", HttpMethod::Get, request)
            .await
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GetSystemStatusRequest {
    /// If true, ResourceSpace checks database connectivity only and returns
    /// early — [`SystemStatus::results`] will be empty.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub basic: Option<bool>,
}

impl GetSystemStatusRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn basic(mut self, basic: bool) -> Self {
        self.basic = Some(basic);
        self
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DoReportRequest {
    /// The ID of the report to run, as returned by
    /// [`SystemApi::get_reports`].
    pub report_ref: u32,
    /// Start of the range, as `YYYY-MM-DD`. Defaults to seven days ago when
    /// omitted.
    pub from_date: Option<String>,
    /// End of the range, as `YYYY-MM-DD`. Defaults to today when omitted.
    pub to_date: Option<String>,
}

impl DoReportRequest {
    pub fn new(report_ref: u32) -> Self {
        Self {
            report_ref,
            from_date: None,
            to_date: None,
        }
    }

    pub fn from_date(mut self, from_date: impl Into<String>) -> Self {
        self.from_date = Some(from_date.into());
        self
    }

    pub fn to_date(mut self, to_date: impl Into<String>) -> Self {
        self.to_date = Some(to_date.into());
        self
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Validate)]
pub struct GetDailyStatSummaryRequest {
    /// Number of past days to include in the summary (1–365). Defaults to 30 when omitted.
    #[validate(range(min = 1, max = 365))]
    pub days: Option<u16>,
}

impl GetDailyStatSummaryRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn days(mut self, days: u16) -> Self {
        self.days = Some(days);
        self
    }
}
