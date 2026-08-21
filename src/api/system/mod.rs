use std::collections::HashMap;

use validator::Validate;

use crate::client::{Client, HttpMethod};
use crate::error::Error;

mod request;
mod response;
mod shared;

pub use request::{DoReportRequest, GetDailyStatSummaryRequest, GetSystemStatusRequest};
pub use response::{DailyStat, Report, SystemCheck, SystemStatus};

/// Sub-API for system endpoints.
#[derive(Debug)]
pub struct SystemApi<'a> {
    client: &'a Client,
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
