use serde::Serialize;
use serde_with::skip_serializing_none;
use validator::Validate;

use crate::client::{Client, HttpMethod};
use crate::error::Error;

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
    /// `None`
    ///
    /// ## Returns
    /// Returns back system status information (configuration dependant - e.g mysql_log_transactions).
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let status = client.system().get_system_status().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_system_status(&self) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_system_status", HttpMethod::Get, ())
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
    /// Returns an array of daily statistics.
    ///
    /// ## TODO: Errors
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
    ) -> Result<serde_json::Value, Error> {
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
    pub async fn get_reports(&self) -> Result<serde_json::Value, Error> {
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
    /// The report rows.
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
    pub async fn do_report(&self, request: DoReportRequest) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("do_report", HttpMethod::Get, request)
            .await
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
